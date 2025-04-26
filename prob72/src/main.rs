use num::integer::gcd;
use std::time::Instant;
use std::sync::atomic::{AtomicU64, Ordering};
use rayon::prelude::*;

fn is_reduced_proper(n: u32, d: u32) -> bool {
    gcd(n, d) == 1
}

fn main() {
    let t1 = Instant::now();

    let max_denominator = 1_000_000;
    let counter = AtomicU64::new(0);

    (2..=max_denominator).into_par_iter().for_each(|d| {
        let mut local_count = 0;
        for n in 1..d {
            if is_reduced_proper(n, d) {
                local_count += 1;
            }
        }
        counter.fetch_add(local_count, Ordering::Relaxed);
    });

    let n_reduced_proper = counter.load(Ordering::Relaxed);
    let t2 = Instant::now();

    println!("Run time: {:?}", t2.duration_since(t1));
    println!("Solution: {:?}", n_reduced_proper);
}
