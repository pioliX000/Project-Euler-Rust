// Find the difference between the sum of the squares of 
// the first one hundred natural numbers and the square of the sum.

fn main() {    
    let sum1: u32 = (1..101).map(|x| x*x).sum();
    let sum2: u32 = (1..101).map(|x| x).sum();

    println!("{}", sum2*sum2-sum1);
}