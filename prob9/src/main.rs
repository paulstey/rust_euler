use time::Instant;


fn main() {
    let t0 = Instant::now();

    'outer: for a in 1..999 {
        for b in 1..999 {
            for c in 1..999 {
                if a + b + c == 1000 {
                    if i32::pow(a, 2) + i32::pow(b, 2) == i32::pow(c, 2) {
                         let res = a*b*c;

                         println!("{:?}", res);

                         break 'outer;
                    }
                }
            }
        }
    }
    println!("{:?}", t0.elapsed());

    ()
}
