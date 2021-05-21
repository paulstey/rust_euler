use std::env;

use prob5::primes;



fn main() {
    let args: Vec<String> = env::args().collect();
    let k = args[1].parse::<i32>().unwrap();

    let mut n = 1.0;
    let mut i = 1;
    let mut check = true;
    let limit: f64 = (k as f64).sqrt();

    let p = primes::primes(100);
    let mut a = vec![0.0_f64; 100 as usize];

    while p[i] <= k {
        println!("{:?}", p[i]);

        
        a[i] = 1.0;
        if check {
            if (p[i] as f64) < limit {
                let a_i = (k as f64).ln()/(p[i] as f64).ln();
                a[i] = a_i.floor();
            }
            else {
                check = false;
            }
            n = n * f64::powf(p[i] as f64, a[i]);
            i += 1;
        }
    }  
    println!("{:?}", n)
}
