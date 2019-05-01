
// Seive basics: 
//   1. Cross out multiples of 2, then multiples of 3, etc...
//   2. By "cross out" we probably mean flip a bit in `mask`

fn primes(n: u64) -> Vec<u64> {
    let nums: Vec<u64> = (2..=(n+3)).collect();
    let mut not_prime = vec![false; (n+1) as usize];
    // let prime_flag = vec![false; n as usize];

    for i in 0..=n as usize {
        println!("{}", i);
        // println!("length not_prime: {}", not_prime.len());
        
        if not_prime[i] {
            continue;
        }

        for j in (i+1)..=n as usize {
            let remain = nums[j] % &nums[i];

            println!("{} % {}: {}", nums[j], nums[i], remain);

            if remain == 0 {
                not_prime[j] = true;
                println!("{} is not prime: {}", nums[j], not_prime[j]);
            }   
        }
    }
    let mut prime_nums: Vec<u64> = Vec::new();
    for i in 0..=n as usize {
        if not_prime[i] {
            continue;
        }
        else {
            prime_nums.push(nums[i])
        }
    }
    
    prime_nums
}

fn main() {
    println!("{:?}", primes(20));
}
