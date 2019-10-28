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

impl_single_op!(Sub, sub, &Integer, Integer, Integer::sub, ref rhs);
impl_single_op!(Sub, sub, Integer, &Integer, Integer::sub, ref self);
impl_single_op!(Sub, sub, &Integer, &Integer, Integer::sub);
impl_single_op!(Sub, sub, Integer, Integer, Integer::sub, ref self, ref rhs);

impl_single_op!(Sub, sub, Integer, i8, Integer::sub_c_long, ref self);
impl_single_op!(Sub, sub, &Integer, i8, Integer::sub_c_long);
impl_single_op!(Sub, sub, Integer, &i8, Integer::sub_c_long, ref self, deref rhs);
impl_single_op!(Sub, sub, &Integer, &i8, Integer::sub_c_long, deref rhs);

impl_single_op!(Sub, sub, Integer, u8, Integer::sub_c_long, ref self);
impl_single_op!(Sub, sub, &Integer, u8, Integer::sub_c_long);
impl_single_op!(Sub, sub, Integer, &u8, Integer::sub_c_long, ref self, deref rhs);
impl_single_op!(Sub, sub, &Integer, &u8, Integer::sub_c_long, deref rhs);

impl_single_op!(Sub, sub, Integer, i16, Integer::sub_c_long, ref self);
impl_single_op!(Sub, sub, &Integer, i16, Integer::sub_c_long);
impl_single_op!(Sub, sub, Integer, &i16, Integer::sub_c_long, ref self, deref rhs);
impl_single_op!(Sub, sub, &Integer, &i16, Integer::sub_c_long, deref rhs);

impl_single_op!(Sub, sub, Integer, u16, Integer::sub_c_long, ref self);
impl_single_op!(Sub, sub, &Integer, u16, Integer::sub_c_long);
impl_single_op!(Sub, sub, Integer, &u16, Integer::sub_c_long, ref self, deref rhs);
impl_single_op!(Sub, sub, &Integer, &u16, Integer::sub_c_long, deref rhs);

impl_single_op!(Sub, sub, Integer, i32, Integer::sub_c_long, ref self);
impl_single_op!(Sub, sub, &Integer, i32, Integer::sub_c_long);
impl_single_op!(Sub, sub, Integer, &i32, Integer::sub_c_long, ref self, deref rhs);
impl_single_op!(Sub, sub, &Integer, &i32, Integer::sub_c_long, deref rhs);

impl_single_op!(Sub, sub, Integer, u32, Integer::sub_c_long, ref self);
impl_single_op!(Sub, sub, &Integer, u32, Integer::sub_c_long);
impl_single_op!(Sub, sub, Integer, &u32, Integer::sub_c_long, ref self, deref rhs);
impl_single_op!(Sub, sub, &Integer, &u32, Integer::sub_c_long, deref rhs);

cfg_if::cfg_if! {
    if #[cfg(all(target_pointer_width = "64", not(windows)))] {
        impl_single_op!(Sub, sub, Integer, i64, Integer::sub_c_long, ref self);
        impl_single_op!(Sub, sub, &Integer, i64, Integer::sub_c_long);
        impl_single_op!(Sub, sub, Integer, &i64, Integer::sub_c_long, ref self, deref rhs);
        impl_single_op!(Sub, sub, &Integer, &i64, Integer::sub_c_long, deref rhs);
    } else {
        impl_single_op!(Sub, sub, Integer, i64, Integer::sub, ref self, into rhs);
        impl_single_op!(Sub, sub, &Integer, i64, Integer::sub, into rhs);
        impl_single_op!(Sub, sub, Integer, &i64, Integer::sub, ref self, into rhs);
        impl_single_op!(Sub, sub, &Integer, &i64, Integer::sub, into rhs);
    }
}

impl_single_op!(Sub, sub, Integer, u64, Integer::sub, ref self, into rhs);
impl_single_op!(Sub, sub, &Integer, u64, Integer::sub, into rhs);
impl_single_op!(Sub, sub, Integer, &u64, Integer::sub, ref self, into rhs);
impl_single_op!(Sub, sub, &Integer, &u64, Integer::sub, into rhs);

impl_single_op!(Sub, sub, Integer, i128, Integer::sub, ref self, into rhs);
impl_single_op!(Sub, sub, &Integer, i128, Integer::sub, into rhs);
impl_single_op!(Sub, sub, Integer, &i128, Integer::sub, ref self, into rhs);
impl_single_op!(Sub, sub, &Integer, &i128, Integer::sub, into rhs);

impl_single_op!(Sub, sub, Integer, u128, Integer::sub, ref self, into rhs);
impl_single_op!(Sub, sub, &Integer, u128, Integer::sub, into rhs);
impl_single_op!(Sub, sub, Integer, &u128, Integer::sub, ref self, into rhs);
impl_single_op!(Sub, sub, &Integer, &u128, Integer::sub, into rhs);

// MULTIPLICATION

impl_single_op!(Mul, mul, &Integer, Integer, Integer::mul, ref rhs);
impl_single_op!(Mul, mul, Integer, &Integer, Integer::mul, ref self);
impl_single_op!(Mul, mul, &Integer, &Integer, Integer::mul);
impl_single_op!(Mul, mul, Integer, Integer, Integer::mul, ref self, ref rhs);

impl_single_op!(Mul, mul, Integer, i8, Integer::mul_c_long, ref self);
impl_single_op!(Mul, mul, &Integer, i8, Integer::mul_c_long);
impl_single_op!(Mul, mul, Integer, &i8, Integer::mul_c_long, ref self, deref rhs);
impl_single_op!(Mul, mul, &Integer, &i8, Integer::mul_c_long, deref rhs);

impl_single_op!(Mul, mul, Integer, u8, Integer::mul_c_long, ref self);
impl_single_op!(Mul, mul, &Integer, u8, Integer::mul_c_long);
impl_single_op!(Mul, mul, Integer, &u8, Integer::mul_c_long, ref self, deref rhs);
impl_single_op!(Mul, mul, &Integer, &u8, Integer::mul_c_long, deref rhs);

impl_single_op!(Mul, mul, Integer, i16, Integer::mul_c_long, ref self);
impl_single_op!(Mul, mul, &Integer, i16, Integer::mul_c_long);
impl_single_op!(Mul, mul, Integer, &i16, Integer::mul_c_long, ref self, deref rhs);
impl_single_op!(Mul, mul, &Integer, &i16, Integer::mul_c_long, deref rhs);

impl_single_op!(Mul, mul, Integer, u16, Integer::mul_c_long, ref self);
impl_single_op!(Mul, mul, &Integer, u16, Integer::mul_c_long);
impl_single_op!(Mul, mul, Integer, &u16, Integer::mul_c_long, ref self, deref rhs);
impl_single_op!(Mul, mul, &Integer, &u16, Integer::mul_c_long, deref rhs);

impl_single_op!(Mul, mul, Integer, i32, Integer::mul_c_long, ref self);
impl_single_op!(Mul, mul, &Integer, i32, Integer::mul_c_long);
impl_single_op!(Mul, mul, Integer, &i32, Integer::mul_c_long, ref self, deref rhs);
impl_single_op!(Mul, mul, &Integer, &i32, Integer::mul_c_long, deref rhs);

impl_single_op!(Mul, mul, Integer, u32, Integer::mul_c_long, ref self);
impl_single_op!(Mul, mul, &Integer, u32, Integer::mul_c_long);
impl_single_op!(Mul, mul, Integer, &u32, Integer::mul_c_long, ref self, deref rhs);
impl_single_op!(Mul, mul, &Integer, &u32, Integer::mul_c_long, deref rhs);

cfg_if::cfg_if! {
    if #[cfg(all(target_pointer_width = "64", not(windows)))] {
        impl_single_op!(Mul, mul, Integer, i64, Integer::mul_c_long, ref self);
        impl_single_op!(Mul, mul, &Integer, i64, Integer::mul_c_long);
        impl_single_op!(Mul, mul, Integer, &i64, Integer::mul_c_long, ref self, deref rhs);
        impl_single_op!(Mul, mul, &Integer, &i64, Integer::mul_c_long, deref rhs);
    } else {
        impl_single_op!(Mul, mul, Integer, i64, Integer::mul, ref self, into rhs);
        impl_single_op!(Mul, mul, &Integer, i64, Integer::mul, into rhs);
        impl_single_op!(Mul, mul, Integer, &i64, Integer::mul, ref self, into rhs);
        impl_single_op!(Mul, mul, &Integer, &i64, Integer::mul, into rhs);
    }
}

impl_single_op!(Mul, mul, Integer, u64, Integer::mul, ref self, into rhs);
impl_single_op!(Mul, mul, &Integer, u64, Integer::mul, into rhs);
impl_single_op!(Mul, mul, Integer, &u64, Integer::mul, ref self, into rhs);
impl_single_op!(Mul, mul, &Integer, &u64, Integer::mul, into rhs);

impl_single_op!(Mul, mul, Integer, i128, Integer::mul, ref self, into rhs);
impl_single_op!(Mul, mul, &Integer, i128, Integer::mul, into rhs);
impl_single_op!(Mul, mul, Integer, &i128, Integer::mul, ref self, into rhs);
impl_single_op!(Mul, mul, &Integer, &i128, Integer::mul, into rhs);

impl_single_op!(Mul, mul, Integer, u128, Integer::mul, ref self, into rhs);
impl_single_op!(Mul, mul, &Integer, u128, Integer::mul, into rhs);
impl_single_op!(Mul, mul, Integer, &u128, Integer::mul, ref self, into rhs);
impl_single_op!(Mul, mul, &Integer, &u128, Integer::mul, into rhs);

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
}
