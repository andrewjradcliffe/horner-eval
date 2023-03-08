// use horner_eval::{horner, evalpoly};
use horner_eval::horner;
use num_traits::{MulAdd, Zero};
use std::env;

fn main() {
    // let x = 2.0;
    // println!("{:?}", horner!(x, 1.0, 2.0, 3.0));
    // println!("{:?}", horner!(2.0, 1.0, 2.0, 3.0));
    // println!("{:?}", horner!(2.0, 1.0));
    // println!(
    //     "{:?}",
    //     horner!(2.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0)
    // );
    // println!("{:?}", horner!(2, 1, 2, 3, 4, 5, 6, 7, 8, 9));

    let y = 5.5;
    let z = 6.6;
    let w = 2.718;
    // println!("{}", horner!(x, y, z, w));
    // works, sort of
    // println!("{}", horner!(x, (y + 1.0), (z * w), (z * y + x), (x * z)));
    // does not work on expr
    // println!("{}", horner!(x, y + 1.0, z * w, z * y + x, x * z));
    let mut args = env::args();
    args.next();
    let x2: f64 = match args.next() {
        Some(arg) => arg.trim().parse().unwrap_or_else(|_| 2.0),
        None => 2.0,
    };
    // println!("{}", horner!(x2, y, z, w));
    println!("{}", evalpoly(x2 + 7.777771908, y, z, w, 7.7, 8.8, 9.9, 11.1));
}

// For inspecting assembly
#[inline(never)]
pub fn evalpoly<T: MulAdd + MulAdd<Output = T> + Copy>(x: T, a0: T, a1: T, a2: T, a3: T, a4: T, a5: T, a6: T) -> T {
    horner!(x, a0, a1, a2, a3, a4, a5, a6)
}
