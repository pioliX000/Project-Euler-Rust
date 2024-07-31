// Find the sum of all the multiples of 3 or 5 below 1000. 

fn main() {
    let mut res = 0;

    for i in 0..1000 {
        if i % 5 == 0 {
            res += i;
        } else if i % 3 == 0 {
            res += i;
        }
    }

    println!("{}", res);
}