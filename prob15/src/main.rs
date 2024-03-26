use num_bigint::BigUint;
use time::Instant;

fn factorial(n: u128) -> BigUint {
    let mut solution = BigUint::from(1 as u64);

    for i in 2..n + 1 {
        solution *= BigUint::from(i as u64);
    }

    solution
}

fn n_choose_k(_n: u128, k: u128) -> BigUint {
    let a = k;

    factorial(2 * a) / (factorial(a).pow(2))
}

fn main() {
    let t1 = Instant::now();

    let solution = n_choose_k(40, 20);

    let t2 = Instant::now();

    println!("{:?}", solution);
    println!("{:?}", t2 - t1);
}
