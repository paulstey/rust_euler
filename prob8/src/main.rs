use std::fs;
use std::io;
use time::Instant;



fn read_txt_file(filename: &str) -> Result<String, io::Error> {
    let dat = fs::read_to_string(filename)?;
    Ok(dat)
}

fn main() -> Result<(), io::Error> {
    let t0 = Instant::now();
    let dat = read_txt_file("src/data.txt")?;

    let num_vec: Vec<u128> = dat
        .replace("\n", "")
        .chars()
        .map(|c| {
            c.to_string().parse::<u128>().unwrap()
        })
        .collect();

    let mut running_prod = Vec::new();
    let mut prod: u128;
    
    let last_idx = num_vec.len() - 12;

    for i in 0..last_idx {
        prod = num_vec[i] * num_vec[i+1] * num_vec[i+2] * num_vec[i+3] * 
            num_vec[i+4] * num_vec[i+5] * num_vec[i+6] * num_vec[i+7] *
            num_vec[i+8] * num_vec[i+9] * num_vec[i+10] * num_vec[i+11] * num_vec[i+12]; 

        running_prod.push(prod);
    }

    println!("{:?}", running_prod.iter().max());
    println!("{:?}", t0.elapsed());

    Ok(())
}


