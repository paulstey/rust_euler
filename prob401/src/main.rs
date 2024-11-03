use num_bigint::BigUint;
use num_traits::{One, Zero};
use time::Instant;

fn small_sigma(n: usize) -> Vec<BigUint> {
    let mut fib_nums = Vec::with_capacity(n + 1);

    let f0: BigUint = Zero::zero();
    let f1: BigUint = One::one();

    fib_nums.push(f0);
    fib_nums.push(f1);

    for i in 2..n + 1 {
        fib_nums.push(&fib_nums[i - 1] + &fib_nums[i - 2]);
    }

    fib_nums
}

fn main() {
    let t1 = Instant::now();
    let n = 100_000;

    let sigma_nums = small_sigma(n);

    let t2 = Instant::now();

    // This is a very large number.
    println!("fib({n}) = {}", sigma_nums[n]);

    println!("{:?}", t2 - t1);
}
