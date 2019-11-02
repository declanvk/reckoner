use crate::integer::Integer;
use core::ops::{Add, Mul, Sub};

macro_rules! impl_single_op {
    ($op_path:ident, $op_fn:ident, $celf:ty, $rhs:ty, $fn:path, ref self, ref rhs) => {
        impl $op_path<$rhs> for $celf {
            type Output = Integer;

            fn $op_fn(self, rhs: $rhs) -> Self::Output {
                $fn(&self, &rhs)
            }
        }
    };
    ($op_path:ident, $op_fn:ident, $celf:ty, $rhs:ty, $fn:path, ref self) => {
        impl $op_path<$rhs> for $celf {
            type Output = Integer;

            fn $op_fn(self, rhs: $rhs) -> Self::Output {
                $fn(&self, rhs)
            }
        }
    };
    ($op_path:ident, $op_fn:ident, $celf:ty, $rhs:ty, $fn:path, ref rhs) => {
        impl $op_path<$rhs> for $celf {
            type Output = Integer;

            fn $op_fn(self, rhs: $rhs) -> Self::Output {
                $fn(self, &rhs)
            }
        }
    };
    ($op_path:ident, $op_fn:ident, $celf:ty, $rhs:ty, $fn:path) => {
        impl $op_path<$rhs> for $celf {
            type Output = Integer;

            fn $op_fn(self, rhs: $rhs) -> Self::Output {
                $fn(self, rhs)
            }
        }
    };
    ($op_path:ident, $op_fn:ident, $celf:ty, $rhs:ty, $fn:path, deref rhs) => {
        impl $op_path<$rhs> for $celf {
            type Output = Integer;

            fn $op_fn(self, rhs: $rhs) -> Self::Output {
                $fn(self, *rhs)
            }
        }
    };
    ($op_path:ident, $op_fn:ident, $celf:ty, $rhs:ty, $fn:path, ref self, deref rhs) => {
        impl $op_path<$rhs> for $celf {
            type Output = Integer;

            fn $op_fn(self, rhs: $rhs) -> Self::Output {
                $fn(&self, *rhs)
            }
        }
    };
    ($op_path:ident, $op_fn:ident, $celf:ty, $rhs:ty, $fn:path, into rhs) => {
        impl $op_path<$rhs> for $celf {
            type Output = Integer;

            fn $op_fn(self, rhs: $rhs) -> Self::Output {
                $fn(self, &Integer::from(rhs))
            }
        }
    };
    ($op_path:ident, $op_fn:ident, $celf:ty, $rhs:ty, $fn:path, ref self, into rhs) => {
        impl $op_path<$rhs> for $celf {
            type Output = Integer;

            fn $op_fn(self, rhs: $rhs) -> Self::Output {
                $fn(&self, &Integer::from(rhs))
            }
        }
    };
}

// ADDITION

impl_single_op!(Add, add, &Integer, Integer, Integer::add, ref rhs);
impl_single_op!(Add, add, Integer, &Integer, Integer::add, ref self);
impl_single_op!(Add, add, &Integer, &Integer, Integer::add);
impl_single_op!(Add, add, Integer, Integer, Integer::add, ref self, ref rhs);

impl_single_op!(Add, add, Integer, i8, Integer::add_c_long, ref self);
impl_single_op!(Add, add, &Integer, i8, Integer::add_c_long);
impl_single_op!(Add, add, Integer, &i8, Integer::add_c_long, ref self, deref rhs);
impl_single_op!(Add, add, &Integer, &i8, Integer::add_c_long, deref rhs);

impl_single_op!(Add, add, Integer, u8, Integer::add_c_long, ref self);
impl_single_op!(Add, add, &Integer, u8, Integer::add_c_long);
impl_single_op!(Add, add, Integer, &u8, Integer::add_c_long, ref self, deref rhs);
impl_single_op!(Add, add, &Integer, &u8, Integer::add_c_long, deref rhs);

impl_single_op!(Add, add, Integer, i16, Integer::add_c_long, ref self);
impl_single_op!(Add, add, &Integer, i16, Integer::add_c_long);
impl_single_op!(Add, add, Integer, &i16, Integer::add_c_long, ref self, deref rhs);
impl_single_op!(Add, add, &Integer, &i16, Integer::add_c_long, deref rhs);

impl_single_op!(Add, add, Integer, u16, Integer::add_c_long, ref self);
impl_single_op!(Add, add, &Integer, u16, Integer::add_c_long);
impl_single_op!(Add, add, Integer, &u16, Integer::add_c_long, ref self, deref rhs);
impl_single_op!(Add, add, &Integer, &u16, Integer::add_c_long, deref rhs);

impl_single_op!(Add, add, Integer, i32, Integer::add_c_long, ref self);
impl_single_op!(Add, add, &Integer, i32, Integer::add_c_long);
impl_single_op!(Add, add, Integer, &i32, Integer::add_c_long, ref self, deref rhs);
impl_single_op!(Add, add, &Integer, &i32, Integer::add_c_long, deref rhs);

impl_single_op!(Add, add, Integer, u32, Integer::add_c_long, ref self);
impl_single_op!(Add, add, &Integer, u32, Integer::add_c_long);
impl_single_op!(Add, add, Integer, &u32, Integer::add_c_long, ref self, deref rhs);
impl_single_op!(Add, add, &Integer, &u32, Integer::add_c_long, deref rhs);

cfg_if::cfg_if! {
    if #[cfg(all(target_pointer_width = "64", not(windows)))] {
        impl_single_op!(Add, add, Integer, i64, Integer::add_c_long, ref self);
        impl_single_op!(Add, add, &Integer, i64, Integer::add_c_long);
        impl_single_op!(Add, add, Integer, &i64, Integer::add_c_long, ref self, deref rhs);
        impl_single_op!(Add, add, &Integer, &i64, Integer::add_c_long, deref rhs);
    } else {
        impl_single_op!(Add, add, Integer, i64, Integer::add, ref self, into rhs);
        impl_single_op!(Add, add, &Integer, i64, Integer::add, into rhs);
        impl_single_op!(Add, add, Integer, &i64, Integer::add, ref self, into rhs);
        impl_single_op!(Add, add, &Integer, &i64, Integer::add, into rhs);
    }
}

impl_single_op!(Add, add, Integer, u64, Integer::add, ref self, into rhs);
impl_single_op!(Add, add, &Integer, u64, Integer::add, into rhs);
impl_single_op!(Add, add, Integer, &u64, Integer::add, ref self, into rhs);
impl_single_op!(Add, add, &Integer, &u64, Integer::add, into rhs);

impl_single_op!(Add, add, Integer, i128, Integer::add, ref self, into rhs);
impl_single_op!(Add, add, &Integer, i128, Integer::add, into rhs);
impl_single_op!(Add, add, Integer, &i128, Integer::add, ref self, into rhs);
impl_single_op!(Add, add, &Integer, &i128, Integer::add, into rhs);

impl_single_op!(Add, add, Integer, u128, Integer::add, ref self, into rhs);
impl_single_op!(Add, add, &Integer, u128, Integer::add, into rhs);
impl_single_op!(Add, add, Integer, &u128, Integer::add, ref self, into rhs);
impl_single_op!(Add, add, &Integer, &u128, Integer::add, into rhs);

// SUBTRACTION

impl_single_op!(Sub, sub, &Integer, Integer, Integer::subtract, ref rhs);
impl_single_op!(Sub, sub, Integer, &Integer, Integer::subtract, ref self);
impl_single_op!(Sub, sub, &Integer, &Integer, Integer::subtract);
impl_single_op!(Sub, sub, Integer, Integer, Integer::subtract, ref self, ref rhs);

impl_single_op!(Sub, sub, Integer, i8, Integer::subtract_c_long, ref self);
impl_single_op!(Sub, sub, &Integer, i8, Integer::subtract_c_long);
impl_single_op!(Sub, sub, Integer, &i8, Integer::subtract_c_long, ref self, deref rhs);
impl_single_op!(Sub, sub, &Integer, &i8, Integer::subtract_c_long, deref rhs);

impl_single_op!(Sub, sub, Integer, u8, Integer::subtract_c_long, ref self);
impl_single_op!(Sub, sub, &Integer, u8, Integer::subtract_c_long);
impl_single_op!(Sub, sub, Integer, &u8, Integer::subtract_c_long, ref self, deref rhs);
impl_single_op!(Sub, sub, &Integer, &u8, Integer::subtract_c_long, deref rhs);

impl_single_op!(Sub, sub, Integer, i16, Integer::subtract_c_long, ref self);
impl_single_op!(Sub, sub, &Integer, i16, Integer::subtract_c_long);
impl_single_op!(Sub, sub, Integer, &i16, Integer::subtract_c_long, ref self, deref rhs);
impl_single_op!(Sub, sub, &Integer, &i16, Integer::subtract_c_long, deref rhs);

impl_single_op!(Sub, sub, Integer, u16, Integer::subtract_c_long, ref self);
impl_single_op!(Sub, sub, &Integer, u16, Integer::subtract_c_long);
impl_single_op!(Sub, sub, Integer, &u16, Integer::subtract_c_long, ref self, deref rhs);
impl_single_op!(Sub, sub, &Integer, &u16, Integer::subtract_c_long, deref rhs);

impl_single_op!(Sub, sub, Integer, i32, Integer::subtract_c_long, ref self);
impl_single_op!(Sub, sub, &Integer, i32, Integer::subtract_c_long);
impl_single_op!(Sub, sub, Integer, &i32, Integer::subtract_c_long, ref self, deref rhs);
impl_single_op!(Sub, sub, &Integer, &i32, Integer::subtract_c_long, deref rhs);

impl_single_op!(Sub, sub, Integer, u32, Integer::subtract_c_long, ref self);
impl_single_op!(Sub, sub, &Integer, u32, Integer::subtract_c_long);
impl_single_op!(Sub, sub, Integer, &u32, Integer::subtract_c_long, ref self, deref rhs);
impl_single_op!(Sub, sub, &Integer, &u32, Integer::subtract_c_long, deref rhs);

cfg_if::cfg_if! {
    if #[cfg(all(target_pointer_width = "64", not(windows)))] {
        impl_single_op!(Sub, sub, Integer, i64, Integer::subtract_c_long, ref self);
        impl_single_op!(Sub, sub, &Integer, i64, Integer::subtract_c_long);
        impl_single_op!(Sub, sub, Integer, &i64, Integer::subtract_c_long, ref self, deref rhs);
        impl_single_op!(Sub, sub, &Integer, &i64, Integer::subtract_c_long, deref rhs);
    } else {
        impl_single_op!(Sub, sub, Integer, i64, Integer::subtract, ref self, into rhs);
        impl_single_op!(Sub, sub, &Integer, i64, Integer::subtract, into rhs);
        impl_single_op!(Sub, sub, Integer, &i64, Integer::subtract, ref self, into rhs);
        impl_single_op!(Sub, sub, &Integer, &i64, Integer::subtract, into rhs);
    }
}

impl_single_op!(Sub, sub, Integer, u64, Integer::subtract, ref self, into rhs);
impl_single_op!(Sub, sub, &Integer, u64, Integer::subtract, into rhs);
impl_single_op!(Sub, sub, Integer, &u64, Integer::subtract, ref self, into rhs);
impl_single_op!(Sub, sub, &Integer, &u64, Integer::subtract, into rhs);

impl_single_op!(Sub, sub, Integer, i128, Integer::subtract, ref self, into rhs);
impl_single_op!(Sub, sub, &Integer, i128, Integer::subtract, into rhs);
impl_single_op!(Sub, sub, Integer, &i128, Integer::subtract, ref self, into rhs);
impl_single_op!(Sub, sub, &Integer, &i128, Integer::subtract, into rhs);

impl_single_op!(Sub, sub, Integer, u128, Integer::subtract, ref self, into rhs);
impl_single_op!(Sub, sub, &Integer, u128, Integer::subtract, into rhs);
impl_single_op!(Sub, sub, Integer, &u128, Integer::subtract, ref self, into rhs);
impl_single_op!(Sub, sub, &Integer, &u128, Integer::subtract, into rhs);

// MULTIPLICATION

impl_single_op!(Mul, mul, &Integer, Integer, Integer::multiply, ref rhs);
impl_single_op!(Mul, mul, Integer, &Integer, Integer::multiply, ref self);
impl_single_op!(Mul, mul, &Integer, &Integer, Integer::multiply);
impl_single_op!(Mul, mul, Integer, Integer, Integer::multiply, ref self, ref rhs);

impl_single_op!(Mul, mul, Integer, i8, Integer::multiply_c_long, ref self);
impl_single_op!(Mul, mul, &Integer, i8, Integer::multiply_c_long);
impl_single_op!(Mul, mul, Integer, &i8, Integer::multiply_c_long, ref self, deref rhs);
impl_single_op!(Mul, mul, &Integer, &i8, Integer::multiply_c_long, deref rhs);

impl_single_op!(Mul, mul, Integer, u8, Integer::multiply_c_long, ref self);
impl_single_op!(Mul, mul, &Integer, u8, Integer::multiply_c_long);
impl_single_op!(Mul, mul, Integer, &u8, Integer::multiply_c_long, ref self, deref rhs);
impl_single_op!(Mul, mul, &Integer, &u8, Integer::multiply_c_long, deref rhs);

impl_single_op!(Mul, mul, Integer, i16, Integer::multiply_c_long, ref self);
impl_single_op!(Mul, mul, &Integer, i16, Integer::multiply_c_long);
impl_single_op!(Mul, mul, Integer, &i16, Integer::multiply_c_long, ref self, deref rhs);
impl_single_op!(Mul, mul, &Integer, &i16, Integer::multiply_c_long, deref rhs);

impl_single_op!(Mul, mul, Integer, u16, Integer::multiply_c_long, ref self);
impl_single_op!(Mul, mul, &Integer, u16, Integer::multiply_c_long);
impl_single_op!(Mul, mul, Integer, &u16, Integer::multiply_c_long, ref self, deref rhs);
impl_single_op!(Mul, mul, &Integer, &u16, Integer::multiply_c_long, deref rhs);

impl_single_op!(Mul, mul, Integer, i32, Integer::multiply_c_long, ref self);
impl_single_op!(Mul, mul, &Integer, i32, Integer::multiply_c_long);
impl_single_op!(Mul, mul, Integer, &i32, Integer::multiply_c_long, ref self, deref rhs);
impl_single_op!(Mul, mul, &Integer, &i32, Integer::multiply_c_long, deref rhs);

impl_single_op!(Mul, mul, Integer, u32, Integer::multiply_c_long, ref self);
impl_single_op!(Mul, mul, &Integer, u32, Integer::multiply_c_long);
impl_single_op!(Mul, mul, Integer, &u32, Integer::multiply_c_long, ref self, deref rhs);
impl_single_op!(Mul, mul, &Integer, &u32, Integer::multiply_c_long, deref rhs);

cfg_if::cfg_if! {
    if #[cfg(all(target_pointer_width = "64", not(windows)))] {
        impl_single_op!(Mul, mul, Integer, i64, Integer::multiply_c_long, ref self);
        impl_single_op!(Mul, mul, &Integer, i64, Integer::multiply_c_long);
        impl_single_op!(Mul, mul, Integer, &i64, Integer::multiply_c_long, ref self, deref rhs);
        impl_single_op!(Mul, mul, &Integer, &i64, Integer::multiply_c_long, deref rhs);
    } else {
        impl_single_op!(Mul, mul, Integer, i64, Integer::multiply, ref self, into rhs);
        impl_single_op!(Mul, mul, &Integer, i64, Integer::multiply, into rhs);
        impl_single_op!(Mul, mul, Integer, &i64, Integer::multiply, ref self, into rhs);
        impl_single_op!(Mul, mul, &Integer, &i64, Integer::multiply, into rhs);
    }
}

impl_single_op!(Mul, mul, Integer, u64, Integer::multiply, ref self, into rhs);
impl_single_op!(Mul, mul, &Integer, u64, Integer::multiply, into rhs);
impl_single_op!(Mul, mul, Integer, &u64, Integer::multiply, ref self, into rhs);
impl_single_op!(Mul, mul, &Integer, &u64, Integer::multiply, into rhs);

impl_single_op!(Mul, mul, Integer, i128, Integer::multiply, ref self, into rhs);
impl_single_op!(Mul, mul, &Integer, i128, Integer::multiply, into rhs);
impl_single_op!(Mul, mul, Integer, &i128, Integer::multiply, ref self, into rhs);
impl_single_op!(Mul, mul, &Integer, &i128, Integer::multiply, into rhs);

impl_single_op!(Mul, mul, Integer, u128, Integer::multiply, ref self, into rhs);
impl_single_op!(Mul, mul, &Integer, u128, Integer::multiply, into rhs);
impl_single_op!(Mul, mul, Integer, &u128, Integer::multiply, ref self, into rhs);
impl_single_op!(Mul, mul, &Integer, &u128, Integer::multiply, into rhs);

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
}
