use std::env;
use std::time::{Instant}; 


fn is_factor(a: u64, n: u64) -> bool {
    let result = n % a == 0_u64;
    result
} 

fn main() {
    let args: Vec<String> = env::args().collect();
    let k = args[1].parse::<u64>().unwrap();

    let now = Instant::now();

    let mut found = false;
    let mut n = k;

    while !found {
        for i in 2..=k {

            if !is_factor(i, n) {
                break;
            }
            else if is_factor(i, n) && i == k {
                let new_now = Instant::now();
                
                println!("The solution is:  {:?}", n);
                
                println!("{:?}", new_now.checked_duration_since(now));
                found = true;
                break
            }
        }
        n = n + 1;
    }
}
