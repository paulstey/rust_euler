use std::fs;
use time::Instant;

fn main() {
    let t1 = Instant::now();
    let filename = "data/p022_names.txt";
    let contents = fs::read_to_string(filename).expect("Error reading from file...");
    let names = contents.replace('"', "");

    let mut names_vec: Vec<&str> = names.split(',').collect();

    names_vec.sort_unstable();

    let soln = sum_name_scores(&names_vec);
    let runtime = Instant::now() - t1;

    println!("{:?}", soln);
    println!("{:?}", runtime);
}

fn name_score(name: String, pos: i32) -> i32 {
    let chars_vec = name.chars();

    let num: i32 = chars_vec.map(|x| ((x as u8) as i32) - 64).sum();

    num * (pos + 1)
}

fn sum_name_scores(all_names: &[&str]) -> i32 {
    let mut soln = 0;

    for (i, val) in all_names.iter().enumerate() {
        soln += name_score(val.to_string(), i.try_into().unwrap())
    }

    soln
}
