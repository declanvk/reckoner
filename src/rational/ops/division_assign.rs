use crate::{integer::Integer, rational::Rational};
use core::ops::DivAssign;

// DivAssign          The division assignment operator /=.

// ['Rational', 'Rational', 'Rational::divide_assign', 'no', [], ['ref']]
impl DivAssign<Rational> for Rational {
    fn div_assign(&mut self, rhs: Rational) {
        Rational::divide_assign(self, &rhs)
    }
}

// ['Rational', '&Rational', 'Rational::divide_assign', 'no', [], []]
impl DivAssign<&Rational> for Rational {
    fn div_assign(&mut self, rhs: &Rational) {
        Rational::divide_assign(self, rhs)
    }
}

// ['Rational', 'Integer', 'Rational::divide_assign_integer', 'no', [], ['ref']]
impl DivAssign<Integer> for Rational {
    fn div_assign(&mut self, rhs: Integer) {
        Rational::divide_assign_integer(self, &rhs)
    }
}

// ['Rational', '&Integer', 'Rational::divide_assign_integer', 'no', [], []]
impl DivAssign<&Integer> for Rational {
    fn div_assign(&mut self, rhs: &Integer) {
        Rational::divide_assign_integer(self, rhs)
    }
}

// ['Rational', 'i8', 'Rational::divide_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}]]
impl DivAssign<i8> for Rational {
    fn div_assign(&mut self, rhs: i8) {
        Rational::divide_assign_integer(self, &Integer::from(rhs))
    }
}

// ['Rational', '&i8', 'Rational::divide_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
impl DivAssign<&i8> for Rational {
    fn div_assign(&mut self, rhs: &i8) {
        Rational::divide_assign_integer(self, &Integer::from(*rhs))
    }
}

// ['Rational', 'u8', 'Rational::divide_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}]]
impl DivAssign<u8> for Rational {
    fn div_assign(&mut self, rhs: u8) {
        Rational::divide_assign_integer(self, &Integer::from(rhs))
    }
}

// ['Rational', '&u8', 'Rational::divide_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
impl DivAssign<&u8> for Rational {
    fn div_assign(&mut self, rhs: &u8) {
        Rational::divide_assign_integer(self, &Integer::from(*rhs))
    }
}

// ['Rational', 'i16', 'Rational::divide_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}]]
impl DivAssign<i16> for Rational {
    fn div_assign(&mut self, rhs: i16) {
        Rational::divide_assign_integer(self, &Integer::from(rhs))
    }
}

// ['Rational', '&i16', 'Rational::divide_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
impl DivAssign<&i16> for Rational {
    fn div_assign(&mut self, rhs: &i16) {
        Rational::divide_assign_integer(self, &Integer::from(*rhs))
    }
}

// ['Rational', 'u16', 'Rational::divide_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}]]
impl DivAssign<u16> for Rational {
    fn div_assign(&mut self, rhs: u16) {
        Rational::divide_assign_integer(self, &Integer::from(rhs))
    }
}

// ['Rational', '&u16', 'Rational::divide_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
impl DivAssign<&u16> for Rational {
    fn div_assign(&mut self, rhs: &u16) {
        Rational::divide_assign_integer(self, &Integer::from(*rhs))
    }
}

// ['Rational', 'i32', 'Rational::divide_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}]]
impl DivAssign<i32> for Rational {
    fn div_assign(&mut self, rhs: i32) {
        Rational::divide_assign_integer(self, &Integer::from(rhs))
    }
}

// ['Rational', '&i32', 'Rational::divide_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
impl DivAssign<&i32> for Rational {
    fn div_assign(&mut self, rhs: &i32) {
        Rational::divide_assign_integer(self, &Integer::from(*rhs))
    }
}

// ['Rational', 'u32', 'Rational::divide_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}]]
impl DivAssign<u32> for Rational {
    fn div_assign(&mut self, rhs: u32) {
        Rational::divide_assign_integer(self, &Integer::from(rhs))
    }
}

// ['Rational', '&u32', 'Rational::divide_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
impl DivAssign<&u32> for Rational {
    fn div_assign(&mut self, rhs: &u32) {
        Rational::divide_assign_integer(self, &Integer::from(*rhs))
    }
}

// ['Rational', 'i64', 'Rational::divide_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}]]
impl DivAssign<i64> for Rational {
    fn div_assign(&mut self, rhs: i64) {
        Rational::divide_assign_integer(self, &Integer::from(rhs))
    }
}

// ['Rational', '&i64', 'Rational::divide_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
impl DivAssign<&i64> for Rational {
    fn div_assign(&mut self, rhs: &i64) {
        Rational::divide_assign_integer(self, &Integer::from(*rhs))
    }
}

// ['Rational', 'u64', 'Rational::divide_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}]]
impl DivAssign<u64> for Rational {
    fn div_assign(&mut self, rhs: u64) {
        Rational::divide_assign_integer(self, &Integer::from(rhs))
    }
}

// ['Rational', '&u64', 'Rational::divide_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
impl DivAssign<&u64> for Rational {
    fn div_assign(&mut self, rhs: &u64) {
        Rational::divide_assign_integer(self, &Integer::from(*rhs))
    }
}

// ['Rational', 'i128', 'Rational::divide_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}]]
impl DivAssign<i128> for Rational {
    fn div_assign(&mut self, rhs: i128) {
        Rational::divide_assign_integer(self, &Integer::from(rhs))
    }
}

// ['Rational', '&i128', 'Rational::divide_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
impl DivAssign<&i128> for Rational {
    fn div_assign(&mut self, rhs: &i128) {
        Rational::divide_assign_integer(self, &Integer::from(*rhs))
    }
}

// ['Rational', 'u128', 'Rational::divide_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}]]
impl DivAssign<u128> for Rational {
    fn div_assign(&mut self, rhs: u128) {
        Rational::divide_assign_integer(self, &Integer::from(rhs))
    }
}

// ['Rational', '&u128', 'Rational::divide_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
impl DivAssign<&u128> for Rational {
    fn div_assign(&mut self, rhs: &u128) {
        Rational::divide_assign_integer(self, &Integer::from(*rhs))
    }
}
