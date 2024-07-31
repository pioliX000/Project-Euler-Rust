// By considering the terms in the Fibonacci sequence whose values do not exceed four million, 
// find the sum of the even-valued terms.

fn main() {
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;

    let mut res = 0;

    while c < 4000000 {
        c = a + b;
        a = b;
        b = c;

        if c % 2 == 0 {
            res += c;
        }
    }

    println!("{}", res);
}