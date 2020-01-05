use super::helpers::subtract_reuse_lhs;
use crate::{integer::Integer, rational::Rational};
use core::ops::Sub;

// Sub          The subtraction operator -.

// ['Rational', 'Rational', 'Rational', 'Rational::subtract_assign', 'lhs',
// ['ref_mut'], ['ref']]
impl Sub<Rational> for Rational {
    type Output = Rational;

    fn sub(mut self, rhs: Rational) -> Self::Output {
        Rational::subtract_assign(&mut self, &rhs);
        self
    }
}

// ['Rational', '&Rational', 'Rational', 'Rational::subtract_assign', 'lhs',
// ['ref_mut'], []]
impl Sub<&Rational> for Rational {
    type Output = Rational;

    fn sub(mut self, rhs: &Rational) -> Self::Output {
        Rational::subtract_assign(&mut self, rhs);
        self
    }
}

// ['&Rational', 'Rational', 'Rational', 'Rational::subtract', 'no', [],
// ['ref']]
impl Sub<Rational> for &Rational {
    type Output = Rational;

    fn sub(self, rhs: Rational) -> Self::Output {
        Rational::subtract(self, &rhs)
    }
}

// ['&Rational', '&Rational', 'Rational', 'Rational::subtract', 'no', [], []]
impl Sub<&Rational> for &Rational {
    type Output = Rational;

    fn sub(self, rhs: &Rational) -> Self::Output {
        Rational::subtract(self, rhs)
    }
}

// ['Rational', 'Integer', 'Rational', 'Rational::subtract_assign_integer',
// 'lhs', ['ref_mut'], ['ref']]
impl Sub<Integer> for Rational {
    type Output = Rational;

    fn sub(mut self, rhs: Integer) -> Self::Output {
        Rational::subtract_assign_integer(&mut self, &rhs);
        self
    }
}

// ['Rational', '&Integer', 'Rational', 'Rational::subtract_assign_integer',
// 'lhs', ['ref_mut'], []]
impl Sub<&Integer> for Rational {
    type Output = Rational;

    fn sub(mut self, rhs: &Integer) -> Self::Output {
        Rational::subtract_assign_integer(&mut self, rhs);
        self
    }
}

// ['&Rational', 'Integer', 'Rational', 'Rational::subtract_integer', 'no', [],
// ['ref']]
impl Sub<Integer> for &Rational {
    type Output = Rational;

    fn sub(self, rhs: Integer) -> Self::Output {
        Rational::subtract_integer(self, &rhs)
    }
}

// ['&Rational', '&Integer', 'Rational', 'Rational::subtract_integer', 'no', [],
// []]
impl Sub<&Integer> for &Rational {
    type Output = Rational;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Rational::subtract_integer(self, rhs)
    }
}

// ['Integer', 'Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], ['ref']]
impl Sub<Rational> for Integer {
    type Output = Rational;

    fn sub(self, rhs: Rational) -> Self::Output {
        subtract_reuse_lhs(self, &rhs)
    }
}

// ['Integer', '&Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], []]
impl Sub<&Rational> for Integer {
    type Output = Rational;

    fn sub(self, rhs: &Rational) -> Self::Output {
        subtract_reuse_lhs(self, rhs)
    }
}

// ['&Integer', 'Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], ['ref']]
impl Sub<Rational> for &Integer {
    type Output = Rational;

    fn sub(self, rhs: Rational) -> Self::Output {
        subtract_reuse_lhs(self, &rhs)
    }
}

// ['&Integer', '&Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], []]
impl Sub<&Rational> for &Integer {
    type Output = Rational;

    fn sub(self, rhs: &Rational) -> Self::Output {
        subtract_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'i8', 'Rational', 'Rational::subtract_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}]]
impl Sub<i8> for Rational {
    type Output = Rational;

    fn sub(mut self, rhs: i8) -> Self::Output {
        Rational::subtract_assign_integer(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Rational', '&i8', 'Rational', 'Rational::subtract_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}, 'deref']]
impl Sub<&i8> for Rational {
    type Output = Rational;

    fn sub(mut self, rhs: &i8) -> Self::Output {
        Rational::subtract_assign_integer(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Rational', 'i8', 'Rational', 'Rational::subtract_integer', 'no', [],
// ['ref', {'convert': 'Integer'}]]
impl Sub<i8> for &Rational {
    type Output = Rational;

    fn sub(self, rhs: i8) -> Self::Output {
        Rational::subtract_integer(self, &Integer::from(rhs))
    }
}

// ['&Rational', '&i8', 'Rational', 'Rational::subtract_integer', 'no', [],
// ['ref', {'convert': 'Integer'}, 'deref']]
impl Sub<&i8> for &Rational {
    type Output = Rational;

    fn sub(self, rhs: &i8) -> Self::Output {
        Rational::subtract_integer(self, &Integer::from(*rhs))
    }
}

// ['i8', 'Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], ['ref']]
impl Sub<Rational> for i8 {
    type Output = Rational;

    fn sub(self, rhs: Rational) -> Self::Output {
        subtract_reuse_lhs(self, &rhs)
    }
}

// ['i8', '&Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], []]
impl Sub<&Rational> for i8 {
    type Output = Rational;

    fn sub(self, rhs: &Rational) -> Self::Output {
        subtract_reuse_lhs(self, rhs)
    }
}

// ['&i8', 'Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], ['ref']]
impl Sub<Rational> for &i8 {
    type Output = Rational;

    fn sub(self, rhs: Rational) -> Self::Output {
        subtract_reuse_lhs(self, &rhs)
    }
}

// ['&i8', '&Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], []]
impl Sub<&Rational> for &i8 {
    type Output = Rational;

    fn sub(self, rhs: &Rational) -> Self::Output {
        subtract_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'u8', 'Rational', 'Rational::subtract_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}]]
impl Sub<u8> for Rational {
    type Output = Rational;

    fn sub(mut self, rhs: u8) -> Self::Output {
        Rational::subtract_assign_integer(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Rational', '&u8', 'Rational', 'Rational::subtract_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}, 'deref']]
impl Sub<&u8> for Rational {
    type Output = Rational;

    fn sub(mut self, rhs: &u8) -> Self::Output {
        Rational::subtract_assign_integer(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Rational', 'u8', 'Rational', 'Rational::subtract_integer', 'no', [],
// ['ref', {'convert': 'Integer'}]]
impl Sub<u8> for &Rational {
    type Output = Rational;

    fn sub(self, rhs: u8) -> Self::Output {
        Rational::subtract_integer(self, &Integer::from(rhs))
    }
}

// ['&Rational', '&u8', 'Rational', 'Rational::subtract_integer', 'no', [],
// ['ref', {'convert': 'Integer'}, 'deref']]
impl Sub<&u8> for &Rational {
    type Output = Rational;

    fn sub(self, rhs: &u8) -> Self::Output {
        Rational::subtract_integer(self, &Integer::from(*rhs))
    }
}

// ['u8', 'Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], ['ref']]
impl Sub<Rational> for u8 {
    type Output = Rational;

    fn sub(self, rhs: Rational) -> Self::Output {
        subtract_reuse_lhs(self, &rhs)
    }
}

// ['u8', '&Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], []]
impl Sub<&Rational> for u8 {
    type Output = Rational;

    fn sub(self, rhs: &Rational) -> Self::Output {
        subtract_reuse_lhs(self, rhs)
    }
}

// ['&u8', 'Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], ['ref']]
impl Sub<Rational> for &u8 {
    type Output = Rational;

    fn sub(self, rhs: Rational) -> Self::Output {
        subtract_reuse_lhs(self, &rhs)
    }
}

// ['&u8', '&Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], []]
impl Sub<&Rational> for &u8 {
    type Output = Rational;

    fn sub(self, rhs: &Rational) -> Self::Output {
        subtract_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'i16', 'Rational', 'Rational::subtract_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}]]
impl Sub<i16> for Rational {
    type Output = Rational;

    fn sub(mut self, rhs: i16) -> Self::Output {
        Rational::subtract_assign_integer(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Rational', '&i16', 'Rational', 'Rational::subtract_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}, 'deref']]
impl Sub<&i16> for Rational {
    type Output = Rational;

    fn sub(mut self, rhs: &i16) -> Self::Output {
        Rational::subtract_assign_integer(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Rational', 'i16', 'Rational', 'Rational::subtract_integer', 'no', [],
// ['ref', {'convert': 'Integer'}]]
impl Sub<i16> for &Rational {
    type Output = Rational;

    fn sub(self, rhs: i16) -> Self::Output {
        Rational::subtract_integer(self, &Integer::from(rhs))
    }
}

// ['&Rational', '&i16', 'Rational', 'Rational::subtract_integer', 'no', [],
// ['ref', {'convert': 'Integer'}, 'deref']]
impl Sub<&i16> for &Rational {
    type Output = Rational;

    fn sub(self, rhs: &i16) -> Self::Output {
        Rational::subtract_integer(self, &Integer::from(*rhs))
    }
}

// ['i16', 'Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], ['ref']]
impl Sub<Rational> for i16 {
    type Output = Rational;

    fn sub(self, rhs: Rational) -> Self::Output {
        subtract_reuse_lhs(self, &rhs)
    }
}

// ['i16', '&Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], []]
impl Sub<&Rational> for i16 {
    type Output = Rational;

    fn sub(self, rhs: &Rational) -> Self::Output {
        subtract_reuse_lhs(self, rhs)
    }
}

// ['&i16', 'Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], ['ref']]
impl Sub<Rational> for &i16 {
    type Output = Rational;

    fn sub(self, rhs: Rational) -> Self::Output {
        subtract_reuse_lhs(self, &rhs)
    }
}

// ['&i16', '&Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], []]
impl Sub<&Rational> for &i16 {
    type Output = Rational;

    fn sub(self, rhs: &Rational) -> Self::Output {
        subtract_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'u16', 'Rational', 'Rational::subtract_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}]]
impl Sub<u16> for Rational {
    type Output = Rational;

    fn sub(mut self, rhs: u16) -> Self::Output {
        Rational::subtract_assign_integer(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Rational', '&u16', 'Rational', 'Rational::subtract_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}, 'deref']]
impl Sub<&u16> for Rational {
    type Output = Rational;

    fn sub(mut self, rhs: &u16) -> Self::Output {
        Rational::subtract_assign_integer(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Rational', 'u16', 'Rational', 'Rational::subtract_integer', 'no', [],
// ['ref', {'convert': 'Integer'}]]
impl Sub<u16> for &Rational {
    type Output = Rational;

    fn sub(self, rhs: u16) -> Self::Output {
        Rational::subtract_integer(self, &Integer::from(rhs))
    }
}

// ['&Rational', '&u16', 'Rational', 'Rational::subtract_integer', 'no', [],
// ['ref', {'convert': 'Integer'}, 'deref']]
impl Sub<&u16> for &Rational {
    type Output = Rational;

    fn sub(self, rhs: &u16) -> Self::Output {
        Rational::subtract_integer(self, &Integer::from(*rhs))
    }
}

// ['u16', 'Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], ['ref']]
impl Sub<Rational> for u16 {
    type Output = Rational;

    fn sub(self, rhs: Rational) -> Self::Output {
        subtract_reuse_lhs(self, &rhs)
    }
}

// ['u16', '&Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], []]
impl Sub<&Rational> for u16 {
    type Output = Rational;

    fn sub(self, rhs: &Rational) -> Self::Output {
        subtract_reuse_lhs(self, rhs)
    }
}

// ['&u16', 'Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], ['ref']]
impl Sub<Rational> for &u16 {
    type Output = Rational;

    fn sub(self, rhs: Rational) -> Self::Output {
        subtract_reuse_lhs(self, &rhs)
    }
}

// ['&u16', '&Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], []]
impl Sub<&Rational> for &u16 {
    type Output = Rational;

    fn sub(self, rhs: &Rational) -> Self::Output {
        subtract_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'i32', 'Rational', 'Rational::subtract_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}]]
impl Sub<i32> for Rational {
    type Output = Rational;

    fn sub(mut self, rhs: i32) -> Self::Output {
        Rational::subtract_assign_integer(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Rational', '&i32', 'Rational', 'Rational::subtract_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}, 'deref']]
impl Sub<&i32> for Rational {
    type Output = Rational;

    fn sub(mut self, rhs: &i32) -> Self::Output {
        Rational::subtract_assign_integer(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Rational', 'i32', 'Rational', 'Rational::subtract_integer', 'no', [],
// ['ref', {'convert': 'Integer'}]]
impl Sub<i32> for &Rational {
    type Output = Rational;

    fn sub(self, rhs: i32) -> Self::Output {
        Rational::subtract_integer(self, &Integer::from(rhs))
    }
}

// ['&Rational', '&i32', 'Rational', 'Rational::subtract_integer', 'no', [],
// ['ref', {'convert': 'Integer'}, 'deref']]
impl Sub<&i32> for &Rational {
    type Output = Rational;

    fn sub(self, rhs: &i32) -> Self::Output {
        Rational::subtract_integer(self, &Integer::from(*rhs))
    }
}

// ['i32', 'Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], ['ref']]
impl Sub<Rational> for i32 {
    type Output = Rational;

    fn sub(self, rhs: Rational) -> Self::Output {
        subtract_reuse_lhs(self, &rhs)
    }
}

// ['i32', '&Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], []]
impl Sub<&Rational> for i32 {
    type Output = Rational;

    fn sub(self, rhs: &Rational) -> Self::Output {
        subtract_reuse_lhs(self, rhs)
    }
}

// ['&i32', 'Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], ['ref']]
impl Sub<Rational> for &i32 {
    type Output = Rational;

    fn sub(self, rhs: Rational) -> Self::Output {
        subtract_reuse_lhs(self, &rhs)
    }
}

// ['&i32', '&Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], []]
impl Sub<&Rational> for &i32 {
    type Output = Rational;

    fn sub(self, rhs: &Rational) -> Self::Output {
        subtract_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'u32', 'Rational', 'Rational::subtract_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}]]
impl Sub<u32> for Rational {
    type Output = Rational;

    fn sub(mut self, rhs: u32) -> Self::Output {
        Rational::subtract_assign_integer(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Rational', '&u32', 'Rational', 'Rational::subtract_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}, 'deref']]
impl Sub<&u32> for Rational {
    type Output = Rational;

    fn sub(mut self, rhs: &u32) -> Self::Output {
        Rational::subtract_assign_integer(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Rational', 'u32', 'Rational', 'Rational::subtract_integer', 'no', [],
// ['ref', {'convert': 'Integer'}]]
impl Sub<u32> for &Rational {
    type Output = Rational;

    fn sub(self, rhs: u32) -> Self::Output {
        Rational::subtract_integer(self, &Integer::from(rhs))
    }
}

// ['&Rational', '&u32', 'Rational', 'Rational::subtract_integer', 'no', [],
// ['ref', {'convert': 'Integer'}, 'deref']]
impl Sub<&u32> for &Rational {
    type Output = Rational;

    fn sub(self, rhs: &u32) -> Self::Output {
        Rational::subtract_integer(self, &Integer::from(*rhs))
    }
}

// ['u32', 'Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], ['ref']]
impl Sub<Rational> for u32 {
    type Output = Rational;

    fn sub(self, rhs: Rational) -> Self::Output {
        subtract_reuse_lhs(self, &rhs)
    }
}

// ['u32', '&Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], []]
impl Sub<&Rational> for u32 {
    type Output = Rational;

    fn sub(self, rhs: &Rational) -> Self::Output {
        subtract_reuse_lhs(self, rhs)
    }
}

// ['&u32', 'Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], ['ref']]
impl Sub<Rational> for &u32 {
    type Output = Rational;

    fn sub(self, rhs: Rational) -> Self::Output {
        subtract_reuse_lhs(self, &rhs)
    }
}

// ['&u32', '&Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], []]
impl Sub<&Rational> for &u32 {
    type Output = Rational;

    fn sub(self, rhs: &Rational) -> Self::Output {
        subtract_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'i64', 'Rational', 'Rational::subtract_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}]]
impl Sub<i64> for Rational {
    type Output = Rational;

    fn sub(mut self, rhs: i64) -> Self::Output {
        Rational::subtract_assign_integer(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Rational', '&i64', 'Rational', 'Rational::subtract_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}, 'deref']]
impl Sub<&i64> for Rational {
    type Output = Rational;

    fn sub(mut self, rhs: &i64) -> Self::Output {
        Rational::subtract_assign_integer(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Rational', 'i64', 'Rational', 'Rational::subtract_integer', 'no', [],
// ['ref', {'convert': 'Integer'}]]
impl Sub<i64> for &Rational {
    type Output = Rational;

    fn sub(self, rhs: i64) -> Self::Output {
        Rational::subtract_integer(self, &Integer::from(rhs))
    }
}

// ['&Rational', '&i64', 'Rational', 'Rational::subtract_integer', 'no', [],
// ['ref', {'convert': 'Integer'}, 'deref']]
impl Sub<&i64> for &Rational {
    type Output = Rational;

    fn sub(self, rhs: &i64) -> Self::Output {
        Rational::subtract_integer(self, &Integer::from(*rhs))
    }
}

// ['i64', 'Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], ['ref']]
impl Sub<Rational> for i64 {
    type Output = Rational;

    fn sub(self, rhs: Rational) -> Self::Output {
        subtract_reuse_lhs(self, &rhs)
    }
}

// ['i64', '&Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], []]
impl Sub<&Rational> for i64 {
    type Output = Rational;

    fn sub(self, rhs: &Rational) -> Self::Output {
        subtract_reuse_lhs(self, rhs)
    }
}

// ['&i64', 'Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], ['ref']]
impl Sub<Rational> for &i64 {
    type Output = Rational;

    fn sub(self, rhs: Rational) -> Self::Output {
        subtract_reuse_lhs(self, &rhs)
    }
}

// ['&i64', '&Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], []]
impl Sub<&Rational> for &i64 {
    type Output = Rational;

    fn sub(self, rhs: &Rational) -> Self::Output {
        subtract_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'u64', 'Rational', 'Rational::subtract_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}]]
impl Sub<u64> for Rational {
    type Output = Rational;

    fn sub(mut self, rhs: u64) -> Self::Output {
        Rational::subtract_assign_integer(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Rational', '&u64', 'Rational', 'Rational::subtract_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}, 'deref']]
impl Sub<&u64> for Rational {
    type Output = Rational;

    fn sub(mut self, rhs: &u64) -> Self::Output {
        Rational::subtract_assign_integer(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Rational', 'u64', 'Rational', 'Rational::subtract_integer', 'no', [],
// ['ref', {'convert': 'Integer'}]]
impl Sub<u64> for &Rational {
    type Output = Rational;

    fn sub(self, rhs: u64) -> Self::Output {
        Rational::subtract_integer(self, &Integer::from(rhs))
    }
}

// ['&Rational', '&u64', 'Rational', 'Rational::subtract_integer', 'no', [],
// ['ref', {'convert': 'Integer'}, 'deref']]
impl Sub<&u64> for &Rational {
    type Output = Rational;

    fn sub(self, rhs: &u64) -> Self::Output {
        Rational::subtract_integer(self, &Integer::from(*rhs))
    }
}

// ['u64', 'Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], ['ref']]
impl Sub<Rational> for u64 {
    type Output = Rational;

    fn sub(self, rhs: Rational) -> Self::Output {
        subtract_reuse_lhs(self, &rhs)
    }
}

// ['u64', '&Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], []]
impl Sub<&Rational> for u64 {
    type Output = Rational;

    fn sub(self, rhs: &Rational) -> Self::Output {
        subtract_reuse_lhs(self, rhs)
    }
}

// ['&u64', 'Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], ['ref']]
impl Sub<Rational> for &u64 {
    type Output = Rational;

    fn sub(self, rhs: Rational) -> Self::Output {
        subtract_reuse_lhs(self, &rhs)
    }
}

// ['&u64', '&Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], []]
impl Sub<&Rational> for &u64 {
    type Output = Rational;

    fn sub(self, rhs: &Rational) -> Self::Output {
        subtract_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'i128', 'Rational', 'Rational::subtract_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}]]
impl Sub<i128> for Rational {
    type Output = Rational;

    fn sub(mut self, rhs: i128) -> Self::Output {
        Rational::subtract_assign_integer(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Rational', '&i128', 'Rational', 'Rational::subtract_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}, 'deref']]
impl Sub<&i128> for Rational {
    type Output = Rational;

    fn sub(mut self, rhs: &i128) -> Self::Output {
        Rational::subtract_assign_integer(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Rational', 'i128', 'Rational', 'Rational::subtract_integer', 'no', [],
// ['ref', {'convert': 'Integer'}]]
impl Sub<i128> for &Rational {
    type Output = Rational;

    fn sub(self, rhs: i128) -> Self::Output {
        Rational::subtract_integer(self, &Integer::from(rhs))
    }
}

// ['&Rational', '&i128', 'Rational', 'Rational::subtract_integer', 'no', [],
// ['ref', {'convert': 'Integer'}, 'deref']]
impl Sub<&i128> for &Rational {
    type Output = Rational;

    fn sub(self, rhs: &i128) -> Self::Output {
        Rational::subtract_integer(self, &Integer::from(*rhs))
    }
}

// ['i128', 'Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], ['ref']]
impl Sub<Rational> for i128 {
    type Output = Rational;

    fn sub(self, rhs: Rational) -> Self::Output {
        subtract_reuse_lhs(self, &rhs)
    }
}

// ['i128', '&Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], []]
impl Sub<&Rational> for i128 {
    type Output = Rational;

    fn sub(self, rhs: &Rational) -> Self::Output {
        subtract_reuse_lhs(self, rhs)
    }
}

// ['&i128', 'Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], ['ref']]
impl Sub<Rational> for &i128 {
    type Output = Rational;

    fn sub(self, rhs: Rational) -> Self::Output {
        subtract_reuse_lhs(self, &rhs)
    }
}

// ['&i128', '&Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], []]
impl Sub<&Rational> for &i128 {
    type Output = Rational;

    fn sub(self, rhs: &Rational) -> Self::Output {
        subtract_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'u128', 'Rational', 'Rational::subtract_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}]]
impl Sub<u128> for Rational {
    type Output = Rational;

    fn sub(mut self, rhs: u128) -> Self::Output {
        Rational::subtract_assign_integer(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Rational', '&u128', 'Rational', 'Rational::subtract_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}, 'deref']]
impl Sub<&u128> for Rational {
    type Output = Rational;

    fn sub(mut self, rhs: &u128) -> Self::Output {
        Rational::subtract_assign_integer(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Rational', 'u128', 'Rational', 'Rational::subtract_integer', 'no', [],
// ['ref', {'convert': 'Integer'}]]
impl Sub<u128> for &Rational {
    type Output = Rational;

    fn sub(self, rhs: u128) -> Self::Output {
        Rational::subtract_integer(self, &Integer::from(rhs))
    }
}

// ['&Rational', '&u128', 'Rational', 'Rational::subtract_integer', 'no', [],
// ['ref', {'convert': 'Integer'}, 'deref']]
impl Sub<&u128> for &Rational {
    type Output = Rational;

    fn sub(self, rhs: &u128) -> Self::Output {
        Rational::subtract_integer(self, &Integer::from(*rhs))
    }
}

// ['u128', 'Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], ['ref']]
impl Sub<Rational> for u128 {
    type Output = Rational;

    fn sub(self, rhs: Rational) -> Self::Output {
        subtract_reuse_lhs(self, &rhs)
    }
}

// ['u128', '&Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], []]
impl Sub<&Rational> for u128 {
    type Output = Rational;

    fn sub(self, rhs: &Rational) -> Self::Output {
        subtract_reuse_lhs(self, rhs)
    }
}

// ['&u128', 'Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], ['ref']]
impl Sub<Rational> for &u128 {
    type Output = Rational;

    fn sub(self, rhs: Rational) -> Self::Output {
        subtract_reuse_lhs(self, &rhs)
    }
}

// ['&u128', '&Rational', 'Rational', 'subtract_reuse_lhs', 'no', [], []]
impl Sub<&Rational> for &u128 {
    type Output = Rational;

    fn sub(self, rhs: &Rational) -> Self::Output {
        subtract_reuse_lhs(self, rhs)
    }
}
