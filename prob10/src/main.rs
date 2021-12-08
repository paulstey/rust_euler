use primes::{PrimeSet, Sieve};
use time::Instant;

fn main() {
    let t0 = Instant::now();
    let mut pset = Sieve::new();

    let res: u64 = pset.iter().take(300_000).sum();

    println!("{:?}", res);
    println!("{:?}", t0.elapsed());
}
