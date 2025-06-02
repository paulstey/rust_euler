use std::time::Instant;

fn count_arrangements(target: usize, tiles: &[usize]) -> usize {
    // dp[i] = number of ways to tile a row of length i
    let mut dp = vec![0; target + 1];
    dp[0] = 1; // Base case: one way to tile length 0 (use no tiles)

    // For each position from 1 to target
    for length in 1..=target {
        // Try each tile size
        for &tile_size in tiles {
            if tile_size <= length {
                dp[length] += dp[length - tile_size];
            }
        }
    }

    dp[target]
}

fn main() {
    let start = Instant::now();

    let target_len = 50;

    let tile_sizes = vec![1, 2, 3, 4]; // grey, red, green, blue

    println!("Counting tile arrangements (order matters)...");

    let n_arrangements = count_arrangements(target_len, &tile_sizes);

    let duration = start.elapsed();

    println!(
        "Number of ways to tile a row of length {}: {}",
        target_len, n_arrangements
    );

    println!("Time taken: {:?}", duration);
}
