

pub fn factorial_vec(n: usize) -> Vec<u64> {
    let mut fib_nums = vec![1_u64; n]; 

    for i in 2..n {
        fib_nums[i] = fib_nums[i-1] * fib_nums[i-2];
    }
    fib_nums
}

