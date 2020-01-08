extern crate time;


use prob4::palindr;


fn main() {
    let t0: u64 = time::precise_time_ns();



    let time_ms = (time::precise_time_ns() - t0) / 1000;

    println!("Runtime: {:?} microseconds", time_ms);   //  runtime is about
}
