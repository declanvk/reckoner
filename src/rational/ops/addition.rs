use super::helpers::{add_reuse_lhs, add_reuse_rhs};
use crate::{integer::Integer, rational::Rational};
use core::ops::Add;

// Add          The addition operator +.

// ['Rational', 'Rational', 'Rational', 'add_reuse_lhs', 'no', [], ['ref']]
impl Add<Rational> for Rational {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Self::Output {
        add_reuse_lhs(self, &rhs)
    }
}

// ['Rational', '&Rational', 'Rational', 'add_reuse_lhs', 'no', [], []]
impl Add<&Rational> for Rational {
    type Output = Rational;

    fn add(self, rhs: &Rational) -> Self::Output {
        add_reuse_lhs(self, rhs)
    }
}

// ['&Rational', 'Rational', 'Rational', 'add_reuse_rhs', 'no', [], []]
impl Add<Rational> for &Rational {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Self::Output {
        add_reuse_rhs(self, rhs)
    }
}

// ['&Rational', '&Rational', 'Rational', 'Rational::add', 'no', [], []]
impl Add<&Rational> for &Rational {
    type Output = Rational;

    fn add(self, rhs: &Rational) -> Self::Output {
        Rational::add(self, rhs)
    }
}

// ['Rational', 'Integer', 'Rational', 'add_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Add<Integer> for Rational {
    type Output = Rational;

    fn add(self, rhs: Integer) -> Self::Output {
        add_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['Rational', '&Integer', 'Rational', 'add_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Add<&Integer> for Rational {
    type Output = Rational;

    fn add(self, rhs: &Integer) -> Self::Output {
        add_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['&Rational', 'Integer', 'Rational', 'add_reuse_rhs', 'no', [], []]
impl Add<Integer> for &Rational {
    type Output = Rational;

    fn add(self, rhs: Integer) -> Self::Output {
        add_reuse_rhs(self, rhs)
    }
}

// ['&Rational', '&Integer', 'Rational', 'add_reuse_rhs', 'no', [], []]
impl Add<&Integer> for &Rational {
    type Output = Rational;

    fn add(self, rhs: &Integer) -> Self::Output {
        add_reuse_rhs(self, rhs)
    }
}

// ['Integer', 'Rational', 'Rational', 'add_reuse_rhs', 'no', ['ref',
// {'convert': 'Rational'}], []]
impl Add<Rational> for Integer {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Self::Output {
        add_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['Integer', '&Rational', 'Rational', 'add_reuse_lhs', 'no', [], []]
impl Add<&Rational> for Integer {
    type Output = Rational;

    fn add(self, rhs: &Rational) -> Self::Output {
        add_reuse_lhs(self, rhs)
    }
}

// ['&Integer', 'Rational', 'Rational', 'add_reuse_rhs', 'no', ['ref',
// {'convert': 'Rational'}], []]
impl Add<Rational> for &Integer {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Self::Output {
        add_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['&Integer', '&Rational', 'Rational', 'add_reuse_lhs', 'no', [], []]
impl Add<&Rational> for &Integer {
    type Output = Rational;

    fn add(self, rhs: &Rational) -> Self::Output {
        add_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'i8', 'Rational', 'add_reuse_lhs', 'no', [], ['ref', {'convert':
// 'Rational'}]]
impl Add<i8> for Rational {
    type Output = Rational;

    fn add(self, rhs: i8) -> Self::Output {
        add_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['Rational', '&i8', 'Rational', 'add_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Add<&i8> for Rational {
    type Output = Rational;

    fn add(self, rhs: &i8) -> Self::Output {
        add_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['&Rational', 'i8', 'Rational', 'add_reuse_rhs', 'no', [], []]
impl Add<i8> for &Rational {
    type Output = Rational;

    fn add(self, rhs: i8) -> Self::Output {
        add_reuse_rhs(self, rhs)
    }
}

// ['&Rational', '&i8', 'Rational', 'add_reuse_rhs', 'no', [], []]
impl Add<&i8> for &Rational {
    type Output = Rational;

    fn add(self, rhs: &i8) -> Self::Output {
        add_reuse_rhs(self, rhs)
    }
}

// ['i8', 'Rational', 'Rational', 'add_reuse_rhs', 'no', ['ref', {'convert':
// 'Rational'}], []]
impl Add<Rational> for i8 {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Self::Output {
        add_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['i8', '&Rational', 'Rational', 'add_reuse_lhs', 'no', [], []]
impl Add<&Rational> for i8 {
    type Output = Rational;

    fn add(self, rhs: &Rational) -> Self::Output {
        add_reuse_lhs(self, rhs)
    }
}

// ['&i8', 'Rational', 'Rational', 'add_reuse_rhs', 'no', ['ref', {'convert':
// 'Rational'}], []]
impl Add<Rational> for &i8 {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Self::Output {
        add_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['&i8', '&Rational', 'Rational', 'add_reuse_lhs', 'no', [], []]
impl Add<&Rational> for &i8 {
    type Output = Rational;

    fn add(self, rhs: &Rational) -> Self::Output {
        add_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'u8', 'Rational', 'add_reuse_lhs', 'no', [], ['ref', {'convert':
// 'Rational'}]]
impl Add<u8> for Rational {
    type Output = Rational;

    fn add(self, rhs: u8) -> Self::Output {
        add_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['Rational', '&u8', 'Rational', 'add_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Add<&u8> for Rational {
    type Output = Rational;

    fn add(self, rhs: &u8) -> Self::Output {
        add_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['&Rational', 'u8', 'Rational', 'add_reuse_rhs', 'no', [], []]
impl Add<u8> for &Rational {
    type Output = Rational;

    fn add(self, rhs: u8) -> Self::Output {
        add_reuse_rhs(self, rhs)
    }
}

// ['&Rational', '&u8', 'Rational', 'add_reuse_rhs', 'no', [], []]
impl Add<&u8> for &Rational {
    type Output = Rational;

    fn add(self, rhs: &u8) -> Self::Output {
        add_reuse_rhs(self, rhs)
    }
}

// ['u8', 'Rational', 'Rational', 'add_reuse_rhs', 'no', ['ref', {'convert':
// 'Rational'}], []]
impl Add<Rational> for u8 {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Self::Output {
        add_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['u8', '&Rational', 'Rational', 'add_reuse_lhs', 'no', [], []]
impl Add<&Rational> for u8 {
    type Output = Rational;

    fn add(self, rhs: &Rational) -> Self::Output {
        add_reuse_lhs(self, rhs)
    }
}

// ['&u8', 'Rational', 'Rational', 'add_reuse_rhs', 'no', ['ref', {'convert':
// 'Rational'}], []]
impl Add<Rational> for &u8 {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Self::Output {
        add_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['&u8', '&Rational', 'Rational', 'add_reuse_lhs', 'no', [], []]
impl Add<&Rational> for &u8 {
    type Output = Rational;

    fn add(self, rhs: &Rational) -> Self::Output {
        add_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'i16', 'Rational', 'add_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Add<i16> for Rational {
    type Output = Rational;

    fn add(self, rhs: i16) -> Self::Output {
        add_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['Rational', '&i16', 'Rational', 'add_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Add<&i16> for Rational {
    type Output = Rational;

    fn add(self, rhs: &i16) -> Self::Output {
        add_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['&Rational', 'i16', 'Rational', 'add_reuse_rhs', 'no', [], []]
impl Add<i16> for &Rational {
    type Output = Rational;

    fn add(self, rhs: i16) -> Self::Output {
        add_reuse_rhs(self, rhs)
    }
}

// ['&Rational', '&i16', 'Rational', 'add_reuse_rhs', 'no', [], []]
impl Add<&i16> for &Rational {
    type Output = Rational;

    fn add(self, rhs: &i16) -> Self::Output {
        add_reuse_rhs(self, rhs)
    }
}

// ['i16', 'Rational', 'Rational', 'add_reuse_rhs', 'no', ['ref', {'convert':
// 'Rational'}], []]
impl Add<Rational> for i16 {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Self::Output {
        add_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['i16', '&Rational', 'Rational', 'add_reuse_lhs', 'no', [], []]
impl Add<&Rational> for i16 {
    type Output = Rational;

    fn add(self, rhs: &Rational) -> Self::Output {
        add_reuse_lhs(self, rhs)
    }
}

// ['&i16', 'Rational', 'Rational', 'add_reuse_rhs', 'no', ['ref', {'convert':
// 'Rational'}], []]
impl Add<Rational> for &i16 {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Self::Output {
        add_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['&i16', '&Rational', 'Rational', 'add_reuse_lhs', 'no', [], []]
impl Add<&Rational> for &i16 {
    type Output = Rational;

    fn add(self, rhs: &Rational) -> Self::Output {
        add_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'u16', 'Rational', 'add_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Add<u16> for Rational {
    type Output = Rational;

    fn add(self, rhs: u16) -> Self::Output {
        add_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['Rational', '&u16', 'Rational', 'add_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Add<&u16> for Rational {
    type Output = Rational;

    fn add(self, rhs: &u16) -> Self::Output {
        add_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['&Rational', 'u16', 'Rational', 'add_reuse_rhs', 'no', [], []]
impl Add<u16> for &Rational {
    type Output = Rational;

    fn add(self, rhs: u16) -> Self::Output {
        add_reuse_rhs(self, rhs)
    }
}

// ['&Rational', '&u16', 'Rational', 'add_reuse_rhs', 'no', [], []]
impl Add<&u16> for &Rational {
    type Output = Rational;

    fn add(self, rhs: &u16) -> Self::Output {
        add_reuse_rhs(self, rhs)
    }
}

// ['u16', 'Rational', 'Rational', 'add_reuse_rhs', 'no', ['ref', {'convert':
// 'Rational'}], []]
impl Add<Rational> for u16 {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Self::Output {
        add_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['u16', '&Rational', 'Rational', 'add_reuse_lhs', 'no', [], []]
impl Add<&Rational> for u16 {
    type Output = Rational;

    fn add(self, rhs: &Rational) -> Self::Output {
        add_reuse_lhs(self, rhs)
    }
}

// ['&u16', 'Rational', 'Rational', 'add_reuse_rhs', 'no', ['ref', {'convert':
// 'Rational'}], []]
impl Add<Rational> for &u16 {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Self::Output {
        add_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['&u16', '&Rational', 'Rational', 'add_reuse_lhs', 'no', [], []]
impl Add<&Rational> for &u16 {
    type Output = Rational;

    fn add(self, rhs: &Rational) -> Self::Output {
        add_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'i32', 'Rational', 'add_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Add<i32> for Rational {
    type Output = Rational;

    fn add(self, rhs: i32) -> Self::Output {
        add_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['Rational', '&i32', 'Rational', 'add_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Add<&i32> for Rational {
    type Output = Rational;

    fn add(self, rhs: &i32) -> Self::Output {
        add_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['&Rational', 'i32', 'Rational', 'add_reuse_rhs', 'no', [], []]
impl Add<i32> for &Rational {
    type Output = Rational;

    fn add(self, rhs: i32) -> Self::Output {
        add_reuse_rhs(self, rhs)
    }
}

// ['&Rational', '&i32', 'Rational', 'add_reuse_rhs', 'no', [], []]
impl Add<&i32> for &Rational {
    type Output = Rational;

    fn add(self, rhs: &i32) -> Self::Output {
        add_reuse_rhs(self, rhs)
    }
}

// ['i32', 'Rational', 'Rational', 'add_reuse_rhs', 'no', ['ref', {'convert':
// 'Rational'}], []]
impl Add<Rational> for i32 {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Self::Output {
        add_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['i32', '&Rational', 'Rational', 'add_reuse_lhs', 'no', [], []]
impl Add<&Rational> for i32 {
    type Output = Rational;

    fn add(self, rhs: &Rational) -> Self::Output {
        add_reuse_lhs(self, rhs)
    }
}

// ['&i32', 'Rational', 'Rational', 'add_reuse_rhs', 'no', ['ref', {'convert':
// 'Rational'}], []]
impl Add<Rational> for &i32 {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Self::Output {
        add_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['&i32', '&Rational', 'Rational', 'add_reuse_lhs', 'no', [], []]
impl Add<&Rational> for &i32 {
    type Output = Rational;

    fn add(self, rhs: &Rational) -> Self::Output {
        add_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'u32', 'Rational', 'add_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Add<u32> for Rational {
    type Output = Rational;

    fn add(self, rhs: u32) -> Self::Output {
        add_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['Rational', '&u32', 'Rational', 'add_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Add<&u32> for Rational {
    type Output = Rational;

    fn add(self, rhs: &u32) -> Self::Output {
        add_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['&Rational', 'u32', 'Rational', 'add_reuse_rhs', 'no', [], []]
impl Add<u32> for &Rational {
    type Output = Rational;

    fn add(self, rhs: u32) -> Self::Output {
        add_reuse_rhs(self, rhs)
    }
}

// ['&Rational', '&u32', 'Rational', 'add_reuse_rhs', 'no', [], []]
impl Add<&u32> for &Rational {
    type Output = Rational;

    fn add(self, rhs: &u32) -> Self::Output {
        add_reuse_rhs(self, rhs)
    }
}

// ['u32', 'Rational', 'Rational', 'add_reuse_rhs', 'no', ['ref', {'convert':
// 'Rational'}], []]
impl Add<Rational> for u32 {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Self::Output {
        add_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['u32', '&Rational', 'Rational', 'add_reuse_lhs', 'no', [], []]
impl Add<&Rational> for u32 {
    type Output = Rational;

    fn add(self, rhs: &Rational) -> Self::Output {
        add_reuse_lhs(self, rhs)
    }
}

// ['&u32', 'Rational', 'Rational', 'add_reuse_rhs', 'no', ['ref', {'convert':
// 'Rational'}], []]
impl Add<Rational> for &u32 {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Self::Output {
        add_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['&u32', '&Rational', 'Rational', 'add_reuse_lhs', 'no', [], []]
impl Add<&Rational> for &u32 {
    type Output = Rational;

    fn add(self, rhs: &Rational) -> Self::Output {
        add_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'i64', 'Rational', 'add_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Add<i64> for Rational {
    type Output = Rational;

    fn add(self, rhs: i64) -> Self::Output {
        add_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['Rational', '&i64', 'Rational', 'add_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Add<&i64> for Rational {
    type Output = Rational;

    fn add(self, rhs: &i64) -> Self::Output {
        add_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['&Rational', 'i64', 'Rational', 'add_reuse_rhs', 'no', [], []]
impl Add<i64> for &Rational {
    type Output = Rational;

    fn add(self, rhs: i64) -> Self::Output {
        add_reuse_rhs(self, rhs)
    }
}

// ['&Rational', '&i64', 'Rational', 'add_reuse_rhs', 'no', [], []]
impl Add<&i64> for &Rational {
    type Output = Rational;

    fn add(self, rhs: &i64) -> Self::Output {
        add_reuse_rhs(self, rhs)
    }
}

// ['i64', 'Rational', 'Rational', 'add_reuse_rhs', 'no', ['ref', {'convert':
// 'Rational'}], []]
impl Add<Rational> for i64 {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Self::Output {
        add_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['i64', '&Rational', 'Rational', 'add_reuse_lhs', 'no', [], []]
impl Add<&Rational> for i64 {
    type Output = Rational;

    fn add(self, rhs: &Rational) -> Self::Output {
        add_reuse_lhs(self, rhs)
    }
}

// ['&i64', 'Rational', 'Rational', 'add_reuse_rhs', 'no', ['ref', {'convert':
// 'Rational'}], []]
impl Add<Rational> for &i64 {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Self::Output {
        add_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['&i64', '&Rational', 'Rational', 'add_reuse_lhs', 'no', [], []]
impl Add<&Rational> for &i64 {
    type Output = Rational;

    fn add(self, rhs: &Rational) -> Self::Output {
        add_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'u64', 'Rational', 'add_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Add<u64> for Rational {
    type Output = Rational;

    fn add(self, rhs: u64) -> Self::Output {
        add_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['Rational', '&u64', 'Rational', 'add_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Add<&u64> for Rational {
    type Output = Rational;

    fn add(self, rhs: &u64) -> Self::Output {
        add_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['&Rational', 'u64', 'Rational', 'add_reuse_rhs', 'no', [], []]
impl Add<u64> for &Rational {
    type Output = Rational;

    fn add(self, rhs: u64) -> Self::Output {
        add_reuse_rhs(self, rhs)
    }
}

// ['&Rational', '&u64', 'Rational', 'add_reuse_rhs', 'no', [], []]
impl Add<&u64> for &Rational {
    type Output = Rational;

    fn add(self, rhs: &u64) -> Self::Output {
        add_reuse_rhs(self, rhs)
    }
}

// ['u64', 'Rational', 'Rational', 'add_reuse_rhs', 'no', ['ref', {'convert':
// 'Rational'}], []]
impl Add<Rational> for u64 {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Self::Output {
        add_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['u64', '&Rational', 'Rational', 'add_reuse_lhs', 'no', [], []]
impl Add<&Rational> for u64 {
    type Output = Rational;

    fn add(self, rhs: &Rational) -> Self::Output {
        add_reuse_lhs(self, rhs)
    }
}

// ['&u64', 'Rational', 'Rational', 'add_reuse_rhs', 'no', ['ref', {'convert':
// 'Rational'}], []]
impl Add<Rational> for &u64 {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Self::Output {
        add_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['&u64', '&Rational', 'Rational', 'add_reuse_lhs', 'no', [], []]
impl Add<&Rational> for &u64 {
    type Output = Rational;

    fn add(self, rhs: &Rational) -> Self::Output {
        add_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'i128', 'Rational', 'add_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Add<i128> for Rational {
    type Output = Rational;

    fn add(self, rhs: i128) -> Self::Output {
        add_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['Rational', '&i128', 'Rational', 'add_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Add<&i128> for Rational {
    type Output = Rational;

    fn add(self, rhs: &i128) -> Self::Output {
        add_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['&Rational', 'i128', 'Rational', 'add_reuse_rhs', 'no', [], []]
impl Add<i128> for &Rational {
    type Output = Rational;

    fn add(self, rhs: i128) -> Self::Output {
        add_reuse_rhs(self, rhs)
    }
}

// ['&Rational', '&i128', 'Rational', 'add_reuse_rhs', 'no', [], []]
impl Add<&i128> for &Rational {
    type Output = Rational;

    fn add(self, rhs: &i128) -> Self::Output {
        add_reuse_rhs(self, rhs)
    }
}

// ['i128', 'Rational', 'Rational', 'add_reuse_rhs', 'no', ['ref', {'convert':
// 'Rational'}], []]
impl Add<Rational> for i128 {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Self::Output {
        add_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['i128', '&Rational', 'Rational', 'add_reuse_lhs', 'no', [], []]
impl Add<&Rational> for i128 {
    type Output = Rational;

    fn add(self, rhs: &Rational) -> Self::Output {
        add_reuse_lhs(self, rhs)
    }
}

// ['&i128', 'Rational', 'Rational', 'add_reuse_rhs', 'no', ['ref', {'convert':
// 'Rational'}], []]
impl Add<Rational> for &i128 {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Self::Output {
        add_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['&i128', '&Rational', 'Rational', 'add_reuse_lhs', 'no', [], []]
impl Add<&Rational> for &i128 {
    type Output = Rational;

    fn add(self, rhs: &Rational) -> Self::Output {
        add_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'u128', 'Rational', 'add_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Add<u128> for Rational {
    type Output = Rational;

    fn add(self, rhs: u128) -> Self::Output {
        add_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['Rational', '&u128', 'Rational', 'add_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Add<&u128> for Rational {
    type Output = Rational;

    fn add(self, rhs: &u128) -> Self::Output {
        add_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['&Rational', 'u128', 'Rational', 'add_reuse_rhs', 'no', [], []]
impl Add<u128> for &Rational {
    type Output = Rational;

    fn add(self, rhs: u128) -> Self::Output {
        add_reuse_rhs(self, rhs)
    }
}

// ['&Rational', '&u128', 'Rational', 'add_reuse_rhs', 'no', [], []]
impl Add<&u128> for &Rational {
    type Output = Rational;

    fn add(self, rhs: &u128) -> Self::Output {
        add_reuse_rhs(self, rhs)
    }
}

// ['u128', 'Rational', 'Rational', 'add_reuse_rhs', 'no', ['ref', {'convert':
// 'Rational'}], []]
impl Add<Rational> for u128 {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Self::Output {
        add_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['u128', '&Rational', 'Rational', 'add_reuse_lhs', 'no', [], []]
impl Add<&Rational> for u128 {
    type Output = Rational;

    fn add(self, rhs: &Rational) -> Self::Output {
        add_reuse_lhs(self, rhs)
    }
}

// ['&u128', 'Rational', 'Rational', 'add_reuse_rhs', 'no', ['ref', {'convert':
// 'Rational'}], []]
impl Add<Rational> for &u128 {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Self::Output {
        add_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['&u128', '&Rational', 'Rational', 'add_reuse_lhs', 'no', [], []]
impl Add<&Rational> for &u128 {
    type Output = Rational;

    fn add(self, rhs: &Rational) -> Self::Output {
        add_reuse_lhs(self, rhs)
    }
}
