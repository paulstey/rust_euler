// This solution was an adaptation of the excellent C++ solution by Stephan Brumme
// which can be found here: http://euler.stephan-brumme.com/142/
//
// # Problem
// Find the smallest x + y + z with integers x > y > z > 0 such that x + y, x − y, x + z, x − z, y + z, y − z are all perfect squares.
//

// # Algorithm
// I have to solve six equations:
// (1) `a^2 = x + y`
// (2) `b^2 = x - y`
// (3) `c^2 = x + z`
// (4) `d^2 = x - z`
// (5) `e^2 = y + z`
// (6) `f^2 = y - z`
//
// And this condition has to be obeyed:
// (7) `x > y > z > 0`
//
// If I add (1) and (2), then I have `x`:
// (8) `a^2 + b^2 = 2x` ==> `x = frac{1}{2}(a^2 + b^2)`
//
// After reordering (1) I get the value of `y`:
// (9) `y = a^2 - x`
//
// The same trick applied to (3):
// (10) `z = c^2 - x`
//
// All equations can be solved if I find valid values for `a^2`, `b^2` and `c^2`.
// However, when computing `d^2`, `e^2` and `f^2` based on (4), (5) and (6) then a few "fake" solutions appear where
// `d^2`, `e^2` and `f^2` are not perfect squares.
//
// My program has three nested loops that iterate over possible values `a`, `b` and `c`.
// A few optimizations are required to finish the algorithm in a reasonable amount of time:
// - `x` must be an integer and from (8) follows that `a^2 + b^2` must be even, that means `a` and `b` are both odd or both even
// - all numbers are positive therefore from (10) follows that `c^2 > x` which is the same as `c > sqrt{x}`
// - instead of checking a number on-the-fly whether it is a perfect square, I precompute a small look-up table

use time::Instant;

fn get_perfect_sq_lookup(n_max: i32) -> Vec<bool> {
    // Record all square numbers
    // [n] => [true, if n is a perfect square]

    let mut lookup = vec![false; n_max as usize];

    let sqrt_n_max = (n_max as f64).sqrt() as usize;

    for i in 1..sqrt_n_max {
        lookup[i * i] = true
    }

    lookup
}

fn find_candidates(a_max: usize, is_square_lkup: &[bool]) -> Vec<i32> {
    let mut candidates = Vec::new();

    // Substitute in (3):
    // (9) z = c^2 - x
    for a in 3..a_max {
        let b_start = if a % 2 == 0 { 2 } else { 1 };
        let b_end = a - 1;

        // a and b must be both odd or both even
        for b in (b_start..b_end).step_by(2) {
            // From (8): compute x
            let x = ((a * a + b * b) as f64 / 2.0) as i32;

            // From (9): compute y
            let y = (a * a) as i32 - x;

            // Ensure x > y
            if x <= y as i32 {
                break;
            }

            let c_start = (x as f64).sqrt() as usize + 1;
            let c_end = a_max;

            for c in c_start..c_end {
                // From (10): compute z
                let z = (c * c) as i32 - x;

                if y <= z {
                    break;
                }

                // Check whether d^2, e^2 and f^2 are perfect squares
                if is_square_lkup[(x - z) as usize]
                    && is_square_lkup[(y + z) as usize]
                    && is_square_lkup[(y - z) as usize]
                {
                    println!("x = {x}, y = {y}, z = {z}");

                    candidates.push(x as i32 + y as i32 + z as i32);

                    return candidates;
                }
            }
        }
    }
    candidates
}

fn main() {
    let t1 = Instant::now();

    let limit = 3_000_000;
    let is_square_lkup = get_perfect_sq_lookup(limit);

    let candidates = find_candidates(10_000, &is_square_lkup);

    let t2 = Instant::now();

    println!("{:?}", candidates.iter().min().unwrap());

    println!("{:?}", t2 - t1);
}
