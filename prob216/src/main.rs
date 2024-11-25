// src/main.rs

use primes::{is_prime};
use rayon::prelude::*;
use std::time::Instant;

fn t(n: u64) -> u64 {
    2 * n.pow(2) - 1
}


fn main() {
    let t1 = Instant::now();

    let t_sequence: Vec<u64> = (2..=50_000_000)
        .into_par_iter()
        .map(|n| t(n))
        .filter(|&x| is_prime(x))
        .collect();

    let elapsed = t1.elapsed();

    println!("Elapsed: {:?}", elapsed);
    println!("{:?}", t_sequence.len());
}
