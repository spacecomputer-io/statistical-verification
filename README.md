# testu01-runner

A Rust wrapper for TestU01 statistical test batteries. Test any `RngCore` implementation with SmallCrush, Crush, or BigCrush.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
testu01-runner = { git = "https://github.com/spacecomputer-io/statistical-verification.git" }
```

TestU01 is automatically downloaded and built during compilation - no manual setup required.

## Usage

```rust
use rand_core::RngCore;
use testu01_runner::{bbattery_SmallCrush, register_rng, make_unif01_gen, delete_unif01_gen};

// Create your RNG (any RngCore implementation)
let mut rng = MyRng::new();

// Register it with TestU01
register_rng(rng);

unsafe {
    let generator = make_unif01_gen("my-rng");
    
    // Run test batteries
    bbattery_SmallCrush(generator);
    // bbattery_Crush(generator);
    // bbattery_BigCrush(generator);
    
    delete_unif01_gen(generator);
}
```

## Example

See `examples/chacha_example.rs` for a complete example using ChaCha20:

```bash
cargo run --example chacha_example
```

## Requirements

- Rust toolchain
- C compiler (gcc/clang)
- `make`, autotools, `curl`, `tar` (for building TestU01)
