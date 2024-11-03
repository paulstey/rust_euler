use std::time::Instant;

fn to_base3_string(mut n: usize) -> String {
    if n == 0 {
        return "0".to_string();
    }

    let mut digits = Vec::new();

    while n > 0 {
        digits.push((n % 3).to_string());
        n /= 3;
    }

    digits.reverse();

    digits.join("")
}

fn f(n: u64, base3_nums_base10: &Vec<u64>) -> Option<u64> {
    for base3 in base3_nums_base10.iter() {
        if *base3 % n == 0 {
            return Some(*base3);
        }
    }
    None
}

fn main() {
    let start = Instant::now();

    let n_max = 500_000_000;

    let base3_nums_base10: Vec<u64> = (1..n_max + 1)
        .map(|n| to_base3_string(n).parse().unwrap())
        .collect::<Vec<_>>();

    let mut result = 0.0;

    for i in 1..=10000 {
        result += f(i, &base3_nums_base10).expect("Error: value of f() is None") as f64 / i as f64;
    }

    let duration = start.elapsed();

    println!("Result: {}", result);

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
