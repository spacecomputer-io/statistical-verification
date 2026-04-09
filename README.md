# SpaceComputer | statistical-verification

![spacecomputer logo](https://raw.githubusercontent.com/spacecomputer-io/media-kit/refs/heads/main/SpaceComputer/logo/SpaceComputer_banner.png)

![Tests](https://github.com/spacecomputer-io/statistical-verification/actions/workflows/rust.yml/badge.svg?branch=main)

This repository contains the statistical-verification project by SpaceComputer.

# statistical-verification

A Rust library for statistical testing of random number generators. Test any `RngCore` implementation with TestU01 (SmallCrush, Crush, BigCrush) or PractRand test suites.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
statistical-verification = { git = "https://github.com/spacecomputer-io/statistical-verification.git" }
```

TestU01 and PractRand are automatically downloaded and built during compilation.

## Usage

### TestU01

```rust
use rng_statistical_tests::testu01::*;

register_rng(my_rng);
unsafe {
    let gen = make_unif01_gen("my-rng");
    bbattery_SmallCrush(gen);
    delete_unif01_gen(gen);
}
```

### PractRand

```rust
use rng_statistical_tests::practrand::{run_test, Config};

let result = run_test(my_rng, Config {
    test_size_kb: Some(64 * 1024),
    ..Default::default()
})?;

let result = run_test(my_rng, Config {
    test_size_kb: None, 
    ..Default::default()
})?;

println!("Passed: {}", result.passed);
```

## Examples

```bash
cargo run --example chacha_testu01      # TestU01
cargo run --example chacha_practrand    # PractRand
```

## Configuration

```rust
use rng_statistical_tests::practrand::{Config, run_test};

let result = run_test(my_rng, Config {
    test_size_kb: Some(1024 * 1024), // 1GB, or None for infinite test
    tf: 1, // -tf FOLDING: 0 = raw data only; 1 = raw + a simple transform emphasizing low bits; 2 = wider variety of transforms
    te: 0, // -te EXPANDED: 0 = normal/core tests (optimized for sensitivity per time); 1 = expanded set (optimized per bit); 10 = special Birthday Spacings mode
    multithreading: false, // false = -singlethreaded (default); true = pass -multithreaded
})?;

let result = run_test(my_rng, Config {
    test_size_kb: None, // None = infinite test (runs until failure or manual stop)
    ..Default::default()
})?;
```


## Requirements

- Rust toolchain
- C/C++ compiler (gcc/g++ or clang/clang++)
- `make`, `curl`, `tar`, `unzip`
