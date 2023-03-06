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


fn main() {
    let x = 2.0;
    let c: Vec<f64> = vec![1.0, 2.0, 3.0];
    println!("{}", (c[2].mul_add(x, c[1])).mul_add(x, c[0]));
    println!("{}", horner_eval(x, &c));
    println!("{}", horner_eval2(x, &c));
    println!("{}", horner_eval3(x, &c));
}

