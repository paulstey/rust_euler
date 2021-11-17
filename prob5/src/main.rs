use std::env;
use std::time::Instant; 


fn is_factor(a: i32, n: i32) -> bool {
    n % a == 0
} 

fn main() {
    let args: Vec<String> = env::args().collect();
    let k = args[1].parse::<i32>().unwrap();

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
                
                let runtime = new_now.checked_duration_since(now)
                    .unwrap();

                println!("{:?}", runtime);
                found = true;
                break
            }
        }
        n = n + 1;
    }
}
