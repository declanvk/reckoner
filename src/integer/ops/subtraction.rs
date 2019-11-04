// Sub          The subtraction operator -.
// SubAssign    The subtraction assignment operator -=.

use crate::integer::Integer;
use core::ops::Sub;

impl_single_binop!(
    Sub,
    sub,
    &Integer,
    Integer,
    Integer::subtract,
    Integer,
    ref rhs
);
impl_single_binop!(Sub, sub, Integer, &Integer, Integer::subtract_assign, Integer, ref self);
impl_single_binop!(Sub, sub, &Integer, &Integer, Integer::subtract, Integer);
impl_single_binop!(Sub, sub, Integer, Integer, Integer::subtract_assign, Integer, ref self, ref rhs);

impl_single_binop!(Sub, sub, Integer, i8, Integer::subtract_c_long_assign, Integer, ref self);
impl_single_binop!(Sub, sub, &Integer, i8, Integer::subtract_c_long, Integer);
impl_single_binop!(Sub, sub, Integer, &i8, Integer::subtract_c_long_assign, Integer, ref self, deref rhs);
impl_single_binop!(Sub, sub, &Integer, &i8, Integer::subtract_c_long, Integer, deref rhs);

impl_single_binop!(Sub, sub, Integer, u8, Integer::subtract_c_long_assign, Integer, ref self);
impl_single_binop!(Sub, sub, &Integer, u8, Integer::subtract_c_long, Integer);
impl_single_binop!(Sub, sub, Integer, &u8, Integer::subtract_c_long_assign, Integer, ref self, deref rhs);
impl_single_binop!(Sub, sub, &Integer, &u8, Integer::subtract_c_long, Integer, deref rhs);

impl_single_binop!(Sub, sub, Integer, i16, Integer::subtract_c_long_assign, Integer, ref self);
impl_single_binop!(Sub, sub, &Integer, i16, Integer::subtract_c_long, Integer);
impl_single_binop!(Sub, sub, Integer, &i16, Integer::subtract_c_long_assign, Integer, ref self, deref rhs);
impl_single_binop!(Sub, sub, &Integer, &i16, Integer::subtract_c_long, Integer, deref rhs);

impl_single_binop!(Sub, sub, Integer, u16, Integer::subtract_c_long_assign, Integer, ref self);
impl_single_binop!(Sub, sub, &Integer, u16, Integer::subtract_c_long, Integer);
impl_single_binop!(Sub, sub, Integer, &u16, Integer::subtract_c_long_assign, Integer, ref self, deref rhs);
impl_single_binop!(Sub, sub, &Integer, &u16, Integer::subtract_c_long, Integer, deref rhs);

impl_single_binop!(Sub, sub, Integer, i32, Integer::subtract_c_long_assign, Integer, ref self);
impl_single_binop!(Sub, sub, &Integer, i32, Integer::subtract_c_long, Integer);
impl_single_binop!(Sub, sub, Integer, &i32, Integer::subtract_c_long_assign, Integer, ref self, deref rhs);
impl_single_binop!(Sub, sub, &Integer, &i32, Integer::subtract_c_long, Integer, deref rhs);

impl_single_binop!(Sub, sub, Integer, u32, Integer::subtract_c_long_assign, Integer, ref self);
impl_single_binop!(Sub, sub, &Integer, u32, Integer::subtract_c_long, Integer);
impl_single_binop!(Sub, sub, Integer, &u32, Integer::subtract_c_long_assign, Integer, ref self, deref rhs);
impl_single_binop!(Sub, sub, &Integer, &u32, Integer::subtract_c_long, Integer, deref rhs);

cfg_if::cfg_if! {
    if #[cfg(all(target_pointer_width = "64", not(windows)))] {
        impl_single_binop!(Sub, sub, Integer, i64, Integer::subtract_c_long_assign, Integer, ref self);
        impl_single_binop!(Sub, sub, &Integer, i64, Integer::subtract_c_long, Integer);
        impl_single_binop!(Sub, sub, Integer, &i64, Integer::subtract_c_long_assign, Integer, ref self, deref rhs);
        impl_single_binop!(Sub, sub, &Integer, &i64, Integer::subtract_c_long, Integer, deref rhs);
    } else {
        impl_single_binop!(Sub, sub, Integer, i64, Integer::subtract_assign, Integer, ref self, into rhs);
        impl_single_binop!(Sub, sub, &Integer, i64, Integer::subtract, Integer, into rhs);
        impl_single_binop!(Sub, sub, Integer, &i64, Integer::subtract_assign, Integer, ref self, into rhs);
        impl_single_binop!(Sub, sub, &Integer, &i64, Integer::subtract, Integer, into rhs);
    }
}

impl_single_binop!(Sub, sub, Integer, u64, Integer::subtract_assign, Integer, ref self, into rhs);
impl_single_binop!(Sub, sub, &Integer, u64, Integer::subtract, Integer, into rhs);
impl_single_binop!(Sub, sub, Integer, &u64, Integer::subtract_assign, Integer, ref self, into rhs);
impl_single_binop!(Sub, sub, &Integer, &u64, Integer::subtract, Integer, into rhs);

impl_single_binop!(Sub, sub, Integer, i128, Integer::subtract_assign, Integer, ref self, into rhs);
impl_single_binop!(Sub, sub, &Integer, i128, Integer::subtract, Integer, into rhs);
impl_single_binop!(Sub, sub, Integer, &i128, Integer::subtract_assign, Integer, ref self, into rhs);
impl_single_binop!(Sub, sub, &Integer, &i128, Integer::subtract, Integer, into rhs);

impl_single_binop!(Sub, sub, Integer, u128, Integer::subtract_assign, Integer, ref self, into rhs);
impl_single_binop!(Sub, sub, &Integer, u128, Integer::subtract, Integer, into rhs);
impl_single_binop!(Sub, sub, Integer, &u128, Integer::subtract_assign, Integer, ref self, into rhs);
impl_single_binop!(Sub, sub, &Integer, &u128, Integer::subtract, Integer, into rhs);
