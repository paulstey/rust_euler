use std::error::Error;
use rayon::prelude::*;
use std::time::Instant;


fn main() -> Result<(), E> {
    let t0 = Instant::now();

    let mut start_num: Vec<i64> = (1..1_000_000).collect();

    let res: Vec<_> = start_num
        .par_iter_mut()
        .enumerate()
        .for_each(|x| {

            collatz(*x.1);
    })?;

    let max = res
        .iter()
        .max()
        .unwrap();
    let idx = res
        .iter()
        .position(|elem| {
            elem == max
        });
    println!("{:?}", idx);
    println!("{:?}", t0.elapsed());
}


fn collatz(n0: i64) -> i64 {
    let mut len = 1;
    let mut n = n0;

    println!("{:?}", n0);

    while n != 1 {
        if n % 2 == 0 {
            n = n/2;
            len += 1;
        } else {
            n = (3 * n) + 1;
            len += 1;
        }
    }
    len
}
