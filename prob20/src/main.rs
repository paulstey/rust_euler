use num_bigint::BigInt;
use time::Instant;

fn factorial(n: i32) -> BigInt {
    let mut res: BigInt = BigInt::from(n);

    for m in 2..n {
        res *= BigInt::from(m);
    }
    res
}

fn main() {
    let t0 = Instant::now();

    let num: BigInt = factorial(100);

    let soln: u32 = num
        .to_string()
        .chars()
        .map(|x| x.to_digit(10_u32).unwrap())
        .sum();

    println!("{:?}", soln);

    println!("{:?}", t0.elapsed());
}
