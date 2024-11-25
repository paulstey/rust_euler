// src/main.rs
use prob125::palindr;
use std::collections::HashSet;
use std::hash::Hash;
use std::time::Instant;

fn unique_elements<T: Eq + Hash + Clone>(slice: &[T]) -> Vec<T> {
    let mut seen = HashSet::new();

    slice
        .iter()
        .cloned()
        .filter(|item| seen.insert(item.clone()))
        .collect()
}

fn gen_consectutive_squares(n: u64) -> Vec<u64> {
    (1_u64..n).into_iter().map(|x| x.pow(2)).collect()
}

fn get_all_possible_sums(squares: &[u64]) -> Vec<u64> {
    let mut sums = Vec::new();

    for i in 0..(squares.len() - 1) {
        for window in 2..(squares.len() - i + 1) {
            let end_idx = i + window;

            // println!("{:?}", squares[i..end_idx].into_iter());

            sums.push(squares[i..end_idx].into_iter().sum());
        }
    }

    sums.into_iter().filter(|&x| x < 100_000_000).collect()
}

fn sum_palindromes(sums: &[u64]) -> u64 {
    let unique_sums = unique_elements(sums);

    unique_sums
        .into_iter()
        .filter(|&x| palindr::is_palindrome(x))
        .sum()
}

fn main() {
    let t1 = Instant::now();

    let squares = gen_consectutive_squares(10_000_u64);

    let sums = get_all_possible_sums(&squares);

    let sum = sum_palindromes(&sums);

    let elapsed = t1.elapsed();

    println!("Elapsed: {:?}", elapsed);
    println!("{:?}", sum);
}
