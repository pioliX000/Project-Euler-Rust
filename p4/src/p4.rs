// Find the largest palindrome made from the product of two 3-digit numbers.

fn main() {
    let mut res = 0;
    let mut suckmyass = Vec::new();
    
    for x in (100..999).rev() {
        for y in (100..999).rev() {
            res = x * y;
            if res.to_string().chars().rev().collect::<String>() == res.to_string() {
                suckmyass.push(res);
            }
        }
    }

    println!("{}", suckmyass.iter().max().unwrap());
}