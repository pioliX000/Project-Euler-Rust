// Find the sum of the digits in the number 100!.

use num_bigint::BigUint;
use num_traits::{One, FromPrimitive};

fn main() {	
	let sum = (1..=100).fold(BigUint::one(), |acc, x| acc * BigUint::from_u32(x).unwrap()).to_string()
		.chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>();

	println!("{}", sum);
}