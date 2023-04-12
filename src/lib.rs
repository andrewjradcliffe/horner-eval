//! # horner-eval
//!
//! A macro for evaluating polynomials via Horner's rule.

use num_traits::MulAdd;

/// Identical to `x.mul_add(a, b)`; used to generate expression nest without
/// provoking ambiguities which would otherwise arise due to automatic dereferencing.
///
/// # Examples
/// ```
/// use horner_eval::muladd;
///
/// assert_eq!(7.0_f64.mul_add(2.0, 3.0), muladd(7.0, 2.0, 3.0))
/// ```
#[inline]
pub fn muladd<T: MulAdd + MulAdd<Output = T>>(x: T, a: T, b: T) -> T {
    x.mul_add(a, b)
}
// pub fn __zero<T: Zero + MulAdd + MulAdd<Output = T>>(x: T) -> T {
//     T::zero()
// }

/// Evaluate the polynomial `a₀ + a₁x + ⋯ + aₙ₋₁xⁿ⁻¹ + aⁿxⁿ` via Horner's rule.
/// This macro unrolls what would otherwise be a loop into the
/// expression `(⋯(aₙx + aₙ₋₁)x + ⋯ + a₁)x + a₀`.
///
/// # Examples
/// ```
/// use horner_eval::horner;
///
/// let x = 2.0_f64;
///
/// let (a0, a1, a2) = (1.0, 2.0, 3.0);
///
/// // Coefficients are given in ascending order by power of `x`.
/// assert_eq!(17.0, horner!(x, a0, a1, a2));
///
/// // Arbitrary expressions are permitted for the coefficients.
/// assert_eq!(53.5, horner!(x + 5.0, x - 1.0, 2.0 * x, x / 4.0));
///
/// // Works on any type which implements `num_traits::MulAdd`
/// assert_eq!(79, horner!(2, 1, 3, 0, 5, 0, 1));
/// ```
#[macro_export]
macro_rules! horner {
    // ( $x:tt, $a0:tt, $a1:tt ) => {
    //     $crate::muladd($a1, $x, $a0)
    // };
    // ( $x:tt, $a0:tt, $( $a1:tt ),+ ) => {
    //     $crate::muladd( $crate::horner!( $x, $( $a1 ),+ ), $x, $a0 )
    // };
    // ( $x:tt, $a0:tt ) => { $a0 }
    // expr? more permissible...
    ( $x:expr, $a0:expr, $a1:expr ) => {
        $crate::muladd($a1, $x, $a0)
    };
    ( $x:expr, $a0:expr, $( $a1:expr ),+ ) => {
        $crate::muladd( $crate::horner!( $x, $( $a1 ),+ ), $x, $a0 )
    };
    ( $x:expr, $a0:expr ) => { $a0 }
    // ( $x:expr ) => { __zero($x) }
}

/// Evaluate the polynomial `a₀ + a₁x + ⋯ + aₙ₋₁xⁿ⁻¹ + aⁿxⁿ` via Horner's rule.
/// As the name indicates, this function uses an explicit loop
/// to accommodate dynamically-sized coefficient slices.
///
/// # Examples
/// ```
/// use horner_eval::horner_loop;
///
/// let x = 2.0_f64;
///
/// let c: Vec<f64> = vec![1.0, 2.0, 3.0];
///
/// assert_eq!(17.0, horner_loop(x, &c));
/// ```
pub fn horner_loop<T>(x: T, coefficients: &[T]) -> T
where
    T: Copy + MulAdd + MulAdd<Output = T>,
{
    let n = coefficients.len();
    if n > 0 {
        let a_n = coefficients[n - 1];
        coefficients[0..n - 1]
            .iter()
            .rfold(a_n, |result, &a| result.mul_add(x, a))
    } else {
        panic!(
            "coefficients.len() must be greater than or equal to 1, got {}",
            n
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal_muladd_integer() {
        macro_rules! test_muladd {
            ($($t:ident)+) => {
                $(
                    {
                        let x: $t = 2;
                        let a: $t = 19;
                        let b: $t = 4;

                        assert_eq!(muladd(x, a, b), (x * a + b));
                    }
                )+
            };
        }

        test_muladd!(usize u8 u16 u32 u64 isize i8 i16 i32 i64);
    }

    #[test]
    fn internal_muladd_float() {
        macro_rules! test_muladd {
            ($($t:ident)+) => {
                $(
                    {
                        use core::$t;

                        let x: $t = 12.0;
                        let a: $t = 3.4;
                        let b: $t = 5.6;

                        let abs_difference = (muladd(x, a, b) - (x * a + b)).abs();

                        assert!(abs_difference <= 46.4 * $t::EPSILON);
                    }
                )+
            };
        }

        test_muladd!(f32 f64);
    }

    #[test]
    fn horner_integer() {
        macro_rules! test_horner_integer {
            ($($t:ident)+) => {
                $(
                    {
                        let x: $t = 2;
                        let a0: $t = 1;
                        let a1: $t = 2;
                        let a2: $t = 3;

                        assert_eq!(17, horner!(x, a0, a1, a2));

                        assert_eq!(101, horner!(x, a0, a1, a1, a2, a1, a0));
                    }
                )+
            };
        }

        test_horner_integer!(usize u8 u16 u32 u64 isize i8 i16 i32 i64);
    }

    #[test]
    fn horner_float() {
        macro_rules! test_horner_float {
            ($($t:ident)+) => {
                $(
                    {
                        let x: $t = 2.0;
                        let a0: $t = 1.0;
                        let a1: $t = 2.0;
                        let a2: $t = 3.0;

                        assert_eq!(17.0, horner!(x, a0, a1, a2));

                        assert_eq!(101.0, horner!(x, a0, a1, a1, a2, a1, a0));

                        let y: $t = 5.5;
                        let abs_difference = 1.1985066439401153 - horner!(y, 7.72156649015328655494e-02, 6.73523010531292681824e-02, 7.38555086081402883957e-03, 1.19270763183362067845e-03, 2.20862790713908385557e-04, 2.52144565451257326939e-05);
                        assert!(abs_difference <= $t::EPSILON);

                        let a3: $t = 4.0;
                        let a4: $t = 5.0;
                        let a5: $t = 6.0;
                        let a6: $t = 7.0;
                        let a7: $t = 8.0;
                        let a8: $t = 9.0;

                        assert_eq!(4097.0, horner!(x, a0, a1, a2, a3, a4, a5, a6, a7, a8));
                    }
                )+
            };
        }

        test_horner_float!(f32 f64);
    }

    #[test]
    fn horner_loop_integer() {
        macro_rules! test_horner_loop_integer {
            ($($t:ident)+) => {
                $(
                    {
                        let x: $t = 2;
                        let c: Vec<$t> = vec![1, 2, 3];
                        let c1: Vec<$t> = vec![1];
                        assert_eq!(17, horner_loop(x, &c));
                        assert_eq!(1, horner_loop(x, &c1));
                    }
                )+
            }
        }

        test_horner_loop_integer!(usize u8 u16 u32 u64 isize i8 i16 i32 i64);
    }

    #[test]
    fn horner_loop_float() {
        macro_rules! test_horner_loop_float {
            ($($t:ident)+) => {
                $(
                    {
                        let x: $t = 2.0;
                        let c: Vec<$t> = vec![1.0, 2.0, 3.0];
                        let c1: Vec<$t> = vec![1.0];
                        assert_eq!(17.0, horner_loop(x, &c));
                        assert_eq!(1.0, horner_loop(x, &c1));
                    }
                )+
            }
        }

        test_horner_loop_float!(f32 f64);
    }

    #[test]
    #[should_panic(expected = "coefficients.len() must be greater than")]
    fn horner_loop_empty_vec() {
        let x = 2.0;
        let c: Vec<f64> = vec![];
        horner_loop(x, &c);
    }
}
