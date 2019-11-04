// Add          The addition operator +.
// AddAssign    The addition assignment operator +=.

use crate::integer::Integer;
use core::ops::{Add, AddAssign};

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
