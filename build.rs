use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rustc-check-cfg=cfg(has_testu01_bindings)");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
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
            .expect("Failed to execute make");

        if !make_status.success() {
            panic!("Failed to build TestU01");
        }

        let install_status = Command::new("make")
            .arg("install")
            .current_dir(&testu01_dir)
            .status()
            .expect("Failed to execute make install");

        if !install_status.success() {
            panic!("Failed to install TestU01");
        }

        std::fs::remove_file(&archive_path).ok();
    }

    let lib_dir = testu01_root.join("lib");
    if !lib_dir.exists() {
        panic!("No lib directory found");
    }

    let lib_dir_abs = std::fs::canonicalize(&lib_dir)
        .expect("Failed to canonicalize lib directory");
    println!("cargo:rustc-link-search=native={}", lib_dir_abs.display());
    println!("cargo:rustc-link-lib=static=testu01");
    println!("cargo:rustc-link-lib=static=probdist");
    println!("cargo:rustc-link-lib=static=mylib");

    let include_dir = testu01_root.join("include");
    if !include_dir.exists() {
        panic!("Missing include directory");
    }

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

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Unable to write bindings.rs");

    println!("cargo:rustc-cfg=has_testu01_bindings");
}
