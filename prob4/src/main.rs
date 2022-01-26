use time::Instant;
use prob4::palindr;

fn largest_palindrome_prod(max_n: u64) -> u64 {
    let mut soln: u64 = 0;
    for i in 1..max_n {
        for j in 1..max_n {
            soln = if palindr::is_palindrome(i*j) {
                i*j
            }
            else {
                soln
            }
        } 
    }
    soln
}

fn main() {
    let t0 = Instant::now();
    let soln = largest_palindrome_prod(999);

    let runtime = Instant::now() - t0; 

    println!("The solution is:  {:?}", soln);
    println!("{:?}", runtime);   //  runtime is about
}
