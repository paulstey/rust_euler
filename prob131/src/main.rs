use primal::Sieve;
use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

const P_MAX: u64 = 1_000_000;
const N_MAX: u64 = 5_000_000;

fn f(n: u64, p: u64) -> u64 {
    n.pow(3) + n.pow(2) * p
}

fn is_perfect_cube(n: u64) -> bool {
    if n == 0 {
        return false;
    }

    let cube_root = (n as f64).powf(1.0 / 3.0).round() as i64;

    cube_root * cube_root * cube_root == n.try_into().unwrap()
}

fn prime_satisfies_condition(p: u64, eligible_values_of_n: &mut Vec<u64>) -> bool {
    for &mut n in &mut *eligible_values_of_n {
        let f_of_p = f(n, p);

        if is_perfect_cube(f_of_p) {
            eligible_values_of_n.retain(|&x| x != n);

            return true;
        }
    }
    false
}

fn get_primes_under_pmax() -> Vec<u64> {
    let sieve = Sieve::new(P_MAX as usize);

    sieve
        .primes_from(2)
        .take_while(|&p| p < P_MAX as usize)
        .map(|p| p as u64)
        .collect()
}

fn main() {
    let t1 = Instant::now();

    let candidate_primes = get_primes_under_pmax();

    let mut eligible_values_of_n: Vec<u64> = (1..=N_MAX).collect();

    let counter = AtomicU64::new(0);

    for p in candidate_primes.into_iter() {
        let mut local_count = 0;

        if prime_satisfies_condition(p, &mut eligible_values_of_n) {
            local_count += 1;
        }

        counter.fetch_add(local_count, Ordering::Relaxed);
    }

    let t2 = Instant::now();

    println!("Number of primes that qualify: {:?}", counter);

    println!("Elapsed time: {:?}", t2 - t1);
}
