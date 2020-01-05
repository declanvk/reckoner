use super::helpers::{remainder_ref, remainder_reuse, reverse_remainder};
use crate::integer::Integer;
use core::ops::Rem;

// Rem          The remainder operator %.

// ['Integer', 'Integer', 'Integer', 'Integer::remainder_assign', 'lhs',
// ['ref_mut'], ['ref']]
impl Rem<Integer> for Integer {
    type Output = Integer;

    fn rem(mut self, rhs: Integer) -> Self::Output {
        Integer::remainder_assign(&mut self, &rhs);
        self
    }
}

// ['Integer', '&Integer', 'Integer', 'Integer::remainder_assign', 'lhs',
// ['ref_mut'], []]
impl Rem<&Integer> for Integer {
    type Output = Integer;

    fn rem(mut self, rhs: &Integer) -> Self::Output {
        Integer::remainder_assign(&mut self, rhs);
        self
    }
}

// ['&Integer', 'Integer', 'Integer', 'Integer::remainder', 'no', [], ['ref']]
impl Rem<Integer> for &Integer {
    type Output = Integer;

    fn rem(self, rhs: Integer) -> Self::Output {
        Integer::remainder(self, &rhs)
    }
}

// ['&Integer', '&Integer', 'Integer', 'Integer::remainder', 'no', [], []]
impl Rem<&Integer> for &Integer {
    type Output = Integer;

    fn rem(self, rhs: &Integer) -> Self::Output {
        Integer::remainder(self, rhs)
    }
}

// ['Integer', 'i8', 'i8', 'Integer::remainder_c_long', 'no', ['ref'], []]
impl Rem<i8> for Integer {
    type Output = i8;

    fn rem(self, rhs: i8) -> Self::Output {
        Integer::remainder_c_long(&self, rhs)
    }
}

// ['Integer', '&i8', 'i8', 'Integer::remainder_c_long', 'no', ['ref'],
// ['deref']]
impl Rem<&i8> for Integer {
    type Output = i8;

    fn rem(self, rhs: &i8) -> Self::Output {
        Integer::remainder_c_long(&self, *rhs)
    }
}

// ['&Integer', 'i8', 'i8', 'Integer::remainder_c_long', 'no', [], []]
impl Rem<i8> for &Integer {
    type Output = i8;

    fn rem(self, rhs: i8) -> Self::Output {
        Integer::remainder_c_long(self, rhs)
    }
}

// ['&Integer', '&i8', 'i8', 'Integer::remainder_c_long', 'no', [], ['deref']]
impl Rem<&i8> for &Integer {
    type Output = i8;

    fn rem(self, rhs: &i8) -> Self::Output {
        Integer::remainder_c_long(self, *rhs)
    }
}

// ['i8', 'Integer', 'i8', 'reverse_remainder', 'no', [], ['ref']]
impl Rem<Integer> for i8 {
    type Output = i8;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(self, &rhs)
    }
}

// ['i8', '&Integer', 'i8', 'reverse_remainder', 'no', [], []]
impl Rem<&Integer> for i8 {
    type Output = i8;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(self, rhs)
    }
}

// ['&i8', 'Integer', 'i8', 'reverse_remainder', 'no', ['deref'], ['ref']]
impl Rem<Integer> for &i8 {
    type Output = i8;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(*self, &rhs)
    }
}

// ['&i8', '&Integer', 'i8', 'reverse_remainder', 'no', ['deref'], []]
impl Rem<&Integer> for &i8 {
    type Output = i8;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(*self, rhs)
    }
}

// ['Integer', 'u8', 'u8', 'Integer::remainder_c_long', 'no', ['ref'], []]
impl Rem<u8> for Integer {
    type Output = u8;

    fn rem(self, rhs: u8) -> Self::Output {
        Integer::remainder_c_long(&self, rhs)
    }
}

// ['Integer', '&u8', 'u8', 'Integer::remainder_c_long', 'no', ['ref'],
// ['deref']]
impl Rem<&u8> for Integer {
    type Output = u8;

    fn rem(self, rhs: &u8) -> Self::Output {
        Integer::remainder_c_long(&self, *rhs)
    }
}

// ['&Integer', 'u8', 'u8', 'Integer::remainder_c_long', 'no', [], []]
impl Rem<u8> for &Integer {
    type Output = u8;

    fn rem(self, rhs: u8) -> Self::Output {
        Integer::remainder_c_long(self, rhs)
    }
}

// ['&Integer', '&u8', 'u8', 'Integer::remainder_c_long', 'no', [], ['deref']]
impl Rem<&u8> for &Integer {
    type Output = u8;

    fn rem(self, rhs: &u8) -> Self::Output {
        Integer::remainder_c_long(self, *rhs)
    }
}

// ['u8', 'Integer', 'u8', 'reverse_remainder', 'no', [], ['ref']]
impl Rem<Integer> for u8 {
    type Output = u8;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(self, &rhs)
    }
}

// ['u8', '&Integer', 'u8', 'reverse_remainder', 'no', [], []]
impl Rem<&Integer> for u8 {
    type Output = u8;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(self, rhs)
    }
}

// ['&u8', 'Integer', 'u8', 'reverse_remainder', 'no', ['deref'], ['ref']]
impl Rem<Integer> for &u8 {
    type Output = u8;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(*self, &rhs)
    }
}

// ['&u8', '&Integer', 'u8', 'reverse_remainder', 'no', ['deref'], []]
impl Rem<&Integer> for &u8 {
    type Output = u8;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(*self, rhs)
    }
}

// ['Integer', 'i16', 'i16', 'Integer::remainder_c_long', 'no', ['ref'], []]
impl Rem<i16> for Integer {
    type Output = i16;

    fn rem(self, rhs: i16) -> Self::Output {
        Integer::remainder_c_long(&self, rhs)
    }
}

// ['Integer', '&i16', 'i16', 'Integer::remainder_c_long', 'no', ['ref'],
// ['deref']]
impl Rem<&i16> for Integer {
    type Output = i16;

    fn rem(self, rhs: &i16) -> Self::Output {
        Integer::remainder_c_long(&self, *rhs)
    }
}

// ['&Integer', 'i16', 'i16', 'Integer::remainder_c_long', 'no', [], []]
impl Rem<i16> for &Integer {
    type Output = i16;

    fn rem(self, rhs: i16) -> Self::Output {
        Integer::remainder_c_long(self, rhs)
    }
}

// ['&Integer', '&i16', 'i16', 'Integer::remainder_c_long', 'no', [], ['deref']]
impl Rem<&i16> for &Integer {
    type Output = i16;

    fn rem(self, rhs: &i16) -> Self::Output {
        Integer::remainder_c_long(self, *rhs)
    }
}

// ['i16', 'Integer', 'i16', 'reverse_remainder', 'no', [], ['ref']]
impl Rem<Integer> for i16 {
    type Output = i16;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(self, &rhs)
    }
}

// ['i16', '&Integer', 'i16', 'reverse_remainder', 'no', [], []]
impl Rem<&Integer> for i16 {
    type Output = i16;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(self, rhs)
    }
}

// ['&i16', 'Integer', 'i16', 'reverse_remainder', 'no', ['deref'], ['ref']]
impl Rem<Integer> for &i16 {
    type Output = i16;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(*self, &rhs)
    }
}

// ['&i16', '&Integer', 'i16', 'reverse_remainder', 'no', ['deref'], []]
impl Rem<&Integer> for &i16 {
    type Output = i16;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(*self, rhs)
    }
}

// ['Integer', 'u16', 'u16', 'Integer::remainder_c_long', 'no', ['ref'], []]
impl Rem<u16> for Integer {
    type Output = u16;

    fn rem(self, rhs: u16) -> Self::Output {
        Integer::remainder_c_long(&self, rhs)
    }
}

// ['Integer', '&u16', 'u16', 'Integer::remainder_c_long', 'no', ['ref'],
// ['deref']]
impl Rem<&u16> for Integer {
    type Output = u16;

    fn rem(self, rhs: &u16) -> Self::Output {
        Integer::remainder_c_long(&self, *rhs)
    }
}

// ['&Integer', 'u16', 'u16', 'Integer::remainder_c_long', 'no', [], []]
impl Rem<u16> for &Integer {
    type Output = u16;

    fn rem(self, rhs: u16) -> Self::Output {
        Integer::remainder_c_long(self, rhs)
    }
}

// ['&Integer', '&u16', 'u16', 'Integer::remainder_c_long', 'no', [], ['deref']]
impl Rem<&u16> for &Integer {
    type Output = u16;

    fn rem(self, rhs: &u16) -> Self::Output {
        Integer::remainder_c_long(self, *rhs)
    }
}

// ['u16', 'Integer', 'u16', 'reverse_remainder', 'no', [], ['ref']]
impl Rem<Integer> for u16 {
    type Output = u16;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(self, &rhs)
    }
}

// ['u16', '&Integer', 'u16', 'reverse_remainder', 'no', [], []]
impl Rem<&Integer> for u16 {
    type Output = u16;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(self, rhs)
    }
}

// ['&u16', 'Integer', 'u16', 'reverse_remainder', 'no', ['deref'], ['ref']]
impl Rem<Integer> for &u16 {
    type Output = u16;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(*self, &rhs)
    }
}

// ['&u16', '&Integer', 'u16', 'reverse_remainder', 'no', ['deref'], []]
impl Rem<&Integer> for &u16 {
    type Output = u16;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(*self, rhs)
    }
}

// ['Integer', 'i32', 'i32', 'Integer::remainder_c_long', 'no', ['ref'], []]
impl Rem<i32> for Integer {
    type Output = i32;

    fn rem(self, rhs: i32) -> Self::Output {
        Integer::remainder_c_long(&self, rhs)
    }
}

// ['Integer', '&i32', 'i32', 'Integer::remainder_c_long', 'no', ['ref'],
// ['deref']]
impl Rem<&i32> for Integer {
    type Output = i32;

    fn rem(self, rhs: &i32) -> Self::Output {
        Integer::remainder_c_long(&self, *rhs)
    }
}

// ['&Integer', 'i32', 'i32', 'Integer::remainder_c_long', 'no', [], []]
impl Rem<i32> for &Integer {
    type Output = i32;

    fn rem(self, rhs: i32) -> Self::Output {
        Integer::remainder_c_long(self, rhs)
    }
}

// ['&Integer', '&i32', 'i32', 'Integer::remainder_c_long', 'no', [], ['deref']]
impl Rem<&i32> for &Integer {
    type Output = i32;

    fn rem(self, rhs: &i32) -> Self::Output {
        Integer::remainder_c_long(self, *rhs)
    }
}

// ['i32', 'Integer', 'i32', 'reverse_remainder', 'no', [], ['ref']]
impl Rem<Integer> for i32 {
    type Output = i32;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(self, &rhs)
    }
}

// ['i32', '&Integer', 'i32', 'reverse_remainder', 'no', [], []]
impl Rem<&Integer> for i32 {
    type Output = i32;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(self, rhs)
    }
}

// ['&i32', 'Integer', 'i32', 'reverse_remainder', 'no', ['deref'], ['ref']]
impl Rem<Integer> for &i32 {
    type Output = i32;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(*self, &rhs)
    }
}

// ['&i32', '&Integer', 'i32', 'reverse_remainder', 'no', ['deref'], []]
impl Rem<&Integer> for &i32 {
    type Output = i32;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(*self, rhs)
    }
}

// ['Integer', 'u32', 'u32', 'Integer::remainder_c_long', 'no', ['ref'], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Rem<u32> for Integer {
    type Output = u32;

    fn rem(self, rhs: u32) -> Self::Output {
        Integer::remainder_c_long(&self, rhs)
    }
}

// ['Integer', '&u32', 'u32', 'Integer::remainder_c_long', 'no', ['ref'],
// ['deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Rem<&u32> for Integer {
    type Output = u32;

    fn rem(self, rhs: &u32) -> Self::Output {
        Integer::remainder_c_long(&self, *rhs)
    }
}

// ['&Integer', 'u32', 'u32', 'Integer::remainder_c_long', 'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Rem<u32> for &Integer {
    type Output = u32;

    fn rem(self, rhs: u32) -> Self::Output {
        Integer::remainder_c_long(self, rhs)
    }
}

// ['&Integer', '&u32', 'u32', 'Integer::remainder_c_long', 'no', [], ['deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Rem<&u32> for &Integer {
    type Output = u32;

    fn rem(self, rhs: &u32) -> Self::Output {
        Integer::remainder_c_long(self, *rhs)
    }
}

// ['u32', 'Integer', 'u32', 'reverse_remainder', 'no', [], ['ref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Rem<Integer> for u32 {
    type Output = u32;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(self, &rhs)
    }
}

// ['u32', '&Integer', 'u32', 'reverse_remainder', 'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Rem<&Integer> for u32 {
    type Output = u32;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(self, rhs)
    }
}

// ['&u32', 'Integer', 'u32', 'reverse_remainder', 'no', ['deref'], ['ref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Rem<Integer> for &u32 {
    type Output = u32;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(*self, &rhs)
    }
}

// ['&u32', '&Integer', 'u32', 'reverse_remainder', 'no', ['deref'], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Rem<&Integer> for &u32 {
    type Output = u32;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(*self, rhs)
    }
}

// ['Integer', 'i64', 'i64', 'Integer::remainder_c_long', 'no', ['ref'], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Rem<i64> for Integer {
    type Output = i64;

    fn rem(self, rhs: i64) -> Self::Output {
        Integer::remainder_c_long(&self, rhs)
    }
}

// ['Integer', '&i64', 'i64', 'Integer::remainder_c_long', 'no', ['ref'],
// ['deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Rem<&i64> for Integer {
    type Output = i64;

    fn rem(self, rhs: &i64) -> Self::Output {
        Integer::remainder_c_long(&self, *rhs)
    }
}

// ['&Integer', 'i64', 'i64', 'Integer::remainder_c_long', 'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Rem<i64> for &Integer {
    type Output = i64;

    fn rem(self, rhs: i64) -> Self::Output {
        Integer::remainder_c_long(self, rhs)
    }
}

// ['&Integer', '&i64', 'i64', 'Integer::remainder_c_long', 'no', [], ['deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Rem<&i64> for &Integer {
    type Output = i64;

    fn rem(self, rhs: &i64) -> Self::Output {
        Integer::remainder_c_long(self, *rhs)
    }
}

// ['i64', 'Integer', 'i64', 'reverse_remainder', 'no', [], ['ref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Rem<Integer> for i64 {
    type Output = i64;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(self, &rhs)
    }
}

// ['i64', '&Integer', 'i64', 'reverse_remainder', 'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Rem<&Integer> for i64 {
    type Output = i64;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(self, rhs)
    }
}

// ['&i64', 'Integer', 'i64', 'reverse_remainder', 'no', ['deref'], ['ref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Rem<Integer> for &i64 {
    type Output = i64;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(*self, &rhs)
    }
}

// ['&i64', '&Integer', 'i64', 'reverse_remainder', 'no', ['deref'], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Rem<&Integer> for &i64 {
    type Output = i64;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(*self, rhs)
    }
}

// ['Integer', 'u32', 'u32', 'remainder_reuse', 'no', [], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Rem<u32> for Integer {
    type Output = u32;

    fn rem(self, rhs: u32) -> Self::Output {
        remainder_reuse(self, rhs)
    }
}

// ['Integer', '&u32', 'u32', 'remainder_reuse', 'no', [], ['deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Rem<&u32> for Integer {
    type Output = u32;

    fn rem(self, rhs: &u32) -> Self::Output {
        remainder_reuse(self, *rhs)
    }
}

// ['&Integer', 'u32', 'u32', 'remainder_ref', 'no', [], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Rem<u32> for &Integer {
    type Output = u32;

    fn rem(self, rhs: u32) -> Self::Output {
        remainder_ref(self, rhs)
    }
}

// ['&Integer', '&u32', 'u32', 'remainder_ref', 'no', [], ['deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Rem<&u32> for &Integer {
    type Output = u32;

    fn rem(self, rhs: &u32) -> Self::Output {
        remainder_ref(self, *rhs)
    }
}

// ['u32', 'Integer', 'u32', 'reverse_remainder', 'no', [], ['ref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Rem<Integer> for u32 {
    type Output = u32;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(self, &rhs)
    }
}

// ['u32', '&Integer', 'u32', 'reverse_remainder', 'no', [], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Rem<&Integer> for u32 {
    type Output = u32;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(self, rhs)
    }
}

// ['&u32', 'Integer', 'u32', 'reverse_remainder', 'no', ['deref'], ['ref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Rem<Integer> for &u32 {
    type Output = u32;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(*self, &rhs)
    }
}

// ['&u32', '&Integer', 'u32', 'reverse_remainder', 'no', ['deref'], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Rem<&Integer> for &u32 {
    type Output = u32;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(*self, rhs)
    }
}

// ['Integer', 'i64', 'i64', 'remainder_reuse', 'no', [], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Rem<i64> for Integer {
    type Output = i64;

    fn rem(self, rhs: i64) -> Self::Output {
        remainder_reuse(self, rhs)
    }
}

// ['Integer', '&i64', 'i64', 'remainder_reuse', 'no', [], ['deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Rem<&i64> for Integer {
    type Output = i64;

    fn rem(self, rhs: &i64) -> Self::Output {
        remainder_reuse(self, *rhs)
    }
}

// ['&Integer', 'i64', 'i64', 'remainder_ref', 'no', [], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Rem<i64> for &Integer {
    type Output = i64;

    fn rem(self, rhs: i64) -> Self::Output {
        remainder_ref(self, rhs)
    }
}

// ['&Integer', '&i64', 'i64', 'remainder_ref', 'no', [], ['deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Rem<&i64> for &Integer {
    type Output = i64;

    fn rem(self, rhs: &i64) -> Self::Output {
        remainder_ref(self, *rhs)
    }
}

// ['i64', 'Integer', 'i64', 'reverse_remainder', 'no', [], ['ref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Rem<Integer> for i64 {
    type Output = i64;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(self, &rhs)
    }
}

// ['i64', '&Integer', 'i64', 'reverse_remainder', 'no', [], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Rem<&Integer> for i64 {
    type Output = i64;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(self, rhs)
    }
}

// ['&i64', 'Integer', 'i64', 'reverse_remainder', 'no', ['deref'], ['ref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Rem<Integer> for &i64 {
    type Output = i64;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(*self, &rhs)
    }
}

// ['&i64', '&Integer', 'i64', 'reverse_remainder', 'no', ['deref'], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Rem<&Integer> for &i64 {
    type Output = i64;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(*self, rhs)
    }
}

// ['Integer', 'u64', 'u64', 'remainder_reuse', 'no', [], []]
impl Rem<u64> for Integer {
    type Output = u64;

    fn rem(self, rhs: u64) -> Self::Output {
        remainder_reuse(self, rhs)
    }
}

// ['Integer', '&u64', 'u64', 'remainder_reuse', 'no', [], ['deref']]
impl Rem<&u64> for Integer {
    type Output = u64;

    fn rem(self, rhs: &u64) -> Self::Output {
        remainder_reuse(self, *rhs)
    }
}

// ['&Integer', 'u64', 'u64', 'remainder_ref', 'no', [], []]
impl Rem<u64> for &Integer {
    type Output = u64;

    fn rem(self, rhs: u64) -> Self::Output {
        remainder_ref(self, rhs)
    }
}

// ['&Integer', '&u64', 'u64', 'remainder_ref', 'no', [], ['deref']]
impl Rem<&u64> for &Integer {
    type Output = u64;

    fn rem(self, rhs: &u64) -> Self::Output {
        remainder_ref(self, *rhs)
    }
}

// ['u64', 'Integer', 'u64', 'reverse_remainder', 'no', [], ['ref']]
impl Rem<Integer> for u64 {
    type Output = u64;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(self, &rhs)
    }
}

// ['u64', '&Integer', 'u64', 'reverse_remainder', 'no', [], []]
impl Rem<&Integer> for u64 {
    type Output = u64;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(self, rhs)
    }
}

// ['&u64', 'Integer', 'u64', 'reverse_remainder', 'no', ['deref'], ['ref']]
impl Rem<Integer> for &u64 {
    type Output = u64;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(*self, &rhs)
    }
}

// ['&u64', '&Integer', 'u64', 'reverse_remainder', 'no', ['deref'], []]
impl Rem<&Integer> for &u64 {
    type Output = u64;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(*self, rhs)
    }
}

// ['Integer', 'i128', 'i128', 'remainder_reuse', 'no', [], []]
impl Rem<i128> for Integer {
    type Output = i128;

    fn rem(self, rhs: i128) -> Self::Output {
        remainder_reuse(self, rhs)
    }
}

// ['Integer', '&i128', 'i128', 'remainder_reuse', 'no', [], ['deref']]
impl Rem<&i128> for Integer {
    type Output = i128;

    fn rem(self, rhs: &i128) -> Self::Output {
        remainder_reuse(self, *rhs)
    }
}

// ['&Integer', 'i128', 'i128', 'remainder_ref', 'no', [], []]
impl Rem<i128> for &Integer {
    type Output = i128;

    fn rem(self, rhs: i128) -> Self::Output {
        remainder_ref(self, rhs)
    }
}

// ['&Integer', '&i128', 'i128', 'remainder_ref', 'no', [], ['deref']]
impl Rem<&i128> for &Integer {
    type Output = i128;

    fn rem(self, rhs: &i128) -> Self::Output {
        remainder_ref(self, *rhs)
    }
}

// ['i128', 'Integer', 'i128', 'reverse_remainder', 'no', [], ['ref']]
impl Rem<Integer> for i128 {
    type Output = i128;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(self, &rhs)
    }
}

// ['i128', '&Integer', 'i128', 'reverse_remainder', 'no', [], []]
impl Rem<&Integer> for i128 {
    type Output = i128;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(self, rhs)
    }
}

// ['&i128', 'Integer', 'i128', 'reverse_remainder', 'no', ['deref'], ['ref']]
impl Rem<Integer> for &i128 {
    type Output = i128;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(*self, &rhs)
    }
}

// ['&i128', '&Integer', 'i128', 'reverse_remainder', 'no', ['deref'], []]
impl Rem<&Integer> for &i128 {
    type Output = i128;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(*self, rhs)
    }
}

// ['Integer', 'u128', 'u128', 'remainder_reuse', 'no', [], []]
impl Rem<u128> for Integer {
    type Output = u128;

    fn rem(self, rhs: u128) -> Self::Output {
        remainder_reuse(self, rhs)
    }
}

// ['Integer', '&u128', 'u128', 'remainder_reuse', 'no', [], ['deref']]
impl Rem<&u128> for Integer {
    type Output = u128;

    fn rem(self, rhs: &u128) -> Self::Output {
        remainder_reuse(self, *rhs)
    }
}

// ['&Integer', 'u128', 'u128', 'remainder_ref', 'no', [], []]
impl Rem<u128> for &Integer {
    type Output = u128;

    fn rem(self, rhs: u128) -> Self::Output {
        remainder_ref(self, rhs)
    }
}

// ['&Integer', '&u128', 'u128', 'remainder_ref', 'no', [], ['deref']]
impl Rem<&u128> for &Integer {
    type Output = u128;

    fn rem(self, rhs: &u128) -> Self::Output {
        remainder_ref(self, *rhs)
    }
}

// ['u128', 'Integer', 'u128', 'reverse_remainder', 'no', [], ['ref']]
impl Rem<Integer> for u128 {
    type Output = u128;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(self, &rhs)
    }
}

// ['u128', '&Integer', 'u128', 'reverse_remainder', 'no', [], []]
impl Rem<&Integer> for u128 {
    type Output = u128;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(self, rhs)
    }
}

// ['&u128', 'Integer', 'u128', 'reverse_remainder', 'no', ['deref'], ['ref']]
impl Rem<Integer> for &u128 {
    type Output = u128;

    fn rem(self, rhs: Integer) -> Self::Output {
        reverse_remainder(*self, &rhs)
    }
}

// ['&u128', '&Integer', 'u128', 'reverse_remainder', 'no', ['deref'], []]
impl Rem<&Integer> for &u128 {
    type Output = u128;

    fn rem(self, rhs: &Integer) -> Self::Output {
        reverse_remainder(*self, rhs)
    }
}
