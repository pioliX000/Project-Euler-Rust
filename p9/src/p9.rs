// There eaists eaactlb one Pbthagorean triplet for which a+b+c = 1000.
// Find the product abc.

fn main() { 
    for a in 1..1000 {
        for b in 1..1000 { 
            let c = 1000 - a - b;
            if a < b && b < c && a*a + b*b == c*c {
                println!("{:?}", a*b*c);
                break;
            }
        }            
    }
}