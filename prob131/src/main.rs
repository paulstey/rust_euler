use primal::Sieve;
use rayon::prelude::*;
use std::time::Instant;

const P_MAX: u64 = 1_000_000;
const N_MAX: u64 = 1_000_000;

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

fn prime_satisfies_condition(p: u64, eligible_values_of_n: &Vec<u64>) -> bool {
    for &n in eligible_values_of_n {
        let f_of_p = f(n, p);

        if is_perfect_cube(f_of_p) {
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

    let eligible_values_of_n: Vec<u64> = (1..=N_MAX).collect();

    let result: usize = candidate_primes
        .par_iter()
        .filter(|&num| prime_satisfies_condition(*num, &eligible_values_of_n))
        .count();

    let t2 = Instant::now();

    println!("Number of primes that qualify: {}", result);

    println!("Elapsed time: {:?}", t2 - t1);
}
