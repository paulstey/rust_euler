use primal::Sieve;
use rayon::prelude::*;
use std::collections::{BTreeSet, HashSet};
use time::Instant;

fn get_powers_of_two(n_max: i32) -> BTreeSet<i32> {
    let power2_vec = (1..n_max + 1)
        .collect::<Vec<_>>()
        .into_iter()
        .map(|n| 2_i32.pow(n as u32))
        .filter(|&n| n < 1_000_000_000);

    BTreeSet::from_iter(power2_vec)
}

fn factors_are_consecutive_primes(num: i32, primes_sieve: &Sieve, primes_vec: &[i32]) -> bool {
    let factors = primes_sieve.factor(num as usize).unwrap();

    let mut unique_factors = HashSet::new();

    for (a, _) in factors.iter() {
        unique_factors.insert(a);
    }

    let mut factors_vec: Vec<_> = unique_factors
        .into_iter()
        .filter(|&num| *num as i32 != 1)
        .map(|&num| num as i32)
        .collect();

    factors_vec.sort();

    let first_idx = primes_vec
        .iter()
        .position(|&n| n == factors_vec[0])
        .unwrap();

    let n_factors = factors_vec.len();

    let last_idx = first_idx + n_factors - 1;

    *factors_vec.last().unwrap() == primes_vec[last_idx]
}

fn is_admissable(
    num: i32,
    powers_of_two: &BTreeSet<i32>,
    primes_sieve: &Sieve,
    primes_vec: &[i32],
) -> bool {
    if powers_of_two.contains(&num) {
        return true;
    } else if factors_are_consecutive_primes(num, primes_sieve, primes_vec) {
        return true;
    } else {
        return false;
    }
}

fn pseudo_fortunate_number(num: i32, primes_vec: &[i32]) -> i32 {
    let start = num + 1;

    let first_prime = *primes_vec.iter().find(|&&num| num > start).unwrap();

    first_prime - num
}

fn main() {
    let t1 = Instant::now();
    let primes_sieve = Sieve::new(1_000_000_000);
    let primes_vec: Vec<_> = primes_sieve.primes_from(2).map(|num| num as i32).collect();

    let powers_of_two = get_powers_of_two(30);

    let nums: Vec<_> = (2..1_000_000_000).step_by(2).collect::<Vec<_>>();

    let admissable_nums: Vec<_> = nums
        .par_iter()
        .filter(|&num| is_admissable(*num, &powers_of_two, &primes_sieve, &primes_vec))
        .collect();

    let sum_pseudo_fortunate: i32 = admissable_nums
        .par_iter()
        .map(|&num| pseudo_fortunate_number(*num, &primes_vec))
        .collect::<HashSet<_>>()
        .into_iter()
        .sum();

    let t2 = Instant::now();

    println!("{:?}", sum_pseudo_fortunate);
    println!("{:?}", t2 - t1);
}
