use num_bigint::BigUint;
use num_traits::Zero;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, ErrorKind};
use time::Instant;

fn read(path: &str) -> Result<Vec<BigUint>, io::Error> {
    let file = File::open(path)?;
    
    // wrap file into generic buffered reader, it will use 4 KB buffer internally
    // to reduce number of syscalls, thus improving performance
    let br = BufReader::new(file);
    let mut v: Vec<BigUint> = Vec::new();
    // br.lines() creates an iterator over lines in the reader
    // see: https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines
    for line in br.lines() {
        // IO operations generally can return error, we check if got
        // an error,in which case we return this error as the function result
        let line = line?;
        let n = line
            .trim() // trim "whitespaces"
            .parse() // call `str::parse::<i64>(&self)` method on the trimmed line, which parses integer
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?; // parse() can return error (e.g. for string "abc"), here if we got it, we convert it to `std::io::Error` type and return it as function result
        v.push(n); // push acquired integer to the vector
    }
    Ok(v)
}



fn main() {
    let num_vec = read("data/numbers.txt").unwrap();
    let n_nums = num_vec.len();    
    let mut acc: BigUint = Zero::zero();
    let t0 = Instant::now();

    for i in 0..n_nums {
        acc += &num_vec[i];
    }
    
    let soln = acc
        .to_string()
        .chars()
        .take(10)
        .collect::<String>();
    println!("{:?}",soln);
    println!("Elapsed time is: {:?}", t0.elapsed());
}
