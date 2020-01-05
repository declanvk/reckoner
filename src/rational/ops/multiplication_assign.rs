use super::helpers::multiply_assign_into_rational;
use crate::{integer::Integer, rational::Rational};
use core::ops::MulAssign;

// MulAssign    The multiplication assignment operator *=.

// ['Rational', 'Rational', 'Rational::multiply_assign', 'no', [], ['ref']]
impl MulAssign<Rational> for Rational {
    fn mul_assign(&mut self, rhs: Rational) {
        Rational::multiply_assign(self, &rhs)
    }
}

// ['Rational', '&Rational', 'Rational::multiply_assign', 'no', [], []]
impl MulAssign<&Rational> for Rational {
    fn mul_assign(&mut self, rhs: &Rational) {
        Rational::multiply_assign(self, rhs)
    }
}

// ['Rational', 'Integer', 'Rational::multiply_assign_integer', 'no', [],
// ['ref']]
impl MulAssign<Integer> for Rational {
    fn mul_assign(&mut self, rhs: Integer) {
        Rational::multiply_assign_integer(self, &rhs)
    }
}

// ['Rational', '&Integer', 'Rational::multiply_assign_integer', 'no', [], []]
impl MulAssign<&Integer> for Rational {
    fn mul_assign(&mut self, rhs: &Integer) {
        Rational::multiply_assign_integer(self, rhs)
    }
}

// ['Rational', 'i8', 'multiply_assign_into_rational', 'no', [], []]
impl MulAssign<i8> for Rational {
    fn mul_assign(&mut self, rhs: i8) {
        multiply_assign_into_rational(self, rhs)
    }
}

// ['Rational', '&i8', 'multiply_assign_into_rational', 'no', [], ['deref']]
impl MulAssign<&i8> for Rational {
    fn mul_assign(&mut self, rhs: &i8) {
        multiply_assign_into_rational(self, *rhs)
    }
}

// ['Rational', 'u8', 'multiply_assign_into_rational', 'no', [], []]
impl MulAssign<u8> for Rational {
    fn mul_assign(&mut self, rhs: u8) {
        multiply_assign_into_rational(self, rhs)
    }
}

// ['Rational', '&u8', 'multiply_assign_into_rational', 'no', [], ['deref']]
impl MulAssign<&u8> for Rational {
    fn mul_assign(&mut self, rhs: &u8) {
        multiply_assign_into_rational(self, *rhs)
    }
}

// ['Rational', 'i16', 'multiply_assign_into_rational', 'no', [], []]
impl MulAssign<i16> for Rational {
    fn mul_assign(&mut self, rhs: i16) {
        multiply_assign_into_rational(self, rhs)
    }
}

// ['Rational', '&i16', 'multiply_assign_into_rational', 'no', [], ['deref']]
impl MulAssign<&i16> for Rational {
    fn mul_assign(&mut self, rhs: &i16) {
        multiply_assign_into_rational(self, *rhs)
    }
}

// ['Rational', 'u16', 'multiply_assign_into_rational', 'no', [], []]
impl MulAssign<u16> for Rational {
    fn mul_assign(&mut self, rhs: u16) {
        multiply_assign_into_rational(self, rhs)
    }
}

// ['Rational', '&u16', 'multiply_assign_into_rational', 'no', [], ['deref']]
impl MulAssign<&u16> for Rational {
    fn mul_assign(&mut self, rhs: &u16) {
        multiply_assign_into_rational(self, *rhs)
    }
}

// ['Rational', 'i32', 'multiply_assign_into_rational', 'no', [], []]
impl MulAssign<i32> for Rational {
    fn mul_assign(&mut self, rhs: i32) {
        multiply_assign_into_rational(self, rhs)
    }
}

// ['Rational', '&i32', 'multiply_assign_into_rational', 'no', [], ['deref']]
impl MulAssign<&i32> for Rational {
    fn mul_assign(&mut self, rhs: &i32) {
        multiply_assign_into_rational(self, *rhs)
    }
}

// ['Rational', 'u32', 'multiply_assign_into_rational', 'no', [], []]
impl MulAssign<u32> for Rational {
    fn mul_assign(&mut self, rhs: u32) {
        multiply_assign_into_rational(self, rhs)
    }
}

// ['Rational', '&u32', 'multiply_assign_into_rational', 'no', [], ['deref']]
impl MulAssign<&u32> for Rational {
    fn mul_assign(&mut self, rhs: &u32) {
        multiply_assign_into_rational(self, *rhs)
    }
}

// ['Rational', 'i64', 'multiply_assign_into_rational', 'no', [], []]
impl MulAssign<i64> for Rational {
    fn mul_assign(&mut self, rhs: i64) {
        multiply_assign_into_rational(self, rhs)
    }
}

// ['Rational', '&i64', 'multiply_assign_into_rational', 'no', [], ['deref']]
impl MulAssign<&i64> for Rational {
    fn mul_assign(&mut self, rhs: &i64) {
        multiply_assign_into_rational(self, *rhs)
    }
}

// ['Rational', 'u64', 'multiply_assign_into_rational', 'no', [], []]
impl MulAssign<u64> for Rational {
    fn mul_assign(&mut self, rhs: u64) {
        multiply_assign_into_rational(self, rhs)
    }
}

// ['Rational', '&u64', 'multiply_assign_into_rational', 'no', [], ['deref']]
impl MulAssign<&u64> for Rational {
    fn mul_assign(&mut self, rhs: &u64) {
        multiply_assign_into_rational(self, *rhs)
    }
}

// ['Rational', 'i128', 'multiply_assign_into_rational', 'no', [], []]
impl MulAssign<i128> for Rational {
    fn mul_assign(&mut self, rhs: i128) {
        multiply_assign_into_rational(self, rhs)
    }
}

// ['Rational', '&i128', 'multiply_assign_into_rational', 'no', [], ['deref']]
impl MulAssign<&i128> for Rational {
    fn mul_assign(&mut self, rhs: &i128) {
        multiply_assign_into_rational(self, *rhs)
    }
}

// ['Rational', 'u128', 'multiply_assign_into_rational', 'no', [], []]
impl MulAssign<u128> for Rational {
    fn mul_assign(&mut self, rhs: u128) {
        multiply_assign_into_rational(self, rhs)
    }
}

// ['Rational', '&u128', 'multiply_assign_into_rational', 'no', [], ['deref']]
impl MulAssign<&u128> for Rational {
    fn mul_assign(&mut self, rhs: &u128) {
        multiply_assign_into_rational(self, *rhs)
    }
}
