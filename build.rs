use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rustc-check-cfg=cfg(has_testu01_bindings)");
    println!("cargo:rustc-check-cfg=cfg(has_practrand)");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    build_testu01(&manifest_dir);
    build_practrand(&manifest_dir);
}

fn build_testu01(manifest_dir: &PathBuf) {
    let testu01_root = manifest_dir.join("vendor/testu01-2009/install");

    if !testu01_root.exists() {
        let vendor_dir = manifest_dir.join("vendor");
        let testu01_dir = vendor_dir.join("testu01-2009");
        let archive_path = vendor_dir.join("testu01.tar.gz");

        std::fs::create_dir_all(&vendor_dir).expect("Failed to create vendor directory");

        if !archive_path.exists() {
            let status = Command::new("curl")
                .arg("-fL")
                .arg("https://github.com/umontreal-simul/TestU01-2009/archive/refs/heads/master.tar.gz")
                .arg("-o")
                .arg(&archive_path)
                .status()
                .expect("Failed to execute curl");

            if !status.success() {
                panic!("Failed to download TestU01");
            }
        }

        let extract_dir = vendor_dir.join("extract");
        if !extract_dir.exists() {
            std::fs::create_dir_all(&extract_dir).expect("Failed to create extract directory");

            let status = Command::new("tar")
                .arg("-xzf")
                .arg(&archive_path)
                .arg("-C")
                .arg(&extract_dir)
                .arg("--strip-components=1")
                .status()
                .expect("Failed to execute tar");

            if !status.success() {
                panic!("Failed to extract TestU01 archive");
            }

            if testu01_dir.exists() {
                std::fs::remove_dir_all(&testu01_dir).ok();
            }

            std::fs::rename(&extract_dir, &testu01_dir)
                .expect("Failed to move extracted TestU01 directory");
        }

        let configure_status = Command::new("./configure")
            .arg(format!("--prefix={}", testu01_root.display()))
            .arg("--disable-shared")
            .current_dir(&testu01_dir)
            .status()
            .expect("Failed to execute configure");

        if !configure_status.success() {
            panic!("Failed to configure TestU01");
        }

        let make_status = Command::new("make")
            .current_dir(&testu01_dir)
            .status()
            .expect("Failed to build TestU01");

        if !make_status.success() {
            panic!("Failed to build TestU01");
        }

        let install_status = Command::new("make")
            .arg("install")
            .current_dir(&testu01_dir)
            .status()
            .expect("Failed to execute install");

        if !install_status.success() {
            panic!("Failed to install TestU01");
        }

        std::fs::remove_file(&archive_path).ok();
    }

    let lib_dir = std::fs::canonicalize(testu01_root.join("lib")).expect("Missing lib directory");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=testu01");
    println!("cargo:rustc-link-lib=static=probdist");
    println!("cargo:rustc-link-lib=static=mylib");

    let include_dir = testu01_root.join("include");
    let include_str = include_dir.to_str().expect("include path not UTF-8");

    let bindings = bindgen::Builder::default()
        .header(format!("{}/unif01.h", include_str))
        .header(format!("{}/bbattery.h", include_str))
        .allowlist_function("unif01_.*")
        .allowlist_function("bbattery_.*")
        .allowlist_type("unif01_.*")
        .clang_arg(format!("-I{}", include_str))
        .generate()
        .expect("Unable to generate TestU01 bindings");

    bindings
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("Unable to write bindings");

    println!("cargo:rustc-cfg=has_testu01_bindings");
}

fn build_practrand(manifest_dir: &PathBuf) {
    let practrand_root = manifest_dir.join("vendor/practrand");
    let rng_test_exe = practrand_root.join("RNG_test");

    if !rng_test_exe.exists() {
        let vendor_dir = manifest_dir.join("vendor");
        let archive_path = vendor_dir.join("practrand.zip");

        std::fs::create_dir_all(&vendor_dir).expect("Failed to create vendor directory");

        if !archive_path.exists() {
            let status = Command::new("curl")
                .arg("-fL")
                .arg("https://sourceforge.net/projects/pracrand/files/latest/download")
                .arg("-o")
                .arg(&archive_path)
                .status()
                .expect("Failed to execute curl");

            if !status.success() {
                panic!("Failed to download PractRand");
            }
        }

        if !practrand_root.exists() {
            std::fs::create_dir_all(&practrand_root).expect("Failed to create practrand directory");

            let status = Command::new("unzip")
                .arg("-q")
                .arg(&archive_path)
                .arg("-d")
                .arg(&practrand_root)
                .status()
                .expect("Failed to execute unzip");

            if !status.success() {
                panic!("Failed to extract PractRand archive");
            }
        }

        let practrand_src = practrand_root
            .join("PractRand")
            .canonicalize()
            .unwrap_or_else(|_| {
                std::fs::read_dir(&practrand_root)
                    .expect("Failed to read practrand directory")
                    .filter_map(Result::ok)
                    .find(|e| e.file_name().to_string_lossy().starts_with("PractRand"))
                    .map(|e| e.path())
                    .expect("PractRand directory not found")
            });

        let include_arg = format!("-I{}/include", practrand_src.display());
        let compile_args = ["-c", "-O3", &include_arg, "-Wno-constant-logical-operand"];

        let compile_cpp = |path: &str| {
            Command::new("sh")
                .arg("-c")
                .arg(format!(
                    "g++ {} {}/{}/*.cpp",
                    compile_args.join(" "),
                    practrand_src.display(),
                    path
                ))
                .current_dir(&practrand_src)
                .status()
                .expect("Failed to compile PractRand sources")
        };

        compile_cpp("src");
        compile_cpp("src/RNGs");
        compile_cpp("src/RNGs/other");

        let status = Command::new("sh")
            .arg("-c")
            .arg("ar rcs libPractRand.a *.o")
            .current_dir(&practrand_src)
            .status()
            .expect("Failed to execute ar");

        if !status.success() {
            panic!("Failed to create PractRand library");
        }

        let status = Command::new("g++")
            .arg("-o")
            .arg(&rng_test_exe)
            .arg("tools/RNG_test.cpp")
            .arg("libPractRand.a")
            .arg("-O3")
            .arg(&include_arg)
            .arg(format!("-I{}/tools", practrand_src.display()))
            .arg("-pthread")
            .current_dir(&practrand_src)
            .status()
            .expect("Failed to build RNG_test");

        if !status.success() {
            panic!("Failed to build RNG_test executable");
        }

        Command::new("sh")
            .arg("-c")
            .arg("rm -f *.o")
            .current_dir(&practrand_src)
            .status()
            .ok();

        std::fs::remove_file(&archive_path).ok();
    }

    println!(
        "cargo:rustc-env=PRACTRAND_RNG_TEST={}",
        rng_test_exe.display()
    );
    println!("cargo:rustc-cfg=has_practrand");
}
