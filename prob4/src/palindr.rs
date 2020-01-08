// palidnr.rs


pub fn is_palidrome(num: i64) -> bool {
    let num_str = num.to_string();
    let rev_num_str = num_str.chars().rev().collect::<String>();

    num_str == rev_num_str
}

pub fn reverse_int<T>(x: T) -> T {
    let result = 0;
    let x_remaining = abs(x);

    while x_remaining > 0 {
        let result = result * 10 + x_remaining % 10;
        x_remaining /= 10;
    }
    let soln = if x < 0 {
        -result;
    } 
    else {
        result;
    }
    soln
}