#![cfg(has_testu01_bindings)]

use rand::rngs::OsRng;
use statistical_verification::testu01::{
    bbattery_SmallCrush, delete_unif01_gen, make_unif01_gen, register_rng,
};

#[test]
fn os_rng_smallcrush() {
    let rng = OsRng;
    register_rng(rng);

    unsafe {
        let generator = make_unif01_gen("os-rng");
        bbattery_SmallCrush(generator);
        delete_unif01_gen(generator);
    }
}


