use primal::Sieve;
use rayon::prelude::*;
use std::time::Instant;

const P_MAX: u64 = 100;
const N_MAX: u64 = 50;

fn f(n: &mut u64, p: &u64) -> u64 {
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
    for n in eligible_values_of_n {
        let f_of_p = f(&mut n, &p);

        if is_perfect_cube(f_of_p as u64) {
            eligible_values_of_n.retain(|&num| num != *n);

            return true;
        }
    }
    false
}

fn get_primes_under_pmax() -> Vec<usize> {
    let sieve = Sieve::new(P_MAX as usize);

    sieve
        .primes_from(2)
        .take_while(|&p| p < P_MAX as usize)
        .collect()
}

fn main() {
    let t1 = Instant::now();

    let candidate_primes = get_primes_under_pmax();

    let mut eligible_values_of_n: Vec<u64> = (1..=N_MAX).collect();

    let result: u64 = candidate_primes
        .par_iter_mut()
        .filter(|&num| prime_satisfies_condition(*num, &mut eligible_values_of_n))
        .len();

    let t2 = Instant::now();

    println!("Sum of perfect squares: {}", result);

    println!("Elapsed time: {:?}", t2 - t1);
}
