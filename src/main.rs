use num_traits::{MulAdd, Zero};
use horner_eval::horner;

fn main() {
    let x = 2.0;
    println!("{:?}", horner!(x, 1.0, 2.0, 3.0));
    println!("{:?}", horner!(2.0, 1.0, 2.0, 3.0));
    println!("{:?}", horner!(2.0, 1.0));
    println!("{:?}", horner!(2.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0));
    println!("{:?}", horner!(2, 1, 2, 3, 4, 5, 6, 7, 8, 9));

    let y = 5.5;
    let z = 6.6;
    let w = 2.718;
    println!("{}", horner!(x, y, z, w));
    // println!("{}", horner!(x, y + 1.0, z * w, z * y + x, x * z));

}

