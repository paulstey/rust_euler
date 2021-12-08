use primes::{PrimeSet, Sieve};
use time::Instant;

fn main() {
    let t0 = Instant::now();
    let mut pset = Sieve::new();

    let max_n = 2_000_000; 
    let prime_vec: Vec<u64> = pset.iter().take(max_n).collect();

    let mut res = 0_u64;

    for prime in prime_vec {
        if prime < max_n.try_into().unwrap() {
            res += prime;
        }
    }
    println!("{:?}", res);
    println!("{:?}", t0.elapsed());
}
