use rust_prac::{caesar_cipher::caesar_cipher, pi::monte_carlo_pi};

fn main() {
    let c = caesar_cipher("tomiwa", 4);

    let pi = monte_carlo_pi(10000000000000, 20);

    println!("PI {pi}");
    println!("Hello, world!{c}");
}
