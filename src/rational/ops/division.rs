use super::helpers::divide_reuse_lhs;
use crate::{integer::Integer, rational::Rational};
use core::ops::Div;

// Div          The division operator /.

// ['Rational', 'Rational', 'Rational', 'Rational::divide_assign', 'lhs',
// ['ref_mut'], ['ref']]
impl Div<Rational> for Rational {
    type Output = Rational;

    fn div(mut self, rhs: Rational) -> Self::Output {
        Rational::divide_assign(&mut self, &rhs);
        self
    }
}

// ['Rational', '&Rational', 'Rational', 'Rational::divide_assign', 'lhs',
// ['ref_mut'], []]
impl Div<&Rational> for Rational {
    type Output = Rational;

    fn div(mut self, rhs: &Rational) -> Self::Output {
        Rational::divide_assign(&mut self, rhs);
        self
    }
}

// ['&Rational', 'Rational', 'Rational', 'Rational::divide', 'no', [], ['ref']]
impl Div<Rational> for &Rational {
    type Output = Rational;

    fn div(self, rhs: Rational) -> Self::Output {
        Rational::divide(self, &rhs)
    }
}

// ['&Rational', '&Rational', 'Rational', 'Rational::divide', 'no', [], []]
impl Div<&Rational> for &Rational {
    type Output = Rational;

    fn div(self, rhs: &Rational) -> Self::Output {
        Rational::divide(self, rhs)
    }
}

// ['Rational', 'Integer', 'Rational', 'Rational::divide_assign_integer', 'lhs',
// ['ref_mut'], ['ref']]
impl Div<Integer> for Rational {
    type Output = Rational;

    fn div(mut self, rhs: Integer) -> Self::Output {
        Rational::divide_assign_integer(&mut self, &rhs);
        self
    }
}

// ['Rational', '&Integer', 'Rational', 'Rational::divide_assign_integer',
// 'lhs', ['ref_mut'], []]
impl Div<&Integer> for Rational {
    type Output = Rational;

    fn div(mut self, rhs: &Integer) -> Self::Output {
        Rational::divide_assign_integer(&mut self, rhs);
        self
    }
}

// ['&Rational', 'Integer', 'Rational', 'Rational::divide_integer', 'no', [],
// ['ref']]
impl Div<Integer> for &Rational {
    type Output = Rational;

    fn div(self, rhs: Integer) -> Self::Output {
        Rational::divide_integer(self, &rhs)
    }
}

// ['&Rational', '&Integer', 'Rational', 'Rational::divide_integer', 'no', [],
// []]
impl Div<&Integer> for &Rational {
    type Output = Rational;

    fn div(self, rhs: &Integer) -> Self::Output {
        Rational::divide_integer(self, rhs)
    }
}

// ['Integer', 'Rational', 'Rational', 'divide_reuse_lhs', 'no', [], ['ref']]
impl Div<Rational> for Integer {
    type Output = Rational;

    fn div(self, rhs: Rational) -> Self::Output {
        divide_reuse_lhs(self, &rhs)
    }
}

// ['Integer', '&Rational', 'Rational', 'divide_reuse_lhs', 'no', [], []]
impl Div<&Rational> for Integer {
    type Output = Rational;

    fn div(self, rhs: &Rational) -> Self::Output {
        divide_reuse_lhs(self, rhs)
    }
}

// ['&Integer', 'Rational', 'Rational', 'divide_reuse_lhs', 'no', [], ['ref']]
impl Div<Rational> for &Integer {
    type Output = Rational;

    fn div(self, rhs: Rational) -> Self::Output {
        divide_reuse_lhs(self, &rhs)
    }
}

// ['&Integer', '&Rational', 'Rational', 'divide_reuse_lhs', 'no', [], []]
impl Div<&Rational> for &Integer {
    type Output = Rational;

    fn div(self, rhs: &Rational) -> Self::Output {
        divide_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'i8', 'Rational', 'Rational::divide_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}]]
impl Div<i8> for Rational {
    type Output = Rational;

    fn div(mut self, rhs: i8) -> Self::Output {
        Rational::divide_assign_integer(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Rational', '&i8', 'Rational', 'Rational::divide_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}, 'deref']]
impl Div<&i8> for Rational {
    type Output = Rational;

    fn div(mut self, rhs: &i8) -> Self::Output {
        Rational::divide_assign_integer(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Rational', 'i8', 'Rational', 'Rational::divide_integer', 'no', [], ['ref',
// {'convert': 'Integer'}]]
impl Div<i8> for &Rational {
    type Output = Rational;

    fn div(self, rhs: i8) -> Self::Output {
        Rational::divide_integer(self, &Integer::from(rhs))
    }
}

// ['&Rational', '&i8', 'Rational', 'Rational::divide_integer', 'no', [],
// ['ref', {'convert': 'Integer'}, 'deref']]
impl Div<&i8> for &Rational {
    type Output = Rational;

    fn div(self, rhs: &i8) -> Self::Output {
        Rational::divide_integer(self, &Integer::from(*rhs))
    }
}

// ['i8', 'Rational', 'Rational', 'divide_reuse_lhs', 'no', [], ['ref']]
impl Div<Rational> for i8 {
    type Output = Rational;

    fn div(self, rhs: Rational) -> Self::Output {
        divide_reuse_lhs(self, &rhs)
    }
}

// ['i8', '&Rational', 'Rational', 'divide_reuse_lhs', 'no', [], []]
impl Div<&Rational> for i8 {
    type Output = Rational;

    fn div(self, rhs: &Rational) -> Self::Output {
        divide_reuse_lhs(self, rhs)
    }
}

// ['&i8', 'Rational', 'Rational', 'divide_reuse_lhs', 'no', [], ['ref']]
impl Div<Rational> for &i8 {
    type Output = Rational;

    fn div(self, rhs: Rational) -> Self::Output {
        divide_reuse_lhs(self, &rhs)
    }
}

// ['&i8', '&Rational', 'Rational', 'divide_reuse_lhs', 'no', [], []]
impl Div<&Rational> for &i8 {
    type Output = Rational;

    fn div(self, rhs: &Rational) -> Self::Output {
        divide_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'u8', 'Rational', 'Rational::divide_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}]]
impl Div<u8> for Rational {
    type Output = Rational;

    fn div(mut self, rhs: u8) -> Self::Output {
        Rational::divide_assign_integer(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Rational', '&u8', 'Rational', 'Rational::divide_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}, 'deref']]
impl Div<&u8> for Rational {
    type Output = Rational;

    fn div(mut self, rhs: &u8) -> Self::Output {
        Rational::divide_assign_integer(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Rational', 'u8', 'Rational', 'Rational::divide_integer', 'no', [], ['ref',
// {'convert': 'Integer'}]]
impl Div<u8> for &Rational {
    type Output = Rational;

    fn div(self, rhs: u8) -> Self::Output {
        Rational::divide_integer(self, &Integer::from(rhs))
    }
}

// ['&Rational', '&u8', 'Rational', 'Rational::divide_integer', 'no', [],
// ['ref', {'convert': 'Integer'}, 'deref']]
impl Div<&u8> for &Rational {
    type Output = Rational;

    fn div(self, rhs: &u8) -> Self::Output {
        Rational::divide_integer(self, &Integer::from(*rhs))
    }
}

// ['u8', 'Rational', 'Rational', 'divide_reuse_lhs', 'no', [], ['ref']]
impl Div<Rational> for u8 {
    type Output = Rational;

    fn div(self, rhs: Rational) -> Self::Output {
        divide_reuse_lhs(self, &rhs)
    }
}

// ['u8', '&Rational', 'Rational', 'divide_reuse_lhs', 'no', [], []]
impl Div<&Rational> for u8 {
    type Output = Rational;

    fn div(self, rhs: &Rational) -> Self::Output {
        divide_reuse_lhs(self, rhs)
    }
}

// ['&u8', 'Rational', 'Rational', 'divide_reuse_lhs', 'no', [], ['ref']]
impl Div<Rational> for &u8 {
    type Output = Rational;

    fn div(self, rhs: Rational) -> Self::Output {
        divide_reuse_lhs(self, &rhs)
    }
}

// ['&u8', '&Rational', 'Rational', 'divide_reuse_lhs', 'no', [], []]
impl Div<&Rational> for &u8 {
    type Output = Rational;

    fn div(self, rhs: &Rational) -> Self::Output {
        divide_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'i16', 'Rational', 'Rational::divide_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}]]
impl Div<i16> for Rational {
    type Output = Rational;

    fn div(mut self, rhs: i16) -> Self::Output {
        Rational::divide_assign_integer(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Rational', '&i16', 'Rational', 'Rational::divide_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}, 'deref']]
impl Div<&i16> for Rational {
    type Output = Rational;

    fn div(mut self, rhs: &i16) -> Self::Output {
        Rational::divide_assign_integer(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Rational', 'i16', 'Rational', 'Rational::divide_integer', 'no', [],
// ['ref', {'convert': 'Integer'}]]
impl Div<i16> for &Rational {
    type Output = Rational;

    fn div(self, rhs: i16) -> Self::Output {
        Rational::divide_integer(self, &Integer::from(rhs))
    }
}

// ['&Rational', '&i16', 'Rational', 'Rational::divide_integer', 'no', [],
// ['ref', {'convert': 'Integer'}, 'deref']]
impl Div<&i16> for &Rational {
    type Output = Rational;

    fn div(self, rhs: &i16) -> Self::Output {
        Rational::divide_integer(self, &Integer::from(*rhs))
    }
}

// ['i16', 'Rational', 'Rational', 'divide_reuse_lhs', 'no', [], ['ref']]
impl Div<Rational> for i16 {
    type Output = Rational;

    fn div(self, rhs: Rational) -> Self::Output {
        divide_reuse_lhs(self, &rhs)
    }
}

// ['i16', '&Rational', 'Rational', 'divide_reuse_lhs', 'no', [], []]
impl Div<&Rational> for i16 {
    type Output = Rational;

    fn div(self, rhs: &Rational) -> Self::Output {
        divide_reuse_lhs(self, rhs)
    }
}

// ['&i16', 'Rational', 'Rational', 'divide_reuse_lhs', 'no', [], ['ref']]
impl Div<Rational> for &i16 {
    type Output = Rational;

    fn div(self, rhs: Rational) -> Self::Output {
        divide_reuse_lhs(self, &rhs)
    }
}

// ['&i16', '&Rational', 'Rational', 'divide_reuse_lhs', 'no', [], []]
impl Div<&Rational> for &i16 {
    type Output = Rational;

    fn div(self, rhs: &Rational) -> Self::Output {
        divide_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'u16', 'Rational', 'Rational::divide_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}]]
impl Div<u16> for Rational {
    type Output = Rational;

    fn div(mut self, rhs: u16) -> Self::Output {
        Rational::divide_assign_integer(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Rational', '&u16', 'Rational', 'Rational::divide_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}, 'deref']]
impl Div<&u16> for Rational {
    type Output = Rational;

    fn div(mut self, rhs: &u16) -> Self::Output {
        Rational::divide_assign_integer(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Rational', 'u16', 'Rational', 'Rational::divide_integer', 'no', [],
// ['ref', {'convert': 'Integer'}]]
impl Div<u16> for &Rational {
    type Output = Rational;

    fn div(self, rhs: u16) -> Self::Output {
        Rational::divide_integer(self, &Integer::from(rhs))
    }
}

// ['&Rational', '&u16', 'Rational', 'Rational::divide_integer', 'no', [],
// ['ref', {'convert': 'Integer'}, 'deref']]
impl Div<&u16> for &Rational {
    type Output = Rational;

    fn div(self, rhs: &u16) -> Self::Output {
        Rational::divide_integer(self, &Integer::from(*rhs))
    }
}

// ['u16', 'Rational', 'Rational', 'divide_reuse_lhs', 'no', [], ['ref']]
impl Div<Rational> for u16 {
    type Output = Rational;

    fn div(self, rhs: Rational) -> Self::Output {
        divide_reuse_lhs(self, &rhs)
    }
}

// ['u16', '&Rational', 'Rational', 'divide_reuse_lhs', 'no', [], []]
impl Div<&Rational> for u16 {
    type Output = Rational;

    fn div(self, rhs: &Rational) -> Self::Output {
        divide_reuse_lhs(self, rhs)
    }
}

// ['&u16', 'Rational', 'Rational', 'divide_reuse_lhs', 'no', [], ['ref']]
impl Div<Rational> for &u16 {
    type Output = Rational;

    fn div(self, rhs: Rational) -> Self::Output {
        divide_reuse_lhs(self, &rhs)
    }
}

// ['&u16', '&Rational', 'Rational', 'divide_reuse_lhs', 'no', [], []]
impl Div<&Rational> for &u16 {
    type Output = Rational;

    fn div(self, rhs: &Rational) -> Self::Output {
        divide_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'i32', 'Rational', 'Rational::divide_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}]]
impl Div<i32> for Rational {
    type Output = Rational;

    fn div(mut self, rhs: i32) -> Self::Output {
        Rational::divide_assign_integer(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Rational', '&i32', 'Rational', 'Rational::divide_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}, 'deref']]
impl Div<&i32> for Rational {
    type Output = Rational;

    fn div(mut self, rhs: &i32) -> Self::Output {
        Rational::divide_assign_integer(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Rational', 'i32', 'Rational', 'Rational::divide_integer', 'no', [],
// ['ref', {'convert': 'Integer'}]]
impl Div<i32> for &Rational {
    type Output = Rational;

    fn div(self, rhs: i32) -> Self::Output {
        Rational::divide_integer(self, &Integer::from(rhs))
    }
}

// ['&Rational', '&i32', 'Rational', 'Rational::divide_integer', 'no', [],
// ['ref', {'convert': 'Integer'}, 'deref']]
impl Div<&i32> for &Rational {
    type Output = Rational;

    fn div(self, rhs: &i32) -> Self::Output {
        Rational::divide_integer(self, &Integer::from(*rhs))
    }
}

// ['i32', 'Rational', 'Rational', 'divide_reuse_lhs', 'no', [], ['ref']]
impl Div<Rational> for i32 {
    type Output = Rational;

    fn div(self, rhs: Rational) -> Self::Output {
        divide_reuse_lhs(self, &rhs)
    }
}

// ['i32', '&Rational', 'Rational', 'divide_reuse_lhs', 'no', [], []]
impl Div<&Rational> for i32 {
    type Output = Rational;

    fn div(self, rhs: &Rational) -> Self::Output {
        divide_reuse_lhs(self, rhs)
    }
}

// ['&i32', 'Rational', 'Rational', 'divide_reuse_lhs', 'no', [], ['ref']]
impl Div<Rational> for &i32 {
    type Output = Rational;

    fn div(self, rhs: Rational) -> Self::Output {
        divide_reuse_lhs(self, &rhs)
    }
}

// ['&i32', '&Rational', 'Rational', 'divide_reuse_lhs', 'no', [], []]
impl Div<&Rational> for &i32 {
    type Output = Rational;

    fn div(self, rhs: &Rational) -> Self::Output {
        divide_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'u32', 'Rational', 'Rational::divide_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}]]
impl Div<u32> for Rational {
    type Output = Rational;

    fn div(mut self, rhs: u32) -> Self::Output {
        Rational::divide_assign_integer(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Rational', '&u32', 'Rational', 'Rational::divide_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}, 'deref']]
impl Div<&u32> for Rational {
    type Output = Rational;

    fn div(mut self, rhs: &u32) -> Self::Output {
        Rational::divide_assign_integer(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Rational', 'u32', 'Rational', 'Rational::divide_integer', 'no', [],
// ['ref', {'convert': 'Integer'}]]
impl Div<u32> for &Rational {
    type Output = Rational;

    fn div(self, rhs: u32) -> Self::Output {
        Rational::divide_integer(self, &Integer::from(rhs))
    }
}

// ['&Rational', '&u32', 'Rational', 'Rational::divide_integer', 'no', [],
// ['ref', {'convert': 'Integer'}, 'deref']]
impl Div<&u32> for &Rational {
    type Output = Rational;

    fn div(self, rhs: &u32) -> Self::Output {
        Rational::divide_integer(self, &Integer::from(*rhs))
    }
}

// ['u32', 'Rational', 'Rational', 'divide_reuse_lhs', 'no', [], ['ref']]
impl Div<Rational> for u32 {
    type Output = Rational;

    fn div(self, rhs: Rational) -> Self::Output {
        divide_reuse_lhs(self, &rhs)
    }
}

// ['u32', '&Rational', 'Rational', 'divide_reuse_lhs', 'no', [], []]
impl Div<&Rational> for u32 {
    type Output = Rational;

    fn div(self, rhs: &Rational) -> Self::Output {
        divide_reuse_lhs(self, rhs)
    }
}

// ['&u32', 'Rational', 'Rational', 'divide_reuse_lhs', 'no', [], ['ref']]
impl Div<Rational> for &u32 {
    type Output = Rational;

    fn div(self, rhs: Rational) -> Self::Output {
        divide_reuse_lhs(self, &rhs)
    }
}

// ['&u32', '&Rational', 'Rational', 'divide_reuse_lhs', 'no', [], []]
impl Div<&Rational> for &u32 {
    type Output = Rational;

    fn div(self, rhs: &Rational) -> Self::Output {
        divide_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'i64', 'Rational', 'Rational::divide_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}]]
impl Div<i64> for Rational {
    type Output = Rational;

    fn div(mut self, rhs: i64) -> Self::Output {
        Rational::divide_assign_integer(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Rational', '&i64', 'Rational', 'Rational::divide_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}, 'deref']]
impl Div<&i64> for Rational {
    type Output = Rational;

    fn div(mut self, rhs: &i64) -> Self::Output {
        Rational::divide_assign_integer(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Rational', 'i64', 'Rational', 'Rational::divide_integer', 'no', [],
// ['ref', {'convert': 'Integer'}]]
impl Div<i64> for &Rational {
    type Output = Rational;

    fn div(self, rhs: i64) -> Self::Output {
        Rational::divide_integer(self, &Integer::from(rhs))
    }
}

// ['&Rational', '&i64', 'Rational', 'Rational::divide_integer', 'no', [],
// ['ref', {'convert': 'Integer'}, 'deref']]
impl Div<&i64> for &Rational {
    type Output = Rational;

    fn div(self, rhs: &i64) -> Self::Output {
        Rational::divide_integer(self, &Integer::from(*rhs))
    }
}

// ['i64', 'Rational', 'Rational', 'divide_reuse_lhs', 'no', [], ['ref']]
impl Div<Rational> for i64 {
    type Output = Rational;

    fn div(self, rhs: Rational) -> Self::Output {
        divide_reuse_lhs(self, &rhs)
    }
}

// ['i64', '&Rational', 'Rational', 'divide_reuse_lhs', 'no', [], []]
impl Div<&Rational> for i64 {
    type Output = Rational;

    fn div(self, rhs: &Rational) -> Self::Output {
        divide_reuse_lhs(self, rhs)
    }
}

// ['&i64', 'Rational', 'Rational', 'divide_reuse_lhs', 'no', [], ['ref']]
impl Div<Rational> for &i64 {
    type Output = Rational;

    fn div(self, rhs: Rational) -> Self::Output {
        divide_reuse_lhs(self, &rhs)
    }
}

// ['&i64', '&Rational', 'Rational', 'divide_reuse_lhs', 'no', [], []]
impl Div<&Rational> for &i64 {
    type Output = Rational;

    fn div(self, rhs: &Rational) -> Self::Output {
        divide_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'u64', 'Rational', 'Rational::divide_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}]]
impl Div<u64> for Rational {
    type Output = Rational;

    fn div(mut self, rhs: u64) -> Self::Output {
        Rational::divide_assign_integer(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Rational', '&u64', 'Rational', 'Rational::divide_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}, 'deref']]
impl Div<&u64> for Rational {
    type Output = Rational;

    fn div(mut self, rhs: &u64) -> Self::Output {
        Rational::divide_assign_integer(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Rational', 'u64', 'Rational', 'Rational::divide_integer', 'no', [],
// ['ref', {'convert': 'Integer'}]]
impl Div<u64> for &Rational {
    type Output = Rational;

    fn div(self, rhs: u64) -> Self::Output {
        Rational::divide_integer(self, &Integer::from(rhs))
    }
}

// ['&Rational', '&u64', 'Rational', 'Rational::divide_integer', 'no', [],
// ['ref', {'convert': 'Integer'}, 'deref']]
impl Div<&u64> for &Rational {
    type Output = Rational;

    fn div(self, rhs: &u64) -> Self::Output {
        Rational::divide_integer(self, &Integer::from(*rhs))
    }
}

// ['u64', 'Rational', 'Rational', 'divide_reuse_lhs', 'no', [], ['ref']]
impl Div<Rational> for u64 {
    type Output = Rational;

    fn div(self, rhs: Rational) -> Self::Output {
        divide_reuse_lhs(self, &rhs)
    }
}

// ['u64', '&Rational', 'Rational', 'divide_reuse_lhs', 'no', [], []]
impl Div<&Rational> for u64 {
    type Output = Rational;

    fn div(self, rhs: &Rational) -> Self::Output {
        divide_reuse_lhs(self, rhs)
    }
}

// ['&u64', 'Rational', 'Rational', 'divide_reuse_lhs', 'no', [], ['ref']]
impl Div<Rational> for &u64 {
    type Output = Rational;

    fn div(self, rhs: Rational) -> Self::Output {
        divide_reuse_lhs(self, &rhs)
    }
}

// ['&u64', '&Rational', 'Rational', 'divide_reuse_lhs', 'no', [], []]
impl Div<&Rational> for &u64 {
    type Output = Rational;

    fn div(self, rhs: &Rational) -> Self::Output {
        divide_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'i128', 'Rational', 'Rational::divide_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}]]
impl Div<i128> for Rational {
    type Output = Rational;

    fn div(mut self, rhs: i128) -> Self::Output {
        Rational::divide_assign_integer(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Rational', '&i128', 'Rational', 'Rational::divide_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}, 'deref']]
impl Div<&i128> for Rational {
    type Output = Rational;

    fn div(mut self, rhs: &i128) -> Self::Output {
        Rational::divide_assign_integer(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Rational', 'i128', 'Rational', 'Rational::divide_integer', 'no', [],
// ['ref', {'convert': 'Integer'}]]
impl Div<i128> for &Rational {
    type Output = Rational;

    fn div(self, rhs: i128) -> Self::Output {
        Rational::divide_integer(self, &Integer::from(rhs))
    }
}

// ['&Rational', '&i128', 'Rational', 'Rational::divide_integer', 'no', [],
// ['ref', {'convert': 'Integer'}, 'deref']]
impl Div<&i128> for &Rational {
    type Output = Rational;

    fn div(self, rhs: &i128) -> Self::Output {
        Rational::divide_integer(self, &Integer::from(*rhs))
    }
}

// ['i128', 'Rational', 'Rational', 'divide_reuse_lhs', 'no', [], ['ref']]
impl Div<Rational> for i128 {
    type Output = Rational;

    fn div(self, rhs: Rational) -> Self::Output {
        divide_reuse_lhs(self, &rhs)
    }
}

// ['i128', '&Rational', 'Rational', 'divide_reuse_lhs', 'no', [], []]
impl Div<&Rational> for i128 {
    type Output = Rational;

    fn div(self, rhs: &Rational) -> Self::Output {
        divide_reuse_lhs(self, rhs)
    }
}

// ['&i128', 'Rational', 'Rational', 'divide_reuse_lhs', 'no', [], ['ref']]
impl Div<Rational> for &i128 {
    type Output = Rational;

    fn div(self, rhs: Rational) -> Self::Output {
        divide_reuse_lhs(self, &rhs)
    }
}

// ['&i128', '&Rational', 'Rational', 'divide_reuse_lhs', 'no', [], []]
impl Div<&Rational> for &i128 {
    type Output = Rational;

    fn div(self, rhs: &Rational) -> Self::Output {
        divide_reuse_lhs(self, rhs)
    }
}

// ['Rational', 'u128', 'Rational', 'Rational::divide_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}]]
impl Div<u128> for Rational {
    type Output = Rational;

    fn div(mut self, rhs: u128) -> Self::Output {
        Rational::divide_assign_integer(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Rational', '&u128', 'Rational', 'Rational::divide_assign_integer', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}, 'deref']]
impl Div<&u128> for Rational {
    type Output = Rational;

    fn div(mut self, rhs: &u128) -> Self::Output {
        Rational::divide_assign_integer(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Rational', 'u128', 'Rational', 'Rational::divide_integer', 'no', [],
// ['ref', {'convert': 'Integer'}]]
impl Div<u128> for &Rational {
    type Output = Rational;

    fn div(self, rhs: u128) -> Self::Output {
        Rational::divide_integer(self, &Integer::from(rhs))
    }
}

// ['&Rational', '&u128', 'Rational', 'Rational::divide_integer', 'no', [],
// ['ref', {'convert': 'Integer'}, 'deref']]
impl Div<&u128> for &Rational {
    type Output = Rational;

    fn div(self, rhs: &u128) -> Self::Output {
        Rational::divide_integer(self, &Integer::from(*rhs))
    }
}

// ['u128', 'Rational', 'Rational', 'divide_reuse_lhs', 'no', [], ['ref']]
impl Div<Rational> for u128 {
    type Output = Rational;

    fn div(self, rhs: Rational) -> Self::Output {
        divide_reuse_lhs(self, &rhs)
    }
}

// ['u128', '&Rational', 'Rational', 'divide_reuse_lhs', 'no', [], []]
impl Div<&Rational> for u128 {
    type Output = Rational;

    fn div(self, rhs: &Rational) -> Self::Output {
        divide_reuse_lhs(self, rhs)
    }
}

// ['&u128', 'Rational', 'Rational', 'divide_reuse_lhs', 'no', [], ['ref']]
impl Div<Rational> for &u128 {
    type Output = Rational;

    fn div(self, rhs: Rational) -> Self::Output {
        divide_reuse_lhs(self, &rhs)
    }
}

// ['&u128', '&Rational', 'Rational', 'divide_reuse_lhs', 'no', [], []]
impl Div<&Rational> for &u128 {
    type Output = Rational;

    fn div(self, rhs: &Rational) -> Self::Output {
        divide_reuse_lhs(self, rhs)
    }
}
