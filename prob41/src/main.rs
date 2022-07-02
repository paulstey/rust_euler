use primes::{PrimeSet, Sieve};
use time::Instant;

fn main() {
    let t0 = Instant::now();
    let mut pset = Sieve::new();

    let max_n = 9_000_000;
    let prime_vec: Vec<u64> = pset.iter().take(max_n).collect();

    let mut res = 0_u64;
    let mut max_len = 0;

    for prime in prime_vec {
        if prime.to_string().chars().collect() > max_len  {
            max_len = prime.to_string().chars().len(); 
            println!("{:?}", prime); 
        }
    }
    println!("{:?}", res);
    println!("{:?}", t0.elapsed());
}

