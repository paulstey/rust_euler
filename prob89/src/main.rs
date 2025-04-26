// src/main.rs

use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn read_roman_numerals_file(filename: &str) -> Result<Vec<String>> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    // Collect lines into a Vec<String>
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    Ok(lines)
}

fn parse_roman_numeral(roman: &str) -> Result<i32> {
    let mut total = 0;
    let mut prev_value = 0;

    for c in roman.chars() {
        let value = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => return Err(anyhow!("Invalid character")),
        };

        // If the current value is greater than the previous value, it indicates a subtraction
        // case because Roman numerals are typically written in descending order. And when a
        // smaller numeral appears before a larger one, it should be subtracted. We multiply the
        // previous value by 2 to adjust the total to reflect the fact that we have already added
        // it once.
        if value > prev_value && prev_value > 0 {
            total += value - 2 * prev_value; // Adjust for previous value
        } else {
            total += value;
        }
        prev_value = value;
    }

    Ok(total)
}

fn to_roman_numeral(mut num: i32) -> String {
    // Define lookup tables for Roman numeral conversion
    // These arrays are parallel - each value corresponds to its symbol at the same index
    // Note that we include subtractive combinations (CM, CD, XC, etc.) as single entries
    let values = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    let symbols = [
        "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
    ];

    let mut result = String::new();

    for i in 0..values.len() {
        while num >= values[i] {
            result.push_str(symbols[i]);
            num -= values[i];
        }
    }

    result
}

fn diff_minimal_roman(roman: &str, num: i32) -> Result<i32> {
    let best_roman = to_roman_numeral(num);

    let n_chars_saved = roman.chars().count() - best_roman.chars().count();

    Ok(n_chars_saved as i32)
}

fn main() -> Result<()> {
    let start = Instant::now();

    let mut roman_numerals_vec = read_roman_numerals_file("data/roman.txt")?;

    let num_chars_saved = roman_numerals_vec.iter_mut().fold(0, |mut acc, roman| {
        let roman_num = parse_roman_numeral(roman).expect("Invalid Roman numeral");

        let diff = diff_minimal_roman(roman, roman_num).expect("Failed to calculate difference");

        acc += diff;

        acc
    });

    let elapsed = start.elapsed();

    println!("Number of characters saved: {}", num_chars_saved);

    println!("Time elapsed: {:?}", elapsed);

    Ok(())
}
