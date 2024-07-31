// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

fn main() {    
    'outer: for i in (20..1000000000).step_by(20) {
        for d in 1..20 {
            if i % d == 0 {
                if d == 19 {
                    println!("{}", i);
                    break 'outer;
                }
            } else {
                break;
           }
        }
    }
}