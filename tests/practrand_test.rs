#![cfg(has_practrand)]

use rand_chacha::ChaCha20Rng;
use rand_core::SeedableRng;
use rng_statistical_tests::practrand::{run_test, Config};

#[test]
fn practrand_smoke_test_chacha20() {
    let rng = ChaCha20Rng::from_seed([0u8; 32]);

    let result = run_test(
        rng,
        Config {
            test_size_kb: Some(1024), // 1 MiB
            tf: 1,
            te: 0,
            multithreading: false,
        },
    )
    .expect("failed to run PractRand RNG_test");

    assert!(result.passed, "PractRand reported failure");
}


