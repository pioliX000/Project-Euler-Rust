// n -> n/2 (n is even)
// n -> 3n+1 (n is odd)

use std::collections::HashMap;

fn main() {
    let mut nums: HashMap<String, u32> = HashMap::new();

    for mut i in 1..1000000 {      
        let mut count = 0;  
        let x:i64 = i;
        
        while i != 1 {
            if i % 2 == 0 {
                i = i/2;
            } else if i % 2 != 0 {
                i = 3*i+1;
            }

            count += 1  
        }

        nums.insert(x.to_string(), count);
    }

    println!("{:?}", nums.iter().max_by_key(|entry | entry.1).unwrap().0);
}