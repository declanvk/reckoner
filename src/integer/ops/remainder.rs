// Rem          The remainder operator %.
// RemAssign    The remainder assignment operator %=.

use crate::integer::Integer;
use core::ops::Rem;

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

impl_single_binop!(Rem, rem, Integer, u32, Integer::remainder_c_long, u32, ref self, no reuse);
impl_single_binop!(Rem, rem, &Integer, u32, Integer::remainder_c_long, u32);
impl_single_binop!(Rem, rem, Integer, &u32, Integer::remainder_c_long, u32, ref self, deref rhs, no reuse);
impl_single_binop!(Rem, rem, &Integer, &u32, Integer::remainder_c_long, u32, deref rhs);

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
