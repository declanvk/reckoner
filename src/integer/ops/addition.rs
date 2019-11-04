use crate::integer::Integer;
use core::ops::{Add, AddAssign};

// Add          The addition operator +.

impl_single_binop!(Add, add, &Integer, Integer, Integer::add, Integer, ref rhs);
impl_single_binop!(Add, add, Integer, &Integer, Integer::add_assign, Integer, ref self);
impl_single_binop!(Add, add, &Integer, &Integer, Integer::add, Integer);
impl_single_binop!(Add, add, Integer, Integer, Integer::add_assign, Integer, ref self, ref rhs);

impl_single_binop!(Add, add, Integer, i8, Integer::add_c_long_assign, Integer, ref self);
impl_single_binop!(Add, add, &Integer, i8, Integer::add_c_long, Integer);
impl_single_binop!(Add, add, Integer, &i8, Integer::add_c_long_assign, Integer, ref self, deref rhs);
impl_single_binop!(Add, add, &Integer, &i8, Integer::add_c_long, Integer, deref rhs);

impl_single_binop!(Add, add, Integer, u8, Integer::add_c_long_assign, Integer, ref self);
impl_single_binop!(Add, add, &Integer, u8, Integer::add_c_long, Integer);
impl_single_binop!(Add, add, Integer, &u8, Integer::add_c_long_assign, Integer, ref self, deref rhs);
impl_single_binop!(Add, add, &Integer, &u8, Integer::add_c_long, Integer, deref rhs);

impl_single_binop!(Add, add, Integer, i16, Integer::add_c_long_assign, Integer, ref self);
impl_single_binop!(Add, add, &Integer, i16, Integer::add_c_long, Integer);
impl_single_binop!(Add, add, Integer, &i16, Integer::add_c_long_assign, Integer, ref self, deref rhs);
impl_single_binop!(Add, add, &Integer, &i16, Integer::add_c_long, Integer, deref rhs);

impl_single_binop!(Add, add, Integer, u16, Integer::add_c_long_assign, Integer, ref self);
impl_single_binop!(Add, add, &Integer, u16, Integer::add_c_long, Integer);
impl_single_binop!(Add, add, Integer, &u16, Integer::add_c_long_assign, Integer, ref self, deref rhs);
impl_single_binop!(Add, add, &Integer, &u16, Integer::add_c_long, Integer, deref rhs);

impl_single_binop!(Add, add, Integer, i32, Integer::add_c_long_assign, Integer, ref self);
impl_single_binop!(Add, add, &Integer, i32, Integer::add_c_long, Integer);
impl_single_binop!(Add, add, Integer, &i32, Integer::add_c_long_assign, Integer, ref self, deref rhs);
impl_single_binop!(Add, add, &Integer, &i32, Integer::add_c_long, Integer, deref rhs);

impl_single_binop!(Add, add, Integer, u32, Integer::add_c_long_assign, Integer, ref self);
impl_single_binop!(Add, add, &Integer, u32, Integer::add_c_long, Integer);
impl_single_binop!(Add, add, Integer, &u32, Integer::add_c_long_assign, Integer, ref self, deref rhs);
impl_single_binop!(Add, add, &Integer, &u32, Integer::add_c_long, Integer, deref rhs);

cfg_if::cfg_if! {
    if #[cfg(all(target_pointer_width = "64", not(windows)))] {
        impl_single_binop!(Add, add, Integer, i64, Integer::add_c_long_assign, Integer, ref self);
        impl_single_binop!(Add, add, &Integer, i64, Integer::add_c_long, Integer);
        impl_single_binop!(Add, add, Integer, &i64, Integer::add_c_long_assign, Integer, ref self, deref rhs);
        impl_single_binop!(Add, add, &Integer, &i64, Integer::add_c_long, Integer, deref rhs);
    } else {
        impl_single_binop!(Add, add, Integer, i64, Integer::add_assign, Integer, ref self, into rhs);
        impl_single_binop!(Add, add, &Integer, i64, Integer::add, Integer, into rhs);
        impl_single_binop!(Add, add, Integer, &i64, Integer::add_assign, Integer, ref self, into rhs);
        impl_single_binop!(Add, add, &Integer, &i64, Integer::add, Integer, into rhs);
    }
}

impl_single_binop!(Add, add, Integer, u64, Integer::add_assign, Integer, ref self, into rhs);
impl_single_binop!(Add, add, &Integer, u64, Integer::add, Integer, into rhs);
impl_single_binop!(Add, add, Integer, &u64, Integer::add_assign, Integer, ref self, into rhs);
impl_single_binop!(Add, add, &Integer, &u64, Integer::add, Integer, into rhs);

impl_single_binop!(Add, add, Integer, i128, Integer::add_assign, Integer, ref self, into rhs);
impl_single_binop!(Add, add, &Integer, i128, Integer::add, Integer, into rhs);
impl_single_binop!(Add, add, Integer, &i128, Integer::add_assign, Integer, ref self, into rhs);
impl_single_binop!(Add, add, &Integer, &i128, Integer::add, Integer, into rhs);

impl_single_binop!(Add, add, Integer, u128, Integer::add_assign, Integer, ref self, into rhs);
impl_single_binop!(Add, add, &Integer, u128, Integer::add, Integer, into rhs);
impl_single_binop!(Add, add, Integer, &u128, Integer::add_assign, Integer, ref self, into rhs);
impl_single_binop!(Add, add, &Integer, &u128, Integer::add, Integer, into rhs);

// AddAssign    The addition assignment operator +=.

impl_single_op_assign!(
    AddAssign,
    add_assign,
    Integer,
    &Integer,
    Integer::add_assign
);
impl_single_op_assign!(
    AddAssign,
    add_assign,
    Integer,
    Integer,
    Integer::add_assign,
    ref rhs
);

impl_single_op_assign!(
    AddAssign,
    add_assign,
    Integer,
    i8,
    Integer::add_c_long_assign
);
impl_single_op_assign!(AddAssign, add_assign, Integer, &i8, Integer::add_c_long_assign, deref rhs);

impl_single_op_assign!(
    AddAssign,
    add_assign,
    Integer,
    u8,
    Integer::add_c_long_assign
);
impl_single_op_assign!(AddAssign, add_assign, Integer, &u8, Integer::add_c_long_assign, deref rhs);

impl_single_op_assign!(
    AddAssign,
    add_assign,
    Integer,
    i16,
    Integer::add_c_long_assign
);
impl_single_op_assign!(AddAssign, add_assign, Integer, &i16, Integer::add_c_long_assign, deref rhs);

impl_single_op_assign!(
    AddAssign,
    add_assign,
    Integer,
    u16,
    Integer::add_c_long_assign
);
impl_single_op_assign!(AddAssign, add_assign, Integer, &u16, Integer::add_c_long_assign, deref rhs);

impl_single_op_assign!(
    AddAssign,
    add_assign,
    Integer,
    i32,
    Integer::add_c_long_assign
);
impl_single_op_assign!(AddAssign, add_assign, Integer, &i32, Integer::add_c_long_assign, deref rhs);

impl_single_op_assign!(
    AddAssign,
    add_assign,
    Integer,
    u32,
    Integer::add_c_long_assign
);
impl_single_op_assign!(AddAssign, add_assign, Integer, &u32, Integer::add_c_long_assign, deref rhs);

cfg_if::cfg_if! {
    if #[cfg(all(target_pointer_width = "64", not(windows)))] {
        impl_single_op_assign!(AddAssign, add_assign, Integer, i64, Integer::add_c_long_assign);
        impl_single_op_assign!(AddAssign, add_assign, Integer, &i64, Integer::add_c_long_assign, deref rhs);
    } else {
        impl_single_op_assign!(AddAssign, add_assign, Integer, i64, Integer::add_assign, into rhs);
        impl_single_op_assign!(AddAssign, add_assign, Integer, &i64, Integer::add_assign, into rhs);
    }
}

impl_single_op_assign!(AddAssign, add_assign, Integer, u64, Integer::add_assign, into rhs);
impl_single_op_assign!(AddAssign, add_assign, Integer, &u64, Integer::add_assign, into rhs);

impl_single_op_assign!(AddAssign, add_assign, Integer, i128, Integer::add_assign, into rhs);
impl_single_op_assign!(AddAssign, add_assign, Integer, &i128, Integer::add_assign, into rhs);

impl_single_op_assign!(AddAssign, add_assign, Integer, u128, Integer::add_assign, into rhs);
impl_single_op_assign!(AddAssign, add_assign, Integer, &u128, Integer::add_assign, into rhs);
