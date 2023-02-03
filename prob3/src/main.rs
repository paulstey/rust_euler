
use time::Instant;
use prob3::primes;



fn main() {
    let t0 = Instant::now();
    let primes_vec = primes::primes(13000);
    let runtime = Instant::now() - t0;
    println!("{:?}", primes_vec);
    println!("{:?}", runtime);   //  runtime is about
}
