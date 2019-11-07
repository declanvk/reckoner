use crate::integer::Integer;
use core::ops::{Mul, MulAssign};

// Mul          The multiplication operator *.

impl_single_binop!(
    Mul,
    mul,
    &Integer,
    Integer,
    Integer::multiply,
    Integer,
    ref rhs
);
impl_single_binop!(Mul, mul, Integer, &Integer, Integer::multiply_assign, Integer, ref self);
impl_single_binop!(Mul, mul, &Integer, &Integer, Integer::multiply, Integer);
impl_single_binop!(Mul, mul, Integer, Integer, Integer::multiply_assign, Integer, ref self, ref rhs);

impl_single_binop!(Mul, mul, Integer, i8, Integer::multiply_c_long_assign, Integer, ref self);
impl_single_binop!(Mul, mul, &Integer, i8, Integer::multiply_c_long, Integer);
impl_single_binop!(Mul, mul, Integer, &i8, Integer::multiply_c_long_assign, Integer, ref self, deref rhs);
impl_single_binop!(Mul, mul, &Integer, &i8, Integer::multiply_c_long, Integer, deref rhs);

impl_single_binop!(Mul, mul, Integer, u8, Integer::multiply_c_long_assign, Integer, ref self);
impl_single_binop!(Mul, mul, &Integer, u8, Integer::multiply_c_long, Integer);
impl_single_binop!(Mul, mul, Integer, &u8, Integer::multiply_c_long_assign, Integer, ref self, deref rhs);
impl_single_binop!(Mul, mul, &Integer, &u8, Integer::multiply_c_long, Integer, deref rhs);

impl_single_binop!(Mul, mul, Integer, i16, Integer::multiply_c_long_assign, Integer, ref self);
impl_single_binop!(Mul, mul, &Integer, i16, Integer::multiply_c_long, Integer);
impl_single_binop!(Mul, mul, Integer, &i16, Integer::multiply_c_long_assign, Integer, ref self, deref rhs);
impl_single_binop!(Mul, mul, &Integer, &i16, Integer::multiply_c_long, Integer, deref rhs);

impl_single_binop!(Mul, mul, Integer, u16, Integer::multiply_c_long_assign, Integer, ref self);
impl_single_binop!(Mul, mul, &Integer, u16, Integer::multiply_c_long, Integer);
impl_single_binop!(Mul, mul, Integer, &u16, Integer::multiply_c_long_assign, Integer, ref self, deref rhs);
impl_single_binop!(Mul, mul, &Integer, &u16, Integer::multiply_c_long, Integer, deref rhs);

impl_single_binop!(Mul, mul, Integer, i32, Integer::multiply_c_long_assign, Integer, ref self);
impl_single_binop!(Mul, mul, &Integer, i32, Integer::multiply_c_long, Integer);
impl_single_binop!(Mul, mul, Integer, &i32, Integer::multiply_c_long_assign, Integer, ref self, deref rhs);
impl_single_binop!(Mul, mul, &Integer, &i32, Integer::multiply_c_long, Integer, deref rhs);

cfg_if::cfg_if! {
    if #[cfg(all(target_pointer_width = "64", not(windows)))] {
        impl_single_binop!(Mul, mul, Integer, u32, Integer::multiply_c_long_assign, Integer, ref self);
        impl_single_binop!(Mul, mul, &Integer, u32, Integer::multiply_c_long, Integer);
        impl_single_binop!(Mul, mul, Integer, &u32, Integer::multiply_c_long_assign, Integer, ref self, deref rhs);
        impl_single_binop!(Mul, mul, &Integer, &u32, Integer::multiply_c_long, Integer, deref rhs);

        impl_single_binop!(Mul, mul, Integer, i64, Integer::multiply_c_long_assign, Integer, ref self);
        impl_single_binop!(Mul, mul, &Integer, i64, Integer::multiply_c_long, Integer);
        impl_single_binop!(Mul, mul, Integer, &i64, Integer::multiply_c_long_assign, Integer, ref self, deref rhs);
        impl_single_binop!(Mul, mul, &Integer, &i64, Integer::multiply_c_long, Integer, deref rhs);
    } else {
        impl_single_binop!(Mul, mul, Integer, u32, Integer::multiply_assign, Integer, ref self, into rhs);
        impl_single_binop!(Mul, mul, &Integer, u32, Integer::multiply, Integer, into rhs);
        impl_single_binop!(Mul, mul, Integer, &u32, Integer::multiply_assign, Integer, ref self, into rhs);
        impl_single_binop!(Mul, mul, &Integer, &u32, Integer::multiply, Integer, into rhs);

        impl_single_binop!(Mul, mul, Integer, i64, Integer::multiply_assign, Integer, ref self, into rhs);
        impl_single_binop!(Mul, mul, &Integer, i64, Integer::multiply, Integer, into rhs);
        impl_single_binop!(Mul, mul, Integer, &i64, Integer::multiply_assign, Integer, ref self, into rhs);
        impl_single_binop!(Mul, mul, &Integer, &i64, Integer::multiply, Integer, into rhs);
    }
}

impl_single_binop!(Mul, mul, Integer, u64, Integer::multiply_assign, Integer, ref self, into rhs);
impl_single_binop!(Mul, mul, &Integer, u64, Integer::multiply, Integer, into rhs);
impl_single_binop!(Mul, mul, Integer, &u64, Integer::multiply_assign, Integer, ref self, into rhs);
impl_single_binop!(Mul, mul, &Integer, &u64, Integer::multiply, Integer, into rhs);

impl_single_binop!(Mul, mul, Integer, i128, Integer::multiply_assign, Integer, ref self, into rhs);
impl_single_binop!(Mul, mul, &Integer, i128, Integer::multiply, Integer, into rhs);
impl_single_binop!(Mul, mul, Integer, &i128, Integer::multiply_assign, Integer, ref self, into rhs);
impl_single_binop!(Mul, mul, &Integer, &i128, Integer::multiply, Integer, into rhs);

impl_single_binop!(Mul, mul, Integer, u128, Integer::multiply_assign, Integer, ref self, into rhs);
impl_single_binop!(Mul, mul, &Integer, u128, Integer::multiply, Integer, into rhs);
impl_single_binop!(Mul, mul, Integer, &u128, Integer::multiply_assign, Integer, ref self, into rhs);
impl_single_binop!(Mul, mul, &Integer, &u128, Integer::multiply, Integer, into rhs);

// MulAssign    The multiplication assignment operator *=.

impl_single_op_assign!(
    MulAssign,
    mul_assign,
    Integer,
    &Integer,
    Integer::multiply_assign
);
impl_single_op_assign!(
    MulAssign,
    mul_assign,
    Integer,
    Integer,
    Integer::multiply_assign,
    ref rhs
);

impl_single_op_assign!(
    MulAssign,
    mul_assign,
    Integer,
    i8,
    Integer::multiply_c_long_assign
);
impl_single_op_assign!(MulAssign, mul_assign, Integer, &i8, Integer::multiply_c_long_assign, deref rhs);

impl_single_op_assign!(
    MulAssign,
    mul_assign,
    Integer,
    u8,
    Integer::multiply_c_long_assign
);
impl_single_op_assign!(MulAssign, mul_assign, Integer, &u8, Integer::multiply_c_long_assign, deref rhs);

impl_single_op_assign!(
    MulAssign,
    mul_assign,
    Integer,
    i16,
    Integer::multiply_c_long_assign
);
impl_single_op_assign!(MulAssign, mul_assign, Integer, &i16, Integer::multiply_c_long_assign, deref rhs);

impl_single_op_assign!(
    MulAssign,
    mul_assign,
    Integer,
    u16,
    Integer::multiply_c_long_assign
);
impl_single_op_assign!(MulAssign, mul_assign, Integer, &u16, Integer::multiply_c_long_assign, deref rhs);

impl_single_op_assign!(
    MulAssign,
    mul_assign,
    Integer,
    i32,
    Integer::multiply_c_long_assign
);
impl_single_op_assign!(MulAssign, mul_assign, Integer, &i32, Integer::multiply_c_long_assign, deref rhs);

cfg_if::cfg_if! {
    if #[cfg(all(target_pointer_width = "64", not(windows)))] {
        impl_single_op_assign!(MulAssign, mul_assign, Integer, u32, Integer::multiply_c_long_assign);
        impl_single_op_assign!(MulAssign, mul_assign, Integer, &u32, Integer::multiply_c_long_assign, deref rhs);

        impl_single_op_assign!(MulAssign, mul_assign, Integer, i64, Integer::multiply_c_long_assign);
        impl_single_op_assign!(MulAssign, mul_assign, Integer, &i64, Integer::multiply_c_long_assign, deref rhs);
    } else {
        impl_single_op_assign!(MulAssign, mul_assign, Integer, u32, Integer::multiply_assign, into rhs);
        impl_single_op_assign!(MulAssign, mul_assign, Integer, &u32, Integer::multiply_assign, into rhs);

        impl_single_op_assign!(MulAssign, mul_assign, Integer, i64, Integer::multiply_assign, into rhs);
        impl_single_op_assign!(MulAssign, mul_assign, Integer, &i64, Integer::multiply_assign, into rhs);
    }
}

impl_single_op_assign!(MulAssign, mul_assign, Integer, u64, Integer::multiply_assign, into rhs);
impl_single_op_assign!(MulAssign, mul_assign, Integer, &u64, Integer::multiply_assign, into rhs);

impl_single_op_assign!(MulAssign, mul_assign, Integer, i128, Integer::multiply_assign, into rhs);
impl_single_op_assign!(MulAssign, mul_assign, Integer, &i128, Integer::multiply_assign, into rhs);

impl_single_op_assign!(MulAssign, mul_assign, Integer, u128, Integer::multiply_assign, into rhs);
impl_single_op_assign!(MulAssign, mul_assign, Integer, &u128, Integer::multiply_assign, into rhs);
