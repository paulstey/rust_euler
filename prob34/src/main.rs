use prob34::factorial;

use time::Instant;
use std::collections::HashMap;


fn main() {
    let t1 = Instant::now();
    
    let mut fact_hm = HashMap::new();
    
    for i in 1..10 {
        fact_hm.insert(i, factorial::factorial(i));
    }

    let elapsed = Instant::now() - t1;

    println!("{:?}", fact_hm);
    println!("{:?}", elapsed);
}
