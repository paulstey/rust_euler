use time::Instant;

fn get_digits(n: i32) -> Vec<i32> {
    let mut nums_vec = Vec::new();
    let mut n_copy = n;

    while n_copy > 0 {
        let remainder = n_copy % 10;

        n_copy = n_copy / 10;

        nums_vec.push(remainder);
    }
    nums_vec.reverse();

    nums_vec
}

fn test_all_powers_of_k(n_max: i32, k: i32) -> Vec<(i32, i32)> {
    let powers_of_k = (2..n_max + 1)
        .collect::<Vec<i32>>()
        .into_iter()
        .map(|n| (n, n.pow(k as u32)))
        .filter(|(n, n_raised_to_k)| {
            let digits_vec = get_digits(*n_raised_to_k);

            if digits_vec.len() < 2 {
                return false;
            }

            let summed_digits = digits_vec.iter().sum::<i32>();
            *n == summed_digits
        })
        .collect();

    powers_of_k
}

fn main() {
    let t1 = Instant::now();

    let mut success_cases = Vec::new();

    for k in 2..4150 {
        let mut success_with_k = test_all_powers_of_k(1_000, k);
        success_cases.append(&mut success_with_k);
    }

    let mut a_sequence = success_cases
        .into_iter()
        .map(|(_n, n_raised_to_k)| n_raised_to_k)
        .collect::<Vec<i32>>();

    a_sequence.sort();

    let t2 = Instant::now();

    let elapsed = t2 - t1;

    println!("{:?}", a_sequence);

    println!("a_2 = {:?}", a_sequence[1]);

    println!("a_10 = {:?}", a_sequence[9]);

    println!("a_30 = {:?}", a_sequence[29]);

    println!("len(a) = {:?}", a_sequence.len());

    println!("{:?}", elapsed);
}
