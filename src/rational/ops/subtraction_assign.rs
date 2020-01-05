use crate::{integer::Integer, rational::Rational};
use core::ops::SubAssign;

// SubAssign          The subtraction assignment operator -=.

// ['Rational', 'Rational', 'Rational::subtract_assign', 'no', [], ['ref']]
impl SubAssign<Rational> for Rational {
    fn sub_assign(&mut self, rhs: Rational) {
        Rational::subtract_assign(self, &rhs)
    }
}

// ['Rational', '&Rational', 'Rational::subtract_assign', 'no', [], []]
impl SubAssign<&Rational> for Rational {
    fn sub_assign(&mut self, rhs: &Rational) {
        Rational::subtract_assign(self, rhs)
    }
}

// ['Rational', 'Integer', 'Rational::subtract_assign_integer', 'no', [],
// ['ref']]
impl SubAssign<Integer> for Rational {
    fn sub_assign(&mut self, rhs: Integer) {
        Rational::subtract_assign_integer(self, &rhs)
    }
}

// ['Rational', '&Integer', 'Rational::subtract_assign_integer', 'no', [], []]
impl SubAssign<&Integer> for Rational {
    fn sub_assign(&mut self, rhs: &Integer) {
        Rational::subtract_assign_integer(self, rhs)
    }
}

// ['Rational', 'i8', 'Rational::subtract_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}]]
impl SubAssign<i8> for Rational {
    fn sub_assign(&mut self, rhs: i8) {
        Rational::subtract_assign_integer(self, &Integer::from(rhs))
    }
}

// ['Rational', '&i8', 'Rational::subtract_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
impl SubAssign<&i8> for Rational {
    fn sub_assign(&mut self, rhs: &i8) {
        Rational::subtract_assign_integer(self, &Integer::from(*rhs))
    }
}

// ['Rational', 'u8', 'Rational::subtract_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}]]
impl SubAssign<u8> for Rational {
    fn sub_assign(&mut self, rhs: u8) {
        Rational::subtract_assign_integer(self, &Integer::from(rhs))
    }
}

// ['Rational', '&u8', 'Rational::subtract_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
impl SubAssign<&u8> for Rational {
    fn sub_assign(&mut self, rhs: &u8) {
        Rational::subtract_assign_integer(self, &Integer::from(*rhs))
    }
}

// ['Rational', 'i16', 'Rational::subtract_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}]]
impl SubAssign<i16> for Rational {
    fn sub_assign(&mut self, rhs: i16) {
        Rational::subtract_assign_integer(self, &Integer::from(rhs))
    }
}

// ['Rational', '&i16', 'Rational::subtract_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
impl SubAssign<&i16> for Rational {
    fn sub_assign(&mut self, rhs: &i16) {
        Rational::subtract_assign_integer(self, &Integer::from(*rhs))
    }
}

// ['Rational', 'u16', 'Rational::subtract_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}]]
impl SubAssign<u16> for Rational {
    fn sub_assign(&mut self, rhs: u16) {
        Rational::subtract_assign_integer(self, &Integer::from(rhs))
    }
}

// ['Rational', '&u16', 'Rational::subtract_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
impl SubAssign<&u16> for Rational {
    fn sub_assign(&mut self, rhs: &u16) {
        Rational::subtract_assign_integer(self, &Integer::from(*rhs))
    }
}

// ['Rational', 'i32', 'Rational::subtract_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}]]
impl SubAssign<i32> for Rational {
    fn sub_assign(&mut self, rhs: i32) {
        Rational::subtract_assign_integer(self, &Integer::from(rhs))
    }
}

// ['Rational', '&i32', 'Rational::subtract_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
impl SubAssign<&i32> for Rational {
    fn sub_assign(&mut self, rhs: &i32) {
        Rational::subtract_assign_integer(self, &Integer::from(*rhs))
    }
}

// ['Rational', 'u32', 'Rational::subtract_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}]]
impl SubAssign<u32> for Rational {
    fn sub_assign(&mut self, rhs: u32) {
        Rational::subtract_assign_integer(self, &Integer::from(rhs))
    }
}

// ['Rational', '&u32', 'Rational::subtract_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
impl SubAssign<&u32> for Rational {
    fn sub_assign(&mut self, rhs: &u32) {
        Rational::subtract_assign_integer(self, &Integer::from(*rhs))
    }
}

// ['Rational', 'i64', 'Rational::subtract_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}]]
impl SubAssign<i64> for Rational {
    fn sub_assign(&mut self, rhs: i64) {
        Rational::subtract_assign_integer(self, &Integer::from(rhs))
    }
}

// ['Rational', '&i64', 'Rational::subtract_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
impl SubAssign<&i64> for Rational {
    fn sub_assign(&mut self, rhs: &i64) {
        Rational::subtract_assign_integer(self, &Integer::from(*rhs))
    }
}

// ['Rational', 'u64', 'Rational::subtract_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}]]
impl SubAssign<u64> for Rational {
    fn sub_assign(&mut self, rhs: u64) {
        Rational::subtract_assign_integer(self, &Integer::from(rhs))
    }
}

// ['Rational', '&u64', 'Rational::subtract_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
impl SubAssign<&u64> for Rational {
    fn sub_assign(&mut self, rhs: &u64) {
        Rational::subtract_assign_integer(self, &Integer::from(*rhs))
    }
}

// ['Rational', 'i128', 'Rational::subtract_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}]]
impl SubAssign<i128> for Rational {
    fn sub_assign(&mut self, rhs: i128) {
        Rational::subtract_assign_integer(self, &Integer::from(rhs))
    }
}

// ['Rational', '&i128', 'Rational::subtract_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
impl SubAssign<&i128> for Rational {
    fn sub_assign(&mut self, rhs: &i128) {
        Rational::subtract_assign_integer(self, &Integer::from(*rhs))
    }
}

// ['Rational', 'u128', 'Rational::subtract_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}]]
impl SubAssign<u128> for Rational {
    fn sub_assign(&mut self, rhs: u128) {
        Rational::subtract_assign_integer(self, &Integer::from(rhs))
    }
}

// ['Rational', '&u128', 'Rational::subtract_assign_integer', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
impl SubAssign<&u128> for Rational {
    fn sub_assign(&mut self, rhs: &u128) {
        Rational::subtract_assign_integer(self, &Integer::from(*rhs))
    }
}
