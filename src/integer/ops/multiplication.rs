use super::helpers::{
    reverse_multiply_assign, reverse_multiply_c_long, reverse_multiply_c_long_assign,
};
use crate::integer::Integer;
use core::ops::Mul;

// Mul          The multiplication operator *.

// [u'Integer', u'Integer', u'Integer', u'Integer::multiply_assign', u'lhs',
// [u'ref_mut'], [u'ref']]
impl Mul<Integer> for Integer {
    type Output = Integer;

    fn mul(mut self, rhs: Integer) -> Self::Output {
        Integer::multiply_assign(&mut self, &rhs);
        self
    }
}

// [u'Integer', u'&Integer', u'Integer', u'Integer::multiply_assign', u'lhs',
// [u'ref_mut'], []]
impl Mul<&Integer> for Integer {
    type Output = Integer;

    fn mul(mut self, rhs: &Integer) -> Self::Output {
        Integer::multiply_assign(&mut self, rhs);
        self
    }
}

// [u'&Integer', u'Integer', u'Integer', u'reverse_multiply_assign', u'rhs', [],
// [u'ref_mut']]
impl Mul<Integer> for &Integer {
    type Output = Integer;

    fn mul(self, mut rhs: Integer) -> Self::Output {
        reverse_multiply_assign(self, &mut rhs);
        rhs
    }
}

// [u'&Integer', u'&Integer', u'Integer', u'Integer::multiply', u'no', [], []]
impl Mul<&Integer> for &Integer {
    type Output = Integer;

    fn mul(self, rhs: &Integer) -> Self::Output {
        Integer::multiply(self, rhs)
    }
}

// [u'Integer', u'i8', u'Integer', u'Integer::multiply_c_long_assign', u'lhs',
// [u'ref_mut'], []]
impl Mul<i8> for Integer {
    type Output = Integer;

    fn mul(mut self, rhs: i8) -> Self::Output {
        Integer::multiply_c_long_assign(&mut self, rhs);
        self
    }
}

// [u'Integer', u'&i8', u'Integer', u'Integer::multiply_c_long_assign', u'lhs',
// [u'ref_mut'], [u'deref']]
impl Mul<&i8> for Integer {
    type Output = Integer;

    fn mul(mut self, rhs: &i8) -> Self::Output {
        Integer::multiply_c_long_assign(&mut self, *rhs);
        self
    }
}

// [u'&Integer', u'i8', u'Integer', u'Integer::multiply_c_long', u'no', [], []]
impl Mul<i8> for &Integer {
    type Output = Integer;

    fn mul(self, rhs: i8) -> Self::Output {
        Integer::multiply_c_long(self, rhs)
    }
}

// [u'&Integer', u'&i8', u'Integer', u'Integer::multiply_c_long', u'no', [],
// [u'deref']]
impl Mul<&i8> for &Integer {
    type Output = Integer;

    fn mul(self, rhs: &i8) -> Self::Output {
        Integer::multiply_c_long(self, *rhs)
    }
}

// [u'i8', u'Integer', u'Integer', u'reverse_multiply_c_long_assign', u'rhs',
// [], [u'ref_mut']]
impl Mul<Integer> for i8 {
    type Output = Integer;

    fn mul(self, mut rhs: Integer) -> Self::Output {
        reverse_multiply_c_long_assign(self, &mut rhs);
        rhs
    }
}

// [u'i8', u'&Integer', u'Integer', u'reverse_multiply_c_long', u'no', [], []]
impl Mul<&Integer> for i8 {
    type Output = Integer;

    fn mul(self, rhs: &Integer) -> Self::Output {
        reverse_multiply_c_long(self, rhs)
    }
}

// [u'&i8', u'Integer', u'Integer', u'reverse_multiply_c_long_assign', u'rhs',
// [u'deref'], [u'ref_mut']]
impl Mul<Integer> for &i8 {
    type Output = Integer;

    fn mul(self, mut rhs: Integer) -> Self::Output {
        reverse_multiply_c_long_assign(*self, &mut rhs);
        rhs
    }
}

// [u'&i8', u'&Integer', u'Integer', u'reverse_multiply_c_long', u'no',
// [u'deref'], []]
impl Mul<&Integer> for &i8 {
    type Output = Integer;

    fn mul(self, rhs: &Integer) -> Self::Output {
        reverse_multiply_c_long(*self, rhs)
    }
}

// [u'Integer', u'u8', u'Integer', u'Integer::multiply_c_long_assign', u'lhs',
// [u'ref_mut'], []]
impl Mul<u8> for Integer {
    type Output = Integer;

    fn mul(mut self, rhs: u8) -> Self::Output {
        Integer::multiply_c_long_assign(&mut self, rhs);
        self
    }
}

// [u'Integer', u'&u8', u'Integer', u'Integer::multiply_c_long_assign', u'lhs',
// [u'ref_mut'], [u'deref']]
impl Mul<&u8> for Integer {
    type Output = Integer;

    fn mul(mut self, rhs: &u8) -> Self::Output {
        Integer::multiply_c_long_assign(&mut self, *rhs);
        self
    }
}

// [u'&Integer', u'u8', u'Integer', u'Integer::multiply_c_long', u'no', [], []]
impl Mul<u8> for &Integer {
    type Output = Integer;

    fn mul(self, rhs: u8) -> Self::Output {
        Integer::multiply_c_long(self, rhs)
    }
}

// [u'&Integer', u'&u8', u'Integer', u'Integer::multiply_c_long', u'no', [],
// [u'deref']]
impl Mul<&u8> for &Integer {
    type Output = Integer;

    fn mul(self, rhs: &u8) -> Self::Output {
        Integer::multiply_c_long(self, *rhs)
    }
}

// [u'u8', u'Integer', u'Integer', u'reverse_multiply_c_long_assign', u'rhs',
// [], [u'ref_mut']]
impl Mul<Integer> for u8 {
    type Output = Integer;

    fn mul(self, mut rhs: Integer) -> Self::Output {
        reverse_multiply_c_long_assign(self, &mut rhs);
        rhs
    }
}

// [u'u8', u'&Integer', u'Integer', u'reverse_multiply_c_long', u'no', [], []]
impl Mul<&Integer> for u8 {
    type Output = Integer;

    fn mul(self, rhs: &Integer) -> Self::Output {
        reverse_multiply_c_long(self, rhs)
    }
}

// [u'&u8', u'Integer', u'Integer', u'reverse_multiply_c_long_assign', u'rhs',
// [u'deref'], [u'ref_mut']]
impl Mul<Integer> for &u8 {
    type Output = Integer;

    fn mul(self, mut rhs: Integer) -> Self::Output {
        reverse_multiply_c_long_assign(*self, &mut rhs);
        rhs
    }
}

// [u'&u8', u'&Integer', u'Integer', u'reverse_multiply_c_long', u'no',
// [u'deref'], []]
impl Mul<&Integer> for &u8 {
    type Output = Integer;

    fn mul(self, rhs: &Integer) -> Self::Output {
        reverse_multiply_c_long(*self, rhs)
    }
}

// [u'Integer', u'i16', u'Integer', u'Integer::multiply_c_long_assign', u'lhs',
// [u'ref_mut'], []]
impl Mul<i16> for Integer {
    type Output = Integer;

    fn mul(mut self, rhs: i16) -> Self::Output {
        Integer::multiply_c_long_assign(&mut self, rhs);
        self
    }
}

// [u'Integer', u'&i16', u'Integer', u'Integer::multiply_c_long_assign', u'lhs',
// [u'ref_mut'], [u'deref']]
impl Mul<&i16> for Integer {
    type Output = Integer;

    fn mul(mut self, rhs: &i16) -> Self::Output {
        Integer::multiply_c_long_assign(&mut self, *rhs);
        self
    }
}

// [u'&Integer', u'i16', u'Integer', u'Integer::multiply_c_long', u'no', [], []]
impl Mul<i16> for &Integer {
    type Output = Integer;

    fn mul(self, rhs: i16) -> Self::Output {
        Integer::multiply_c_long(self, rhs)
    }
}

// [u'&Integer', u'&i16', u'Integer', u'Integer::multiply_c_long', u'no', [],
// [u'deref']]
impl Mul<&i16> for &Integer {
    type Output = Integer;

    fn mul(self, rhs: &i16) -> Self::Output {
        Integer::multiply_c_long(self, *rhs)
    }
}

// [u'i16', u'Integer', u'Integer', u'reverse_multiply_c_long_assign', u'rhs',
// [], [u'ref_mut']]
impl Mul<Integer> for i16 {
    type Output = Integer;

    fn mul(self, mut rhs: Integer) -> Self::Output {
        reverse_multiply_c_long_assign(self, &mut rhs);
        rhs
    }
}

// [u'i16', u'&Integer', u'Integer', u'reverse_multiply_c_long', u'no', [], []]
impl Mul<&Integer> for i16 {
    type Output = Integer;

    fn mul(self, rhs: &Integer) -> Self::Output {
        reverse_multiply_c_long(self, rhs)
    }
}

// [u'&i16', u'Integer', u'Integer', u'reverse_multiply_c_long_assign', u'rhs',
// [u'deref'], [u'ref_mut']]
impl Mul<Integer> for &i16 {
    type Output = Integer;

    fn mul(self, mut rhs: Integer) -> Self::Output {
        reverse_multiply_c_long_assign(*self, &mut rhs);
        rhs
    }
}

// [u'&i16', u'&Integer', u'Integer', u'reverse_multiply_c_long', u'no',
// [u'deref'], []]
impl Mul<&Integer> for &i16 {
    type Output = Integer;

    fn mul(self, rhs: &Integer) -> Self::Output {
        reverse_multiply_c_long(*self, rhs)
    }
}

// [u'Integer', u'u16', u'Integer', u'Integer::multiply_c_long_assign', u'lhs',
// [u'ref_mut'], []]
impl Mul<u16> for Integer {
    type Output = Integer;

    fn mul(mut self, rhs: u16) -> Self::Output {
        Integer::multiply_c_long_assign(&mut self, rhs);
        self
    }
}

// [u'Integer', u'&u16', u'Integer', u'Integer::multiply_c_long_assign', u'lhs',
// [u'ref_mut'], [u'deref']]
impl Mul<&u16> for Integer {
    type Output = Integer;

    fn mul(mut self, rhs: &u16) -> Self::Output {
        Integer::multiply_c_long_assign(&mut self, *rhs);
        self
    }
}

// [u'&Integer', u'u16', u'Integer', u'Integer::multiply_c_long', u'no', [], []]
impl Mul<u16> for &Integer {
    type Output = Integer;

    fn mul(self, rhs: u16) -> Self::Output {
        Integer::multiply_c_long(self, rhs)
    }
}

// [u'&Integer', u'&u16', u'Integer', u'Integer::multiply_c_long', u'no', [],
// [u'deref']]
impl Mul<&u16> for &Integer {
    type Output = Integer;

    fn mul(self, rhs: &u16) -> Self::Output {
        Integer::multiply_c_long(self, *rhs)
    }
}

// [u'u16', u'Integer', u'Integer', u'reverse_multiply_c_long_assign', u'rhs',
// [], [u'ref_mut']]
impl Mul<Integer> for u16 {
    type Output = Integer;

    fn mul(self, mut rhs: Integer) -> Self::Output {
        reverse_multiply_c_long_assign(self, &mut rhs);
        rhs
    }
}

// [u'u16', u'&Integer', u'Integer', u'reverse_multiply_c_long', u'no', [], []]
impl Mul<&Integer> for u16 {
    type Output = Integer;

    fn mul(self, rhs: &Integer) -> Self::Output {
        reverse_multiply_c_long(self, rhs)
    }
}

// [u'&u16', u'Integer', u'Integer', u'reverse_multiply_c_long_assign', u'rhs',
// [u'deref'], [u'ref_mut']]
impl Mul<Integer> for &u16 {
    type Output = Integer;

    fn mul(self, mut rhs: Integer) -> Self::Output {
        reverse_multiply_c_long_assign(*self, &mut rhs);
        rhs
    }
}

// [u'&u16', u'&Integer', u'Integer', u'reverse_multiply_c_long', u'no',
// [u'deref'], []]
impl Mul<&Integer> for &u16 {
    type Output = Integer;

    fn mul(self, rhs: &Integer) -> Self::Output {
        reverse_multiply_c_long(*self, rhs)
    }
}

// [u'Integer', u'i32', u'Integer', u'Integer::multiply_c_long_assign', u'lhs',
// [u'ref_mut'], []]
impl Mul<i32> for Integer {
    type Output = Integer;

    fn mul(mut self, rhs: i32) -> Self::Output {
        Integer::multiply_c_long_assign(&mut self, rhs);
        self
    }
}

// [u'Integer', u'&i32', u'Integer', u'Integer::multiply_c_long_assign', u'lhs',
// [u'ref_mut'], [u'deref']]
impl Mul<&i32> for Integer {
    type Output = Integer;

    fn mul(mut self, rhs: &i32) -> Self::Output {
        Integer::multiply_c_long_assign(&mut self, *rhs);
        self
    }
}

// [u'&Integer', u'i32', u'Integer', u'Integer::multiply_c_long', u'no', [], []]
impl Mul<i32> for &Integer {
    type Output = Integer;

    fn mul(self, rhs: i32) -> Self::Output {
        Integer::multiply_c_long(self, rhs)
    }
}

// [u'&Integer', u'&i32', u'Integer', u'Integer::multiply_c_long', u'no', [],
// [u'deref']]
impl Mul<&i32> for &Integer {
    type Output = Integer;

    fn mul(self, rhs: &i32) -> Self::Output {
        Integer::multiply_c_long(self, *rhs)
    }
}

// [u'i32', u'Integer', u'Integer', u'reverse_multiply_c_long_assign', u'rhs',
// [], [u'ref_mut']]
impl Mul<Integer> for i32 {
    type Output = Integer;

    fn mul(self, mut rhs: Integer) -> Self::Output {
        reverse_multiply_c_long_assign(self, &mut rhs);
        rhs
    }
}

// [u'i32', u'&Integer', u'Integer', u'reverse_multiply_c_long', u'no', [], []]
impl Mul<&Integer> for i32 {
    type Output = Integer;

    fn mul(self, rhs: &Integer) -> Self::Output {
        reverse_multiply_c_long(self, rhs)
    }
}

// [u'&i32', u'Integer', u'Integer', u'reverse_multiply_c_long_assign', u'rhs',
// [u'deref'], [u'ref_mut']]
impl Mul<Integer> for &i32 {
    type Output = Integer;

    fn mul(self, mut rhs: Integer) -> Self::Output {
        reverse_multiply_c_long_assign(*self, &mut rhs);
        rhs
    }
}

// [u'&i32', u'&Integer', u'Integer', u'reverse_multiply_c_long', u'no',
// [u'deref'], []]
impl Mul<&Integer> for &i32 {
    type Output = Integer;

    fn mul(self, rhs: &Integer) -> Self::Output {
        reverse_multiply_c_long(*self, rhs)
    }
}

// [u'Integer', u'u32', u'Integer', u'Integer::multiply_c_long_assign', u'lhs',
// [u'ref_mut'], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Mul<u32> for Integer {
    type Output = Integer;

    fn mul(mut self, rhs: u32) -> Self::Output {
        Integer::multiply_c_long_assign(&mut self, rhs);
        self
    }
}

// [u'Integer', u'&u32', u'Integer', u'Integer::multiply_c_long_assign', u'lhs',
// [u'ref_mut'], [u'deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Mul<&u32> for Integer {
    type Output = Integer;

    fn mul(mut self, rhs: &u32) -> Self::Output {
        Integer::multiply_c_long_assign(&mut self, *rhs);
        self
    }
}

// [u'&Integer', u'u32', u'Integer', u'Integer::multiply_c_long', u'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Mul<u32> for &Integer {
    type Output = Integer;

    fn mul(self, rhs: u32) -> Self::Output {
        Integer::multiply_c_long(self, rhs)
    }
}

// [u'&Integer', u'&u32', u'Integer', u'Integer::multiply_c_long', u'no', [],
// [u'deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Mul<&u32> for &Integer {
    type Output = Integer;

    fn mul(self, rhs: &u32) -> Self::Output {
        Integer::multiply_c_long(self, *rhs)
    }
}

// [u'u32', u'Integer', u'Integer', u'reverse_multiply_c_long_assign', u'rhs',
// [], [u'ref_mut']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Mul<Integer> for u32 {
    type Output = Integer;

    fn mul(self, mut rhs: Integer) -> Self::Output {
        reverse_multiply_c_long_assign(self, &mut rhs);
        rhs
    }
}

// [u'u32', u'&Integer', u'Integer', u'reverse_multiply_c_long', u'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Mul<&Integer> for u32 {
    type Output = Integer;

    fn mul(self, rhs: &Integer) -> Self::Output {
        reverse_multiply_c_long(self, rhs)
    }
}

// [u'&u32', u'Integer', u'Integer', u'reverse_multiply_c_long_assign', u'rhs',
// [u'deref'], [u'ref_mut']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Mul<Integer> for &u32 {
    type Output = Integer;

    fn mul(self, mut rhs: Integer) -> Self::Output {
        reverse_multiply_c_long_assign(*self, &mut rhs);
        rhs
    }
}

// [u'&u32', u'&Integer', u'Integer', u'reverse_multiply_c_long', u'no',
// [u'deref'], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Mul<&Integer> for &u32 {
    type Output = Integer;

    fn mul(self, rhs: &Integer) -> Self::Output {
        reverse_multiply_c_long(*self, rhs)
    }
}

// [u'Integer', u'i64', u'Integer', u'Integer::multiply_c_long_assign', u'lhs',
// [u'ref_mut'], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Mul<i64> for Integer {
    type Output = Integer;

    fn mul(mut self, rhs: i64) -> Self::Output {
        Integer::multiply_c_long_assign(&mut self, rhs);
        self
    }
}

// [u'Integer', u'&i64', u'Integer', u'Integer::multiply_c_long_assign', u'lhs',
// [u'ref_mut'], [u'deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Mul<&i64> for Integer {
    type Output = Integer;

    fn mul(mut self, rhs: &i64) -> Self::Output {
        Integer::multiply_c_long_assign(&mut self, *rhs);
        self
    }
}

// [u'&Integer', u'i64', u'Integer', u'Integer::multiply_c_long', u'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Mul<i64> for &Integer {
    type Output = Integer;

    fn mul(self, rhs: i64) -> Self::Output {
        Integer::multiply_c_long(self, rhs)
    }
}

// [u'&Integer', u'&i64', u'Integer', u'Integer::multiply_c_long', u'no', [],
// [u'deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Mul<&i64> for &Integer {
    type Output = Integer;

    fn mul(self, rhs: &i64) -> Self::Output {
        Integer::multiply_c_long(self, *rhs)
    }
}

// [u'i64', u'Integer', u'Integer', u'reverse_multiply_c_long_assign', u'rhs',
// [], [u'ref_mut']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Mul<Integer> for i64 {
    type Output = Integer;

    fn mul(self, mut rhs: Integer) -> Self::Output {
        reverse_multiply_c_long_assign(self, &mut rhs);
        rhs
    }
}

// [u'i64', u'&Integer', u'Integer', u'reverse_multiply_c_long', u'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Mul<&Integer> for i64 {
    type Output = Integer;

    fn mul(self, rhs: &Integer) -> Self::Output {
        reverse_multiply_c_long(self, rhs)
    }
}

// [u'&i64', u'Integer', u'Integer', u'reverse_multiply_c_long_assign', u'rhs',
// [u'deref'], [u'ref_mut']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Mul<Integer> for &i64 {
    type Output = Integer;

    fn mul(self, mut rhs: Integer) -> Self::Output {
        reverse_multiply_c_long_assign(*self, &mut rhs);
        rhs
    }
}

// [u'&i64', u'&Integer', u'Integer', u'reverse_multiply_c_long', u'no',
// [u'deref'], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Mul<&Integer> for &i64 {
    type Output = Integer;

    fn mul(self, rhs: &Integer) -> Self::Output {
        reverse_multiply_c_long(*self, rhs)
    }
}

// [u'Integer', u'u32', u'Integer', u'Integer::multiply_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Mul<u32> for Integer {
    type Output = Integer;

    fn mul(mut self, rhs: u32) -> Self::Output {
        Integer::multiply_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// [u'Integer', u'&u32', u'Integer', u'Integer::multiply_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}, u'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Mul<&u32> for Integer {
    type Output = Integer;

    fn mul(mut self, rhs: &u32) -> Self::Output {
        Integer::multiply_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// [u'&Integer', u'u32', u'Integer', u'let mut rhs =
// Integer::from(rhs);\nreverse_multiply_assign(self, &mut rhs);\nrhs']
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Mul<u32> for &Integer {
    type Output = Integer;

    fn mul(self, rhs: u32) -> Self::Output {
        let mut rhs = Integer::from(rhs);
        reverse_multiply_assign(self, &mut rhs);
        rhs
    }
}

// [u'&Integer', u'&u32', u'Integer', u'let mut rhs =
// Integer::from(*rhs);\nreverse_multiply_assign(self, &mut rhs);\nrhs']
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Mul<&u32> for &Integer {
    type Output = Integer;

    fn mul(self, rhs: &u32) -> Self::Output {
        let mut rhs = Integer::from(*rhs);
        reverse_multiply_assign(self, &mut rhs);
        rhs
    }
}

// [u'u32', u'Integer', u'Integer', u'reverse_multiply_assign', u'rhs', [u'ref',
// {u'convert': u'Integer'}], [u'ref_mut']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Mul<Integer> for u32 {
    type Output = Integer;

    fn mul(self, mut rhs: Integer) -> Self::Output {
        reverse_multiply_assign(&Integer::from(self), &mut rhs);
        rhs
    }
}

// [u'u32', u'&Integer', u'Integer', u'let mut lhs =
// Integer::from(self);\nInteger::multiply_assign(&mut lhs, rhs);\nlhs']
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Mul<&Integer> for u32 {
    type Output = Integer;

    fn mul(self, rhs: &Integer) -> Self::Output {
        let mut lhs = Integer::from(self);
        Integer::multiply_assign(&mut lhs, rhs);
        lhs
    }
}

// [u'&u32', u'Integer', u'Integer', u'reverse_multiply_assign', u'rhs',
// [u'ref', {u'convert': u'Integer'}, u'deref'], [u'ref_mut']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Mul<Integer> for &u32 {
    type Output = Integer;

    fn mul(self, mut rhs: Integer) -> Self::Output {
        reverse_multiply_assign(&Integer::from(*self), &mut rhs);
        rhs
    }
}

// [u'&u32', u'&Integer', u'Integer', u'let mut lhs =
// Integer::from(*self);\nInteger::multiply_assign(&mut lhs, rhs);\nlhs']
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Mul<&Integer> for &u32 {
    type Output = Integer;

    fn mul(self, rhs: &Integer) -> Self::Output {
        let mut lhs = Integer::from(*self);
        Integer::multiply_assign(&mut lhs, rhs);
        lhs
    }
}

// [u'Integer', u'i64', u'Integer', u'Integer::multiply_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Mul<i64> for Integer {
    type Output = Integer;

    fn mul(mut self, rhs: i64) -> Self::Output {
        Integer::multiply_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// [u'Integer', u'&i64', u'Integer', u'Integer::multiply_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}, u'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Mul<&i64> for Integer {
    type Output = Integer;

    fn mul(mut self, rhs: &i64) -> Self::Output {
        Integer::multiply_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// [u'&Integer', u'i64', u'Integer', u'let mut rhs =
// Integer::from(rhs);\nreverse_multiply_assign(self, &mut rhs);\nrhs']
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Mul<i64> for &Integer {
    type Output = Integer;

    fn mul(self, rhs: i64) -> Self::Output {
        let mut rhs = Integer::from(rhs);
        reverse_multiply_assign(self, &mut rhs);
        rhs
    }
}

// [u'&Integer', u'&i64', u'Integer', u'let mut rhs =
// Integer::from(*rhs);\nreverse_multiply_assign(self, &mut rhs);\nrhs']
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Mul<&i64> for &Integer {
    type Output = Integer;

    fn mul(self, rhs: &i64) -> Self::Output {
        let mut rhs = Integer::from(*rhs);
        reverse_multiply_assign(self, &mut rhs);
        rhs
    }
}

// [u'i64', u'Integer', u'Integer', u'reverse_multiply_assign', u'rhs', [u'ref',
// {u'convert': u'Integer'}], [u'ref_mut']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Mul<Integer> for i64 {
    type Output = Integer;

    fn mul(self, mut rhs: Integer) -> Self::Output {
        reverse_multiply_assign(&Integer::from(self), &mut rhs);
        rhs
    }
}

// [u'i64', u'&Integer', u'Integer', u'let mut lhs =
// Integer::from(self);\nInteger::multiply_assign(&mut lhs, rhs);\nlhs']
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Mul<&Integer> for i64 {
    type Output = Integer;

    fn mul(self, rhs: &Integer) -> Self::Output {
        let mut lhs = Integer::from(self);
        Integer::multiply_assign(&mut lhs, rhs);
        lhs
    }
}

// [u'&i64', u'Integer', u'Integer', u'reverse_multiply_assign', u'rhs',
// [u'ref', {u'convert': u'Integer'}, u'deref'], [u'ref_mut']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Mul<Integer> for &i64 {
    type Output = Integer;

    fn mul(self, mut rhs: Integer) -> Self::Output {
        reverse_multiply_assign(&Integer::from(*self), &mut rhs);
        rhs
    }
}

// [u'&i64', u'&Integer', u'Integer', u'let mut lhs =
// Integer::from(*self);\nInteger::multiply_assign(&mut lhs, rhs);\nlhs']
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Mul<&Integer> for &i64 {
    type Output = Integer;

    fn mul(self, rhs: &Integer) -> Self::Output {
        let mut lhs = Integer::from(*self);
        Integer::multiply_assign(&mut lhs, rhs);
        lhs
    }
}

// [u'Integer', u'u64', u'Integer', u'Integer::multiply_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}]]
impl Mul<u64> for Integer {
    type Output = Integer;

    fn mul(mut self, rhs: u64) -> Self::Output {
        Integer::multiply_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// [u'Integer', u'&u64', u'Integer', u'Integer::multiply_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}, u'deref']]
impl Mul<&u64> for Integer {
    type Output = Integer;

    fn mul(mut self, rhs: &u64) -> Self::Output {
        Integer::multiply_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// [u'&Integer', u'u64', u'Integer', u'let mut rhs =
// Integer::from(rhs);\nreverse_multiply_assign(self, &mut rhs);\nrhs']
impl Mul<u64> for &Integer {
    type Output = Integer;

    fn mul(self, rhs: u64) -> Self::Output {
        let mut rhs = Integer::from(rhs);
        reverse_multiply_assign(self, &mut rhs);
        rhs
    }
}

// [u'&Integer', u'&u64', u'Integer', u'let mut rhs =
// Integer::from(*rhs);\nreverse_multiply_assign(self, &mut rhs);\nrhs']
impl Mul<&u64> for &Integer {
    type Output = Integer;

    fn mul(self, rhs: &u64) -> Self::Output {
        let mut rhs = Integer::from(*rhs);
        reverse_multiply_assign(self, &mut rhs);
        rhs
    }
}

// [u'u64', u'Integer', u'Integer', u'reverse_multiply_assign', u'rhs', [u'ref',
// {u'convert': u'Integer'}], [u'ref_mut']]
impl Mul<Integer> for u64 {
    type Output = Integer;

    fn mul(self, mut rhs: Integer) -> Self::Output {
        reverse_multiply_assign(&Integer::from(self), &mut rhs);
        rhs
    }
}

// [u'u64', u'&Integer', u'Integer', u'let mut lhs =
// Integer::from(self);\nInteger::multiply_assign(&mut lhs, rhs);\nlhs']
impl Mul<&Integer> for u64 {
    type Output = Integer;

    fn mul(self, rhs: &Integer) -> Self::Output {
        let mut lhs = Integer::from(self);
        Integer::multiply_assign(&mut lhs, rhs);
        lhs
    }
}

// [u'&u64', u'Integer', u'Integer', u'reverse_multiply_assign', u'rhs',
// [u'ref', {u'convert': u'Integer'}, u'deref'], [u'ref_mut']]
impl Mul<Integer> for &u64 {
    type Output = Integer;

    fn mul(self, mut rhs: Integer) -> Self::Output {
        reverse_multiply_assign(&Integer::from(*self), &mut rhs);
        rhs
    }
}

// [u'&u64', u'&Integer', u'Integer', u'let mut lhs =
// Integer::from(*self);\nInteger::multiply_assign(&mut lhs, rhs);\nlhs']
impl Mul<&Integer> for &u64 {
    type Output = Integer;

    fn mul(self, rhs: &Integer) -> Self::Output {
        let mut lhs = Integer::from(*self);
        Integer::multiply_assign(&mut lhs, rhs);
        lhs
    }
}

// [u'Integer', u'i128', u'Integer', u'Integer::multiply_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}]]
impl Mul<i128> for Integer {
    type Output = Integer;

    fn mul(mut self, rhs: i128) -> Self::Output {
        Integer::multiply_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// [u'Integer', u'&i128', u'Integer', u'Integer::multiply_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}, u'deref']]
impl Mul<&i128> for Integer {
    type Output = Integer;

    fn mul(mut self, rhs: &i128) -> Self::Output {
        Integer::multiply_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// [u'&Integer', u'i128', u'Integer', u'let mut rhs =
// Integer::from(rhs);\nreverse_multiply_assign(self, &mut rhs);\nrhs']
impl Mul<i128> for &Integer {
    type Output = Integer;

    fn mul(self, rhs: i128) -> Self::Output {
        let mut rhs = Integer::from(rhs);
        reverse_multiply_assign(self, &mut rhs);
        rhs
    }
}

// [u'&Integer', u'&i128', u'Integer', u'let mut rhs =
// Integer::from(*rhs);\nreverse_multiply_assign(self, &mut rhs);\nrhs']
impl Mul<&i128> for &Integer {
    type Output = Integer;

    fn mul(self, rhs: &i128) -> Self::Output {
        let mut rhs = Integer::from(*rhs);
        reverse_multiply_assign(self, &mut rhs);
        rhs
    }
}

// [u'i128', u'Integer', u'Integer', u'reverse_multiply_assign', u'rhs',
// [u'ref', {u'convert': u'Integer'}], [u'ref_mut']]
impl Mul<Integer> for i128 {
    type Output = Integer;

    fn mul(self, mut rhs: Integer) -> Self::Output {
        reverse_multiply_assign(&Integer::from(self), &mut rhs);
        rhs
    }
}

// [u'i128', u'&Integer', u'Integer', u'let mut lhs =
// Integer::from(self);\nInteger::multiply_assign(&mut lhs, rhs);\nlhs']
impl Mul<&Integer> for i128 {
    type Output = Integer;

    fn mul(self, rhs: &Integer) -> Self::Output {
        let mut lhs = Integer::from(self);
        Integer::multiply_assign(&mut lhs, rhs);
        lhs
    }
}

// [u'&i128', u'Integer', u'Integer', u'reverse_multiply_assign', u'rhs',
// [u'ref', {u'convert': u'Integer'}, u'deref'], [u'ref_mut']]
impl Mul<Integer> for &i128 {
    type Output = Integer;

    fn mul(self, mut rhs: Integer) -> Self::Output {
        reverse_multiply_assign(&Integer::from(*self), &mut rhs);
        rhs
    }
}

// [u'&i128', u'&Integer', u'Integer', u'let mut lhs =
// Integer::from(*self);\nInteger::multiply_assign(&mut lhs, rhs);\nlhs']
impl Mul<&Integer> for &i128 {
    type Output = Integer;

    fn mul(self, rhs: &Integer) -> Self::Output {
        let mut lhs = Integer::from(*self);
        Integer::multiply_assign(&mut lhs, rhs);
        lhs
    }
}

// [u'Integer', u'u128', u'Integer', u'Integer::multiply_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}]]
impl Mul<u128> for Integer {
    type Output = Integer;

    fn mul(mut self, rhs: u128) -> Self::Output {
        Integer::multiply_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// [u'Integer', u'&u128', u'Integer', u'Integer::multiply_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}, u'deref']]
impl Mul<&u128> for Integer {
    type Output = Integer;

    fn mul(mut self, rhs: &u128) -> Self::Output {
        Integer::multiply_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// [u'&Integer', u'u128', u'Integer', u'let mut rhs =
// Integer::from(rhs);\nreverse_multiply_assign(self, &mut rhs);\nrhs']
impl Mul<u128> for &Integer {
    type Output = Integer;

    fn mul(self, rhs: u128) -> Self::Output {
        let mut rhs = Integer::from(rhs);
        reverse_multiply_assign(self, &mut rhs);
        rhs
    }
}

// [u'&Integer', u'&u128', u'Integer', u'let mut rhs =
// Integer::from(*rhs);\nreverse_multiply_assign(self, &mut rhs);\nrhs']
impl Mul<&u128> for &Integer {
    type Output = Integer;

    fn mul(self, rhs: &u128) -> Self::Output {
        let mut rhs = Integer::from(*rhs);
        reverse_multiply_assign(self, &mut rhs);
        rhs
    }
}

// [u'u128', u'Integer', u'Integer', u'reverse_multiply_assign', u'rhs',
// [u'ref', {u'convert': u'Integer'}], [u'ref_mut']]
impl Mul<Integer> for u128 {
    type Output = Integer;

    fn mul(self, mut rhs: Integer) -> Self::Output {
        reverse_multiply_assign(&Integer::from(self), &mut rhs);
        rhs
    }
}

// [u'u128', u'&Integer', u'Integer', u'let mut lhs =
// Integer::from(self);\nInteger::multiply_assign(&mut lhs, rhs);\nlhs']
impl Mul<&Integer> for u128 {
    type Output = Integer;

    fn mul(self, rhs: &Integer) -> Self::Output {
        let mut lhs = Integer::from(self);
        Integer::multiply_assign(&mut lhs, rhs);
        lhs
    }
}

// [u'&u128', u'Integer', u'Integer', u'reverse_multiply_assign', u'rhs',
// [u'ref', {u'convert': u'Integer'}, u'deref'], [u'ref_mut']]
impl Mul<Integer> for &u128 {
    type Output = Integer;

    fn mul(self, mut rhs: Integer) -> Self::Output {
        reverse_multiply_assign(&Integer::from(*self), &mut rhs);
        rhs
    }
}

// [u'&u128', u'&Integer', u'Integer', u'let mut lhs =
// Integer::from(*self);\nInteger::multiply_assign(&mut lhs, rhs);\nlhs']
impl Mul<&Integer> for &u128 {
    type Output = Integer;

    fn mul(self, rhs: &Integer) -> Self::Output {
        let mut lhs = Integer::from(*self);
        Integer::multiply_assign(&mut lhs, rhs);
        lhs
    }
}
