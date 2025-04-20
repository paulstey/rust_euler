use rayon::prelude::*;
use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;

// This function calculates Euler's totient function (phi) for a given number `n`. It uses a
// precomputed map of factors to determine the numbers relatively prime to `n`.
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

    const N_MAX: u64 = 100_000;

    // Precompute the factors for all numbers in the range [1, 10,000,000] using parallel iteration.
    let factors_btmap = (1..N_MAX)
        .into_par_iter()
        .map(|i| {
            let factors_btset = prob70::get_factors_btset(i);
            (i, factors_btset)
        })
        .collect::<BTreeMap<u64, BTreeSet<u64>>>();

    let n_and_phi_of_n: Vec<_> = (1..N_MAX)
        .into_par_iter()
        .map(|n| {
            let phi_of_n = phi(n, &factors_btmap);
            (n, phi_of_n)
        })
        .filter(|(n, phi_of_n)| {
            // Check if `n` and `phi(n)` are digit permutations of each other.
            prob70::is_digit_permutation(*n, *phi_of_n)
        })
        .collect();

    // Initialize variables to track the minimum ratio and corresponding number.
    let mut n_min_ratio = 0;
    let mut min_ratio = 999_999_999.9;
    let mut ratio: f64;

    // Iterate through numbers to find the one with the minimum ratio of n/phi(n).
    for (n, phi_of_n) in n_and_phi_of_n {
        println!("n = {:?}, phi(n) = {:?}", n, phi_of_n);
        println!("min_ratio = {:?}", min_ratio);

        ratio = n as f64 / phi_of_n as f64;

        // Update the minimum ratio and corresponding number if a smaller ratio is found.
        if ratio < min_ratio {
            min_ratio = ratio;
            n_min_ratio = n;
        }
    }

    // Calculate and print the elapsed time.
    let elapsed = Instant::now() - t1;

    println!("min_ratio = {:?}", min_ratio);
    println!("n_min_ratio = {:?}", n_min_ratio);
    println!("{:?}", elapsed);
}
