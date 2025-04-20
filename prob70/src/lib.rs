use std::collections::BTreeSet;

// This function finds all factors of a given number and returns them as a vector.
fn find_factors(number: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let limit = (number as f64).sqrt() as u64;

    for divisor in 1..=limit {
        if number % divisor == 0 {
            factors.push(divisor);

            if divisor != number / divisor {
                // To avoid duplicating the square root if it's a perfect square
                factors.push(number / divisor);
            }
        }
    }

    factors
}

// This function converts the vector of factors into a BTreeSet for efficient storage and lookup.
pub fn get_factors_btset(n: u64) -> BTreeSet<u64> {
    let factors_vec: Vec<_> = find_factors(n);

    let mut factors_btset = BTreeSet::new();

    factors_vec.into_iter().for_each(|factor| {
        factors_btset.insert(factor);
    });

    factors_btset
}

// This function extracts the digits of a number and returns them as a vector.
fn get_digits(n: u64) -> Vec<u64> {
    let mut nums_vec = Vec::new();
    let mut n_copy = n;

    while n_copy > 0 {
        let remainder = n_copy % 10;

        n_copy /= 10;

        nums_vec.push(remainder);
    }
    nums_vec.reverse();

    nums_vec
}

// This function checks if two numbers are digit permutations of each other.
pub fn is_digit_permutation(n: u64, phi_of_n: u64) -> bool {
    let mut n_digits = get_digits(n);
    let mut phi_of_n_digits = get_digits(phi_of_n);

    if n_digits.len() != phi_of_n_digits.len() {
        return false;
    }

    n_digits.sort();
    phi_of_n_digits.sort();

    n_digits == phi_of_n_digits
}
