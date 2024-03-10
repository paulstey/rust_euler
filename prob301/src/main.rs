use time::Instant;

fn xor(n1: i32, n2: i32, n3: i32) -> i32 {
    n1 ^ n2 ^ n3
}

fn main() {
    let t1 = Instant::now();

    let result = (1..1073741824 + 1)
        .collect::<Vec<_>>()
        .into_iter()
        .filter(|&n| xor(n, 2 * n, 3 * n) == 0)
        .count();

    let t2 = Instant::now();

    println!("{}", result);

    println!("{:?}", t2 - t1);
}
