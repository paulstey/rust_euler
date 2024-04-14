use primal::Sieve;
use rayon::prelude::*;
use time::Instant;

#[derive(Debug)]
struct Prime {
    value: u64,
    digits: Vec<u64>,
}

impl Prime {
    fn from(num: u64) -> Self {
        let mut nums_vec: Vec<u64> = Vec::new();
        let mut num_copy = num;

        while num_copy > 0 {
            let remainder = num_copy % 10;

            num_copy = num_copy / 10;

            nums_vec.push(remainder as u64);
        }
        nums_vec.reverse();

        Self {
            value: num,
            digits: nums_vec,
        }
    }

    fn contains(&self, d: u64) -> i32 {
        self.digits.iter().filter(|&digit| *digit == d).count() as i32
    }
}

fn m(_n: i32, d: u64, primes_vec: &[Prime]) -> i32 {
    let mut max_repeats = 0;

    for _num_repeats in [9, 8] {
        for prime in primes_vec.iter() {
            let repeats = prime.contains(d);

            if repeats > max_repeats {
                max_repeats = repeats;
            }
        }
    }

    max_repeats
}

fn s(max_repeats: i32, d: u64, primes_vec: &[Prime]) -> u64 {
    primes_vec
        .par_iter()
        .filter(|prime| prime.contains(d) == max_repeats)
        .map(|prime| prime.value)
        .sum::<u64>() as u64
}

fn max_repeated_digits(n: i32, primes_vec: &[Prime]) -> Vec<i32> {
    let mut max_repeats_vec = [0; 10].to_vec();

    for d in 0..10 {
        max_repeats_vec[d] = m(n, d as u64, primes_vec);
    }

    max_repeats_vec
}

fn sum_qualifying_primes(n: i32, primes_vec: &[Prime]) -> u64 {
    // NOTE: This is column `M(4, d)` in the intructions
    let m_vec = max_repeated_digits(n, &primes_vec);

    let sum_primes: u64 = (0..10)
        .into_par_iter()
        .map(|i| s(m_vec[i], i as u64, primes_vec))
        .sum();

    sum_primes
}

fn main() {
    let t1 = Instant::now();

    let primes_sieve = Sieve::new(10_000_000_000);

    let primes_vec: Vec<_> = primes_sieve
        .primes_from(2)
        .map(|num| num as u64)
        .filter(|&num| 999_999_999 < num && num < 10_000_000_000)
        .map(|num| Prime::from(num))
        .collect();

    let solution = sum_qualifying_primes(10, &primes_vec);

    let t2 = Instant::now();

    println!("{:?}", solution);

    println!("{:?}", t2 - t1);
}
