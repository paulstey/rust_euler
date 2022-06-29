use prob34::factorial::Factorial;

fn main() {
    let mut fact_iter = Factorial::new(13 as i64);

    println!("{:?}", fact_iter.next()); 

    loop { 
        if let Some(fact_iter) = fact_iter.next() {
            println!("{:?}", fact_iter);
        }
        else {
            break
        }
    }
}
