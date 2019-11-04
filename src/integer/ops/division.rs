// Div          The division operator /.
// DivAssign    The division assignment operator /=.

use crate::integer::Integer;
use core::ops::Div;

impl_single_binop!(
    Div,
    div,
    &Integer,
    Integer,
    Integer::divide,
    Integer,
    ref rhs
);
impl_single_binop!(Div, div, Integer, &Integer, Integer::divide_assign, Integer, ref self);
impl_single_binop!(Div, div, &Integer, &Integer, Integer::divide, Integer);
impl_single_binop!(Div, div, Integer, Integer, Integer::divide_assign, Integer, ref self, ref rhs);

impl_single_binop!(Div, div, Integer, i8, Integer::divide_c_long_assign, Integer, ref self);
impl_single_binop!(Div, div, &Integer, i8, Integer::divide_c_long, Integer);
impl_single_binop!(Div, div, Integer, &i8, Integer::divide_c_long_assign, Integer, ref self, deref rhs);
impl_single_binop!(Div, div, &Integer, &i8, Integer::divide_c_long, Integer, deref rhs);

impl_single_binop!(Div, div, Integer, u8, Integer::divide_c_long_assign, Integer, ref self);
impl_single_binop!(Div, div, &Integer, u8, Integer::divide_c_long, Integer);
impl_single_binop!(Div, div, Integer, &u8, Integer::divide_c_long_assign, Integer, ref self, deref rhs);
impl_single_binop!(Div, div, &Integer, &u8, Integer::divide_c_long, Integer, deref rhs);

impl_single_binop!(Div, div, Integer, i16, Integer::divide_c_long_assign, Integer, ref self);
impl_single_binop!(Div, div, &Integer, i16, Integer::divide_c_long, Integer);
impl_single_binop!(Div, div, Integer, &i16, Integer::divide_c_long_assign, Integer, ref self, deref rhs);
impl_single_binop!(Div, div, &Integer, &i16, Integer::divide_c_long, Integer, deref rhs);

impl_single_binop!(Div, div, Integer, u16, Integer::divide_c_long_assign, Integer, ref self);
impl_single_binop!(Div, div, &Integer, u16, Integer::divide_c_long, Integer);
impl_single_binop!(Div, div, Integer, &u16, Integer::divide_c_long_assign, Integer, ref self, deref rhs);
impl_single_binop!(Div, div, &Integer, &u16, Integer::divide_c_long, Integer, deref rhs);

impl_single_binop!(Div, div, Integer, i32, Integer::divide_c_long_assign, Integer, ref self);
impl_single_binop!(Div, div, &Integer, i32, Integer::divide_c_long, Integer);
impl_single_binop!(Div, div, Integer, &i32, Integer::divide_c_long_assign, Integer, ref self, deref rhs);
impl_single_binop!(Div, div, &Integer, &i32, Integer::divide_c_long, Integer, deref rhs);

impl_single_binop!(Div, div, Integer, u32, Integer::divide_c_long_assign, Integer, ref self);
impl_single_binop!(Div, div, &Integer, u32, Integer::divide_c_long, Integer);
impl_single_binop!(Div, div, Integer, &u32, Integer::divide_c_long_assign, Integer, ref self, deref rhs);
impl_single_binop!(Div, div, &Integer, &u32, Integer::divide_c_long, Integer, deref rhs);

cfg_if::cfg_if! {
    if #[cfg(all(target_pointer_width = "64", not(windows)))] {
        impl_single_binop!(Div, div, Integer, i64, Integer::divide_c_long_assign, Integer, ref self);
        impl_single_binop!(Div, div, &Integer, i64, Integer::divide_c_long, Integer);
        impl_single_binop!(Div, div, Integer, &i64, Integer::divide_c_long_assign, Integer, ref self, deref rhs);
        impl_single_binop!(Div, div, &Integer, &i64, Integer::divide_c_long, Integer, deref rhs);
    } else {
        impl_single_binop!(Div, div, Integer, i64, Integer::divide_assign, Integer, ref self, into rhs);
        impl_single_binop!(Div, div, &Integer, i64, Integer::divide, Integer, into rhs);
        impl_single_binop!(Div, div, Integer, &i64, Integer::divide_assign, Integer, ref self, into rhs);
        impl_single_binop!(Div, div, &Integer, &i64, Integer::divide, Integer, into rhs);
    }
}

impl_single_binop!(Div, div, Integer, u64, Integer::divide_assign, Integer, ref self, into rhs);
impl_single_binop!(Div, div, &Integer, u64, Integer::divide, Integer, into rhs);
impl_single_binop!(Div, div, Integer, &u64, Integer::divide_assign, Integer, ref self, into rhs);
impl_single_binop!(Div, div, &Integer, &u64, Integer::divide, Integer, into rhs);

impl_single_binop!(Div, div, Integer, i128, Integer::divide_assign, Integer, ref self, into rhs);
impl_single_binop!(Div, div, &Integer, i128, Integer::divide, Integer, into rhs);
impl_single_binop!(Div, div, Integer, &i128, Integer::divide_assign, Integer, ref self, into rhs);
impl_single_binop!(Div, div, &Integer, &i128, Integer::divide, Integer, into rhs);

impl_single_binop!(Div, div, Integer, u128, Integer::divide_assign, Integer, ref self, into rhs);
impl_single_binop!(Div, div, &Integer, u128, Integer::divide, Integer, into rhs);
impl_single_binop!(Div, div, Integer, &u128, Integer::divide_assign, Integer, ref self, into rhs);
impl_single_binop!(Div, div, &Integer, &u128, Integer::divide, Integer, into rhs);
