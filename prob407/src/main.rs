use rayon::prelude::*;
use time::Instant;

fn _a_squared_mod_n(a: u64, n: u64) -> u64 {
    let a_squared = a * a;

    a_squared % n
}

fn big_m(n: u64) -> u64 {
    let mut a_squared_mod_n_max = 0;

    for a in 1..n {
        let a_mod_n = a % n;
        let a_sq_mod_n = a_squared_mod_n(a, n);

        if a_mod_n == a_sq_mod_n && a_sq_mod_n > a_squared_mod_n_max {
            a_squared_mod_n_max = a_sq_mod_n;
        }
    }

    a_squared_mod_n_max as u64
}

fn main() {
    let t1 = Instant::now();

    let solution: u64 = (1..=10_000_000).into_par_iter().map(|n| big_m(n)).sum();

    let t2 = Instant::now();

    println!("{:?}", solution);

    println!("{:?}", t2 - t1);
}
