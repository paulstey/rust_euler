
extern crate time;
// extern crate prob3;

use prob3::primes;



fn main() {
    let t0: u64 = time::precise_time_ns();
    let primes_vec = primes::primes(13000);
    let runtime = (time::precise_time_ns() - t0) / 1000;
    println!("{:?}", primes_vec);
    println!("Runtime: {:?} microseconds", runtime);   //  runtime is about
}
