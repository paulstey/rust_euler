use prob7::primes;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let ans = primes::primes(110_000)[10_000];

    let runtime = Instant::now().checked_duration_since(now).unwrap();

    println!("{:?}", ans);
    println!("{:?}", runtime);
}
