// What is the value of the first triangle number to have over five hundred divisors?
// n ( n + 1 ) / 2

fn main() { 
    for x in 1..100000 {
        let tri = x*(x+1)/2 as i64;

        if divisor_count(tri) >= 500 {
            println!("{:?}", tri);
            break
        }
    }
}

fn divisor_count(x: i64) -> usize {
    let mut divisors = Vec::new();

    let sqrt_x = (x as f64).sqrt() as i64;

    for i in 1..=sqrt_x {
        if x % i == 0 {
            divisors.push(i);
            if x / i != i {
                divisors.push(x / i);
            }
        }
    }

    divisors.len()
}