use rand_chacha::ChaCha20Rng;
use rand_core::SeedableRng;
use testu01_runner::{bbattery_BigCrush, bbattery_Crush, bbattery_SmallCrush};
use testu01_runner::{delete_unif01_gen, make_unif01_gen, register_rng};

fn main() {
    let seed = [0u8; 32];
    let rng = ChaCha20Rng::from_seed(seed);

    register_rng(rng);

    unsafe {
        let generator = make_unif01_gen("chacha20");

        println!("=== SmallCrush ===");
        bbattery_SmallCrush(generator);

        println!("=== Crush ===");
        bbattery_Crush(generator);

        println!("=== BigCrush ===");
        bbattery_BigCrush(generator);

        delete_unif01_gen(generator);
    }
}
