use prob34::factorial::Factorial;

use time::Instant;
use std::collections::HashMap;


fn main() {
    let t1 = Instant::now();

    let mut fact_iter = Factorial::new(13 as i64);

    loop {
        if let Some(fact_iter) = fact_iter.next() {
            println!("{:?}", fact_iter);
        } else {
            break;
        }
    }


    
    let mut fact_hm = HashMap::new();
    
    for i in 1..10 {
        fact_hm.insert(i, factorial::factorial(i));
    }

    let elapsed = Instant::now() - t1;
}
