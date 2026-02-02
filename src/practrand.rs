use rand_core::RngCore;
use std::io::Write;
use std::process::{Command, Stdio};

#[derive(Debug, Clone)]
pub struct Config {
    pub test_size_kb: Option<u64>,
    pub tf: u8,
    pub te: u8,
    pub multithreading: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            test_size_kb: Some(256 * 1024),
            tf: 1,
            te: 0,
            multithreading: true,
        }
    }
}

impl Config {
    fn to_args(&self) -> Vec<String> {
        let mut args = vec!["stdin".to_string()];
        if self.tf != 1 {
            args.push(format!("-tf{}", self.tf));
        }
        if self.te != 0 {
            args.push(format!("-te{}", self.te));
        }
        if self.multithreading {
            args.push("-multithreaded".to_string());
        }
        args
    }
}

#[derive(Debug)]
pub struct TestResult {
    pub output: String,
    pub passed: bool,
}

pub fn run_test<R: RngCore + Send + 'static>(
    mut rng: R,
    config: Config,
) -> std::io::Result<TestResult> {
    let rng_test_path = env!("PRACTRAND_RNG_TEST");

    let args = config.to_args();

    let mut child = Command::new(rng_test_path)
        .args(&args)
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    let bytes_to_write = config.test_size_kb.map(|kb| kb * 1024);

    let write_thread = std::thread::spawn(move || -> std::io::Result<()> {
        const BUFFER_SIZE: usize = 4096;
        let mut buffer = [0u8; BUFFER_SIZE];
        let mut bytes_written = 0u64;

        loop {
            if let Some(limit) = bytes_to_write {
                if bytes_written >= limit {
                    break;
                }
            }

            let chunk_size = if let Some(limit) = bytes_to_write {
                BUFFER_SIZE.min((limit - bytes_written) as usize)
            } else {
                BUFFER_SIZE
            };

            rng.fill_bytes(&mut buffer[..chunk_size]);
            match stdin.write_all(&buffer[..chunk_size]) {
                Ok(_) => bytes_written += chunk_size as u64,
                Err(_) => break,
            }
        }
        drop(stdin);
        Ok(())
    });

    let _ = write_thread.join();
    let status = child.wait()?;

    let passed = status.success();
    Ok(TestResult {
        output: String::new(),
        passed,
    })
}
