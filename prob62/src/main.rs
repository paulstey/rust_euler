// prob62

use rayon::prelude::*;
use std::collections::HashMap;
use std::time::Instant;

// This function takes a number n and returns a vector of its digits.
fn num_to_vec(n: u64) -> Vec<u8> {
    let mut v = Vec::new();
    let mut n = n;

    while n > 0 {
        v.push((n % 10) as u8);
        n /= 10;
    }

    v.reverse();

    v
}

fn add_num_to_hashmap(n: u64, hmap: &mut HashMap<String, Vec<u64>>, cube_set_size: u64) -> bool {
    let mut vec = num_to_vec(n);

    vec.sort();

    let key = vec.iter().map(|x| x.to_string()).collect::<String>();

    let mut found_cube_set = false;

    if hmap.contains_key(&key) {
        let nums_vec = hmap.get_mut(&key).unwrap();
        nums_vec.push(n);

        if nums_vec.len() == cube_set_size.try_into().unwrap() {
            found_cube_set = true;
        }
    } else {
        hmap.insert(key, vec![n]);
    }

    found_cube_set
}

fn main() {
    let t1 = Instant::now();

    let mut hmap: HashMap<String, Vec<u64>> = HashMap::new();

    let cube_set_size = 5;

    let mut cubes = (100..50_000)
        .into_par_iter()
        .map(|n| n * n * n)
        .collect::<Vec<u64>>();

    cubes.sort();

    for n in cubes.into_iter() {
        let found_cube_set = add_num_to_hashmap(n, &mut hmap, cube_set_size);

        if found_cube_set {
            let mut num_vec = num_to_vec(n);
            num_vec.sort();

            let key = num_vec.iter().map(|x| x.to_string()).collect::<String>();

            let nums_vec = hmap.get(&key).unwrap();

            println!("Found cube set: {:?}", nums_vec[0]);
            break;
        }
    }

    let elapsed = t1.elapsed();

    println!("Elapsed: {:?}", elapsed);
}
