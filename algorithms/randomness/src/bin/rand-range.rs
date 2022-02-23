use rand::distributions::{Distribution, Uniform};
use rand::Rng;

fn gen_random_from_range() {
    let mut rng = rand::thread_rng();

    println!("Integer: {}", rng.gen_range(0..10));
    println!("Float: {}", rng.gen_range(0.0..10.0));
}

fn gen_random_from_range_ud() {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);

    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }
}

fn main() {
    println!("Random number from range");
    gen_random_from_range();
    println!("rolling the die");
    gen_random_from_range_ud();
}
