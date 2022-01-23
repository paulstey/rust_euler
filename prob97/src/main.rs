use num_bigint::BigInt;
use num_traits::pow;



fn main() {
    let a: BigInt = BigInt::from(28433);

    let b = 7_830_457; 

    let c: BigInt = BigInt::from(1);



    println!("{:?}", a.checked_mul(&BigInt::from(pow(2, b))).unwrap() + c);
}
