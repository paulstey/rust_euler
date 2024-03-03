use prob357::primes::primes;
use rayon::prelude::*;
use std::collections::BTreeSet;
use std::f64;
use time::Instant;

fn find_divisors(n: u64) -> Vec<u64> {
    let mut divisors = Vec::new();
    let sqrt_n = (n as f64).sqrt() as u64;

    for i in 1..=sqrt_n {
        if n % i == 0 {
            divisors.push(i);
            if i != n / i {
                divisors.push(n / i);
            }
        }
    }

    divisors
}

fn num_satisfies_condition(n: u64, primes_btree: &BTreeSet<&u64>) -> bool {
    let divisors = find_divisors(n);

    divisors
        .iter()
        .map(|d| d + (((n as f64) / (*d as f64)).round() as u64))
        .all(|res| primes_btree.contains(&res))
}

fn main() {
    let t1 = Instant::now();

    // let primes_vec = primes((100_000_000 as f64).sqrt().round() as u64);
    let primes_vec = primes(100_000_000 as u64);

    let primes_btree = BTreeSet::from_iter(primes_vec.iter());

    let soln: u64 = (1..100_000_000)
        .into_par_iter()
        .filter(|n| num_satisfies_condition(*n, &primes_btree))
        .sum();

    let t2 = Instant::now();

    println!("{}", soln);

    println!("{:?}", t2 - t1);
}
