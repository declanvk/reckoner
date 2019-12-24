use super::helpers::{remainder_ref, remainder_reuse, reverse_remainder};
use crate::integer::Integer;
use core::ops::Rem;

// Rem          The remainder operator %.

// [u'Integer', u'Integer', u'Integer', u'Integer::remainder_assign', u'lhs',
// [u'ref_mut'], [u'ref']]
impl Rem<Integer> for Integer {
    type Output = Integer;

    fn rem(mut self, rhs: Integer) -> Self::Output {
        Integer::remainder_assign(&mut self, &rhs);
        self
    }
}

// [u'Integer', u'&Integer', u'Integer', u'Integer::remainder_assign', u'lhs',
// [u'ref_mut'], []]
impl Rem<&Integer> for Integer {
    type Output = Integer;

    fn rem(mut self, rhs: &Integer) -> Self::Output {
        Integer::remainder_assign(&mut self, rhs);
        self
    }
}

// [u'&Integer', u'Integer', u'Integer', u'Integer::remainder', u'no', [],
// [u'ref']]
impl Rem<Integer> for &Integer {
    type Output = Integer;

    fn rem(self, rhs: Integer) -> Self::Output {
        Integer::remainder(self, &rhs)
    }
}

// [u'&Integer', u'&Integer', u'Integer', u'Integer::remainder', u'no', [], []]
impl Rem<&Integer> for &Integer {
    type Output = Integer;

    fn rem(self, rhs: &Integer) -> Self::Output {
        Integer::remainder(self, rhs)
    }
}

// [u'Integer', u'i8', u'i8', u'Integer::remainder_c_long', u'no', [u'ref'], []]
impl Rem<i8> for Integer {
    type Output = i8;

    fn rem(self, rhs: i8) -> Self::Output {
        Integer::remainder_c_long(&self, rhs)
    }
}

// [u'Integer', u'&i8', u'i8', u'Integer::remainder_c_long', u'no', [u'ref'],
// [u'deref']]
impl Rem<&i8> for Integer {
    type Output = i8;

    fn rem(self, rhs: &i8) -> Self::Output {
        Integer::remainder_c_long(&self, *rhs)
    }
}

// [u'&Integer', u'i8', u'i8', u'Integer::remainder_c_long', u'no', [], []]
impl Rem<i8> for &Integer {
    type Output = i8;

    fn rem(self, rhs: i8) -> Self::Output {
        Integer::remainder_c_long(self, rhs)
    }
}

// [u'&Integer', u'&i8', u'i8', u'Integer::remainder_c_long', u'no', [],
// [u'deref']]
impl Rem<&i8> for &Integer {
    type Output = i8;

    fn rem(self, rhs: &i8) -> Self::Output {
        Integer::remainder_c_long(self, *rhs)
    }
}

// [u'i8', u'Integer', u'i8', u'reverse_remainder', u'no', [], [u'ref']]
impl Rem<Integer> for i8 {
    type Output = i8;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(self, &rhs)
    }
}

// [u'i8', u'&Integer', u'i8', u'reverse_remainder', u'no', [], []]
impl Rem<&Integer> for i8 {
    type Output = i8;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(self, rhs)
    }
}

// [u'&i8', u'Integer', u'i8', u'reverse_remainder', u'no', [u'deref'],
// [u'ref']]
impl Rem<Integer> for &i8 {
    type Output = i8;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(*self, &rhs)
    }
}

// [u'&i8', u'&Integer', u'i8', u'reverse_remainder', u'no', [u'deref'], []]
impl Rem<&Integer> for &i8 {
    type Output = i8;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(*self, rhs)
    }
}

// [u'Integer', u'u8', u'u8', u'Integer::remainder_c_long', u'no', [u'ref'], []]
impl Rem<u8> for Integer {
    type Output = u8;

    fn rem(self, rhs: u8) -> Self::Output {
        Integer::remainder_c_long(&self, rhs)
    }
}

// [u'Integer', u'&u8', u'u8', u'Integer::remainder_c_long', u'no', [u'ref'],
// [u'deref']]
impl Rem<&u8> for Integer {
    type Output = u8;

    fn rem(self, rhs: &u8) -> Self::Output {
        Integer::remainder_c_long(&self, *rhs)
    }
}

// [u'&Integer', u'u8', u'u8', u'Integer::remainder_c_long', u'no', [], []]
impl Rem<u8> for &Integer {
    type Output = u8;

    fn rem(self, rhs: u8) -> Self::Output {
        Integer::remainder_c_long(self, rhs)
    }
}

// [u'&Integer', u'&u8', u'u8', u'Integer::remainder_c_long', u'no', [],
// [u'deref']]
impl Rem<&u8> for &Integer {
    type Output = u8;

    fn rem(self, rhs: &u8) -> Self::Output {
        Integer::remainder_c_long(self, *rhs)
    }
}

// [u'u8', u'Integer', u'u8', u'reverse_remainder', u'no', [], [u'ref']]
impl Rem<Integer> for u8 {
    type Output = u8;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(self, &rhs)
    }
}

// [u'u8', u'&Integer', u'u8', u'reverse_remainder', u'no', [], []]
impl Rem<&Integer> for u8 {
    type Output = u8;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(self, rhs)
    }
}

// [u'&u8', u'Integer', u'u8', u'reverse_remainder', u'no', [u'deref'],
// [u'ref']]
impl Rem<Integer> for &u8 {
    type Output = u8;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(*self, &rhs)
    }
}

// [u'&u8', u'&Integer', u'u8', u'reverse_remainder', u'no', [u'deref'], []]
impl Rem<&Integer> for &u8 {
    type Output = u8;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(*self, rhs)
    }
}

// [u'Integer', u'i16', u'i16', u'Integer::remainder_c_long', u'no', [u'ref'],
// []]
impl Rem<i16> for Integer {
    type Output = i16;

    fn rem(self, rhs: i16) -> Self::Output {
        Integer::remainder_c_long(&self, rhs)
    }
}

// [u'Integer', u'&i16', u'i16', u'Integer::remainder_c_long', u'no', [u'ref'],
// [u'deref']]
impl Rem<&i16> for Integer {
    type Output = i16;

    fn rem(self, rhs: &i16) -> Self::Output {
        Integer::remainder_c_long(&self, *rhs)
    }
}

// [u'&Integer', u'i16', u'i16', u'Integer::remainder_c_long', u'no', [], []]
impl Rem<i16> for &Integer {
    type Output = i16;

    fn rem(self, rhs: i16) -> Self::Output {
        Integer::remainder_c_long(self, rhs)
    }
}

// [u'&Integer', u'&i16', u'i16', u'Integer::remainder_c_long', u'no', [],
// [u'deref']]
impl Rem<&i16> for &Integer {
    type Output = i16;

    fn rem(self, rhs: &i16) -> Self::Output {
        Integer::remainder_c_long(self, *rhs)
    }
}

// [u'i16', u'Integer', u'i16', u'reverse_remainder', u'no', [], [u'ref']]
impl Rem<Integer> for i16 {
    type Output = i16;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(self, &rhs)
    }
}

// [u'i16', u'&Integer', u'i16', u'reverse_remainder', u'no', [], []]
impl Rem<&Integer> for i16 {
    type Output = i16;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(self, rhs)
    }
}

// [u'&i16', u'Integer', u'i16', u'reverse_remainder', u'no', [u'deref'],
// [u'ref']]
impl Rem<Integer> for &i16 {
    type Output = i16;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(*self, &rhs)
    }
}

// [u'&i16', u'&Integer', u'i16', u'reverse_remainder', u'no', [u'deref'], []]
impl Rem<&Integer> for &i16 {
    type Output = i16;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(*self, rhs)
    }
}

// [u'Integer', u'u16', u'u16', u'Integer::remainder_c_long', u'no', [u'ref'],
// []]
impl Rem<u16> for Integer {
    type Output = u16;

    fn rem(self, rhs: u16) -> Self::Output {
        Integer::remainder_c_long(&self, rhs)
    }
}

// [u'Integer', u'&u16', u'u16', u'Integer::remainder_c_long', u'no', [u'ref'],
// [u'deref']]
impl Rem<&u16> for Integer {
    type Output = u16;

    fn rem(self, rhs: &u16) -> Self::Output {
        Integer::remainder_c_long(&self, *rhs)
    }
}

// [u'&Integer', u'u16', u'u16', u'Integer::remainder_c_long', u'no', [], []]
impl Rem<u16> for &Integer {
    type Output = u16;

    fn rem(self, rhs: u16) -> Self::Output {
        Integer::remainder_c_long(self, rhs)
    }
}

// [u'&Integer', u'&u16', u'u16', u'Integer::remainder_c_long', u'no', [],
// [u'deref']]
impl Rem<&u16> for &Integer {
    type Output = u16;

    fn rem(self, rhs: &u16) -> Self::Output {
        Integer::remainder_c_long(self, *rhs)
    }
}

// [u'u16', u'Integer', u'u16', u'reverse_remainder', u'no', [], [u'ref']]
impl Rem<Integer> for u16 {
    type Output = u16;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(self, &rhs)
    }
}

// [u'u16', u'&Integer', u'u16', u'reverse_remainder', u'no', [], []]
impl Rem<&Integer> for u16 {
    type Output = u16;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(self, rhs)
    }
}

// [u'&u16', u'Integer', u'u16', u'reverse_remainder', u'no', [u'deref'],
// [u'ref']]
impl Rem<Integer> for &u16 {
    type Output = u16;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(*self, &rhs)
    }
}

// [u'&u16', u'&Integer', u'u16', u'reverse_remainder', u'no', [u'deref'], []]
impl Rem<&Integer> for &u16 {
    type Output = u16;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(*self, rhs)
    }
}

// [u'Integer', u'i32', u'i32', u'Integer::remainder_c_long', u'no', [u'ref'],
// []]
impl Rem<i32> for Integer {
    type Output = i32;

    fn rem(self, rhs: i32) -> Self::Output {
        Integer::remainder_c_long(&self, rhs)
    }
}

// [u'Integer', u'&i32', u'i32', u'Integer::remainder_c_long', u'no', [u'ref'],
// [u'deref']]
impl Rem<&i32> for Integer {
    type Output = i32;

    fn rem(self, rhs: &i32) -> Self::Output {
        Integer::remainder_c_long(&self, *rhs)
    }
}

// [u'&Integer', u'i32', u'i32', u'Integer::remainder_c_long', u'no', [], []]
impl Rem<i32> for &Integer {
    type Output = i32;

    fn rem(self, rhs: i32) -> Self::Output {
        Integer::remainder_c_long(self, rhs)
    }
}

// [u'&Integer', u'&i32', u'i32', u'Integer::remainder_c_long', u'no', [],
// [u'deref']]
impl Rem<&i32> for &Integer {
    type Output = i32;

    fn rem(self, rhs: &i32) -> Self::Output {
        Integer::remainder_c_long(self, *rhs)
    }
}

// [u'i32', u'Integer', u'i32', u'reverse_remainder', u'no', [], [u'ref']]
impl Rem<Integer> for i32 {
    type Output = i32;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(self, &rhs)
    }
}

// [u'i32', u'&Integer', u'i32', u'reverse_remainder', u'no', [], []]
impl Rem<&Integer> for i32 {
    type Output = i32;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(self, rhs)
    }
}

// [u'&i32', u'Integer', u'i32', u'reverse_remainder', u'no', [u'deref'],
// [u'ref']]
impl Rem<Integer> for &i32 {
    type Output = i32;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(*self, &rhs)
    }
}

// [u'&i32', u'&Integer', u'i32', u'reverse_remainder', u'no', [u'deref'], []]
impl Rem<&Integer> for &i32 {
    type Output = i32;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(*self, rhs)
    }
}

// [u'Integer', u'u32', u'u32', u'Integer::remainder_c_long', u'no', [u'ref'],
// []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Rem<u32> for Integer {
    type Output = u32;

    fn rem(self, rhs: u32) -> Self::Output {
        Integer::remainder_c_long(&self, rhs)
    }
}

// [u'Integer', u'&u32', u'u32', u'Integer::remainder_c_long', u'no', [u'ref'],
// [u'deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Rem<&u32> for Integer {
    type Output = u32;

    fn rem(self, rhs: &u32) -> Self::Output {
        Integer::remainder_c_long(&self, *rhs)
    }
}

// [u'&Integer', u'u32', u'u32', u'Integer::remainder_c_long', u'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Rem<u32> for &Integer {
    type Output = u32;

    fn rem(self, rhs: u32) -> Self::Output {
        Integer::remainder_c_long(self, rhs)
    }
}

// [u'&Integer', u'&u32', u'u32', u'Integer::remainder_c_long', u'no', [],
// [u'deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Rem<&u32> for &Integer {
    type Output = u32;

    fn rem(self, rhs: &u32) -> Self::Output {
        Integer::remainder_c_long(self, *rhs)
    }
}

// [u'u32', u'Integer', u'u32', u'reverse_remainder', u'no', [], [u'ref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Rem<Integer> for u32 {
    type Output = u32;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(self, &rhs)
    }
}

// [u'u32', u'&Integer', u'u32', u'reverse_remainder', u'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Rem<&Integer> for u32 {
    type Output = u32;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(self, rhs)
    }
}

// [u'&u32', u'Integer', u'u32', u'reverse_remainder', u'no', [u'deref'],
// [u'ref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Rem<Integer> for &u32 {
    type Output = u32;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(*self, &rhs)
    }
}

// [u'&u32', u'&Integer', u'u32', u'reverse_remainder', u'no', [u'deref'], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Rem<&Integer> for &u32 {
    type Output = u32;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(*self, rhs)
    }
}

// [u'Integer', u'i64', u'i64', u'Integer::remainder_c_long', u'no', [u'ref'],
// []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Rem<i64> for Integer {
    type Output = i64;

    fn rem(self, rhs: i64) -> Self::Output {
        Integer::remainder_c_long(&self, rhs)
    }
}

// [u'Integer', u'&i64', u'i64', u'Integer::remainder_c_long', u'no', [u'ref'],
// [u'deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Rem<&i64> for Integer {
    type Output = i64;

    fn rem(self, rhs: &i64) -> Self::Output {
        Integer::remainder_c_long(&self, *rhs)
    }
}

// [u'&Integer', u'i64', u'i64', u'Integer::remainder_c_long', u'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Rem<i64> for &Integer {
    type Output = i64;

    fn rem(self, rhs: i64) -> Self::Output {
        Integer::remainder_c_long(self, rhs)
    }
}

// [u'&Integer', u'&i64', u'i64', u'Integer::remainder_c_long', u'no', [],
// [u'deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Rem<&i64> for &Integer {
    type Output = i64;

    fn rem(self, rhs: &i64) -> Self::Output {
        Integer::remainder_c_long(self, *rhs)
    }
}

// [u'i64', u'Integer', u'i64', u'reverse_remainder', u'no', [], [u'ref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Rem<Integer> for i64 {
    type Output = i64;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(self, &rhs)
    }
}

// [u'i64', u'&Integer', u'i64', u'reverse_remainder', u'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Rem<&Integer> for i64 {
    type Output = i64;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(self, rhs)
    }
}

// [u'&i64', u'Integer', u'i64', u'reverse_remainder', u'no', [u'deref'],
// [u'ref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Rem<Integer> for &i64 {
    type Output = i64;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(*self, &rhs)
    }
}

// [u'&i64', u'&Integer', u'i64', u'reverse_remainder', u'no', [u'deref'], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Rem<&Integer> for &i64 {
    type Output = i64;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(*self, rhs)
    }
}

// [u'Integer', u'u32', u'u32', u'remainder_reuse', u'no', [], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Rem<u32> for Integer {
    type Output = u32;

    fn rem(self, rhs: u32) -> Self::Output {
        remainder_reuse(self, rhs)
    }
}

// [u'Integer', u'&u32', u'u32', u'remainder_reuse', u'no', [], [u'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Rem<&u32> for Integer {
    type Output = u32;

    fn rem(self, rhs: &u32) -> Self::Output {
        remainder_reuse(self, *rhs)
    }
}

// [u'&Integer', u'u32', u'u32', u'remainder_ref', u'no', [], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Rem<u32> for &Integer {
    type Output = u32;

    fn rem(self, rhs: u32) -> Self::Output {
        remainder_ref(self, rhs)
    }
}

// [u'&Integer', u'&u32', u'u32', u'remainder_ref', u'no', [], [u'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Rem<&u32> for &Integer {
    type Output = u32;

    fn rem(self, rhs: &u32) -> Self::Output {
        remainder_ref(self, *rhs)
    }
}

// [u'u32', u'Integer', u'u32', u'reverse_remainder', u'no', [], [u'ref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Rem<Integer> for u32 {
    type Output = u32;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(self, &rhs)
    }
}

// [u'u32', u'&Integer', u'u32', u'reverse_remainder', u'no', [], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Rem<&Integer> for u32 {
    type Output = u32;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(self, rhs)
    }
}

// [u'&u32', u'Integer', u'u32', u'reverse_remainder', u'no', [u'deref'],
// [u'ref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Rem<Integer> for &u32 {
    type Output = u32;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(*self, &rhs)
    }
}

// [u'&u32', u'&Integer', u'u32', u'reverse_remainder', u'no', [u'deref'], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Rem<&Integer> for &u32 {
    type Output = u32;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(*self, rhs)
    }
}

// [u'Integer', u'i64', u'i64', u'remainder_reuse', u'no', [], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Rem<i64> for Integer {
    type Output = i64;

    fn rem(self, rhs: i64) -> Self::Output {
        remainder_reuse(self, rhs)
    }
}

// [u'Integer', u'&i64', u'i64', u'remainder_reuse', u'no', [], [u'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Rem<&i64> for Integer {
    type Output = i64;

    fn rem(self, rhs: &i64) -> Self::Output {
        remainder_reuse(self, *rhs)
    }
}

// [u'&Integer', u'i64', u'i64', u'remainder_ref', u'no', [], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Rem<i64> for &Integer {
    type Output = i64;

    fn rem(self, rhs: i64) -> Self::Output {
        remainder_ref(self, rhs)
    }
}

// [u'&Integer', u'&i64', u'i64', u'remainder_ref', u'no', [], [u'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Rem<&i64> for &Integer {
    type Output = i64;

    fn rem(self, rhs: &i64) -> Self::Output {
        remainder_ref(self, *rhs)
    }
}

// [u'i64', u'Integer', u'i64', u'reverse_remainder', u'no', [], [u'ref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Rem<Integer> for i64 {
    type Output = i64;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(self, &rhs)
    }
}

// [u'i64', u'&Integer', u'i64', u'reverse_remainder', u'no', [], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Rem<&Integer> for i64 {
    type Output = i64;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(self, rhs)
    }
}

// [u'&i64', u'Integer', u'i64', u'reverse_remainder', u'no', [u'deref'],
// [u'ref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Rem<Integer> for &i64 {
    type Output = i64;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(*self, &rhs)
    }
}

// [u'&i64', u'&Integer', u'i64', u'reverse_remainder', u'no', [u'deref'], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Rem<&Integer> for &i64 {
    type Output = i64;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(*self, rhs)
    }
}

// [u'Integer', u'u64', u'u64', u'remainder_reuse', u'no', [], []]
impl Rem<u64> for Integer {
    type Output = u64;

    fn rem(self, rhs: u64) -> Self::Output {
        remainder_reuse(self, rhs)
    }
}

// [u'Integer', u'&u64', u'u64', u'remainder_reuse', u'no', [], [u'deref']]
impl Rem<&u64> for Integer {
    type Output = u64;

    fn rem(self, rhs: &u64) -> Self::Output {
        remainder_reuse(self, *rhs)
    }
}

// [u'&Integer', u'u64', u'u64', u'remainder_ref', u'no', [], []]
impl Rem<u64> for &Integer {
    type Output = u64;

    fn rem(self, rhs: u64) -> Self::Output {
        remainder_ref(self, rhs)
    }
}

// [u'&Integer', u'&u64', u'u64', u'remainder_ref', u'no', [], [u'deref']]
impl Rem<&u64> for &Integer {
    type Output = u64;

    fn rem(self, rhs: &u64) -> Self::Output {
        remainder_ref(self, *rhs)
    }
}

// [u'u64', u'Integer', u'u64', u'reverse_remainder', u'no', [], [u'ref']]
impl Rem<Integer> for u64 {
    type Output = u64;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(self, &rhs)
    }
}

// [u'u64', u'&Integer', u'u64', u'reverse_remainder', u'no', [], []]
impl Rem<&Integer> for u64 {
    type Output = u64;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(self, rhs)
    }
}

// [u'&u64', u'Integer', u'u64', u'reverse_remainder', u'no', [u'deref'],
// [u'ref']]
impl Rem<Integer> for &u64 {
    type Output = u64;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(*self, &rhs)
    }
}

// [u'&u64', u'&Integer', u'u64', u'reverse_remainder', u'no', [u'deref'], []]
impl Rem<&Integer> for &u64 {
    type Output = u64;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(*self, rhs)
    }
}

// [u'Integer', u'i128', u'i128', u'remainder_reuse', u'no', [], []]
impl Rem<i128> for Integer {
    type Output = i128;

    fn rem(self, rhs: i128) -> Self::Output {
        remainder_reuse(self, rhs)
    }
}

// [u'Integer', u'&i128', u'i128', u'remainder_reuse', u'no', [], [u'deref']]
impl Rem<&i128> for Integer {
    type Output = i128;

    fn rem(self, rhs: &i128) -> Self::Output {
        remainder_reuse(self, *rhs)
    }
}

// [u'&Integer', u'i128', u'i128', u'remainder_ref', u'no', [], []]
impl Rem<i128> for &Integer {
    type Output = i128;

    fn rem(self, rhs: i128) -> Self::Output {
        remainder_ref(self, rhs)
    }
}

// [u'&Integer', u'&i128', u'i128', u'remainder_ref', u'no', [], [u'deref']]
impl Rem<&i128> for &Integer {
    type Output = i128;

    fn rem(self, rhs: &i128) -> Self::Output {
        remainder_ref(self, *rhs)
    }
}

// [u'i128', u'Integer', u'i128', u'reverse_remainder', u'no', [], [u'ref']]
impl Rem<Integer> for i128 {
    type Output = i128;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(self, &rhs)
    }
}

// [u'i128', u'&Integer', u'i128', u'reverse_remainder', u'no', [], []]
impl Rem<&Integer> for i128 {
    type Output = i128;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(self, rhs)
    }
}

// [u'&i128', u'Integer', u'i128', u'reverse_remainder', u'no', [u'deref'],
// [u'ref']]
impl Rem<Integer> for &i128 {
    type Output = i128;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(*self, &rhs)
    }
}

// [u'&i128', u'&Integer', u'i128', u'reverse_remainder', u'no', [u'deref'], []]
impl Rem<&Integer> for &i128 {
    type Output = i128;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(*self, rhs)
    }
}

// [u'Integer', u'u128', u'u128', u'remainder_reuse', u'no', [], []]
impl Rem<u128> for Integer {
    type Output = u128;

    fn rem(self, rhs: u128) -> Self::Output {
        remainder_reuse(self, rhs)
    }
}

// [u'Integer', u'&u128', u'u128', u'remainder_reuse', u'no', [], [u'deref']]
impl Rem<&u128> for Integer {
    type Output = u128;

    fn rem(self, rhs: &u128) -> Self::Output {
        remainder_reuse(self, *rhs)
    }
}

// [u'&Integer', u'u128', u'u128', u'remainder_ref', u'no', [], []]
impl Rem<u128> for &Integer {
    type Output = u128;

    fn rem(self, rhs: u128) -> Self::Output {
        remainder_ref(self, rhs)
    }
}

// [u'&Integer', u'&u128', u'u128', u'remainder_ref', u'no', [], [u'deref']]
impl Rem<&u128> for &Integer {
    type Output = u128;

    fn rem(self, rhs: &u128) -> Self::Output {
        remainder_ref(self, *rhs)
    }
}

// [u'u128', u'Integer', u'u128', u'reverse_remainder', u'no', [], [u'ref']]
impl Rem<Integer> for u128 {
    type Output = u128;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(self, &rhs)
    }
}

// [u'u128', u'&Integer', u'u128', u'reverse_remainder', u'no', [], []]
impl Rem<&Integer> for u128 {
    type Output = u128;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(self, rhs)
    }
}

// [u'&u128', u'Integer', u'u128', u'reverse_remainder', u'no', [u'deref'],
// [u'ref']]
impl Rem<Integer> for &u128 {
    type Output = u128;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(*self, &rhs)
    }
}

// [u'&u128', u'&Integer', u'u128', u'reverse_remainder', u'no', [u'deref'], []]
impl Rem<&Integer> for &u128 {
    type Output = u128;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(*self, rhs)
    }
}
