// palidromes.rs


pub fn is_palidrome(num: i64) -> bool {
    let num_str = num.to_string();
    let rev_num_str = num_str.chars().rev().collect::<String>();

    num_str == rev_num_str
}