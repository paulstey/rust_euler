use rayon::prelude::*;



fn main() {

    let mut start_num: Vec<i64> = (0..100_000).collect();

    start_num.par_iter_mut().for_each(|x| {
        collatz(*x);
    })

}


fn collatz(n0: i64) -> i64 {
    let mut len = 1;
    let mut n = n0;

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
