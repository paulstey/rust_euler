use std::time::Instant;


fn main() { 
    let t0 = Instant::now();

    let mut res: Vec<_> = vec![0; 1_000_000];
    
    for i in 1..1_000_000 {
        res[i] = collatz(i as i64); 
    }
   
    let soln = res
        .iter()
        .max()
        .unwrap(); 
    
    for i in 0..999_999 {
        if res[i] == *soln {
            println!("{:?}", i);
        }
    }

    println!("{:?}", t0.elapsed());
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
