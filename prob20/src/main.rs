use num_bigint::BigInt;
use time::Instant;

fn factorial(n: i32) -> BigInt {
    let res = (2..n).into_iter().product();

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
