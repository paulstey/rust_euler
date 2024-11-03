use anyhow::Result;
use ndarray::Array2;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Coords {
    x: i32,
    y: i32,
}

struct Path {
    start_coords: Coords,
    end_coords: Coords,
}

fn shortest_path(start: Coords, end: Coords, path_costs_hmap: HashMap<Path, i32>) {}

fn read_matrix_from_file(filename: &str) -> Result<Array2<i32>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // Initialize a vector to hold the matrix rows
    let mut matrix_data = Vec::new();

    // Iterate over each line in the file
    for line in reader.lines() {
        let row: Result<Vec<i32>, _> = line?.split(',').map(|s| s.parse::<i32>()).collect();

        // Append the row to the matrix data
        matrix_data.push(row?);
    }

    // Convert the matrix data into an ndarray
    let num_rows = matrix_data.len();
    let num_cols = matrix_data[0].len();
    let mut matrix = Array2::zeros((num_rows, num_cols));

    for (i, row) in matrix_data.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            matrix[[i, j]] = value;
        }
    }

    Ok(matrix)
}

fn main() {
    if let Ok(matrix) = read_matrix_from_file("data/small_matrix.txt") {
        println!("Matrix read from file:");
        println!("{:?}", matrix);
    } else {
        println!("Error reading matrix from file.");
    }
}
