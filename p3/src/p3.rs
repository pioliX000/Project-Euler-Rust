// What is the largest prime factor of the number 600851475143?

fn main() {
    let mut num:u64 = 600851475143;
    let mut i = 2; 

    while num > 1 {
        if num % i == 0 {
            num /= i
        } else {
            i += 1
        }
    }

    println!("{}", i);
}