use time::Instant;
use num_bigint::BigInt;
use num::pow;

fn main() {
    let t0 = Instant::now();
    let num: BigInt = pow::pow(BigInt::from(2), 1000);

    let soln: u32 = num
        .to_string()
        .chars()
        .map(|x| x.to_digit(10_u32).unwrap())
        .sum();

    let t1 = Instant::now();

    println!("{:?}", soln);
    
    println!("{:?}", t1 - t0);
}
