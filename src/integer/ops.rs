use crate::error::RimathError;
use crate::integer::Integer;
use core::ops::Neg;
use core::{convert::TryInto, ptr};
use std::os::raw::c_long;

macro_rules! impl_single_binop {
    ($op_path:ident, $op_fn:ident, $celf:ty, $rhs:ty, $fn:path, $ret:ty, ref self, ref rhs) => {
        impl $op_path<$rhs> for $celf {
            type Output = $ret;

            fn $op_fn(self, rhs: $rhs) -> Self::Output {
                $fn(&self, &rhs)
            }
        }
    };
    ($op_path:ident, $op_fn:ident, $celf:ty, $rhs:ty, $fn:path, $ret:ty, ref self) => {
        impl $op_path<$rhs> for $celf {
            type Output = $ret;

            fn $op_fn(self, rhs: $rhs) -> Self::Output {
                $fn(&self, rhs)
            }
        }
    };
    ($op_path:ident, $op_fn:ident, $celf:ty, $rhs:ty, $fn:path, $ret:ty, ref rhs) => {
        impl $op_path<$rhs> for $celf {
            type Output = $ret;

            fn $op_fn(self, rhs: $rhs) -> Self::Output {
                $fn(self, &rhs)
            }
        }
    };
    ($op_path:ident, $op_fn:ident, $celf:ty, $rhs:ty, $fn:path, $ret:ty) => {
        impl $op_path<$rhs> for $celf {
            type Output = $ret;

            fn $op_fn(self, rhs: $rhs) -> Self::Output {
                $fn(self, rhs)
            }
        }
    };
    ($op_path:ident, $op_fn:ident, $celf:ty, $rhs:ty, $fn:path, $ret:ty, deref rhs) => {
        impl $op_path<$rhs> for $celf {
            type Output = $ret;

            fn $op_fn(self, rhs: $rhs) -> Self::Output {
                $fn(self, *rhs)
            }
        }
    };
    ($op_path:ident, $op_fn:ident, $celf:ty, $rhs:ty, $fn:path, $ret:ty, ref self, deref rhs) => {
        impl $op_path<$rhs> for $celf {
            type Output = $ret;

            fn $op_fn(self, rhs: $rhs) -> Self::Output {
                $fn(&self, *rhs)
            }
        }
    };
    ($op_path:ident, $op_fn:ident, $celf:ty, $rhs:ty, $fn:path, $ret:ty, into rhs) => {
        impl $op_path<$rhs> for $celf {
            type Output = $ret;

            fn $op_fn(self, rhs: $rhs) -> Self::Output {
                $fn(self, &Integer::from(rhs))
            }
        }
    };
    ($op_path:ident, $op_fn:ident, $celf:ty, $rhs:ty, $fn:path, $ret:ty, ref self, into rhs) => {
        impl $op_path<$rhs> for $celf {
            type Output = $ret;

            fn $op_fn(self, rhs: $rhs) -> Self::Output {
                $fn(&self, &Integer::from(rhs))
            }
        }
    };
}

mod addition;
mod division;
mod multiplication;
mod remainder;
mod subtraction;

// NEGATION

impl Neg for Integer {
    type Output = Integer;

    fn neg(self) -> Self::Output {
        self.negate()
    }
}

impl Neg for &Integer {
    type Output = Integer;

    fn neg(self) -> Self::Output {
        self.negate()
    }
}

macro_rules! integer_binops_fn {
    ($name:ident, $raw_fn:path, $c_long_name:ident, $c_long_fn:path) => {
        /// $name two integers
        pub fn $name(&self, other: &Self) -> Self {
            let self_raw = self.as_mut_ptr();
            let other_raw = other.as_mut_ptr();

            let result_int = Integer::new();
            let result_raw = result_int.as_mut_ptr();

            let op_res = unsafe { $raw_fn(self_raw, other_raw, result_raw) };

            if op_res != unsafe { imath_sys::MP_OK } {
                panic!("Operation failed! {:?}", op_res);
            }

            result_int
        }

        pub(crate) fn $c_long_name(&self, value: impl Into<c_long>) -> Self {
            let self_raw = self.as_mut_ptr();
            let result_int = Integer::new();
            let result_raw = result_int.as_mut_ptr();

            let op_res = unsafe { $c_long_fn(self_raw, value.into(), result_raw) };

            if op_res != unsafe { imath_sys::MP_OK } {
                panic!("Operation failed! {:?}", op_res);
            }

            result_int
        }
    };
}

impl Integer {
    integer_binops_fn!(
        add,
        imath_sys::mp_int_add,
        add_c_long,
        imath_sys::mp_int_add_value
    );

    integer_binops_fn!(
        subtract,
        imath_sys::mp_int_sub,
        subtract_c_long,
        imath_sys::mp_int_sub_value
    );

    integer_binops_fn!(
        multiply,
        imath_sys::mp_int_mul,
        multiply_c_long,
        imath_sys::mp_int_mul_value
    );

    /// Return the additive inverse
    pub fn negate(&self) -> Self {
        let self_raw = self.as_mut_ptr();
        let result_int = Integer::new();
        let result_raw = result_int.as_mut_ptr();

        let op_res = unsafe { imath_sys::mp_int_neg(self_raw, result_raw) };

        if op_res != unsafe { imath_sys::MP_OK } {
            panic!("Operation failed! {:?}", op_res);
        }

        result_int
    }

    /// Return the absolute value
    pub fn absolute_value(&self) -> Self {
        let self_raw = self.as_mut_ptr();
        let result_int = Integer::new();
        let result_raw = result_int.as_mut_ptr();

        let op_res = unsafe { imath_sys::mp_int_abs(self_raw, result_raw) };

        if op_res != unsafe { imath_sys::MP_OK } {
            panic!("Operation failed! {:?}", op_res);
        }

        result_int
    }

    fn mp_int_div(
        dividend: &Integer,
        divisor: &Integer,
        out_quotient: imath_sys::mp_int,
        out_remainder: imath_sys::mp_int,
    ) {
        assert!(!(out_quotient.is_null() && out_remainder.is_null()));

        let dividend_raw = dividend.as_mut_ptr();
        let divisor_raw = divisor.as_mut_ptr();

        let op_res = unsafe {
            imath_sys::mp_int_div(dividend_raw, divisor_raw, out_quotient, out_remainder)
        };

        if op_res != unsafe { imath_sys::MP_OK } {
            panic!("Operation failed! {:?}", op_res);
        }
    }

    /// Divide two integers and return quotient and remainder
    pub fn divide_full(&self, rhs: &Self) -> (Self, Self) {
        let quotient = Integer::new();
        let quotient_raw = quotient.as_mut_ptr();
        let remainder = Integer::new();
        let remainder_raw = remainder.as_mut_ptr();

        Integer::mp_int_div(self, rhs, quotient_raw, remainder_raw);

        (quotient, remainder)
    }

    /// Divide two integers and return only quotient
    pub fn divide(&self, rhs: &Self) -> Self {
        let quotient = Integer::new();
        let quotient_raw = quotient.as_mut_ptr();

        Integer::mp_int_div(self, rhs, quotient_raw, ptr::null_mut());

        quotient
    }

    /// Divide two integers and return only remainder
    pub fn remainder(&self, other: &Self) -> Self {
        let result = Integer::new();
        let result_raw = result.as_mut_ptr();

        Integer::mp_int_div(self, other, result_raw, ptr::null_mut());

        result
    }

    fn mp_int_div_value(
        dividend: &Integer,
        divisor: impl Into<c_long>,
        out_quotient: imath_sys::mp_int,
        out_remainder: *mut c_long,
    ) {
        let divident_raw = dividend.as_mut_ptr();

        let op_res = unsafe {
            imath_sys::mp_int_div_value(divident_raw, divisor.into(), out_quotient, out_remainder)
        };

        if op_res != unsafe { imath_sys::MP_OK } {
            panic!("Operation failed! {:?}", op_res);
        }
    }

    #[allow(dead_code)]
    pub(crate) fn divide_full_c_long<V>(&self, value: V) -> (Integer, V)
    where
        V: Into<c_long>,
        c_long: TryInto<V>,
    {
        let mut remainder: c_long = 0;
        let quotient = Integer::new();
        let quotient_raw = quotient.as_mut_ptr();

        Integer::mp_int_div_value(self, value.into(), quotient_raw, &mut remainder);

        // This is safe bc the modulo operation will always return within the range
        // [0, value].
        (
            quotient,
            remainder
                .try_into()
                .map_err(|_| RimathError::RemainedOutsideBounds)
                .unwrap(),
        )
    }

    pub(crate) fn divide_c_long(&self, value: impl Into<c_long>) -> Self {
        let quotient = Integer::new();
        let quotient_raw = quotient.as_mut_ptr();

        Integer::mp_int_div_value(self, value, quotient_raw, ptr::null_mut());

        quotient
    }

    pub(crate) fn remainder_c_long<V>(&self, value: V) -> V
    where
        V: Into<c_long>,
        c_long: TryInto<V>,
    {
        let mut result: c_long = 0;
        let result_raw = (&mut result) as *mut _;

        Integer::mp_int_div_value(self, value, ptr::null_mut(), result_raw);

        // This is safe bc the modulo operation will always return within the range
        // [0, value].
        result
            .try_into()
            .map_err(|_| RimathError::RemainedOutsideBounds)
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn fibonacci(mut n: usize) -> Integer {
        let mut f0 = Integer::new();
        let mut f1 = Integer::from(1);

        if n == 0 {
            f0
        } else if n == 1 {
            f1
        } else {
            n -= 1;

            while n > 0 {
                let temp = f1;
                f1 = &temp + f0;
                f0 = temp;

                n -= 1;
            }

            f1
        }
    }

    #[test]
    fn fibonacci_values() {
        assert_eq!(fibonacci(1).to_string(), "1");
        assert_eq!(fibonacci(10).to_string(), "55");
        assert_eq!(fibonacci(50).to_string(), "12586269025");
        assert_eq!(fibonacci(100).to_string(), "354224848179261915075");
    }

    #[test]
    fn add_integers() {
        let a = Integer::from_c_long(11111);
        let b = Integer::from_c_long(33333);
        let c = a + b;

        let string_repr = c.to_string();
        assert_eq!(&string_repr, "44444");
    }

    #[test]
    fn subtract_integers() {
        let a = Integer::from_c_long(12345);
        let b = Integer::from_c_long(1234);
        let c = a - b;

        let string_repr = c.to_string();
        assert_eq!(&string_repr, "11111");
    }

    #[test]
    fn multiply_integers() {
        let a = Integer::from_c_long(50505);
        let b = Integer::from_c_long(5050);
        let c = a * b;

        let string_repr = c.to_string();
        assert_eq!(&string_repr, "255050250");
    }

    #[test]
    fn divide_integers() {
        let a: Integer = "52384129912341238437480192384".parse().unwrap();

        #[allow(clippy::eq_op)]
        let one = &a / &a;
        assert_eq!(one, 1);
        assert_eq!(&a / 10_000_000_000_000_000_000_000u128, 5_238_412);

        let b: Integer = 1_234_567.into();
        assert_eq!(&b / 123_456, 10);
        assert_eq!(&b / 3, 411_522);
        assert_eq!(&b / -3, -411_522);
        assert_eq!(&b / -230, -5_367)
    }

    #[test]
    fn remainder_integers() {
        let a: Integer = "52384129912341238437480192384".parse().unwrap();

        let res_u8: u8 = &a % 122u8;
        assert_eq!(res_u8, 62);

        let res_u16: u16 = &a % 60_000u16;
        assert_eq!(res_u16, 12384);

        let res_u32: u32 = &a % 127_384u32;
        assert_eq!(res_u32, 85248);
    }

    #[test]
    fn negate_integers() {
        let a: Integer = "52384129912341238437480192384".parse().unwrap();
        let b = -a;

        let string_repr = b.to_string();
        assert_eq!(&string_repr, "-52384129912341238437480192384");
    }

    #[test]
    fn absolute_value_integer() {
        let neg_int: Integer = "-37129740".parse().unwrap();
        let pos_int: Integer = "37129740".parse().unwrap();

        assert_eq!(neg_int.absolute_value(), pos_int);
        assert_eq!(pos_int.absolute_value(), 37_129_740);
    }
}
