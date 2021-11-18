
use std::fs;

fn main() -> Result<String, Box<dyn std::error::Error>> {

    let dat = fs::read_to_string("data.txt")?.parse()?;

    println!("{:?}", dat);

    Ok(dat)
}
