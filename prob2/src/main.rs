// Problem 2: Find the sum of even Fibonacci numbers less than 4_000_000

extern crate time;

fn sum_even_fibs(n: u64) -> u64 {
    let mut a: u64 = 1;
    let mut b: u64 = 1;
    let mut tmp: u64;
    let mut res: u64 = 0;

    while b < n {
        tmp = b;
        b = a + b;       // this is our new Fibonacci number
        a = tmp;

        if b % 2 == 0 {
            res += b;
        }

    }
    res
}

fn main() {
    let t0: u64 = time::precise_time_ns();

    let res = sum_even_fibs(4_000_000);

    let runtime = (time::precise_time_ns() - t0) / 1000;

    println!("Solution: {:?}", res);
    println!("Runtime: {:?} microseconds", runtime);   //  runtime is about 10 microseconds
}
