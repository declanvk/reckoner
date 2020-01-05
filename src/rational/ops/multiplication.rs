use super::helpers::{multiply_reuse_lhs, multiply_reuse_rhs};
use crate::{integer::Integer, rational::Rational};
use core::ops::Mul;

// Mul          The multiplication operator *.

// ['Rational', 'Rational', 'Rational', 'multiply_reuse_lhs', 'no', [], ['ref']]
impl Mul<Rational> for Rational {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Self::Output {
        multiply_reuse_lhs(self, &rhs)
    }
}

// ['Rational', '&Rational', 'Rational', 'multiply_reuse_lhs', 'no', [], []]
impl Mul<&Rational> for Rational {
    type Output = Rational;

    fn mul(self, rhs: &Rational) -> Self::Output {
        multiply_reuse_lhs(self, rhs)
    }
}

// ['&Rational', 'Rational', 'Rational', 'multiply_reuse_rhs', 'no', [], []]
impl Mul<Rational> for &Rational {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Self::Output {
        multiply_reuse_rhs(self, rhs)
    }
}

// ['&Rational', '&Rational', 'Rational', 'Rational::multiply', 'no', [], []]
impl Mul<&Rational> for &Rational {
    type Output = Rational;

    fn mul(self, rhs: &Rational) -> Self::Output {
        Rational::multiply(self, rhs)
    }
}

// ['Rational', 'Integer', 'Rational', 'multiply_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Mul<Integer> for Rational {
    type Output = Rational;

    fn mul(self, rhs: Integer) -> Self::Output {
        multiply_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['Rational', '&Integer', 'Rational', 'multiply_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Mul<&Integer> for Rational {
    type Output = Rational;

    fn mul(self, rhs: &Integer) -> Self::Output {
        multiply_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['&Rational', 'Integer', 'Rational', 'multiply_reuse_rhs', 'no', [], []]
impl Mul<Integer> for &Rational {
    type Output = Rational;

    fn mul(self, rhs: Integer) -> Self::Output {
        multiply_reuse_rhs(self, rhs)
    }
}

// ['&Rational', '&Integer', 'Rational', 'multiply_reuse_rhs', 'no', [], []]
impl Mul<&Integer> for &Rational {
    type Output = Rational;

    fn mul(self, rhs: &Integer) -> Self::Output {
        multiply_reuse_rhs(self, rhs)
    }
}

// ['Integer', 'Rational', 'Rational', 'multiply_reuse_rhs', 'no', ['ref',
// {'convert': 'Rational'}], []]
impl Mul<Rational> for Integer {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Self::Output {
        multiply_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['Integer', '&Rational', 'Rational', 'multiply_reuse_lhs', 'no', [], []]
impl Mul<&Rational> for Integer {
    type Output = Rational;

    fn mul(self, rhs: &Rational) -> Self::Output {
        multiply_reuse_lhs(self, rhs)
    }
}

// ['&Integer', 'Rational', 'Rational', 'multiply_reuse_rhs', 'no', ['ref',
// {'convert': 'Rational'}], []]
impl Mul<Rational> for &Integer {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Self::Output {
        multiply_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['&Integer', '&Rational', 'Rational', 'multiply_reuse_lhs', 'no', [], []]
impl Mul<&Rational> for &Integer {
    type Output = Rational;

    fn mul(self, rhs: &Rational) -> Self::Output {
        multiply_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'i8', 'Rational', 'multiply_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Mul<i8> for Rational {
    type Output = Rational;

    fn mul(self, rhs: i8) -> Self::Output {
        multiply_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['Rational', '&i8', 'Rational', 'multiply_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Mul<&i8> for Rational {
    type Output = Rational;

    fn mul(self, rhs: &i8) -> Self::Output {
        multiply_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['&Rational', 'i8', 'Rational', 'multiply_reuse_rhs', 'no', [], []]
impl Mul<i8> for &Rational {
    type Output = Rational;

    fn mul(self, rhs: i8) -> Self::Output {
        multiply_reuse_rhs(self, rhs)
    }
}

// ['&Rational', '&i8', 'Rational', 'multiply_reuse_rhs', 'no', [], []]
impl Mul<&i8> for &Rational {
    type Output = Rational;

    fn mul(self, rhs: &i8) -> Self::Output {
        multiply_reuse_rhs(self, rhs)
    }
}

// ['i8', 'Rational', 'Rational', 'multiply_reuse_rhs', 'no', ['ref',
// {'convert': 'Rational'}], []]
impl Mul<Rational> for i8 {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Self::Output {
        multiply_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['i8', '&Rational', 'Rational', 'multiply_reuse_lhs', 'no', [], []]
impl Mul<&Rational> for i8 {
    type Output = Rational;

    fn mul(self, rhs: &Rational) -> Self::Output {
        multiply_reuse_lhs(self, rhs)
    }
}

// ['&i8', 'Rational', 'Rational', 'multiply_reuse_rhs', 'no', ['ref',
// {'convert': 'Rational'}], []]
impl Mul<Rational> for &i8 {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Self::Output {
        multiply_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['&i8', '&Rational', 'Rational', 'multiply_reuse_lhs', 'no', [], []]
impl Mul<&Rational> for &i8 {
    type Output = Rational;

    fn mul(self, rhs: &Rational) -> Self::Output {
        multiply_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'u8', 'Rational', 'multiply_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Mul<u8> for Rational {
    type Output = Rational;

    fn mul(self, rhs: u8) -> Self::Output {
        multiply_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['Rational', '&u8', 'Rational', 'multiply_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Mul<&u8> for Rational {
    type Output = Rational;

    fn mul(self, rhs: &u8) -> Self::Output {
        multiply_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['&Rational', 'u8', 'Rational', 'multiply_reuse_rhs', 'no', [], []]
impl Mul<u8> for &Rational {
    type Output = Rational;

    fn mul(self, rhs: u8) -> Self::Output {
        multiply_reuse_rhs(self, rhs)
    }
}

// ['&Rational', '&u8', 'Rational', 'multiply_reuse_rhs', 'no', [], []]
impl Mul<&u8> for &Rational {
    type Output = Rational;

    fn mul(self, rhs: &u8) -> Self::Output {
        multiply_reuse_rhs(self, rhs)
    }
}

// ['u8', 'Rational', 'Rational', 'multiply_reuse_rhs', 'no', ['ref',
// {'convert': 'Rational'}], []]
impl Mul<Rational> for u8 {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Self::Output {
        multiply_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['u8', '&Rational', 'Rational', 'multiply_reuse_lhs', 'no', [], []]
impl Mul<&Rational> for u8 {
    type Output = Rational;

    fn mul(self, rhs: &Rational) -> Self::Output {
        multiply_reuse_lhs(self, rhs)
    }
}

// ['&u8', 'Rational', 'Rational', 'multiply_reuse_rhs', 'no', ['ref',
// {'convert': 'Rational'}], []]
impl Mul<Rational> for &u8 {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Self::Output {
        multiply_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['&u8', '&Rational', 'Rational', 'multiply_reuse_lhs', 'no', [], []]
impl Mul<&Rational> for &u8 {
    type Output = Rational;

    fn mul(self, rhs: &Rational) -> Self::Output {
        multiply_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'i16', 'Rational', 'multiply_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Mul<i16> for Rational {
    type Output = Rational;

    fn mul(self, rhs: i16) -> Self::Output {
        multiply_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['Rational', '&i16', 'Rational', 'multiply_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Mul<&i16> for Rational {
    type Output = Rational;

    fn mul(self, rhs: &i16) -> Self::Output {
        multiply_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['&Rational', 'i16', 'Rational', 'multiply_reuse_rhs', 'no', [], []]
impl Mul<i16> for &Rational {
    type Output = Rational;

    fn mul(self, rhs: i16) -> Self::Output {
        multiply_reuse_rhs(self, rhs)
    }
}

// ['&Rational', '&i16', 'Rational', 'multiply_reuse_rhs', 'no', [], []]
impl Mul<&i16> for &Rational {
    type Output = Rational;

    fn mul(self, rhs: &i16) -> Self::Output {
        multiply_reuse_rhs(self, rhs)
    }
}

// ['i16', 'Rational', 'Rational', 'multiply_reuse_rhs', 'no', ['ref',
// {'convert': 'Rational'}], []]
impl Mul<Rational> for i16 {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Self::Output {
        multiply_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['i16', '&Rational', 'Rational', 'multiply_reuse_lhs', 'no', [], []]
impl Mul<&Rational> for i16 {
    type Output = Rational;

    fn mul(self, rhs: &Rational) -> Self::Output {
        multiply_reuse_lhs(self, rhs)
    }
}

// ['&i16', 'Rational', 'Rational', 'multiply_reuse_rhs', 'no', ['ref',
// {'convert': 'Rational'}], []]
impl Mul<Rational> for &i16 {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Self::Output {
        multiply_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['&i16', '&Rational', 'Rational', 'multiply_reuse_lhs', 'no', [], []]
impl Mul<&Rational> for &i16 {
    type Output = Rational;

    fn mul(self, rhs: &Rational) -> Self::Output {
        multiply_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'u16', 'Rational', 'multiply_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Mul<u16> for Rational {
    type Output = Rational;

    fn mul(self, rhs: u16) -> Self::Output {
        multiply_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['Rational', '&u16', 'Rational', 'multiply_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Mul<&u16> for Rational {
    type Output = Rational;

    fn mul(self, rhs: &u16) -> Self::Output {
        multiply_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['&Rational', 'u16', 'Rational', 'multiply_reuse_rhs', 'no', [], []]
impl Mul<u16> for &Rational {
    type Output = Rational;

    fn mul(self, rhs: u16) -> Self::Output {
        multiply_reuse_rhs(self, rhs)
    }
}

// ['&Rational', '&u16', 'Rational', 'multiply_reuse_rhs', 'no', [], []]
impl Mul<&u16> for &Rational {
    type Output = Rational;

    fn mul(self, rhs: &u16) -> Self::Output {
        multiply_reuse_rhs(self, rhs)
    }
}

// ['u16', 'Rational', 'Rational', 'multiply_reuse_rhs', 'no', ['ref',
// {'convert': 'Rational'}], []]
impl Mul<Rational> for u16 {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Self::Output {
        multiply_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['u16', '&Rational', 'Rational', 'multiply_reuse_lhs', 'no', [], []]
impl Mul<&Rational> for u16 {
    type Output = Rational;

    fn mul(self, rhs: &Rational) -> Self::Output {
        multiply_reuse_lhs(self, rhs)
    }
}

// ['&u16', 'Rational', 'Rational', 'multiply_reuse_rhs', 'no', ['ref',
// {'convert': 'Rational'}], []]
impl Mul<Rational> for &u16 {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Self::Output {
        multiply_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['&u16', '&Rational', 'Rational', 'multiply_reuse_lhs', 'no', [], []]
impl Mul<&Rational> for &u16 {
    type Output = Rational;

    fn mul(self, rhs: &Rational) -> Self::Output {
        multiply_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'i32', 'Rational', 'multiply_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Mul<i32> for Rational {
    type Output = Rational;

    fn mul(self, rhs: i32) -> Self::Output {
        multiply_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['Rational', '&i32', 'Rational', 'multiply_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Mul<&i32> for Rational {
    type Output = Rational;

    fn mul(self, rhs: &i32) -> Self::Output {
        multiply_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['&Rational', 'i32', 'Rational', 'multiply_reuse_rhs', 'no', [], []]
impl Mul<i32> for &Rational {
    type Output = Rational;

    fn mul(self, rhs: i32) -> Self::Output {
        multiply_reuse_rhs(self, rhs)
    }
}

// ['&Rational', '&i32', 'Rational', 'multiply_reuse_rhs', 'no', [], []]
impl Mul<&i32> for &Rational {
    type Output = Rational;

    fn mul(self, rhs: &i32) -> Self::Output {
        multiply_reuse_rhs(self, rhs)
    }
}

// ['i32', 'Rational', 'Rational', 'multiply_reuse_rhs', 'no', ['ref',
// {'convert': 'Rational'}], []]
impl Mul<Rational> for i32 {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Self::Output {
        multiply_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['i32', '&Rational', 'Rational', 'multiply_reuse_lhs', 'no', [], []]
impl Mul<&Rational> for i32 {
    type Output = Rational;

    fn mul(self, rhs: &Rational) -> Self::Output {
        multiply_reuse_lhs(self, rhs)
    }
}

// ['&i32', 'Rational', 'Rational', 'multiply_reuse_rhs', 'no', ['ref',
// {'convert': 'Rational'}], []]
impl Mul<Rational> for &i32 {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Self::Output {
        multiply_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['&i32', '&Rational', 'Rational', 'multiply_reuse_lhs', 'no', [], []]
impl Mul<&Rational> for &i32 {
    type Output = Rational;

    fn mul(self, rhs: &Rational) -> Self::Output {
        multiply_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'u32', 'Rational', 'multiply_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Mul<u32> for Rational {
    type Output = Rational;

    fn mul(self, rhs: u32) -> Self::Output {
        multiply_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['Rational', '&u32', 'Rational', 'multiply_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Mul<&u32> for Rational {
    type Output = Rational;

    fn mul(self, rhs: &u32) -> Self::Output {
        multiply_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['&Rational', 'u32', 'Rational', 'multiply_reuse_rhs', 'no', [], []]
impl Mul<u32> for &Rational {
    type Output = Rational;

    fn mul(self, rhs: u32) -> Self::Output {
        multiply_reuse_rhs(self, rhs)
    }
}

// ['&Rational', '&u32', 'Rational', 'multiply_reuse_rhs', 'no', [], []]
impl Mul<&u32> for &Rational {
    type Output = Rational;

    fn mul(self, rhs: &u32) -> Self::Output {
        multiply_reuse_rhs(self, rhs)
    }
}

// ['u32', 'Rational', 'Rational', 'multiply_reuse_rhs', 'no', ['ref',
// {'convert': 'Rational'}], []]
impl Mul<Rational> for u32 {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Self::Output {
        multiply_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['u32', '&Rational', 'Rational', 'multiply_reuse_lhs', 'no', [], []]
impl Mul<&Rational> for u32 {
    type Output = Rational;

    fn mul(self, rhs: &Rational) -> Self::Output {
        multiply_reuse_lhs(self, rhs)
    }
}

// ['&u32', 'Rational', 'Rational', 'multiply_reuse_rhs', 'no', ['ref',
// {'convert': 'Rational'}], []]
impl Mul<Rational> for &u32 {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Self::Output {
        multiply_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['&u32', '&Rational', 'Rational', 'multiply_reuse_lhs', 'no', [], []]
impl Mul<&Rational> for &u32 {
    type Output = Rational;

    fn mul(self, rhs: &Rational) -> Self::Output {
        multiply_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'i64', 'Rational', 'multiply_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Mul<i64> for Rational {
    type Output = Rational;

    fn mul(self, rhs: i64) -> Self::Output {
        multiply_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['Rational', '&i64', 'Rational', 'multiply_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Mul<&i64> for Rational {
    type Output = Rational;

    fn mul(self, rhs: &i64) -> Self::Output {
        multiply_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['&Rational', 'i64', 'Rational', 'multiply_reuse_rhs', 'no', [], []]
impl Mul<i64> for &Rational {
    type Output = Rational;

    fn mul(self, rhs: i64) -> Self::Output {
        multiply_reuse_rhs(self, rhs)
    }
}

// ['&Rational', '&i64', 'Rational', 'multiply_reuse_rhs', 'no', [], []]
impl Mul<&i64> for &Rational {
    type Output = Rational;

    fn mul(self, rhs: &i64) -> Self::Output {
        multiply_reuse_rhs(self, rhs)
    }
}

// ['i64', 'Rational', 'Rational', 'multiply_reuse_rhs', 'no', ['ref',
// {'convert': 'Rational'}], []]
impl Mul<Rational> for i64 {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Self::Output {
        multiply_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['i64', '&Rational', 'Rational', 'multiply_reuse_lhs', 'no', [], []]
impl Mul<&Rational> for i64 {
    type Output = Rational;

    fn mul(self, rhs: &Rational) -> Self::Output {
        multiply_reuse_lhs(self, rhs)
    }
}

// ['&i64', 'Rational', 'Rational', 'multiply_reuse_rhs', 'no', ['ref',
// {'convert': 'Rational'}], []]
impl Mul<Rational> for &i64 {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Self::Output {
        multiply_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['&i64', '&Rational', 'Rational', 'multiply_reuse_lhs', 'no', [], []]
impl Mul<&Rational> for &i64 {
    type Output = Rational;

    fn mul(self, rhs: &Rational) -> Self::Output {
        multiply_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'u64', 'Rational', 'multiply_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Mul<u64> for Rational {
    type Output = Rational;

    fn mul(self, rhs: u64) -> Self::Output {
        multiply_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['Rational', '&u64', 'Rational', 'multiply_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Mul<&u64> for Rational {
    type Output = Rational;

    fn mul(self, rhs: &u64) -> Self::Output {
        multiply_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['&Rational', 'u64', 'Rational', 'multiply_reuse_rhs', 'no', [], []]
impl Mul<u64> for &Rational {
    type Output = Rational;

    fn mul(self, rhs: u64) -> Self::Output {
        multiply_reuse_rhs(self, rhs)
    }
}

// ['&Rational', '&u64', 'Rational', 'multiply_reuse_rhs', 'no', [], []]
impl Mul<&u64> for &Rational {
    type Output = Rational;

    fn mul(self, rhs: &u64) -> Self::Output {
        multiply_reuse_rhs(self, rhs)
    }
}

// ['u64', 'Rational', 'Rational', 'multiply_reuse_rhs', 'no', ['ref',
// {'convert': 'Rational'}], []]
impl Mul<Rational> for u64 {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Self::Output {
        multiply_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['u64', '&Rational', 'Rational', 'multiply_reuse_lhs', 'no', [], []]
impl Mul<&Rational> for u64 {
    type Output = Rational;

    fn mul(self, rhs: &Rational) -> Self::Output {
        multiply_reuse_lhs(self, rhs)
    }
}

// ['&u64', 'Rational', 'Rational', 'multiply_reuse_rhs', 'no', ['ref',
// {'convert': 'Rational'}], []]
impl Mul<Rational> for &u64 {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Self::Output {
        multiply_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['&u64', '&Rational', 'Rational', 'multiply_reuse_lhs', 'no', [], []]
impl Mul<&Rational> for &u64 {
    type Output = Rational;

    fn mul(self, rhs: &Rational) -> Self::Output {
        multiply_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'i128', 'Rational', 'multiply_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Mul<i128> for Rational {
    type Output = Rational;

    fn mul(self, rhs: i128) -> Self::Output {
        multiply_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['Rational', '&i128', 'Rational', 'multiply_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Mul<&i128> for Rational {
    type Output = Rational;

    fn mul(self, rhs: &i128) -> Self::Output {
        multiply_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['&Rational', 'i128', 'Rational', 'multiply_reuse_rhs', 'no', [], []]
impl Mul<i128> for &Rational {
    type Output = Rational;

    fn mul(self, rhs: i128) -> Self::Output {
        multiply_reuse_rhs(self, rhs)
    }
}

// ['&Rational', '&i128', 'Rational', 'multiply_reuse_rhs', 'no', [], []]
impl Mul<&i128> for &Rational {
    type Output = Rational;

    fn mul(self, rhs: &i128) -> Self::Output {
        multiply_reuse_rhs(self, rhs)
    }
}

// ['i128', 'Rational', 'Rational', 'multiply_reuse_rhs', 'no', ['ref',
// {'convert': 'Rational'}], []]
impl Mul<Rational> for i128 {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Self::Output {
        multiply_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['i128', '&Rational', 'Rational', 'multiply_reuse_lhs', 'no', [], []]
impl Mul<&Rational> for i128 {
    type Output = Rational;

    fn mul(self, rhs: &Rational) -> Self::Output {
        multiply_reuse_lhs(self, rhs)
    }
}

// ['&i128', 'Rational', 'Rational', 'multiply_reuse_rhs', 'no', ['ref',
// {'convert': 'Rational'}], []]
impl Mul<Rational> for &i128 {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Self::Output {
        multiply_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['&i128', '&Rational', 'Rational', 'multiply_reuse_lhs', 'no', [], []]
impl Mul<&Rational> for &i128 {
    type Output = Rational;

    fn mul(self, rhs: &Rational) -> Self::Output {
        multiply_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'u128', 'Rational', 'multiply_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Mul<u128> for Rational {
    type Output = Rational;

    fn mul(self, rhs: u128) -> Self::Output {
        multiply_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['Rational', '&u128', 'Rational', 'multiply_reuse_lhs', 'no', [], ['ref',
// {'convert': 'Rational'}]]
impl Mul<&u128> for Rational {
    type Output = Rational;

    fn mul(self, rhs: &u128) -> Self::Output {
        multiply_reuse_lhs(self, &Rational::from(rhs))
    }
}

// ['&Rational', 'u128', 'Rational', 'multiply_reuse_rhs', 'no', [], []]
impl Mul<u128> for &Rational {
    type Output = Rational;

    fn mul(self, rhs: u128) -> Self::Output {
        multiply_reuse_rhs(self, rhs)
    }
}

// ['&Rational', '&u128', 'Rational', 'multiply_reuse_rhs', 'no', [], []]
impl Mul<&u128> for &Rational {
    type Output = Rational;

    fn mul(self, rhs: &u128) -> Self::Output {
        multiply_reuse_rhs(self, rhs)
    }
}

// ['u128', 'Rational', 'Rational', 'multiply_reuse_rhs', 'no', ['ref',
// {'convert': 'Rational'}], []]
impl Mul<Rational> for u128 {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Self::Output {
        multiply_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['u128', '&Rational', 'Rational', 'multiply_reuse_lhs', 'no', [], []]
impl Mul<&Rational> for u128 {
    type Output = Rational;

    fn mul(self, rhs: &Rational) -> Self::Output {
        multiply_reuse_lhs(self, rhs)
    }
}

// ['&u128', 'Rational', 'Rational', 'multiply_reuse_rhs', 'no', ['ref',
// {'convert': 'Rational'}], []]
impl Mul<Rational> for &u128 {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Self::Output {
        multiply_reuse_rhs(&Rational::from(self), rhs)
    }
}

// ['&u128', '&Rational', 'Rational', 'multiply_reuse_lhs', 'no', [], []]
impl Mul<&Rational> for &u128 {
    type Output = Rational;

    fn mul(self, rhs: &Rational) -> Self::Output {
        multiply_reuse_lhs(self, rhs)
    }
}
