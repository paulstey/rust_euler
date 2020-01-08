extern crate time;

use prob4::palindr;

fn largest_palindrome_prod(max_n: u64) -> u64 {
    let mut soln: u64 = 0;
    for i in 1..max_n {
        for j in 1..max_n {
            soln = if palindr::is_palindrome(i*j) {
                i*j
            }
            else {
                soln
            }
        } 
    }
    soln
}

fn main() {
    let t0: u64 = time::precise_time_ns();
    let soln = largest_palindrome_prod(999 as u64);

    let time_ms = (time::precise_time_ns() - t0) / 1000;

    println!("{:?}", soln);
    println!("Runtime: {:?} microseconds", time_ms);   //  runtime is about
}
