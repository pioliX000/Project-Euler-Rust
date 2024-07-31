// How many such routes are there through a 20x20 grid
// T = Total number of paths from (0,0) to (n,k) = (n+k)!/(n!k!).

pub fn factorial(num: u8) -> f64 {
    let numbers: Vec<u8> = (1..=num).collect();
    numbers.iter().map(|&x| x as f64).product()
}

fn main() {
    println!("{:?}", format!("{:.0}", factorial(20+20)/(factorial(20)*factorial(20))));
}