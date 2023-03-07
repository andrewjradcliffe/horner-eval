use num_traits::{MulAdd, Zero};

#[inline]
pub fn muladd<T: MulAdd + MulAdd<Output = T>>(x: T, a: T, b: T) -> T { x.mul_add(a, b) }

/// Evaluate the polynomial a_n * x^n + a_n_1 * x^(n-1) + ... + a_1 * x + a_0
/// via Horner's rule. This macro unrolls what would otherwise be a loop into the
/// expression `(...(a_n * x + a_n_1) * x + ... + a_1) * x + a_0`
///
/// # Examples
/// ```
/// let x = 2.0_f64;
///
/// assert_eq!(17.0, horner(x, 1.0, 2.0, 3.0));
/// ```
#[macro_export]
macro_rules! horner {
    ( $x:tt, $a0:tt, $a1:tt ) => {
        $crate::muladd($a1, $x, $a0)
    };
    ( $x:tt, $a0:tt, $( $a1:tt ),+ ) => {
        $crate::muladd( $crate::horner!( $x, $( $a1 ),+ ), $x, $a0 )
    };
    ( $x:tt, $a0:tt ) => { $a0 }
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

    // Several ways to express
    fn horner_eval<T>(x: T, coefficients: &[T]) -> T
    where
        T: Copy + Zero + MulAdd + MulAdd<Output = T>
    {
        let mut result = T::zero();
        let n = coefficients.len();
        for i in 0..n {
            result = result.mul_add(x, coefficients[n - i - 1]);
        };
        result
    }
    fn horner_eval2<T>(x: T, coefficients: &[T]) -> T
    where
        T: Copy + Zero + MulAdd + MulAdd<Output = T>
    {
        // coefficients
        //     .iter()
        //     .rev()
        //     .fold(T::zero(), |result, &a| result.mul_add(x, a))
        // Equivalent to:
        coefficients
            .iter()
            .rfold(T::zero(), |result, &a| result.mul_add(x, a))
    }

    fn horner_eval3<T>(x: T, coefficients: &[T]) -> T
    where
        T: Copy + MulAdd + MulAdd<Output = T>
    {
        let n = coefficients.len();
        let a_n = coefficients[n-1];
        // coefficients[0..n-1]
        //     .iter()
        //     .rev()
        //     .fold(a_n, |result, &a| result.mul_add(x, a))
        // Equivalent to:
        coefficients[0..n-1]
            .iter()
            .rfold(a_n, |result, &a| result.mul_add(x, a))
    }


    #[test]
    fn internal_horner_eval() {
        let x = 2.0;
        let c: Vec<f64> = vec![1.0, 2.0, 3.0];
        assert_eq!(17.0, horner_eval(x, &c));
        assert_eq!(17.0, horner_eval2(x, &c));
        assert_eq!(17.0, horner_eval3(x, &c));
        assert_eq!(1.0, horner_eval3(x, &vec![1.0]));
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

    // #[test]
    // fn horner_expr() {
    //     let
    // }

}
