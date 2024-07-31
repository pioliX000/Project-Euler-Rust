// What is the sum of the digits of the number 2^1000

use num_bigint::BigUint;

fn main() {
    let number = BigUint::from(2u32).pow(1000).to_string();
    println!("{:?}", number.chars().map(|x| x as u32 - 48).sum::<u32>());
}