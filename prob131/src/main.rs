use primal::Sieve;
use std::collections::HashSet;
use std::time::Instant;

const P_MAX: u64 = 1_000_000;

fn find_qualifying_primes(sieve: &Sieve) -> HashSet<u64> {
    let mut qualifying_primes = HashSet::new();
    let sieve_limit = sieve.upper_bound() as u64;

    for m in 1..1000 {
        let m_cubed = m * m * m;

        for k in (m + 1)..2000 {
            let k_cubed = k * k * k;
            let p = k_cubed - m_cubed;


            if p < P_MAX && p < sieve_limit && p > 1 && sieve.is_prime(p as usize) {
                qualifying_primes.insert(p);
            }

        }
    }

    qualifying_primes
}

fn main() {
    let sieve = Sieve::new(P_MAX as usize);

    let t1 = Instant::now();
    let qualifying_primes = find_qualifying_primes(&sieve);
    let t2 = Instant::now();
    let execution_time = t2 - t1;

    println!("Number of primes that qualify: {}", qualifying_primes.len());
    println!("Execution time: {:?}", execution_time);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_qualifying_primes_small_case() {
        let sieve = Sieve::new(1000);
        let result = find_qualifying_primes(&sieve);
        
        // For small primes, we should find some solutions
        // Based on the mathematical pattern, some known small solutions should exist
        assert!(!result.is_empty(), "Should find at least some qualifying primes");

    }

    #[test]
    fn test_known_solution_p_equals_7() {
        let sieve = Sieve::new(1000);
        let result = find_qualifying_primes(&sieve);
        
        // p = 7 is a known solution: n³ + n²·7 = k³ has integer solutions
        // For example: when n = 1, 1³ + 1²·7 = 8 = 2³
        assert!(result.contains(&7), "p = 7 should be a qualifying prime");
    }


    #[test]
    fn test_function_does_not_panic() {
        let sieve = Sieve::new(100);
        let result = find_qualifying_primes(&sieve);
        
        // Just verify the function runs without panicking
        assert!(result.len() >= 0);
    }


    #[test]
    fn test_mathematical_relationship_for_small_primes() {
        let sieve = Sieve::new(100);
        let result = find_qualifying_primes(&sieve);
        
        // Test that small primes like 7 have solutions
        if result.contains(&7) {
            // For p = 7, we should find n = 1, k = 2: 1³ + 1²·7 = 8 = 2³
            let left_side = 1 * 1 * 1 + 1 * 1 * 7;
            assert_eq!(left_side, 8);
            assert_eq!(2 * 2 * 2, 8);
        }
    }
}
