use rayon::prelude::*;
use time::Instant;

fn find_num_divisors(n: i32) -> i32 {
    let mut divisors = Vec::new();
    let sqrt_n = (n as f64).sqrt() as i32;

    for i in 1..=sqrt_n {
        if n % i == 0 {
            divisors.push(i);
            if i != n / i {
                divisors.push(n / i);
            }
        }
    }

    divisors.len() as i32
}

fn get_num_divisors_vec(n_max: i32) -> Vec<i32> {
    (1..n_max + 1)
        .into_par_iter()
        .map(|n| find_num_divisors(n))
        .collect()
}

fn main() {
    let t1 = Instant::now();

    let num_divisors_vec = get_num_divisors_vec(10_000_000);

    let mut is_pair = vec![0; 10_000_000];

    (0..9_999_999).into_iter().for_each(|i| {
        if num_divisors_vec[i] == num_divisors_vec[i + 1] {
            is_pair[i] = 1;
        }
    });

    let t2 = Instant::now();

    println!("{:?}", t2 - t1);

    println!("{:?}", is_pair.into_iter().sum::<i32>());
}
