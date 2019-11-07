use crate::integer::Integer;
use core::ops::{Rem, RemAssign};

// Rem          The remainder operator %.

impl_single_binop!(
    Rem,
    rem,
    &Integer,
    Integer,
    Integer::remainder,
    Integer,
    ref rhs
);
impl_single_binop!(Rem, rem, Integer, &Integer, Integer::remainder, Integer, ref self, no reuse);
impl_single_binop!(Rem, rem, &Integer, &Integer, Integer::remainder, Integer);
impl_single_binop!(Rem, rem, Integer, Integer, Integer::remainder, Integer, ref self, ref rhs, no reuse);

impl_single_binop!(Rem, rem, Integer, i8, Integer::remainder_c_long, i8, ref self, no reuse);
impl_single_binop!(Rem, rem, &Integer, i8, Integer::remainder_c_long, i8);
impl_single_binop!(Rem, rem, Integer, &i8, Integer::remainder_c_long, i8, ref self, deref rhs, no reuse);
impl_single_binop!(Rem, rem, &Integer, &i8, Integer::remainder_c_long, i8, deref rhs);

impl_single_binop!(Rem, rem, Integer, u8, Integer::remainder_c_long, u8, ref self, no reuse);
impl_single_binop!(Rem, rem, &Integer, u8, Integer::remainder_c_long, u8);
impl_single_binop!(Rem, rem, Integer, &u8, Integer::remainder_c_long, u8, ref self, deref rhs, no reuse);
impl_single_binop!(Rem, rem, &Integer, &u8, Integer::remainder_c_long, u8, deref rhs);

impl_single_binop!(Rem, rem, Integer, i16, Integer::remainder_c_long, i16, ref self, no reuse);
impl_single_binop!(Rem, rem, &Integer, i16, Integer::remainder_c_long, i16);
impl_single_binop!(Rem, rem, Integer, &i16, Integer::remainder_c_long, i16, ref self, deref rhs, no reuse);
impl_single_binop!(Rem, rem, &Integer, &i16, Integer::remainder_c_long, i16, deref rhs);

impl_single_binop!(Rem, rem, Integer, u16, Integer::remainder_c_long, u16, ref self, no reuse);
impl_single_binop!(Rem, rem, &Integer, u16, Integer::remainder_c_long, u16);
impl_single_binop!(Rem, rem, Integer, &u16, Integer::remainder_c_long, u16, ref self, deref rhs, no reuse);
impl_single_binop!(Rem, rem, &Integer, &u16, Integer::remainder_c_long, u16, deref rhs);

impl_single_binop!(Rem, rem, Integer, i32, Integer::remainder_c_long, i32, ref self, no reuse);
impl_single_binop!(Rem, rem, &Integer, i32, Integer::remainder_c_long, i32);
impl_single_binop!(Rem, rem, Integer, &i32, Integer::remainder_c_long, i32, ref self, deref rhs, no reuse);
impl_single_binop!(Rem, rem, &Integer, &i32, Integer::remainder_c_long, i32, deref rhs);

cfg_if::cfg_if! {
    if #[cfg(all(target_pointer_width = "64", not(windows)))] {
        impl_single_binop!(Rem, rem, Integer, u32, Integer::remainder_c_long, u32, ref self, no reuse);
        impl_single_binop!(Rem, rem, &Integer, u32, Integer::remainder_c_long, u32);
        impl_single_binop!(Rem, rem, Integer, &u32, Integer::remainder_c_long, u32, ref self, deref rhs, no reuse);
        impl_single_binop!(Rem, rem, &Integer, &u32, Integer::remainder_c_long, u32, deref rhs);
    } else {
        impl_single_binop!(Rem, rem, Integer, u32, Integer::remainder, u32, ref self, into rhs, no reuse);
        impl_single_binop!(Rem, rem, &Integer, u32, Integer::remainder, u32, into rhs);
        impl_single_binop!(Rem, rem, Integer, &u32, Integer::remainder, u32, ref self, into rhs, no reuse);
        impl_single_binop!(Rem, rem, &Integer, &u32, Integer::remainder, u32, into rhs);
    }
}

impl_single_binop!(Rem, rem, Integer, i64, Integer::remainder, Integer, ref self, into rhs, no reuse);
impl_single_binop!(Rem, rem, &Integer, i64, Integer::remainder, Integer, into rhs);
impl_single_binop!(Rem, rem, Integer, &i64, Integer::remainder, Integer, ref self, into rhs, no reuse);
impl_single_binop!(Rem, rem, &Integer, &i64, Integer::remainder, Integer, into rhs);

impl_single_binop!(Rem, rem, Integer, u64, Integer::remainder, Integer, ref self, into rhs, no reuse);
impl_single_binop!(Rem, rem, &Integer, u64, Integer::remainder, Integer, into rhs);
impl_single_binop!(Rem, rem, Integer, &u64, Integer::remainder, Integer, ref self, into rhs, no reuse);
impl_single_binop!(Rem, rem, &Integer, &u64, Integer::remainder, Integer, into rhs);

impl_single_binop!(Rem, rem, Integer, i128, Integer::remainder, Integer, ref self, into rhs, no reuse);
impl_single_binop!(Rem, rem, &Integer, i128, Integer::remainder, Integer, into rhs);
impl_single_binop!(Rem, rem, Integer, &i128, Integer::remainder, Integer, ref self, into rhs, no reuse);
impl_single_binop!(Rem, rem, &Integer, &i128, Integer::remainder, Integer, into rhs);

impl_single_binop!(Rem, rem, Integer, u128, Integer::remainder, Integer, ref self, into rhs, no reuse);
impl_single_binop!(Rem, rem, &Integer, u128, Integer::remainder, Integer, into rhs);
impl_single_binop!(Rem, rem, Integer, &u128, Integer::remainder, Integer, ref self, into rhs, no reuse);
impl_single_binop!(Rem, rem, &Integer, &u128, Integer::remainder, Integer, into rhs);

// RemAssign    The remainder assignment operator %=.

impl_single_op_assign!(
    RemAssign,
    rem_assign,
    Integer,
    &Integer,
    Integer::remainder_assign
);
impl_single_op_assign!(
    RemAssign,
    rem_assign,
    Integer,
    Integer,
    Integer::remainder_assign,
    ref rhs
);

impl_single_op_assign!(
    RemAssign,
    rem_assign,
    Integer,
    i8,
    Integer::remainder_c_long_assign
);
impl_single_op_assign!(RemAssign, rem_assign, Integer, &i8, Integer::remainder_c_long_assign, deref rhs);

impl_single_op_assign!(
    RemAssign,
    rem_assign,
    Integer,
    u8,
    Integer::remainder_c_long_assign
);
impl_single_op_assign!(RemAssign, rem_assign, Integer, &u8, Integer::remainder_c_long_assign, deref rhs);

impl_single_op_assign!(
    RemAssign,
    rem_assign,
    Integer,
    i16,
    Integer::remainder_c_long_assign
);
impl_single_op_assign!(RemAssign, rem_assign, Integer, &i16, Integer::remainder_c_long_assign, deref rhs);

impl_single_op_assign!(
    RemAssign,
    rem_assign,
    Integer,
    u16,
    Integer::remainder_c_long_assign
);
impl_single_op_assign!(RemAssign, rem_assign, Integer, &u16, Integer::remainder_c_long_assign, deref rhs);

impl_single_op_assign!(
    RemAssign,
    rem_assign,
    Integer,
    i32,
    Integer::remainder_c_long_assign
);
impl_single_op_assign!(RemAssign, rem_assign, Integer, &i32, Integer::remainder_c_long_assign, deref rhs);

cfg_if::cfg_if! {
    if #[cfg(all(target_pointer_width = "64", not(windows)))] {
        impl_single_op_assign!(RemAssign, rem_assign, Integer, u32, Integer::remainder_c_long_assign);
        impl_single_op_assign!(RemAssign, rem_assign, Integer, &u32, Integer::remainder_c_long_assign, deref rhs);

        impl_single_op_assign!(RemAssign, rem_assign, Integer, i64, Integer::remainder_c_long_assign);
        impl_single_op_assign!(RemAssign, rem_assign, Integer, &i64, Integer::remainder_c_long_assign, deref rhs);
    } else {
        impl_single_op_assign!(RemAssign, rem_assign, Integer, u32, Integer::remainder_assign, into rhs);
        impl_single_op_assign!(RemAssign, rem_assign, Integer, &u32, Integer::remainder_assign, into rhs);

        impl_single_op_assign!(RemAssign, rem_assign, Integer, i64, Integer::remainder_assign, into rhs);
        impl_single_op_assign!(RemAssign, rem_assign, Integer, &i64, Integer::remainder_assign, into rhs);
    }
}

impl_single_op_assign!(RemAssign, rem_assign, Integer, u64, Integer::remainder_assign, into rhs);
impl_single_op_assign!(RemAssign, rem_assign, Integer, &u64, Integer::remainder_assign, into rhs);

impl_single_op_assign!(RemAssign, rem_assign, Integer, i128, Integer::remainder_assign, into rhs);
impl_single_op_assign!(RemAssign, rem_assign, Integer, &i128, Integer::remainder_assign, into rhs);

impl_single_op_assign!(RemAssign, rem_assign, Integer, u128, Integer::remainder_assign, into rhs);
impl_single_op_assign!(RemAssign, rem_assign, Integer, &u128, Integer::remainder_assign, into rhs);
