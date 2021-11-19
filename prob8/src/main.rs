
use std::fs;
use std::io;

fn read_txt_file(filename: &str) -> Result<String, io::Error> {
    let dat = fs::read_to_string(filename)?;
    Ok(dat)
}

fn main() -> Result<(), io::Error> {

    let dat = read_txt_file("src/data.txt")?;

    println!("{:?}", dat);

    Ok(())
}


