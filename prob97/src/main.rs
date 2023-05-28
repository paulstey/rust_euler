use time::Instant;

// This function implements an efficient algorithm for exponentiation,
// known as Exponentiation by Squaring or Fast Exponentiation.
fn fast_exponentiation(mut base: u128, mut exponent: u128) -> u128 {
    let mut intermediate_result = 1;

    while exponent != 0 {
        // If the exponent is odd, multiply `intermediate_result` by `base`
        // and take the modulus to keep the numbers manageable.
        if exponent & 1 == 1 {
            intermediate_result *= base;
            intermediate_result %= 10_000_000_000;
        }

        // Square `base` and take modulus to prevent overflow.
        base *= base;
        base %= 10_000_000_000;

        // Divide the exponent by 2 by right-shifting.
        exponent >>= 1;
    }

    // Return the intermediate result after all iterations are done.
    intermediate_result
}

fn main() {
    // Declare a mutable variable `result` and initialize it to 28433.

    let t1 = Instant::now();

    let multiplication_result: u128 = 28433_u128 * fast_exponentiation(2, 7830457);

    
    // Calculate the remainder of `result` when divided by 10 billion, add 1
    // and print the result.
    let solution = multiplication_result % 10_000_000_000 + 1;

    println!("{:?}", Instant::now() - t1);

    println!("{:?}", solution);
}
