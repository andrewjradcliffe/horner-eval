fn horner_eval(x: f64, coefficients: &[f64]) -> f64 {
    let mut result: f64 = 0.0;
    let n = coefficients.len();
    for i in 0..n {
        result = result.mul_add(x, coefficients[n - i - 1]);
    };
    result
}
fn horner_eval2(x: f64, coefficients: &[f64]) -> f64 {
    coefficients
        .iter()
        .rev()
        .fold(0.0_f64, |result, &a| result.mul_add(x, a))
}
// fn horner_eval3(x: f64, coefficients: &[f64]) -> f64 {
//     coefficients
//         .iter()
//         .rev()
//         .reduce(|&acc, &b| acc.mul_add(x, b)).unwrap()
// }
fn horner_eval3(x: f64, coefficients: &[f64]) -> f64 {
    let n = coefficients.len();
    let a_n = coefficients[n-1];
    coefficients[0..n-1]
        .iter()
        .rev()
        .fold(a_n, |result, &a| result.mul_add(x, a))
}

fn muladd(x: f64, a: f64, b: f64) -> f64 { x.mul_add(a, b) }

macro_rules! horner {
    ( $i:ident, $( $a:literal ),+ ) => {
        ( ($i, $( $a, )+) )
    };
    ( $i:ident, ( $( $a:literal ),+ )) => {
        ( ($i, $( $a, )+) )
    }
}

// macro_rules! horner_1 {
//     ( $i:ident, $( $a:literal, )* $a_n:literal, $a_n1:literal ) => {
//         ( ( $a_n.mul_add($i, $a_n1) ), $( $a, )*)
//     }
// }

macro_rules! horner_eval4 {
    ( $x:tt, $a0:tt, $a1:tt ) => {
        muladd($a1, $x, $a0)
    };
    ( $x:tt, $a0:tt, $( $a1:tt ),+ ) => {
        muladd( horner_eval4!( $x, $( $a1 ),+ ), $x, $a0 )
    };
    ( $x:tt, $a0:tt ) => { $a0 }
}

// macro_rules! mixed_rules {
//     () => {};
//     (trace $name:ident; $($tail:tt)*) => {
//         {
//             println!(concat!(stringify!($name), " = {:?}"), $name);
//             mixed_rules!($($tail)*);
//         }
//     };
//     (trace $name:ident = $init:expr; $($tail:tt)*) => {
//         {
//             let $name = $init;
//             println!(concat!(stringify!($name), " = {:?}"), $name);
//             mixed_rules!($($tail)*);
//         }
//     };
// }

fn main() {
    let x = 2.0;
    let c: Vec<f64> = vec![1.0, 2.0, 3.0];
    println!("{}", (c[2].mul_add(x, c[1])).mul_add(x, c[0]));
    println!("{}", horner_eval(x, &c));
    println!("{}", horner_eval2(x, &c));
    println!("{}", horner_eval3(x, &c));
    println!("{:?}", horner!(x, 1.0, 2.0, 3.0));
    println!("{:?}", horner!(x, (1.0, 2.0, 3.0)));
    // println!("{:?}", horner_1!(x, 1.0, 2.0, 3.0));
    // mixed_rules!(trace z = x + 2.0; trace y = x + 4.0;)
    println!("{:?}", horner_eval4!(2.0, 1.0, 2.0, 3.0));
    println!("{:?}", horner_eval4!(2.0, 1.0));
}

