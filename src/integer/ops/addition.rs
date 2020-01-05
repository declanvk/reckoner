use super::helpers::{reverse_add_assign, reverse_add_c_long, reverse_add_c_long_assign};
use crate::integer::Integer;
use core::ops::Add;

// Add          The addition operator +.

// ['Integer', 'Integer', 'Integer', 'Integer::add_assign', 'lhs', ['ref_mut'],
// ['ref']]
impl Add<Integer> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: Integer) -> Self::Output {
        Integer::add_assign(&mut self, &rhs);
        self
    }
}

// ['Integer', '&Integer', 'Integer', 'Integer::add_assign', 'lhs', ['ref_mut'],
// []]
impl Add<&Integer> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: &Integer) -> Self::Output {
        Integer::add_assign(&mut self, rhs);
        self
    }
}

// ['&Integer', 'Integer', 'Integer', 'reverse_add_assign', 'rhs', [],
// ['ref_mut']]
impl Add<Integer> for &Integer {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_assign(self, &mut rhs);
        rhs
    }
}

// ['&Integer', '&Integer', 'Integer', 'Integer::add', 'no', [], []]
impl Add<&Integer> for &Integer {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        Integer::add(self, rhs)
    }
}

// ['Integer', 'i8', 'Integer', 'Integer::add_c_long_assign', 'lhs',
// ['ref_mut'], []]
impl Add<i8> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: i8) -> Self::Output {
        Integer::add_c_long_assign(&mut self, rhs);
        self
    }
}

// ['Integer', '&i8', 'Integer', 'Integer::add_c_long_assign', 'lhs',
// ['ref_mut'], ['deref']]
impl Add<&i8> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: &i8) -> Self::Output {
        Integer::add_c_long_assign(&mut self, *rhs);
        self
    }
}

// ['&Integer', 'i8', 'Integer', 'Integer::add_c_long', 'no', [], []]
impl Add<i8> for &Integer {
    type Output = Integer;

    fn add(self, rhs: i8) -> Self::Output {
        Integer::add_c_long(self, rhs)
    }
}

// ['&Integer', '&i8', 'Integer', 'Integer::add_c_long', 'no', [], ['deref']]
impl Add<&i8> for &Integer {
    type Output = Integer;

    fn add(self, rhs: &i8) -> Self::Output {
        Integer::add_c_long(self, *rhs)
    }
}

// ['i8', 'Integer', 'Integer', 'reverse_add_c_long_assign', 'rhs', [],
// ['ref_mut']]
impl Add<Integer> for i8 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_c_long_assign(self, &mut rhs);
        rhs
    }
}

// ['i8', '&Integer', 'Integer', 'reverse_add_c_long', 'no', [], []]
impl Add<&Integer> for i8 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        reverse_add_c_long(self, rhs)
    }
}

// ['&i8', 'Integer', 'Integer', 'reverse_add_c_long_assign', 'rhs', ['deref'],
// ['ref_mut']]
impl Add<Integer> for &i8 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_c_long_assign(*self, &mut rhs);
        rhs
    }
}

// ['&i8', '&Integer', 'Integer', 'reverse_add_c_long', 'no', ['deref'], []]
impl Add<&Integer> for &i8 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        reverse_add_c_long(*self, rhs)
    }
}

// ['Integer', 'u8', 'Integer', 'Integer::add_c_long_assign', 'lhs',
// ['ref_mut'], []]
impl Add<u8> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: u8) -> Self::Output {
        Integer::add_c_long_assign(&mut self, rhs);
        self
    }
}

// ['Integer', '&u8', 'Integer', 'Integer::add_c_long_assign', 'lhs',
// ['ref_mut'], ['deref']]
impl Add<&u8> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: &u8) -> Self::Output {
        Integer::add_c_long_assign(&mut self, *rhs);
        self
    }
}

// ['&Integer', 'u8', 'Integer', 'Integer::add_c_long', 'no', [], []]
impl Add<u8> for &Integer {
    type Output = Integer;

    fn add(self, rhs: u8) -> Self::Output {
        Integer::add_c_long(self, rhs)
    }
}

// ['&Integer', '&u8', 'Integer', 'Integer::add_c_long', 'no', [], ['deref']]
impl Add<&u8> for &Integer {
    type Output = Integer;

    fn add(self, rhs: &u8) -> Self::Output {
        Integer::add_c_long(self, *rhs)
    }
}

// ['u8', 'Integer', 'Integer', 'reverse_add_c_long_assign', 'rhs', [],
// ['ref_mut']]
impl Add<Integer> for u8 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_c_long_assign(self, &mut rhs);
        rhs
    }
}

// ['u8', '&Integer', 'Integer', 'reverse_add_c_long', 'no', [], []]
impl Add<&Integer> for u8 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        reverse_add_c_long(self, rhs)
    }
}

// ['&u8', 'Integer', 'Integer', 'reverse_add_c_long_assign', 'rhs', ['deref'],
// ['ref_mut']]
impl Add<Integer> for &u8 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_c_long_assign(*self, &mut rhs);
        rhs
    }
}

// ['&u8', '&Integer', 'Integer', 'reverse_add_c_long', 'no', ['deref'], []]
impl Add<&Integer> for &u8 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        reverse_add_c_long(*self, rhs)
    }
}

// ['Integer', 'i16', 'Integer', 'Integer::add_c_long_assign', 'lhs',
// ['ref_mut'], []]
impl Add<i16> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: i16) -> Self::Output {
        Integer::add_c_long_assign(&mut self, rhs);
        self
    }
}

// ['Integer', '&i16', 'Integer', 'Integer::add_c_long_assign', 'lhs',
// ['ref_mut'], ['deref']]
impl Add<&i16> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: &i16) -> Self::Output {
        Integer::add_c_long_assign(&mut self, *rhs);
        self
    }
}

// ['&Integer', 'i16', 'Integer', 'Integer::add_c_long', 'no', [], []]
impl Add<i16> for &Integer {
    type Output = Integer;

    fn add(self, rhs: i16) -> Self::Output {
        Integer::add_c_long(self, rhs)
    }
}

// ['&Integer', '&i16', 'Integer', 'Integer::add_c_long', 'no', [], ['deref']]
impl Add<&i16> for &Integer {
    type Output = Integer;

    fn add(self, rhs: &i16) -> Self::Output {
        Integer::add_c_long(self, *rhs)
    }
}

// ['i16', 'Integer', 'Integer', 'reverse_add_c_long_assign', 'rhs', [],
// ['ref_mut']]
impl Add<Integer> for i16 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_c_long_assign(self, &mut rhs);
        rhs
    }
}

// ['i16', '&Integer', 'Integer', 'reverse_add_c_long', 'no', [], []]
impl Add<&Integer> for i16 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        reverse_add_c_long(self, rhs)
    }
}

// ['&i16', 'Integer', 'Integer', 'reverse_add_c_long_assign', 'rhs', ['deref'],
// ['ref_mut']]
impl Add<Integer> for &i16 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_c_long_assign(*self, &mut rhs);
        rhs
    }
}

// ['&i16', '&Integer', 'Integer', 'reverse_add_c_long', 'no', ['deref'], []]
impl Add<&Integer> for &i16 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        reverse_add_c_long(*self, rhs)
    }
}

// ['Integer', 'u16', 'Integer', 'Integer::add_c_long_assign', 'lhs',
// ['ref_mut'], []]
impl Add<u16> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: u16) -> Self::Output {
        Integer::add_c_long_assign(&mut self, rhs);
        self
    }
}

// ['Integer', '&u16', 'Integer', 'Integer::add_c_long_assign', 'lhs',
// ['ref_mut'], ['deref']]
impl Add<&u16> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: &u16) -> Self::Output {
        Integer::add_c_long_assign(&mut self, *rhs);
        self
    }
}

// ['&Integer', 'u16', 'Integer', 'Integer::add_c_long', 'no', [], []]
impl Add<u16> for &Integer {
    type Output = Integer;

    fn add(self, rhs: u16) -> Self::Output {
        Integer::add_c_long(self, rhs)
    }
}

// ['&Integer', '&u16', 'Integer', 'Integer::add_c_long', 'no', [], ['deref']]
impl Add<&u16> for &Integer {
    type Output = Integer;

    fn add(self, rhs: &u16) -> Self::Output {
        Integer::add_c_long(self, *rhs)
    }
}

// ['u16', 'Integer', 'Integer', 'reverse_add_c_long_assign', 'rhs', [],
// ['ref_mut']]
impl Add<Integer> for u16 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_c_long_assign(self, &mut rhs);
        rhs
    }
}

// ['u16', '&Integer', 'Integer', 'reverse_add_c_long', 'no', [], []]
impl Add<&Integer> for u16 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        reverse_add_c_long(self, rhs)
    }
}

// ['&u16', 'Integer', 'Integer', 'reverse_add_c_long_assign', 'rhs', ['deref'],
// ['ref_mut']]
impl Add<Integer> for &u16 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_c_long_assign(*self, &mut rhs);
        rhs
    }
}

// ['&u16', '&Integer', 'Integer', 'reverse_add_c_long', 'no', ['deref'], []]
impl Add<&Integer> for &u16 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        reverse_add_c_long(*self, rhs)
    }
}

// ['Integer', 'i32', 'Integer', 'Integer::add_c_long_assign', 'lhs',
// ['ref_mut'], []]
impl Add<i32> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: i32) -> Self::Output {
        Integer::add_c_long_assign(&mut self, rhs);
        self
    }
}

// ['Integer', '&i32', 'Integer', 'Integer::add_c_long_assign', 'lhs',
// ['ref_mut'], ['deref']]
impl Add<&i32> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: &i32) -> Self::Output {
        Integer::add_c_long_assign(&mut self, *rhs);
        self
    }
}

// ['&Integer', 'i32', 'Integer', 'Integer::add_c_long', 'no', [], []]
impl Add<i32> for &Integer {
    type Output = Integer;

    fn add(self, rhs: i32) -> Self::Output {
        Integer::add_c_long(self, rhs)
    }
}

// ['&Integer', '&i32', 'Integer', 'Integer::add_c_long', 'no', [], ['deref']]
impl Add<&i32> for &Integer {
    type Output = Integer;

    fn add(self, rhs: &i32) -> Self::Output {
        Integer::add_c_long(self, *rhs)
    }
}

// ['i32', 'Integer', 'Integer', 'reverse_add_c_long_assign', 'rhs', [],
// ['ref_mut']]
impl Add<Integer> for i32 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_c_long_assign(self, &mut rhs);
        rhs
    }
}

// ['i32', '&Integer', 'Integer', 'reverse_add_c_long', 'no', [], []]
impl Add<&Integer> for i32 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        reverse_add_c_long(self, rhs)
    }
}

// ['&i32', 'Integer', 'Integer', 'reverse_add_c_long_assign', 'rhs', ['deref'],
// ['ref_mut']]
impl Add<Integer> for &i32 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_c_long_assign(*self, &mut rhs);
        rhs
    }
}

// ['&i32', '&Integer', 'Integer', 'reverse_add_c_long', 'no', ['deref'], []]
impl Add<&Integer> for &i32 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        reverse_add_c_long(*self, rhs)
    }
}

// ['Integer', 'u32', 'Integer', 'Integer::add_c_long_assign', 'lhs',
// ['ref_mut'], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Add<u32> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: u32) -> Self::Output {
        Integer::add_c_long_assign(&mut self, rhs);
        self
    }
}

// ['Integer', '&u32', 'Integer', 'Integer::add_c_long_assign', 'lhs',
// ['ref_mut'], ['deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Add<&u32> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: &u32) -> Self::Output {
        Integer::add_c_long_assign(&mut self, *rhs);
        self
    }
}

// ['&Integer', 'u32', 'Integer', 'Integer::add_c_long', 'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Add<u32> for &Integer {
    type Output = Integer;

    fn add(self, rhs: u32) -> Self::Output {
        Integer::add_c_long(self, rhs)
    }
}

// ['&Integer', '&u32', 'Integer', 'Integer::add_c_long', 'no', [], ['deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Add<&u32> for &Integer {
    type Output = Integer;

    fn add(self, rhs: &u32) -> Self::Output {
        Integer::add_c_long(self, *rhs)
    }
}

// ['u32', 'Integer', 'Integer', 'reverse_add_c_long_assign', 'rhs', [],
// ['ref_mut']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Add<Integer> for u32 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_c_long_assign(self, &mut rhs);
        rhs
    }
}

// ['u32', '&Integer', 'Integer', 'reverse_add_c_long', 'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Add<&Integer> for u32 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        reverse_add_c_long(self, rhs)
    }
}

// ['&u32', 'Integer', 'Integer', 'reverse_add_c_long_assign', 'rhs', ['deref'],
// ['ref_mut']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Add<Integer> for &u32 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_c_long_assign(*self, &mut rhs);
        rhs
    }
}

// ['&u32', '&Integer', 'Integer', 'reverse_add_c_long', 'no', ['deref'], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Add<&Integer> for &u32 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        reverse_add_c_long(*self, rhs)
    }
}

// ['Integer', 'i64', 'Integer', 'Integer::add_c_long_assign', 'lhs',
// ['ref_mut'], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Add<i64> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: i64) -> Self::Output {
        Integer::add_c_long_assign(&mut self, rhs);
        self
    }
}

// ['Integer', '&i64', 'Integer', 'Integer::add_c_long_assign', 'lhs',
// ['ref_mut'], ['deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Add<&i64> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: &i64) -> Self::Output {
        Integer::add_c_long_assign(&mut self, *rhs);
        self
    }
}

// ['&Integer', 'i64', 'Integer', 'Integer::add_c_long', 'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Add<i64> for &Integer {
    type Output = Integer;

    fn add(self, rhs: i64) -> Self::Output {
        Integer::add_c_long(self, rhs)
    }
}

// ['&Integer', '&i64', 'Integer', 'Integer::add_c_long', 'no', [], ['deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Add<&i64> for &Integer {
    type Output = Integer;

    fn add(self, rhs: &i64) -> Self::Output {
        Integer::add_c_long(self, *rhs)
    }
}

// ['i64', 'Integer', 'Integer', 'reverse_add_c_long_assign', 'rhs', [],
// ['ref_mut']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Add<Integer> for i64 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_c_long_assign(self, &mut rhs);
        rhs
    }
}

// ['i64', '&Integer', 'Integer', 'reverse_add_c_long', 'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Add<&Integer> for i64 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        reverse_add_c_long(self, rhs)
    }
}

// ['&i64', 'Integer', 'Integer', 'reverse_add_c_long_assign', 'rhs', ['deref'],
// ['ref_mut']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Add<Integer> for &i64 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_c_long_assign(*self, &mut rhs);
        rhs
    }
}

// ['&i64', '&Integer', 'Integer', 'reverse_add_c_long', 'no', ['deref'], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Add<&Integer> for &i64 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        reverse_add_c_long(*self, rhs)
    }
}

// ['Integer', 'u32', 'Integer', 'Integer::add_assign', 'lhs', ['ref_mut'],
// ['ref', {'convert': 'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Add<u32> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: u32) -> Self::Output {
        Integer::add_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Integer', '&u32', 'Integer', 'Integer::add_assign', 'lhs', ['ref_mut'],
// ['ref', {'convert': 'Integer'}, 'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Add<&u32> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: &u32) -> Self::Output {
        Integer::add_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Integer', 'u32', 'Integer', 'let mut rhs =
// Integer::from(rhs);\nreverse_add_assign(self, &mut rhs);\nrhs']
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Add<u32> for &Integer {
    type Output = Integer;

    fn add(self, rhs: u32) -> Self::Output {
        let mut rhs = Integer::from(rhs);
        reverse_add_assign(self, &mut rhs);
        rhs
    }
}

// ['&Integer', '&u32', 'Integer', 'let mut rhs =
// Integer::from(*rhs);\nreverse_add_assign(self, &mut rhs);\nrhs']
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Add<&u32> for &Integer {
    type Output = Integer;

    fn add(self, rhs: &u32) -> Self::Output {
        let mut rhs = Integer::from(*rhs);
        reverse_add_assign(self, &mut rhs);
        rhs
    }
}

// ['u32', 'Integer', 'Integer', 'reverse_add_assign', 'rhs', ['ref',
// {'convert': 'Integer'}], ['ref_mut']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Add<Integer> for u32 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_assign(&Integer::from(self), &mut rhs);
        rhs
    }
}

// ['u32', '&Integer', 'Integer', 'let mut lhs =
// Integer::from(self);\nInteger::add_assign(&mut lhs, rhs);\nlhs']
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Add<&Integer> for u32 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        let mut lhs = Integer::from(self);
        Integer::add_assign(&mut lhs, rhs);
        lhs
    }
}

// ['&u32', 'Integer', 'Integer', 'reverse_add_assign', 'rhs', ['ref',
// {'convert': 'Integer'}, 'deref'], ['ref_mut']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Add<Integer> for &u32 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_assign(&Integer::from(*self), &mut rhs);
        rhs
    }
}

// ['&u32', '&Integer', 'Integer', 'let mut lhs =
// Integer::from(*self);\nInteger::add_assign(&mut lhs, rhs);\nlhs']
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Add<&Integer> for &u32 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        let mut lhs = Integer::from(*self);
        Integer::add_assign(&mut lhs, rhs);
        lhs
    }
}

// ['Integer', 'i64', 'Integer', 'Integer::add_assign', 'lhs', ['ref_mut'],
// ['ref', {'convert': 'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Add<i64> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: i64) -> Self::Output {
        Integer::add_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Integer', '&i64', 'Integer', 'Integer::add_assign', 'lhs', ['ref_mut'],
// ['ref', {'convert': 'Integer'}, 'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Add<&i64> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: &i64) -> Self::Output {
        Integer::add_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Integer', 'i64', 'Integer', 'let mut rhs =
// Integer::from(rhs);\nreverse_add_assign(self, &mut rhs);\nrhs']
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Add<i64> for &Integer {
    type Output = Integer;

    fn add(self, rhs: i64) -> Self::Output {
        let mut rhs = Integer::from(rhs);
        reverse_add_assign(self, &mut rhs);
        rhs
    }
}

// ['&Integer', '&i64', 'Integer', 'let mut rhs =
// Integer::from(*rhs);\nreverse_add_assign(self, &mut rhs);\nrhs']
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Add<&i64> for &Integer {
    type Output = Integer;

    fn add(self, rhs: &i64) -> Self::Output {
        let mut rhs = Integer::from(*rhs);
        reverse_add_assign(self, &mut rhs);
        rhs
    }
}

// ['i64', 'Integer', 'Integer', 'reverse_add_assign', 'rhs', ['ref',
// {'convert': 'Integer'}], ['ref_mut']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Add<Integer> for i64 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_assign(&Integer::from(self), &mut rhs);
        rhs
    }
}

// ['i64', '&Integer', 'Integer', 'let mut lhs =
// Integer::from(self);\nInteger::add_assign(&mut lhs, rhs);\nlhs']
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Add<&Integer> for i64 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        let mut lhs = Integer::from(self);
        Integer::add_assign(&mut lhs, rhs);
        lhs
    }
}

// ['&i64', 'Integer', 'Integer', 'reverse_add_assign', 'rhs', ['ref',
// {'convert': 'Integer'}, 'deref'], ['ref_mut']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Add<Integer> for &i64 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_assign(&Integer::from(*self), &mut rhs);
        rhs
    }
}

// ['&i64', '&Integer', 'Integer', 'let mut lhs =
// Integer::from(*self);\nInteger::add_assign(&mut lhs, rhs);\nlhs']
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Add<&Integer> for &i64 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        let mut lhs = Integer::from(*self);
        Integer::add_assign(&mut lhs, rhs);
        lhs
    }
}

// ['Integer', 'u64', 'Integer', 'Integer::add_assign', 'lhs', ['ref_mut'],
// ['ref', {'convert': 'Integer'}]]
impl Add<u64> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: u64) -> Self::Output {
        Integer::add_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Integer', '&u64', 'Integer', 'Integer::add_assign', 'lhs', ['ref_mut'],
// ['ref', {'convert': 'Integer'}, 'deref']]
impl Add<&u64> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: &u64) -> Self::Output {
        Integer::add_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Integer', 'u64', 'Integer', 'let mut rhs =
// Integer::from(rhs);\nreverse_add_assign(self, &mut rhs);\nrhs']
impl Add<u64> for &Integer {
    type Output = Integer;

    fn add(self, rhs: u64) -> Self::Output {
        let mut rhs = Integer::from(rhs);
        reverse_add_assign(self, &mut rhs);
        rhs
    }
}

// ['&Integer', '&u64', 'Integer', 'let mut rhs =
// Integer::from(*rhs);\nreverse_add_assign(self, &mut rhs);\nrhs']
impl Add<&u64> for &Integer {
    type Output = Integer;

    fn add(self, rhs: &u64) -> Self::Output {
        let mut rhs = Integer::from(*rhs);
        reverse_add_assign(self, &mut rhs);
        rhs
    }
}

// ['u64', 'Integer', 'Integer', 'reverse_add_assign', 'rhs', ['ref',
// {'convert': 'Integer'}], ['ref_mut']]
impl Add<Integer> for u64 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_assign(&Integer::from(self), &mut rhs);
        rhs
    }
}

// ['u64', '&Integer', 'Integer', 'let mut lhs =
// Integer::from(self);\nInteger::add_assign(&mut lhs, rhs);\nlhs']
impl Add<&Integer> for u64 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        let mut lhs = Integer::from(self);
        Integer::add_assign(&mut lhs, rhs);
        lhs
    }
}

// ['&u64', 'Integer', 'Integer', 'reverse_add_assign', 'rhs', ['ref',
// {'convert': 'Integer'}, 'deref'], ['ref_mut']]
impl Add<Integer> for &u64 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_assign(&Integer::from(*self), &mut rhs);
        rhs
    }
}

// ['&u64', '&Integer', 'Integer', 'let mut lhs =
// Integer::from(*self);\nInteger::add_assign(&mut lhs, rhs);\nlhs']
impl Add<&Integer> for &u64 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        let mut lhs = Integer::from(*self);
        Integer::add_assign(&mut lhs, rhs);
        lhs
    }
}

// ['Integer', 'i128', 'Integer', 'Integer::add_assign', 'lhs', ['ref_mut'],
// ['ref', {'convert': 'Integer'}]]
impl Add<i128> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: i128) -> Self::Output {
        Integer::add_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Integer', '&i128', 'Integer', 'Integer::add_assign', 'lhs', ['ref_mut'],
// ['ref', {'convert': 'Integer'}, 'deref']]
impl Add<&i128> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: &i128) -> Self::Output {
        Integer::add_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Integer', 'i128', 'Integer', 'let mut rhs =
// Integer::from(rhs);\nreverse_add_assign(self, &mut rhs);\nrhs']
impl Add<i128> for &Integer {
    type Output = Integer;

    fn add(self, rhs: i128) -> Self::Output {
        let mut rhs = Integer::from(rhs);
        reverse_add_assign(self, &mut rhs);
        rhs
    }
}

// ['&Integer', '&i128', 'Integer', 'let mut rhs =
// Integer::from(*rhs);\nreverse_add_assign(self, &mut rhs);\nrhs']
impl Add<&i128> for &Integer {
    type Output = Integer;

    fn add(self, rhs: &i128) -> Self::Output {
        let mut rhs = Integer::from(*rhs);
        reverse_add_assign(self, &mut rhs);
        rhs
    }
}

// ['i128', 'Integer', 'Integer', 'reverse_add_assign', 'rhs', ['ref',
// {'convert': 'Integer'}], ['ref_mut']]
impl Add<Integer> for i128 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_assign(&Integer::from(self), &mut rhs);
        rhs
    }
}

// ['i128', '&Integer', 'Integer', 'let mut lhs =
// Integer::from(self);\nInteger::add_assign(&mut lhs, rhs);\nlhs']
impl Add<&Integer> for i128 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        let mut lhs = Integer::from(self);
        Integer::add_assign(&mut lhs, rhs);
        lhs
    }
}

// ['&i128', 'Integer', 'Integer', 'reverse_add_assign', 'rhs', ['ref',
// {'convert': 'Integer'}, 'deref'], ['ref_mut']]
impl Add<Integer> for &i128 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_assign(&Integer::from(*self), &mut rhs);
        rhs
    }
}

// ['&i128', '&Integer', 'Integer', 'let mut lhs =
// Integer::from(*self);\nInteger::add_assign(&mut lhs, rhs);\nlhs']
impl Add<&Integer> for &i128 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        let mut lhs = Integer::from(*self);
        Integer::add_assign(&mut lhs, rhs);
        lhs
    }
}

// ['Integer', 'u128', 'Integer', 'Integer::add_assign', 'lhs', ['ref_mut'],
// ['ref', {'convert': 'Integer'}]]
impl Add<u128> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: u128) -> Self::Output {
        Integer::add_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Integer', '&u128', 'Integer', 'Integer::add_assign', 'lhs', ['ref_mut'],
// ['ref', {'convert': 'Integer'}, 'deref']]
impl Add<&u128> for Integer {
    type Output = Integer;

    fn add(mut self, rhs: &u128) -> Self::Output {
        Integer::add_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Integer', 'u128', 'Integer', 'let mut rhs =
// Integer::from(rhs);\nreverse_add_assign(self, &mut rhs);\nrhs']
impl Add<u128> for &Integer {
    type Output = Integer;

    fn add(self, rhs: u128) -> Self::Output {
        let mut rhs = Integer::from(rhs);
        reverse_add_assign(self, &mut rhs);
        rhs
    }
}

// ['&Integer', '&u128', 'Integer', 'let mut rhs =
// Integer::from(*rhs);\nreverse_add_assign(self, &mut rhs);\nrhs']
impl Add<&u128> for &Integer {
    type Output = Integer;

    fn add(self, rhs: &u128) -> Self::Output {
        let mut rhs = Integer::from(*rhs);
        reverse_add_assign(self, &mut rhs);
        rhs
    }
}

// ['u128', 'Integer', 'Integer', 'reverse_add_assign', 'rhs', ['ref',
// {'convert': 'Integer'}], ['ref_mut']]
impl Add<Integer> for u128 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_assign(&Integer::from(self), &mut rhs);
        rhs
    }
}

// ['u128', '&Integer', 'Integer', 'let mut lhs =
// Integer::from(self);\nInteger::add_assign(&mut lhs, rhs);\nlhs']
impl Add<&Integer> for u128 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        let mut lhs = Integer::from(self);
        Integer::add_assign(&mut lhs, rhs);
        lhs
    }
}

// ['&u128', 'Integer', 'Integer', 'reverse_add_assign', 'rhs', ['ref',
// {'convert': 'Integer'}, 'deref'], ['ref_mut']]
impl Add<Integer> for &u128 {
    type Output = Integer;

    fn add(self, mut rhs: Integer) -> Self::Output {
        reverse_add_assign(&Integer::from(*self), &mut rhs);
        rhs
    }
}

// ['&u128', '&Integer', 'Integer', 'let mut lhs =
// Integer::from(*self);\nInteger::add_assign(&mut lhs, rhs);\nlhs']
impl Add<&Integer> for &u128 {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        let mut lhs = Integer::from(*self);
        Integer::add_assign(&mut lhs, rhs);
        lhs
    }
}
