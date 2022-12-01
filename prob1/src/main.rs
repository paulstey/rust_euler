use std::time::Instant;

fn main() {
    let t1 = Instant::now();

    let res: i64 = (1..1000).filter(|n| n % 3 == 0 || n % 5 == 0).sum();

    let runtime = t1 - Instant::now();

    println!("Solution: {:?}", res);
    println!("{:?}", runtime);          //  runtime is about 0 nanoseconds
}
