// primes.rs

// Seive basics:
//   1. Cross out multiples of 2, then multiples of 3, etc...
//   2. By "cross out" we probably mean flip a bit in `mask`

pub fn primes(n: u64) -> Vec<u64> {
    let nums: Vec<u64> = (1..=(n + 3)).collect();
    let mut not_prime = vec![false; (n + 1) as usize];

    for i in 1..=n as usize {
        // Skip the rest of the main loop if we've already crossed off this number
        if not_prime[i] {
            continue;
        }

        // If we haven't crossed off this number, then we walk through all the numbers
        // and cross out the multiples of `numb[i]` appearing in `nums`.
        for j in (i + 1)..=n as usize {
            if not_prime[j] {
                continue;
            }
            let remain = nums[j] % nums[i];

            if remain == 0 {
                not_prime[j] = true;
            }
        }
    }
    let mut prime_nums: Vec<u64> = Vec::new();
    for i in 1..=n as usize {
        if not_prime[i] {
            continue;
        } else {
            prime_nums.push(nums[i])
        }
    }

    prime_nums
}
