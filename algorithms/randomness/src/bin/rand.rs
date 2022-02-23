use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let number_1: u8 = rng.gen();
    let number_2: u16 = rng.gen();

    println!("Random u8: {}", number_1);
    println!("Random u16: {}", number_2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
}
