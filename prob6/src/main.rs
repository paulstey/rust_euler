use std::time::Instant; 


const MAX_NUM: u128 = 100;



fn sum_of_squares(n: u128) -> u128 {
    let mut res = 0_u128;

    for i in 1..=n {
        res += u128::pow(i, 2);
    }
    res
}


fn square_of_sums(n: u128) -> u128 {
    let mut acc = 0_u128;

    for i in 1..=n {
        acc += i;
    }
    u128::pow(acc, 2)
}


fn main() {
    let now = Instant::now();


    let small = sum_of_squares(MAX_NUM);
    let big   = square_of_sums(MAX_NUM);

    let ans = big - small;

    let runtime = Instant::now()
        .checked_duration_since(now)
        .unwrap();

    println!("{:?}", ans);
    println!("{:?}", runtime);

}
