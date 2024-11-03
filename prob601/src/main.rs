use time::Instant;

fn streak(n_start: u64) -> u64 {
    let mut n = n_start;
    let mut fractional = 0.0;
    let mut k = 0;

    while fractional == 0.0 {
        fractional = (n as f64 / (k as f64 + 1.0)).fract();

        n += 1;
        k += 1;
    }

    k - 1
}

fn get_all_streak_values(n_max: u64) -> Vec<u64> {
    (2..n_max + 1)
        .collect::<Vec<_>>()
        .into_iter()
        .map(|s| streak(s))
        .collect()
}

fn capital_p(s: u64, n_max: u64, streak_vec: &Vec<u64>) -> u64 {
    streak_vec[..(n_max - 1) as usize]
        .iter()
        .filter(|&n| *n == s)
        .count()
        .try_into()
        .unwrap()
}

fn main() {
    let t1 = Instant::now();
    let n_max = 4_611_686_018_427_387_904;
    // let n_max = 100_000_000;

    let streak_vec = get_all_streak_values(n_max);

    let result: u64 = (1..=31)
        .collect::<Vec<_>>()
        .iter()
        .map(|&i| capital_p(i, 4_u64.pow(i.try_into().unwrap()), &streak_vec))
        .sum();

    let t2 = Instant::now();

    let elapsed = t2 - t1;

    println!("{:?}", result);

    println!("{:?}", elapsed);
}
