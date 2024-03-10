use primes;
use rayon::prelude::*;
use time::Instant;

fn main() {
    let t1 = Instant::now();

    let hamming_numbers = get_hamming_numbers(100, 1_000_000_000);

    let t2 = Instant::now();

    println!("{:?}", hamming_numbers.len());

    println!("{:?}", t2 - t1);
}

fn get_hamming_numbers(n: u64, check_max: u64) -> Vec<u64> {

    let hamming_numbers = (1..check_max + 1)
        .collect::<Vec<_>>()
        .into_par_iter()
        .filter(|&x| {
            let primes_vec = primes::factors(x);

            primes_vec.iter().all(|&prime| prime <= n)
        })
        .collect();

    hamming_numbers
}
