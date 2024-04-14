use primal::Sieve;
use rayon::prelude::*;
use std::collections::HashSet;
use std::time::Instant;

#[derive(Debug)]
struct Row {
    n: u64,
    rad_n: u64,
}

fn rad(num: u64, primes_sieve: &Sieve) -> u64 {
    let factors = primes_sieve.factor(num as usize).unwrap();

    let mut unique_factors = HashSet::new();

    for (a, _) in factors.iter() {
        unique_factors.insert(a);
    }

    unique_factors
        .into_iter()
        .filter(|&num| *num as u64 != 1)
        .map(|&num| num as u64)
        .product()
}

fn main() {
    let t1 = Instant::now();

    let sieve = Sieve::new(100_000);

    let mut solution: Vec<_> = (1..100_001)
        .into_par_iter()
        .map(|n| Row {
            n: n,
            rad_n: rad(n, &sieve),
        })
        .collect::<Vec<_>>();

    solution.sort_by(|a, b| a.rad_n.cmp(&b.rad_n));

    let t2 = Instant::now();

    println!("{:?}", solution[9999].n);

    println!("{:?}", t2 - t1);
}
