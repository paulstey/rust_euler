use rayon::prelude::*;
use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;

fn phi(n: u64, factors_btmap: &BTreeMap<u64, BTreeSet<u64>>) -> u64 {
    let n_factors = factors_btmap
        .get(&n)
        .expect("ERROR: failed to find `n` in `factors_btmap`");

    let relatively_prime_nums: Vec<_> = (1..n)
        .collect::<Vec<_>>()
        .into_iter()
        .filter(|m| {
            let m_factors = factors_btmap
                .get(m)
                .expect("ERROR: failed to find factors in `factors_btmap`");

            n_factors
                .intersection(m_factors)
                .cloned()
                .collect::<BTreeSet<_>>()
                == BTreeSet::from([1])
        })
        .collect();

    relatively_prime_nums.len() as u64
}

fn main() {
    let t1 = Instant::now();

    let factors_btmap = (1..10_000_000)
        .into_par_iter()
        .map(|i| {
            let factors_btset = prob70::get_factors_btset(i);
            (i, factors_btset)
        })
        .collect::<BTreeMap<u64, BTreeSet<u64>>>();

    let mut n_min_ratio = 0;
    let mut min_ratio = 999_999_999.9;
    let mut ratio: f64;

    for n in 2..10_000_000 {
        let phi_of_n = phi(n, &factors_btmap);

        if prob70::is_digit_permutation(n, phi_of_n) {
            ratio = n as f64 / phi_of_n as f64;

            if ratio < min_ratio {
                min_ratio = ratio;
                n_min_ratio = n;
            }
        }
    }

    let elapsed = Instant::now() - t1;

    println!("min_ratio = {:?}", min_ratio);

    println!("n_min_ratio = {:?}", n_min_ratio);

    println!("{:?}", elapsed);
}
