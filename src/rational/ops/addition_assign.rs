use super::helpers::add_assign_into_rational;
use crate::{integer::Integer, rational::Rational};
use core::ops::AddAssign;

// AddAssign    The addition assignment operator +=.

// ['Rational', 'Rational', 'Rational::add_assign', 'no', [], ['ref']]
impl AddAssign<Rational> for Rational {
    fn add_assign(&mut self, rhs: Rational) {
        Rational::add_assign(self, &rhs)
    }
}

// ['Rational', '&Rational', 'Rational::add_assign', 'no', [], []]
impl AddAssign<&Rational> for Rational {
    fn add_assign(&mut self, rhs: &Rational) {
        Rational::add_assign(self, rhs)
    }
}

// ['Rational', 'Integer', 'Rational::add_assign_integer', 'no', [], ['ref']]
impl AddAssign<Integer> for Rational {
    fn add_assign(&mut self, rhs: Integer) {
        Rational::add_assign_integer(self, &rhs)
    }
}

// ['Rational', '&Integer', 'Rational::add_assign_integer', 'no', [], []]
impl AddAssign<&Integer> for Rational {
    fn add_assign(&mut self, rhs: &Integer) {
        Rational::add_assign_integer(self, rhs)
    }
}

// ['Rational', 'i8', 'add_assign_into_rational', 'no', [], []]
impl AddAssign<i8> for Rational {
    fn add_assign(&mut self, rhs: i8) {
        add_assign_into_rational(self, rhs)
    }
}

// ['Rational', '&i8', 'add_assign_into_rational', 'no', [], ['deref']]
impl AddAssign<&i8> for Rational {
    fn add_assign(&mut self, rhs: &i8) {
        add_assign_into_rational(self, *rhs)
    }
}

// ['Rational', 'u8', 'add_assign_into_rational', 'no', [], []]
impl AddAssign<u8> for Rational {
    fn add_assign(&mut self, rhs: u8) {
        add_assign_into_rational(self, rhs)
    }
}

// ['Rational', '&u8', 'add_assign_into_rational', 'no', [], ['deref']]
impl AddAssign<&u8> for Rational {
    fn add_assign(&mut self, rhs: &u8) {
        add_assign_into_rational(self, *rhs)
    }
}

// ['Rational', 'i16', 'add_assign_into_rational', 'no', [], []]
impl AddAssign<i16> for Rational {
    fn add_assign(&mut self, rhs: i16) {
        add_assign_into_rational(self, rhs)
    }
}

// ['Rational', '&i16', 'add_assign_into_rational', 'no', [], ['deref']]
impl AddAssign<&i16> for Rational {
    fn add_assign(&mut self, rhs: &i16) {
        add_assign_into_rational(self, *rhs)
    }
}

// ['Rational', 'u16', 'add_assign_into_rational', 'no', [], []]
impl AddAssign<u16> for Rational {
    fn add_assign(&mut self, rhs: u16) {
        add_assign_into_rational(self, rhs)
    }
}

// ['Rational', '&u16', 'add_assign_into_rational', 'no', [], ['deref']]
impl AddAssign<&u16> for Rational {
    fn add_assign(&mut self, rhs: &u16) {
        add_assign_into_rational(self, *rhs)
    }
}

// ['Rational', 'i32', 'add_assign_into_rational', 'no', [], []]
impl AddAssign<i32> for Rational {
    fn add_assign(&mut self, rhs: i32) {
        add_assign_into_rational(self, rhs)
    }
}

// ['Rational', '&i32', 'add_assign_into_rational', 'no', [], ['deref']]
impl AddAssign<&i32> for Rational {
    fn add_assign(&mut self, rhs: &i32) {
        add_assign_into_rational(self, *rhs)
    }
}

// ['Rational', 'u32', 'add_assign_into_rational', 'no', [], []]
impl AddAssign<u32> for Rational {
    fn add_assign(&mut self, rhs: u32) {
        add_assign_into_rational(self, rhs)
    }
}

// ['Rational', '&u32', 'add_assign_into_rational', 'no', [], ['deref']]
impl AddAssign<&u32> for Rational {
    fn add_assign(&mut self, rhs: &u32) {
        add_assign_into_rational(self, *rhs)
    }
}

// ['Rational', 'i64', 'add_assign_into_rational', 'no', [], []]
impl AddAssign<i64> for Rational {
    fn add_assign(&mut self, rhs: i64) {
        add_assign_into_rational(self, rhs)
    }
}

// ['Rational', '&i64', 'add_assign_into_rational', 'no', [], ['deref']]
impl AddAssign<&i64> for Rational {
    fn add_assign(&mut self, rhs: &i64) {
        add_assign_into_rational(self, *rhs)
    }
}

// ['Rational', 'u64', 'add_assign_into_rational', 'no', [], []]
impl AddAssign<u64> for Rational {
    fn add_assign(&mut self, rhs: u64) {
        add_assign_into_rational(self, rhs)
    }
}

// ['Rational', '&u64', 'add_assign_into_rational', 'no', [], ['deref']]
impl AddAssign<&u64> for Rational {
    fn add_assign(&mut self, rhs: &u64) {
        add_assign_into_rational(self, *rhs)
    }
}

// ['Rational', 'i128', 'add_assign_into_rational', 'no', [], []]
impl AddAssign<i128> for Rational {
    fn add_assign(&mut self, rhs: i128) {
        add_assign_into_rational(self, rhs)
    }
}

// ['Rational', '&i128', 'add_assign_into_rational', 'no', [], ['deref']]
impl AddAssign<&i128> for Rational {
    fn add_assign(&mut self, rhs: &i128) {
        add_assign_into_rational(self, *rhs)
    }
}

// ['Rational', 'u128', 'add_assign_into_rational', 'no', [], []]
impl AddAssign<u128> for Rational {
    fn add_assign(&mut self, rhs: u128) {
        add_assign_into_rational(self, rhs)
    }
}

// ['Rational', '&u128', 'add_assign_into_rational', 'no', [], ['deref']]
impl AddAssign<&u128> for Rational {
    fn add_assign(&mut self, rhs: &u128) {
        add_assign_into_rational(self, *rhs)
    }
}
