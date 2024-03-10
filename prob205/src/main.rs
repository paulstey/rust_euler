use rand::prelude::*;
use rayon::prelude::*;
use time::Instant;


enum Person {
    Colin,
    Peter,
}


fn simulate_rolls(person: Person, rng: &mut ThreadRng) -> i32 {
    let num_rolls = match person {
        Person::Peter => 9,
        Person::Colin => 6,
    };
    
    let dice_values = match person {
        Person::Peter=> vec![1, 2, 3, 4], 
        Person::Colin=> vec![1, 2, 3, 4, 5, 6],
    };

    let mut score = 0;

    for _i in 0..num_rolls {
        let idx = rng.gen_range(0..(dice_values.len()));
        score += dice_values[idx];
    }

    score
}

fn does_peter_win_game() -> bool {
    let mut rng = thread_rng();

    let colin = simulate_rolls(Person::Colin, &mut rng);
    let peter = simulate_rolls(Person::Peter, &mut rng);

    peter > colin
}

fn main() {
    let t1 = Instant::now();

    let num_games = 20_000_000_000_000 as u64;

    let peter_wins: u64 = (0..num_games)
        .into_par_iter()
        .map(|_i| if does_peter_win_game() { 1 } else { 0 })
        .sum();

    let t2 = Instant::now();

    println!("{:?},{:?}", peter_wins, num_games);
    println!("{:?}", t2 - t1);
}
