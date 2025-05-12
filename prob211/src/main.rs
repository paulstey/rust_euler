use rayon::prelude::*;
use std::collections::HashSet;
use std::time::Instant;

const N_MAX: u64 = 64_000_000;

fn divisors(num: u64) -> Vec<u64> {
    let n_max = (num as f64).sqrt() as u64;

    let mut divisors_vec: Vec<_> = (1..=n_max).filter(|&n| num % n == 0).collect();

    let mut divisor_compliments = Vec::new();

    for d in &divisors_vec {
        let complement = num / d;

        divisor_compliments.push(complement);
    }

    divisors_vec.append(&mut divisor_compliments);

    divisors_vec.push(num);

    let unique_divisors: Vec<u64> = divisors_vec
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    unique_divisors
}

fn is_perfect_square(n: &u64) -> bool {
    let root = (*n as f64).sqrt() as u64;

    root * root == *n
}

fn sigma_2(n: u64) -> u64 {
    let divisors_vec = divisors(n);

    divisors_vec.iter().map(|&d| d * d).sum()
}

fn main() {
    let t1 = Instant::now();

    let result: u64 = (1..N_MAX)
        .into_par_iter()
        .map(|num| {
            let sigma_2_of_n = sigma_2(num);

            if is_perfect_square(&sigma_2_of_n) {
                num
            } else {
                0
            }
        })
        .sum();

    let t2 = Instant::now();

    println!("Sum of perfect squares: {}", result);

    println!("Elapsed time: {:?}", t2 - t1);
}
