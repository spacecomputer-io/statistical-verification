use rand_chacha::ChaCha20Rng;
use rand_core::SeedableRng;
use statistical_verification::practrand::{run_test, Config};

fn main() {
    let seed = [0u8; 32];
    let rng = ChaCha20Rng::from_seed(seed);

    println!("=== Infinite test ===");
    let result = run_test(
        rng,
        Config {
            test_size_kb: None,
            ..Default::default()
        },
    )
    .expect("Failed to run test");
    println!("Passed: {}", result.passed);
}
