use num::integer::gcd;
use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

const MAX_DENOMINATOR: u32 = 12_000;

fn is_reduced_proper(n: u32, d: u32) -> bool {
    gcd(n, d) == 1
}

fn is_in_range(n: u32, d: u32) -> bool {
    0.3333333333333333334 < (n as f64) / (d as f64) && (n as f64) / (d as f64) < 0.5
}

fn main() {
    let t1 = Instant::now();

    let counter = AtomicU64::new(0);

    (2..=MAX_DENOMINATOR).into_par_iter().for_each(|d| {
        let mut local_count = 0;

        for n in 1..d {
            if is_reduced_proper(n, d) && is_in_range(n, d) {
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
