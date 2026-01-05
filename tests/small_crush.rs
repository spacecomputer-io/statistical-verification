use rand::rngs::OsRng;
use testu01_runner::{bbattery_SmallCrush, register_rng, make_unif01_gen, delete_unif01_gen};

#[test]
fn test_os_rng_smallcrush() {
    let rng = OsRng;
    register_rng(rng);

    unsafe {
        let generator = make_unif01_gen("os-rng");
        bbattery_SmallCrush(generator);
        delete_unif01_gen(generator);
    }
}
