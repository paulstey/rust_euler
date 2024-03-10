// primes.rs



extern crate rayon;

use rayon::prelude::*;

fn simple_sieve(limit: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    let mut sieve = vec![true; (limit + 1) as usize];
    sieve[0] = false;
    sieve[1] = false;

    let sqrt_limit = (limit as f64).sqrt() as u64;

    for p in 2..=sqrt_limit {
        if sieve[p as usize] {
            primes.push(p);
            let mut multiple = p * p;
            while multiple <= limit {
                sieve[multiple as usize] = false;
                multiple += p;
            }
        }
    }

    primes.extend((sqrt_limit + 1)..=limit);
    primes
}

fn primes(limit: u64) -> Vec<u64> {
    let small_limit = (limit as f64).sqrt() as u64;
    let small_primes = simple_sieve(small_limit);

    let mut large_primes = Vec::new();
    let chunk_size = 10000;
    let chunks: Vec<_> = (small_limit + 1..=limit).collect();
    chunks
        .into_par_iter()
        .chunks(chunk_size)
        .for_each(|chunk| {
            let mut sieve = vec![true; chunk.len()];
            let start = chunk[0];
            let end = chunk[chunk.len() - 1];

            for &prime in &small_primes {
                let mut start_idx = (start + prime - 1) / prime;
                let mut idx = start_idx * prime - start;
                while idx < chunk.len() as u64 {
                    sieve[idx as usize] = false;
                    start_idx += 1;
                    idx = start_idx * prime - start;
                }
            }

            for (i, is_prime) in sieve.into_iter().enumerate() {
                if is_prime {
                    large_primes.push(start + i as u64);
                }
            }
        });

    let mut result = small_primes;
    result.extend(large_primes);
    result
}



