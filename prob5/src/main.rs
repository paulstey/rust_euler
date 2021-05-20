use std::env;
use std::u64;

use prob5::primes;



fn main() {
    let args: Vec<String> = env::args().collect();
    let k = args[1].parse::<u64>().unwrap();

    let N = 1;
    let mut i = 1;
    let check = true;
    let limit = k.sqrt();

    let p = primes::primes(100);
    let mut a = vec![0.0_f32; 100 as usize];

    while p[i] <= k {
        a[i] = 1.0;
        if check {
            if p[i] < limit {
                let a_i = k.ln()/(p[i]).ln();
                a[i] = a_i.floor();
            }
            else {
                check = false;
            }
            let N = N * p[i] ^ a[i];
            i += 1;
        }
    }  
    println!("{:?}", N)
}
