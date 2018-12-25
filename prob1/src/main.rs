extern crate time;

fn is_case(n: i32) -> bool {
    let res = n % 3 == 0 || n % 5 == 0;
    res
}





fn main() {
    let t0: u64 = time::precise_time_ns();

    let mut res = 0;
    for i in 1..1000 {
        if is_case(i) {
            res += i
        }
    }
    let runtime = (time::precise_time_ns() - t0) / 1000;

    println!("Solution: {:?}", res);
    println!("Runtime: {:?} microseconds", runtime);   //  runtime is about 14 microseconds
}
