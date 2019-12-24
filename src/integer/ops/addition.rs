use super::helpers::{reverse_add_assign, reverse_add_c_long, reverse_add_c_long_assign};
use crate::integer::Integer;
use core::ops::Add;

// Add          The addition operator +.

// [u'Integer', u'Integer', u'Integer', u'Integer::add_assign', u'lhs',
// [u'ref_mut'], [u'ref']]
impl Add<Integer> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: Integer) -> Self::Output {
        Integer::add_assign(&mut self, &rhs);
        self
    }
}

// [u'Integer', u'&Integer', u'Integer', u'Integer::add_assign', u'lhs',
// [u'ref_mut'], []]
impl Add<&Integer> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: &Integer) -> Self::Output {
        Integer::add_assign(&mut self, rhs);
        self
    }
}

// [u'&Integer', u'Integer', u'Integer', u'reverse_add_assign', u'rhs', [],
// [u'ref_mut']]
impl Add<Integer> for &Integer {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_assign(self, &mut rhs);
        rhs
    }
}

// [u'&Integer', u'&Integer', u'Integer', u'Integer::add', u'no', [], []]
impl Add<&Integer> for &Integer {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        Integer::add(self, rhs)
    }
}

// [u'Integer', u'i8', u'Integer', u'Integer::add_c_long_assign', u'lhs',
// [u'ref_mut'], []]
impl Add<i8> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: i8) -> Self::Output {
        Integer::add_c_long_assign(&mut self, rhs);
        self
    }
}

// [u'Integer', u'&i8', u'Integer', u'Integer::add_c_long_assign', u'lhs',
// [u'ref_mut'], [u'deref']]
impl Add<&i8> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: &i8) -> Self::Output {
        Integer::add_c_long_assign(&mut self, *rhs);
        self
    }
}

// [u'&Integer', u'i8', u'Integer', u'Integer::add_c_long', u'no', [], []]
impl Add<i8> for &Integer {
    type Output = Integer;

    fn add(self, rhs: i8) -> Self::Output {
        Integer::add_c_long(self, rhs)
    }
}

// [u'&Integer', u'&i8', u'Integer', u'Integer::add_c_long', u'no', [],
// [u'deref']]
impl Add<&i8> for &Integer {
    type Output = Integer;

    fn add(self, rhs: &i8) -> Self::Output {
        Integer::add_c_long(self, *rhs)
    }
}

// [u'i8', u'Integer', u'Integer', u'reverse_add_c_long_assign', u'rhs', [],
// [u'ref_mut']]
impl Add<Integer> for i8 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_c_long_assign(self, &mut rhs);
        rhs
    }
}

// [u'i8', u'&Integer', u'Integer', u'reverse_add_c_long', u'no', [], []]
impl Add<&Integer> for i8 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        reverse_add_c_long(self, rhs)
    }
}

// [u'&i8', u'Integer', u'Integer', u'reverse_add_c_long_assign', u'rhs',
// [u'deref'], [u'ref_mut']]
impl Add<Integer> for &i8 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_c_long_assign(*self, &mut rhs);
        rhs
    }
}

// [u'&i8', u'&Integer', u'Integer', u'reverse_add_c_long', u'no', [u'deref'],
// []]
impl Add<&Integer> for &i8 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        reverse_add_c_long(*self, rhs)
    }
}

// [u'Integer', u'u8', u'Integer', u'Integer::add_c_long_assign', u'lhs',
// [u'ref_mut'], []]
impl Add<u8> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: u8) -> Self::Output {
        Integer::add_c_long_assign(&mut self, rhs);
        self
    }
}

// [u'Integer', u'&u8', u'Integer', u'Integer::add_c_long_assign', u'lhs',
// [u'ref_mut'], [u'deref']]
impl Add<&u8> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: &u8) -> Self::Output {
        Integer::add_c_long_assign(&mut self, *rhs);
        self
    }
}

// [u'&Integer', u'u8', u'Integer', u'Integer::add_c_long', u'no', [], []]
impl Add<u8> for &Integer {
    type Output = Integer;

    fn add(self, rhs: u8) -> Self::Output {
        Integer::add_c_long(self, rhs)
    }
}

// [u'&Integer', u'&u8', u'Integer', u'Integer::add_c_long', u'no', [],
// [u'deref']]
impl Add<&u8> for &Integer {
    type Output = Integer;

    fn add(self, rhs: &u8) -> Self::Output {
        Integer::add_c_long(self, *rhs)
    }
}

// [u'u8', u'Integer', u'Integer', u'reverse_add_c_long_assign', u'rhs', [],
// [u'ref_mut']]
impl Add<Integer> for u8 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_c_long_assign(self, &mut rhs);
        rhs
    }
}

// [u'u8', u'&Integer', u'Integer', u'reverse_add_c_long', u'no', [], []]
impl Add<&Integer> for u8 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        reverse_add_c_long(self, rhs)
    }
}

// [u'&u8', u'Integer', u'Integer', u'reverse_add_c_long_assign', u'rhs',
// [u'deref'], [u'ref_mut']]
impl Add<Integer> for &u8 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_c_long_assign(*self, &mut rhs);
        rhs
    }
}

// [u'&u8', u'&Integer', u'Integer', u'reverse_add_c_long', u'no', [u'deref'],
// []]
impl Add<&Integer> for &u8 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        reverse_add_c_long(*self, rhs)
    }
}

// [u'Integer', u'i16', u'Integer', u'Integer::add_c_long_assign', u'lhs',
// [u'ref_mut'], []]
impl Add<i16> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: i16) -> Self::Output {
        Integer::add_c_long_assign(&mut self, rhs);
        self
    }
}

// [u'Integer', u'&i16', u'Integer', u'Integer::add_c_long_assign', u'lhs',
// [u'ref_mut'], [u'deref']]
impl Add<&i16> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: &i16) -> Self::Output {
        Integer::add_c_long_assign(&mut self, *rhs);
        self
    }
}

// [u'&Integer', u'i16', u'Integer', u'Integer::add_c_long', u'no', [], []]
impl Add<i16> for &Integer {
    type Output = Integer;

    fn add(self, rhs: i16) -> Self::Output {
        Integer::add_c_long(self, rhs)
    }
}

// [u'&Integer', u'&i16', u'Integer', u'Integer::add_c_long', u'no', [],
// [u'deref']]
impl Add<&i16> for &Integer {
    type Output = Integer;

    fn add(self, rhs: &i16) -> Self::Output {
        Integer::add_c_long(self, *rhs)
    }
}

// [u'i16', u'Integer', u'Integer', u'reverse_add_c_long_assign', u'rhs', [],
// [u'ref_mut']]
impl Add<Integer> for i16 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_c_long_assign(self, &mut rhs);
        rhs
    }
}

// [u'i16', u'&Integer', u'Integer', u'reverse_add_c_long', u'no', [], []]
impl Add<&Integer> for i16 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        reverse_add_c_long(self, rhs)
    }
}

// [u'&i16', u'Integer', u'Integer', u'reverse_add_c_long_assign', u'rhs',
// [u'deref'], [u'ref_mut']]
impl Add<Integer> for &i16 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_c_long_assign(*self, &mut rhs);
        rhs
    }
}

// [u'&i16', u'&Integer', u'Integer', u'reverse_add_c_long', u'no', [u'deref'],
// []]
impl Add<&Integer> for &i16 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        reverse_add_c_long(*self, rhs)
    }
}

// [u'Integer', u'u16', u'Integer', u'Integer::add_c_long_assign', u'lhs',
// [u'ref_mut'], []]
impl Add<u16> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: u16) -> Self::Output {
        Integer::add_c_long_assign(&mut self, rhs);
        self
    }
}

// [u'Integer', u'&u16', u'Integer', u'Integer::add_c_long_assign', u'lhs',
// [u'ref_mut'], [u'deref']]
impl Add<&u16> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: &u16) -> Self::Output {
        Integer::add_c_long_assign(&mut self, *rhs);
        self
    }
}

// [u'&Integer', u'u16', u'Integer', u'Integer::add_c_long', u'no', [], []]
impl Add<u16> for &Integer {
    type Output = Integer;

    fn add(self, rhs: u16) -> Self::Output {
        Integer::add_c_long(self, rhs)
    }
}

// [u'&Integer', u'&u16', u'Integer', u'Integer::add_c_long', u'no', [],
// [u'deref']]
impl Add<&u16> for &Integer {
    type Output = Integer;

    fn add(self, rhs: &u16) -> Self::Output {
        Integer::add_c_long(self, *rhs)
    }
}

// [u'u16', u'Integer', u'Integer', u'reverse_add_c_long_assign', u'rhs', [],
// [u'ref_mut']]
impl Add<Integer> for u16 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_c_long_assign(self, &mut rhs);
        rhs
    }
}

// [u'u16', u'&Integer', u'Integer', u'reverse_add_c_long', u'no', [], []]
impl Add<&Integer> for u16 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        reverse_add_c_long(self, rhs)
    }
}

// [u'&u16', u'Integer', u'Integer', u'reverse_add_c_long_assign', u'rhs',
// [u'deref'], [u'ref_mut']]
impl Add<Integer> for &u16 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_c_long_assign(*self, &mut rhs);
        rhs
    }
}

// [u'&u16', u'&Integer', u'Integer', u'reverse_add_c_long', u'no', [u'deref'],
// []]
impl Add<&Integer> for &u16 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        reverse_add_c_long(*self, rhs)
    }
}

// [u'Integer', u'i32', u'Integer', u'Integer::add_c_long_assign', u'lhs',
// [u'ref_mut'], []]
impl Add<i32> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: i32) -> Self::Output {
        Integer::add_c_long_assign(&mut self, rhs);
        self
    }
}

// [u'Integer', u'&i32', u'Integer', u'Integer::add_c_long_assign', u'lhs',
// [u'ref_mut'], [u'deref']]
impl Add<&i32> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: &i32) -> Self::Output {
        Integer::add_c_long_assign(&mut self, *rhs);
        self
    }
}

// [u'&Integer', u'i32', u'Integer', u'Integer::add_c_long', u'no', [], []]
impl Add<i32> for &Integer {
    type Output = Integer;

    fn add(self, rhs: i32) -> Self::Output {
        Integer::add_c_long(self, rhs)
    }
}

// [u'&Integer', u'&i32', u'Integer', u'Integer::add_c_long', u'no', [],
// [u'deref']]
impl Add<&i32> for &Integer {
    type Output = Integer;

    fn add(self, rhs: &i32) -> Self::Output {
        Integer::add_c_long(self, *rhs)
    }
}

// [u'i32', u'Integer', u'Integer', u'reverse_add_c_long_assign', u'rhs', [],
// [u'ref_mut']]
impl Add<Integer> for i32 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_c_long_assign(self, &mut rhs);
        rhs
    }
}

// [u'i32', u'&Integer', u'Integer', u'reverse_add_c_long', u'no', [], []]
impl Add<&Integer> for i32 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        reverse_add_c_long(self, rhs)
    }
}

// [u'&i32', u'Integer', u'Integer', u'reverse_add_c_long_assign', u'rhs',
// [u'deref'], [u'ref_mut']]
impl Add<Integer> for &i32 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_c_long_assign(*self, &mut rhs);
        rhs
    }
}

// [u'&i32', u'&Integer', u'Integer', u'reverse_add_c_long', u'no', [u'deref'],
// []]
impl Add<&Integer> for &i32 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        reverse_add_c_long(*self, rhs)
    }
}

// [u'Integer', u'u32', u'Integer', u'Integer::add_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}]]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Add<u32> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: u32) -> Self::Output {
        Integer::add_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// [u'Integer', u'&u32', u'Integer', u'Integer::add_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}, u'deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Add<&u32> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: &u32) -> Self::Output {
        Integer::add_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// [u'&Integer', u'u32', u'Integer', u'let mut rhs =
// Integer::from(rhs);\nreverse_add_assign(self, &mut rhs);\nrhs']
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Add<u32> for &Integer {
    type Output = Integer;

    fn add(self, rhs: u32) -> Self::Output {
        let mut rhs = Integer::from(rhs);
        reverse_add_assign(self, &mut rhs);
        rhs
    }
}

// [u'&Integer', u'&u32', u'Integer', u'let mut rhs =
// Integer::from(*rhs);\nreverse_add_assign(self, &mut rhs);\nrhs']
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Add<&u32> for &Integer {
    type Output = Integer;

    fn add(self, rhs: &u32) -> Self::Output {
        let mut rhs = Integer::from(*rhs);
        reverse_add_assign(self, &mut rhs);
        rhs
    }
}

// [u'u32', u'Integer', u'Integer', u'reverse_add_assign', u'rhs', [u'ref',
// {u'convert': u'Integer'}], [u'ref_mut']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Add<Integer> for u32 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_assign(&Integer::from(self), &mut rhs);
        rhs
    }
}

// [u'u32', u'&Integer', u'Integer', u'let mut lhs =
// Integer::from(self);\nInteger::add_assign(&mut lhs, rhs);\nlhs']
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Add<&Integer> for u32 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        let mut lhs = Integer::from(self);
        Integer::add_assign(&mut lhs, rhs);
        lhs
    }
}

// [u'&u32', u'Integer', u'Integer', u'reverse_add_assign', u'rhs', [u'ref',
// {u'convert': u'Integer'}, u'deref'], [u'ref_mut']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Add<Integer> for &u32 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_assign(&Integer::from(*self), &mut rhs);
        rhs
    }
}

// [u'&u32', u'&Integer', u'Integer', u'let mut lhs =
// Integer::from(*self);\nInteger::add_assign(&mut lhs, rhs);\nlhs']
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Add<&Integer> for &u32 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        let mut lhs = Integer::from(*self);
        Integer::add_assign(&mut lhs, rhs);
        lhs
    }
}

// [u'Integer', u'i64', u'Integer', u'Integer::add_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}]]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Add<i64> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: i64) -> Self::Output {
        Integer::add_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// [u'Integer', u'&i64', u'Integer', u'Integer::add_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}, u'deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Add<&i64> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: &i64) -> Self::Output {
        Integer::add_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// [u'&Integer', u'i64', u'Integer', u'let mut rhs =
// Integer::from(rhs);\nreverse_add_assign(self, &mut rhs);\nrhs']
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Add<i64> for &Integer {
    type Output = Integer;

    fn add(self, rhs: i64) -> Self::Output {
        let mut rhs = Integer::from(rhs);
        reverse_add_assign(self, &mut rhs);
        rhs
    }
}

// [u'&Integer', u'&i64', u'Integer', u'let mut rhs =
// Integer::from(*rhs);\nreverse_add_assign(self, &mut rhs);\nrhs']
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Add<&i64> for &Integer {
    type Output = Integer;

    fn add(self, rhs: &i64) -> Self::Output {
        let mut rhs = Integer::from(*rhs);
        reverse_add_assign(self, &mut rhs);
        rhs
    }
}

// [u'i64', u'Integer', u'Integer', u'reverse_add_assign', u'rhs', [u'ref',
// {u'convert': u'Integer'}], [u'ref_mut']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Add<Integer> for i64 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_assign(&Integer::from(self), &mut rhs);
        rhs
    }
}

// [u'i64', u'&Integer', u'Integer', u'let mut lhs =
// Integer::from(self);\nInteger::add_assign(&mut lhs, rhs);\nlhs']
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Add<&Integer> for i64 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        let mut lhs = Integer::from(self);
        Integer::add_assign(&mut lhs, rhs);
        lhs
    }
}

// [u'&i64', u'Integer', u'Integer', u'reverse_add_assign', u'rhs', [u'ref',
// {u'convert': u'Integer'}, u'deref'], [u'ref_mut']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Add<Integer> for &i64 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_assign(&Integer::from(*self), &mut rhs);
        rhs
    }
}

// [u'&i64', u'&Integer', u'Integer', u'let mut lhs =
// Integer::from(*self);\nInteger::add_assign(&mut lhs, rhs);\nlhs']
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Add<&Integer> for &i64 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        let mut lhs = Integer::from(*self);
        Integer::add_assign(&mut lhs, rhs);
        lhs
    }
}

// [u'Integer', u'u32', u'Integer', u'Integer::add_c_long_assign', u'lhs',
// [u'ref_mut'], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Add<u32> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: u32) -> Self::Output {
        Integer::add_c_long_assign(&mut self, rhs);
        self
    }
}

// [u'Integer', u'&u32', u'Integer', u'Integer::add_c_long_assign', u'lhs',
// [u'ref_mut'], [u'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Add<&u32> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: &u32) -> Self::Output {
        Integer::add_c_long_assign(&mut self, *rhs);
        self
    }
}

// [u'&Integer', u'u32', u'Integer', u'Integer::add_c_long', u'no', [], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Add<u32> for &Integer {
    type Output = Integer;

    fn add(self, rhs: u32) -> Self::Output {
        Integer::add_c_long(self, rhs)
    }
}

// [u'&Integer', u'&u32', u'Integer', u'Integer::add_c_long', u'no', [],
// [u'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Add<&u32> for &Integer {
    type Output = Integer;

    fn add(self, rhs: &u32) -> Self::Output {
        Integer::add_c_long(self, *rhs)
    }
}

// [u'u32', u'Integer', u'Integer', u'reverse_add_c_long_assign', u'rhs', [],
// [u'ref_mut']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Add<Integer> for u32 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_c_long_assign(self, &mut rhs);
        rhs
    }
}

// [u'u32', u'&Integer', u'Integer', u'reverse_add_c_long', u'no', [], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Add<&Integer> for u32 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        reverse_add_c_long(self, rhs)
    }
}

// [u'&u32', u'Integer', u'Integer', u'reverse_add_c_long_assign', u'rhs',
// [u'deref'], [u'ref_mut']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Add<Integer> for &u32 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_c_long_assign(*self, &mut rhs);
        rhs
    }
}

// [u'&u32', u'&Integer', u'Integer', u'reverse_add_c_long', u'no', [u'deref'],
// []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Add<&Integer> for &u32 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        reverse_add_c_long(*self, rhs)
    }
}

// [u'Integer', u'i64', u'Integer', u'Integer::add_c_long_assign', u'lhs',
// [u'ref_mut'], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Add<i64> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: i64) -> Self::Output {
        Integer::add_c_long_assign(&mut self, rhs);
        self
    }
}

// [u'Integer', u'&i64', u'Integer', u'Integer::add_c_long_assign', u'lhs',
// [u'ref_mut'], [u'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Add<&i64> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: &i64) -> Self::Output {
        Integer::add_c_long_assign(&mut self, *rhs);
        self
    }
}

// [u'&Integer', u'i64', u'Integer', u'Integer::add_c_long', u'no', [], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Add<i64> for &Integer {
    type Output = Integer;

    fn add(self, rhs: i64) -> Self::Output {
        Integer::add_c_long(self, rhs)
    }
}

// [u'&Integer', u'&i64', u'Integer', u'Integer::add_c_long', u'no', [],
// [u'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Add<&i64> for &Integer {
    type Output = Integer;

    fn add(self, rhs: &i64) -> Self::Output {
        Integer::add_c_long(self, *rhs)
    }
}

// [u'i64', u'Integer', u'Integer', u'reverse_add_c_long_assign', u'rhs', [],
// [u'ref_mut']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Add<Integer> for i64 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_c_long_assign(self, &mut rhs);
        rhs
    }
}

// [u'i64', u'&Integer', u'Integer', u'reverse_add_c_long', u'no', [], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Add<&Integer> for i64 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        reverse_add_c_long(self, rhs)
    }
}

// [u'&i64', u'Integer', u'Integer', u'reverse_add_c_long_assign', u'rhs',
// [u'deref'], [u'ref_mut']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Add<Integer> for &i64 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_c_long_assign(*self, &mut rhs);
        rhs
    }
}

// [u'&i64', u'&Integer', u'Integer', u'reverse_add_c_long', u'no', [u'deref'],
// []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Add<&Integer> for &i64 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        reverse_add_c_long(*self, rhs)
    }
}

// [u'Integer', u'u64', u'Integer', u'Integer::add_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}]]
impl Add<u64> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: u64) -> Self::Output {
        Integer::add_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// [u'Integer', u'&u64', u'Integer', u'Integer::add_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}, u'deref']]
impl Add<&u64> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: &u64) -> Self::Output {
        Integer::add_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// [u'&Integer', u'u64', u'Integer', u'let mut rhs =
// Integer::from(rhs);\nreverse_add_assign(self, &mut rhs);\nrhs']
impl Add<u64> for &Integer {
    type Output = Integer;

    fn add(self, rhs: u64) -> Self::Output {
        let mut rhs = Integer::from(rhs);
        reverse_add_assign(self, &mut rhs);
        rhs
    }
}

// [u'&Integer', u'&u64', u'Integer', u'let mut rhs =
// Integer::from(*rhs);\nreverse_add_assign(self, &mut rhs);\nrhs']
impl Add<&u64> for &Integer {
    type Output = Integer;

    fn add(self, rhs: &u64) -> Self::Output {
        let mut rhs = Integer::from(*rhs);
        reverse_add_assign(self, &mut rhs);
        rhs
    }
}

// [u'u64', u'Integer', u'Integer', u'reverse_add_assign', u'rhs', [u'ref',
// {u'convert': u'Integer'}], [u'ref_mut']]
impl Add<Integer> for u64 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_assign(&Integer::from(self), &mut rhs);
        rhs
    }
}

// [u'u64', u'&Integer', u'Integer', u'let mut lhs =
// Integer::from(self);\nInteger::add_assign(&mut lhs, rhs);\nlhs']
impl Add<&Integer> for u64 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        let mut lhs = Integer::from(self);
        Integer::add_assign(&mut lhs, rhs);
        lhs
    }
}

// [u'&u64', u'Integer', u'Integer', u'reverse_add_assign', u'rhs', [u'ref',
// {u'convert': u'Integer'}, u'deref'], [u'ref_mut']]
impl Add<Integer> for &u64 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_assign(&Integer::from(*self), &mut rhs);
        rhs
    }
}

// [u'&u64', u'&Integer', u'Integer', u'let mut lhs =
// Integer::from(*self);\nInteger::add_assign(&mut lhs, rhs);\nlhs']
impl Add<&Integer> for &u64 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        let mut lhs = Integer::from(*self);
        Integer::add_assign(&mut lhs, rhs);
        lhs
    }
}

// [u'Integer', u'i128', u'Integer', u'Integer::add_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}]]
impl Add<i128> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: i128) -> Self::Output {
        Integer::add_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// [u'Integer', u'&i128', u'Integer', u'Integer::add_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}, u'deref']]
impl Add<&i128> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: &i128) -> Self::Output {
        Integer::add_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// [u'&Integer', u'i128', u'Integer', u'let mut rhs =
// Integer::from(rhs);\nreverse_add_assign(self, &mut rhs);\nrhs']
impl Add<i128> for &Integer {
    type Output = Integer;

    fn add(self, rhs: i128) -> Self::Output {
        let mut rhs = Integer::from(rhs);
        reverse_add_assign(self, &mut rhs);
        rhs
    }
}

// [u'&Integer', u'&i128', u'Integer', u'let mut rhs =
// Integer::from(*rhs);\nreverse_add_assign(self, &mut rhs);\nrhs']
impl Add<&i128> for &Integer {
    type Output = Integer;

    fn add(self, rhs: &i128) -> Self::Output {
        let mut rhs = Integer::from(*rhs);
        reverse_add_assign(self, &mut rhs);
        rhs
    }
}

// [u'i128', u'Integer', u'Integer', u'reverse_add_assign', u'rhs', [u'ref',
// {u'convert': u'Integer'}], [u'ref_mut']]
impl Add<Integer> for i128 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_assign(&Integer::from(self), &mut rhs);
        rhs
    }
}

// [u'i128', u'&Integer', u'Integer', u'let mut lhs =
// Integer::from(self);\nInteger::add_assign(&mut lhs, rhs);\nlhs']
impl Add<&Integer> for i128 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        let mut lhs = Integer::from(self);
        Integer::add_assign(&mut lhs, rhs);
        lhs
    }
}

// [u'&i128', u'Integer', u'Integer', u'reverse_add_assign', u'rhs', [u'ref',
// {u'convert': u'Integer'}, u'deref'], [u'ref_mut']]
impl Add<Integer> for &i128 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_assign(&Integer::from(*self), &mut rhs);
        rhs
    }
}

// [u'&i128', u'&Integer', u'Integer', u'let mut lhs =
// Integer::from(*self);\nInteger::add_assign(&mut lhs, rhs);\nlhs']
impl Add<&Integer> for &i128 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        let mut lhs = Integer::from(*self);
        Integer::add_assign(&mut lhs, rhs);
        lhs
    }
}

// [u'Integer', u'u128', u'Integer', u'Integer::add_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}]]
impl Add<u128> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: u128) -> Self::Output {
        Integer::add_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// [u'Integer', u'&u128', u'Integer', u'Integer::add_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}, u'deref']]
impl Add<&u128> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: &u128) -> Self::Output {
        Integer::add_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// [u'&Integer', u'u128', u'Integer', u'let mut rhs =
// Integer::from(rhs);\nreverse_add_assign(self, &mut rhs);\nrhs']
impl Add<u128> for &Integer {
    type Output = Integer;

    fn add(self, rhs: u128) -> Self::Output {
        let mut rhs = Integer::from(rhs);
        reverse_add_assign(self, &mut rhs);
        rhs
    }
}

// [u'&Integer', u'&u128', u'Integer', u'let mut rhs =
// Integer::from(*rhs);\nreverse_add_assign(self, &mut rhs);\nrhs']
impl Add<&u128> for &Integer {
    type Output = Integer;

    fn add(self, rhs: &u128) -> Self::Output {
        let mut rhs = Integer::from(*rhs);
        reverse_add_assign(self, &mut rhs);
        rhs
    }
}

// [u'u128', u'Integer', u'Integer', u'reverse_add_assign', u'rhs', [u'ref',
// {u'convert': u'Integer'}], [u'ref_mut']]
impl Add<Integer> for u128 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_assign(&Integer::from(self), &mut rhs);
        rhs
    }
}

// [u'u128', u'&Integer', u'Integer', u'let mut lhs =
// Integer::from(self);\nInteger::add_assign(&mut lhs, rhs);\nlhs']
impl Add<&Integer> for u128 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        let mut lhs = Integer::from(self);
        Integer::add_assign(&mut lhs, rhs);
        lhs
    }
}

// [u'&u128', u'Integer', u'Integer', u'reverse_add_assign', u'rhs', [u'ref',
// {u'convert': u'Integer'}, u'deref'], [u'ref_mut']]
impl Add<Integer> for &u128 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_assign(&Integer::from(*self), &mut rhs);
        rhs
    }
}

// [u'&u128', u'&Integer', u'Integer', u'let mut lhs =
// Integer::from(*self);\nInteger::add_assign(&mut lhs, rhs);\nlhs']
impl Add<&Integer> for &u128 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        let mut lhs = Integer::from(*self);
        Integer::add_assign(&mut lhs, rhs);
        lhs
    }
}
