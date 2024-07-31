// What is the 10001st prime number?

fn main() {    
    use primes::{Sieve, PrimeSet};

    let mut pset = Sieve::new();
    for (ix, n) in pset.iter().enumerate().skip(10000).take(1) {
        println!("Prime {}: {}", ix+1, n);
    }
}