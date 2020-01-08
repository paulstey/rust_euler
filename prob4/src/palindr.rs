// palidnr.rs


pub fn is_palindrome_str(num: u64) -> bool {
    let num_str = num.to_string();
    let rev_num_str = num_str.chars().rev().collect::<String>();

    num_str == rev_num_str
}


// NOTE: Using this is _vastly_ more efficient (and faster) than the 
// above function that reliese on string conversion
pub fn is_palindrome(num: u64) -> bool {
    reverse_int(num) == num
}


pub fn reverse_int(x: u64) -> u64{
    let mut result = 0;
    let mut x_remaining = x;

    while x_remaining > 0 {
        result = result * 10 + x_remaining % 10;
        x_remaining /= 10;
    }
    result
}