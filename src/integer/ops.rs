use crate::integer::Integer;
use core::ops::{Add, Mul, Neg, Rem, Sub};

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

// ADDITION

impl_single_binop!(Add, add, &Integer, Integer, Integer::add, Integer, ref rhs);
impl_single_binop!(Add, add, Integer, &Integer, Integer::add, Integer, ref self);
impl_single_binop!(Add, add, &Integer, &Integer, Integer::add, Integer);
impl_single_binop!(Add, add, Integer, Integer, Integer::add, Integer, ref self, ref rhs);

impl_single_binop!(Add, add, Integer, i8, Integer::add_c_long, Integer, ref self);
impl_single_binop!(Add, add, &Integer, i8, Integer::add_c_long, Integer);
impl_single_binop!(Add, add, Integer, &i8, Integer::add_c_long, Integer, ref self, deref rhs);
impl_single_binop!(Add, add, &Integer, &i8, Integer::add_c_long, Integer, deref rhs);

impl_single_binop!(Add, add, Integer, u8, Integer::add_c_long, Integer, ref self);
impl_single_binop!(Add, add, &Integer, u8, Integer::add_c_long, Integer);
impl_single_binop!(Add, add, Integer, &u8, Integer::add_c_long, Integer, ref self, deref rhs);
impl_single_binop!(Add, add, &Integer, &u8, Integer::add_c_long, Integer, deref rhs);

impl_single_binop!(Add, add, Integer, i16, Integer::add_c_long, Integer, ref self);
impl_single_binop!(Add, add, &Integer, i16, Integer::add_c_long, Integer);
impl_single_binop!(Add, add, Integer, &i16, Integer::add_c_long, Integer, ref self, deref rhs);
impl_single_binop!(Add, add, &Integer, &i16, Integer::add_c_long, Integer, deref rhs);

impl_single_binop!(Add, add, Integer, u16, Integer::add_c_long, Integer, ref self);
impl_single_binop!(Add, add, &Integer, u16, Integer::add_c_long, Integer);
impl_single_binop!(Add, add, Integer, &u16, Integer::add_c_long, Integer, ref self, deref rhs);
impl_single_binop!(Add, add, &Integer, &u16, Integer::add_c_long, Integer, deref rhs);

impl_single_binop!(Add, add, Integer, i32, Integer::add_c_long, Integer, ref self);
impl_single_binop!(Add, add, &Integer, i32, Integer::add_c_long, Integer);
impl_single_binop!(Add, add, Integer, &i32, Integer::add_c_long, Integer, ref self, deref rhs);
impl_single_binop!(Add, add, &Integer, &i32, Integer::add_c_long, Integer, deref rhs);

impl_single_binop!(Add, add, Integer, u32, Integer::add_c_long, Integer, ref self);
impl_single_binop!(Add, add, &Integer, u32, Integer::add_c_long, Integer);
impl_single_binop!(Add, add, Integer, &u32, Integer::add_c_long, Integer, ref self, deref rhs);
impl_single_binop!(Add, add, &Integer, &u32, Integer::add_c_long, Integer, deref rhs);

cfg_if::cfg_if! {
    if #[cfg(all(target_pointer_width = "64", not(windows)))] {
        impl_single_binop!(Add, add, Integer, i64, Integer::add_c_long, Integer, ref self);
        impl_single_binop!(Add, add, &Integer, i64, Integer::add_c_long, Integer);
        impl_single_binop!(Add, add, Integer, &i64, Integer::add_c_long, Integer, ref self, deref rhs);
        impl_single_binop!(Add, add, &Integer, &i64, Integer::add_c_long, Integer, deref rhs);
    } else {
        impl_single_binop!(Add, add, Integer, i64, Integer::add, Integer, ref self, into rhs);
        impl_single_binop!(Add, add, &Integer, i64, Integer::add, Integer, into rhs);
        impl_single_binop!(Add, add, Integer, &i64, Integer::add, Integer, ref self, into rhs);
        impl_single_binop!(Add, add, &Integer, &i64, Integer::add, Integer, into rhs);
    }
}

impl_single_binop!(Add, add, Integer, u64, Integer::add, Integer, ref self, into rhs);
impl_single_binop!(Add, add, &Integer, u64, Integer::add, Integer, into rhs);
impl_single_binop!(Add, add, Integer, &u64, Integer::add, Integer, ref self, into rhs);
impl_single_binop!(Add, add, &Integer, &u64, Integer::add, Integer, into rhs);

impl_single_binop!(Add, add, Integer, i128, Integer::add, Integer, ref self, into rhs);
impl_single_binop!(Add, add, &Integer, i128, Integer::add, Integer, into rhs);
impl_single_binop!(Add, add, Integer, &i128, Integer::add, Integer, ref self, into rhs);
impl_single_binop!(Add, add, &Integer, &i128, Integer::add, Integer, into rhs);

impl_single_binop!(Add, add, Integer, u128, Integer::add, Integer, ref self, into rhs);
impl_single_binop!(Add, add, &Integer, u128, Integer::add, Integer, into rhs);
impl_single_binop!(Add, add, Integer, &u128, Integer::add, Integer, ref self, into rhs);
impl_single_binop!(Add, add, &Integer, &u128, Integer::add, Integer, into rhs);

// SUBTRACTION

impl_single_binop!(
    Sub,
    sub,
    &Integer,
    Integer,
    Integer::subtract,
    Integer,
    ref rhs
);
impl_single_binop!(Sub, sub, Integer, &Integer, Integer::subtract, Integer, ref self);
impl_single_binop!(Sub, sub, &Integer, &Integer, Integer::subtract, Integer);
impl_single_binop!(Sub, sub, Integer, Integer, Integer::subtract, Integer, ref self, ref rhs);

impl_single_binop!(Sub, sub, Integer, i8, Integer::subtract_c_long, Integer, ref self);
impl_single_binop!(Sub, sub, &Integer, i8, Integer::subtract_c_long, Integer);
impl_single_binop!(Sub, sub, Integer, &i8, Integer::subtract_c_long, Integer, ref self, deref rhs);
impl_single_binop!(Sub, sub, &Integer, &i8, Integer::subtract_c_long, Integer, deref rhs);

impl_single_binop!(Sub, sub, Integer, u8, Integer::subtract_c_long, Integer, ref self);
impl_single_binop!(Sub, sub, &Integer, u8, Integer::subtract_c_long, Integer);
impl_single_binop!(Sub, sub, Integer, &u8, Integer::subtract_c_long, Integer, ref self, deref rhs);
impl_single_binop!(Sub, sub, &Integer, &u8, Integer::subtract_c_long, Integer, deref rhs);

impl_single_binop!(Sub, sub, Integer, i16, Integer::subtract_c_long, Integer, ref self);
impl_single_binop!(Sub, sub, &Integer, i16, Integer::subtract_c_long, Integer);
impl_single_binop!(Sub, sub, Integer, &i16, Integer::subtract_c_long, Integer, ref self, deref rhs);
impl_single_binop!(Sub, sub, &Integer, &i16, Integer::subtract_c_long, Integer, deref rhs);

impl_single_binop!(Sub, sub, Integer, u16, Integer::subtract_c_long, Integer, ref self);
impl_single_binop!(Sub, sub, &Integer, u16, Integer::subtract_c_long, Integer);
impl_single_binop!(Sub, sub, Integer, &u16, Integer::subtract_c_long, Integer, ref self, deref rhs);
impl_single_binop!(Sub, sub, &Integer, &u16, Integer::subtract_c_long, Integer, deref rhs);

impl_single_binop!(Sub, sub, Integer, i32, Integer::subtract_c_long, Integer, ref self);
impl_single_binop!(Sub, sub, &Integer, i32, Integer::subtract_c_long, Integer);
impl_single_binop!(Sub, sub, Integer, &i32, Integer::subtract_c_long, Integer, ref self, deref rhs);
impl_single_binop!(Sub, sub, &Integer, &i32, Integer::subtract_c_long, Integer, deref rhs);

impl_single_binop!(Sub, sub, Integer, u32, Integer::subtract_c_long, Integer, ref self);
impl_single_binop!(Sub, sub, &Integer, u32, Integer::subtract_c_long, Integer);
impl_single_binop!(Sub, sub, Integer, &u32, Integer::subtract_c_long, Integer, ref self, deref rhs);
impl_single_binop!(Sub, sub, &Integer, &u32, Integer::subtract_c_long, Integer, deref rhs);

cfg_if::cfg_if! {
    if #[cfg(all(target_pointer_width = "64", not(windows)))] {
        impl_single_binop!(Sub, sub, Integer, i64, Integer::subtract_c_long, Integer, ref self);
        impl_single_binop!(Sub, sub, &Integer, i64, Integer::subtract_c_long, Integer);
        impl_single_binop!(Sub, sub, Integer, &i64, Integer::subtract_c_long, Integer, ref self, deref rhs);
        impl_single_binop!(Sub, sub, &Integer, &i64, Integer::subtract_c_long, Integer, deref rhs);
    } else {
        impl_single_binop!(Sub, sub, Integer, i64, Integer::subtract, Integer, ref self, into rhs);
        impl_single_binop!(Sub, sub, &Integer, i64, Integer::subtract, Integer, into rhs);
        impl_single_binop!(Sub, sub, Integer, &i64, Integer::subtract, Integer, ref self, into rhs);
        impl_single_binop!(Sub, sub, &Integer, &i64, Integer::subtract, Integer, into rhs);
    }
}

impl_single_binop!(Sub, sub, Integer, u64, Integer::subtract, Integer, ref self, into rhs);
impl_single_binop!(Sub, sub, &Integer, u64, Integer::subtract, Integer, into rhs);
impl_single_binop!(Sub, sub, Integer, &u64, Integer::subtract, Integer, ref self, into rhs);
impl_single_binop!(Sub, sub, &Integer, &u64, Integer::subtract, Integer, into rhs);

impl_single_binop!(Sub, sub, Integer, i128, Integer::subtract, Integer, ref self, into rhs);
impl_single_binop!(Sub, sub, &Integer, i128, Integer::subtract, Integer, into rhs);
impl_single_binop!(Sub, sub, Integer, &i128, Integer::subtract, Integer, ref self, into rhs);
impl_single_binop!(Sub, sub, &Integer, &i128, Integer::subtract, Integer, into rhs);

impl_single_binop!(Sub, sub, Integer, u128, Integer::subtract, Integer, ref self, into rhs);
impl_single_binop!(Sub, sub, &Integer, u128, Integer::subtract, Integer, into rhs);
impl_single_binop!(Sub, sub, Integer, &u128, Integer::subtract, Integer, ref self, into rhs);
impl_single_binop!(Sub, sub, &Integer, &u128, Integer::subtract, Integer, into rhs);

// MULTIPLICATION

impl_single_binop!(
    Mul,
    mul,
    &Integer,
    Integer,
    Integer::multiply,
    Integer,
    ref rhs
);
impl_single_binop!(Mul, mul, Integer, &Integer, Integer::multiply, Integer, ref self);
impl_single_binop!(Mul, mul, &Integer, &Integer, Integer::multiply, Integer);
impl_single_binop!(Mul, mul, Integer, Integer, Integer::multiply, Integer, ref self, ref rhs);

impl_single_binop!(Mul, mul, Integer, i8, Integer::multiply_c_long, Integer, ref self);
impl_single_binop!(Mul, mul, &Integer, i8, Integer::multiply_c_long, Integer);
impl_single_binop!(Mul, mul, Integer, &i8, Integer::multiply_c_long, Integer, ref self, deref rhs);
impl_single_binop!(Mul, mul, &Integer, &i8, Integer::multiply_c_long, Integer, deref rhs);

impl_single_binop!(Mul, mul, Integer, u8, Integer::multiply_c_long, Integer, ref self);
impl_single_binop!(Mul, mul, &Integer, u8, Integer::multiply_c_long, Integer);
impl_single_binop!(Mul, mul, Integer, &u8, Integer::multiply_c_long, Integer, ref self, deref rhs);
impl_single_binop!(Mul, mul, &Integer, &u8, Integer::multiply_c_long, Integer, deref rhs);

impl_single_binop!(Mul, mul, Integer, i16, Integer::multiply_c_long, Integer, ref self);
impl_single_binop!(Mul, mul, &Integer, i16, Integer::multiply_c_long, Integer);
impl_single_binop!(Mul, mul, Integer, &i16, Integer::multiply_c_long, Integer, ref self, deref rhs);
impl_single_binop!(Mul, mul, &Integer, &i16, Integer::multiply_c_long, Integer, deref rhs);

impl_single_binop!(Mul, mul, Integer, u16, Integer::multiply_c_long, Integer, ref self);
impl_single_binop!(Mul, mul, &Integer, u16, Integer::multiply_c_long, Integer);
impl_single_binop!(Mul, mul, Integer, &u16, Integer::multiply_c_long, Integer, ref self, deref rhs);
impl_single_binop!(Mul, mul, &Integer, &u16, Integer::multiply_c_long, Integer, deref rhs);

impl_single_binop!(Mul, mul, Integer, i32, Integer::multiply_c_long, Integer, ref self);
impl_single_binop!(Mul, mul, &Integer, i32, Integer::multiply_c_long, Integer);
impl_single_binop!(Mul, mul, Integer, &i32, Integer::multiply_c_long, Integer, ref self, deref rhs);
impl_single_binop!(Mul, mul, &Integer, &i32, Integer::multiply_c_long, Integer, deref rhs);

impl_single_binop!(Mul, mul, Integer, u32, Integer::multiply_c_long, Integer, ref self);
impl_single_binop!(Mul, mul, &Integer, u32, Integer::multiply_c_long, Integer);
impl_single_binop!(Mul, mul, Integer, &u32, Integer::multiply_c_long, Integer, ref self, deref rhs);
impl_single_binop!(Mul, mul, &Integer, &u32, Integer::multiply_c_long, Integer, deref rhs);

cfg_if::cfg_if! {
    if #[cfg(all(target_pointer_width = "64", not(windows)))] {
        impl_single_binop!(Mul, mul, Integer, i64, Integer::multiply_c_long, Integer, ref self);
        impl_single_binop!(Mul, mul, &Integer, i64, Integer::multiply_c_long, Integer);
        impl_single_binop!(Mul, mul, Integer, &i64, Integer::multiply_c_long, Integer, ref self, deref rhs);
        impl_single_binop!(Mul, mul, &Integer, &i64, Integer::multiply_c_long, Integer, deref rhs);
    } else {
        impl_single_binop!(Mul, mul, Integer, i64, Integer::multiply, Integer, ref self, into rhs);
        impl_single_binop!(Mul, mul, &Integer, i64, Integer::multiply, Integer, into rhs);
        impl_single_binop!(Mul, mul, Integer, &i64, Integer::multiply, Integer, ref self, into rhs);
        impl_single_binop!(Mul, mul, &Integer, &i64, Integer::multiply, Integer, into rhs);
    }
}

impl_single_binop!(Mul, mul, Integer, u64, Integer::multiply, Integer, ref self, into rhs);
impl_single_binop!(Mul, mul, &Integer, u64, Integer::multiply, Integer, into rhs);
impl_single_binop!(Mul, mul, Integer, &u64, Integer::multiply, Integer, ref self, into rhs);
impl_single_binop!(Mul, mul, &Integer, &u64, Integer::multiply, Integer, into rhs);

impl_single_binop!(Mul, mul, Integer, i128, Integer::multiply, Integer, ref self, into rhs);
impl_single_binop!(Mul, mul, &Integer, i128, Integer::multiply, Integer, into rhs);
impl_single_binop!(Mul, mul, Integer, &i128, Integer::multiply, Integer, ref self, into rhs);
impl_single_binop!(Mul, mul, &Integer, &i128, Integer::multiply, Integer, into rhs);

impl_single_binop!(Mul, mul, Integer, u128, Integer::multiply, Integer, ref self, into rhs);
impl_single_binop!(Mul, mul, &Integer, u128, Integer::multiply, Integer, into rhs);
impl_single_binop!(Mul, mul, Integer, &u128, Integer::multiply, Integer, ref self, into rhs);
impl_single_binop!(Mul, mul, &Integer, &u128, Integer::multiply, Integer, into rhs);

// REMAINDER

impl_single_binop!(
    Rem,
    rem,
    &Integer,
    Integer,
    Integer::remainder,
    Integer,
    ref rhs
);
impl_single_binop!(Rem, rem, Integer, &Integer, Integer::remainder, Integer, ref self);
impl_single_binop!(Rem, rem, &Integer, &Integer, Integer::remainder, Integer);
impl_single_binop!(Rem, rem, Integer, Integer, Integer::remainder, Integer, ref self, ref rhs);

impl_single_binop!(Rem, rem, Integer, i8, Integer::remainder_c_long, i8, ref self);
impl_single_binop!(Rem, rem, &Integer, i8, Integer::remainder_c_long, i8);
impl_single_binop!(Rem, rem, Integer, &i8, Integer::remainder_c_long, i8, ref self, deref rhs);
impl_single_binop!(Rem, rem, &Integer, &i8, Integer::remainder_c_long, i8, deref rhs);

impl_single_binop!(Rem, rem, Integer, u8, Integer::remainder_c_long, u8, ref self);
impl_single_binop!(Rem, rem, &Integer, u8, Integer::remainder_c_long, u8);
impl_single_binop!(Rem, rem, Integer, &u8, Integer::remainder_c_long, u8, ref self, deref rhs);
impl_single_binop!(Rem, rem, &Integer, &u8, Integer::remainder_c_long, u8, deref rhs);

impl_single_binop!(Rem, rem, Integer, i16, Integer::remainder_c_long, i16, ref self);
impl_single_binop!(Rem, rem, &Integer, i16, Integer::remainder_c_long, i16);
impl_single_binop!(Rem, rem, Integer, &i16, Integer::remainder_c_long, i16, ref self, deref rhs);
impl_single_binop!(Rem, rem, &Integer, &i16, Integer::remainder_c_long, i16, deref rhs);

impl_single_binop!(Rem, rem, Integer, u16, Integer::remainder_c_long, u16, ref self);
impl_single_binop!(Rem, rem, &Integer, u16, Integer::remainder_c_long, u16);
impl_single_binop!(Rem, rem, Integer, &u16, Integer::remainder_c_long, u16, ref self, deref rhs);
impl_single_binop!(Rem, rem, &Integer, &u16, Integer::remainder_c_long, u16, deref rhs);

impl_single_binop!(Rem, rem, Integer, i32, Integer::remainder_c_long, i32, ref self);
impl_single_binop!(Rem, rem, &Integer, i32, Integer::remainder_c_long, i32);
impl_single_binop!(Rem, rem, Integer, &i32, Integer::remainder_c_long, i32, ref self, deref rhs);
impl_single_binop!(Rem, rem, &Integer, &i32, Integer::remainder_c_long, i32, deref rhs);

impl_single_binop!(Rem, rem, Integer, u32, Integer::remainder_c_long, u32, ref self);
impl_single_binop!(Rem, rem, &Integer, u32, Integer::remainder_c_long, u32);
impl_single_binop!(Rem, rem, Integer, &u32, Integer::remainder_c_long, u32, ref self, deref rhs);
impl_single_binop!(Rem, rem, &Integer, &u32, Integer::remainder_c_long, u32, deref rhs);

impl_single_binop!(Rem, rem, Integer, i64, Integer::remainder, Integer, ref self, into rhs);
impl_single_binop!(Rem, rem, &Integer, i64, Integer::remainder, Integer, into rhs);
impl_single_binop!(Rem, rem, Integer, &i64, Integer::remainder, Integer, ref self, into rhs);
impl_single_binop!(Rem, rem, &Integer, &i64, Integer::remainder, Integer, into rhs);

impl_single_binop!(Rem, rem, Integer, u64, Integer::remainder, Integer, ref self, into rhs);
impl_single_binop!(Rem, rem, &Integer, u64, Integer::remainder, Integer, into rhs);
impl_single_binop!(Rem, rem, Integer, &u64, Integer::remainder, Integer, ref self, into rhs);
impl_single_binop!(Rem, rem, &Integer, &u64, Integer::remainder, Integer, into rhs);

impl_single_binop!(Rem, rem, Integer, i128, Integer::remainder, Integer, ref self, into rhs);
impl_single_binop!(Rem, rem, &Integer, i128, Integer::remainder, Integer, into rhs);
impl_single_binop!(Rem, rem, Integer, &i128, Integer::remainder, Integer, ref self, into rhs);
impl_single_binop!(Rem, rem, &Integer, &i128, Integer::remainder, Integer, into rhs);

impl_single_binop!(Rem, rem, Integer, u128, Integer::remainder, Integer, ref self, into rhs);
impl_single_binop!(Rem, rem, &Integer, u128, Integer::remainder, Integer, into rhs);
impl_single_binop!(Rem, rem, Integer, &u128, Integer::remainder, Integer, ref self, into rhs);
impl_single_binop!(Rem, rem, &Integer, &u128, Integer::remainder, Integer, into rhs);

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
        let c = a.add(&b);

        let string_repr = c.to_string();
        assert_eq!(&string_repr, "44444");
    }

    #[test]
    fn subtract_integers() {
        let a = Integer::from_c_long(12345);
        let b = Integer::from_c_long(1234);
        let c = a.sub(&b);

        let string_repr = c.to_string();
        assert_eq!(&string_repr, "11111");
    }

    #[test]
    fn multiply_integers() {
        let a = Integer::from_c_long(50505);
        let b = Integer::from_c_long(5050);
        let c = a.mul(&b);

        let string_repr = c.to_string();
        assert_eq!(&string_repr, "255050250");
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
}
