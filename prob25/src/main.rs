use std::time::Instant;
use num_bigint::BigInt;


fn fib(n: usize) -> BigInt {
    let mut a: BigInt = BigInt::from(1);
    let mut b: BigInt = BigInt::from(1);
    
    let mut tmp: BigInt;

    for _i in 3..=n {
        tmp = b.clone();
        b = a + b;
        a = tmp;
    }

    b 
}

fn main() {
    let t0 = Instant::now();
    let mut idx: usize = 3;
    let mut soln: BigInt = BigInt::from(1);

    for i in 3..10_000 {
        if fib(i).to_string().chars().count() >= 1000 {
            idx = i;
            soln = fib(i);
            break
        }
    }
    let t1 = Instant::now();

    println!("{:?}", idx);
    
    println!("{:?}", soln);
    println!("{:?}", t1 - t0);
}
