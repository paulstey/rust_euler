///! src/main.rs
use time::Instant;
use primes;

fn main() {
    let t0 = Instant::now();

    let a_vec: Vec<u64> = (1..=100).collect();
    let cumsum: Vec<u64> = a_vec
        .iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect();

    let t1 = Instant::now();

    for x in cumsum {
        println!("{:?}", x);
        println!("{:?}", primes::factors_uniq(x)); 
    }


    println!("{:?}", t1 - t0);
}
