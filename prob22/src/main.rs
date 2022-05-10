use std::fs;
use time::Instant;


fn main() {
    let t1 = Instant::now();

    let filename = "data/p022_names.txt";

    let contents = fs::read_to_string(filename)
        .expect("Error reading from file...");

    let names = contents
        .replace('"', "");
    
    let mut names_vec: Vec<&str> = names
        .split(',')
        .collect();


    names_vec.sort();

    let runtime = Instant::now() - t1;

    println!("{:?}", names_vec[0]);
    println!("{:?}", name_score(names_vec[0].to_string()));

    println!("{:?}", runtime);
}


fn name_score(name: String) {
    let num: i32 = name
        .chars()
        .into_iter()
        .map(|x| {
            (66_u8 - (x as u8)) as i32
        })
        .fold(0, |acc, x| acc + x);
}
