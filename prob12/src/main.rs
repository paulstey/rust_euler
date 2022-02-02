///! src/main.rs
use time::Instant;

const MAX_NUM: i32 = 100_000_000;

fn get_multiples(n: i32) -> Vec<i32> {
    let mut res_vec: Vec<i32> = Vec::new();

    for i in 1..=n {
        if n % i == 0 {
            res_vec.push(i);
        }
    }
    res_vec
}

fn main() {
    let t0 = Instant::now();

    let a_vec: Vec<i32> = (1..=MAX_NUM).collect();
    let cumsum: Vec<i32> = a_vec
        .iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect();

    for x in cumsum {
        if get_multiples(x).len() > 500 {
            println!("{:?}", x); 
            break 
        }
    }
    

    let t1 = Instant::now();
    
    println!("{:?}", t1 - t0);
}
