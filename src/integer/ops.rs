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
}

// ADDITION

impl_single_op!(Add, add, &Integer, Integer, Integer::add, ref rhs);
impl_single_op!(Add, add, Integer, &Integer, Integer::add, ref self);
impl_single_op!(Add, add, &Integer, &Integer, Integer::add);
impl_single_op!(Add, add, Integer, Integer, Integer::add, ref self, ref rhs);

// SUBTRACTION

impl_single_op!(Sub, sub, &Integer, Integer, Integer::sub, ref rhs);
impl_single_op!(Sub, sub, Integer, &Integer, Integer::sub, ref self);
impl_single_op!(Sub, sub, &Integer, &Integer, Integer::sub);
impl_single_op!(Sub, sub, Integer, Integer, Integer::sub, ref self, ref rhs);

// MULTIPLICATION

impl_single_op!(Mul, mul, &Integer, Integer, Integer::mul, ref rhs);
impl_single_op!(Mul, mul, Integer, &Integer, Integer::mul, ref self);
impl_single_op!(Mul, mul, &Integer, &Integer, Integer::mul);
impl_single_op!(Mul, mul, Integer, Integer, Integer::mul, ref self, ref rhs);
