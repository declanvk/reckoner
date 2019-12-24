use crate::integer::Integer;
use core::ops::MulAssign;

// MulAssign    The multiplication assignment operator *=.

// [u'Integer', u'Integer', u'Integer::multiply_assign', u'no', [], [u'ref']]
impl MulAssign<Integer> for Integer {
    fn mul_assign(&mut self, rhs: Integer) {
        Integer::multiply_assign(self, &rhs)
    }
}

// [u'Integer', u'&Integer', u'Integer::multiply_assign', u'no', [], []]
impl MulAssign<&Integer> for Integer {
    fn mul_assign(&mut self, rhs: &Integer) {
        Integer::multiply_assign(self, rhs)
    }
}

// [u'Integer', u'i8', u'Integer::multiply_c_long_assign', u'no', [], []]
impl MulAssign<i8> for Integer {
    fn mul_assign(&mut self, rhs: i8) {
        Integer::multiply_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&i8', u'Integer::multiply_c_long_assign', u'no', [],
// [u'deref']]
impl MulAssign<&i8> for Integer {
    fn mul_assign(&mut self, rhs: &i8) {
        Integer::multiply_c_long_assign(self, *rhs)
    }
}

// [u'Integer', u'u8', u'Integer::multiply_c_long_assign', u'no', [], []]
impl MulAssign<u8> for Integer {
    fn mul_assign(&mut self, rhs: u8) {
        Integer::multiply_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&u8', u'Integer::multiply_c_long_assign', u'no', [],
// [u'deref']]
impl MulAssign<&u8> for Integer {
    fn mul_assign(&mut self, rhs: &u8) {
        Integer::multiply_c_long_assign(self, *rhs)
    }
}

// [u'Integer', u'i16', u'Integer::multiply_c_long_assign', u'no', [], []]
impl MulAssign<i16> for Integer {
    fn mul_assign(&mut self, rhs: i16) {
        Integer::multiply_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&i16', u'Integer::multiply_c_long_assign', u'no', [],
// [u'deref']]
impl MulAssign<&i16> for Integer {
    fn mul_assign(&mut self, rhs: &i16) {
        Integer::multiply_c_long_assign(self, *rhs)
    }
}

// [u'Integer', u'u16', u'Integer::multiply_c_long_assign', u'no', [], []]
impl MulAssign<u16> for Integer {
    fn mul_assign(&mut self, rhs: u16) {
        Integer::multiply_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&u16', u'Integer::multiply_c_long_assign', u'no', [],
// [u'deref']]
impl MulAssign<&u16> for Integer {
    fn mul_assign(&mut self, rhs: &u16) {
        Integer::multiply_c_long_assign(self, *rhs)
    }
}

// [u'Integer', u'i32', u'Integer::multiply_c_long_assign', u'no', [], []]
impl MulAssign<i32> for Integer {
    fn mul_assign(&mut self, rhs: i32) {
        Integer::multiply_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&i32', u'Integer::multiply_c_long_assign', u'no', [],
// [u'deref']]
impl MulAssign<&i32> for Integer {
    fn mul_assign(&mut self, rhs: &i32) {
        Integer::multiply_c_long_assign(self, *rhs)
    }
}

// [u'Integer', u'u32', u'Integer::multiply_c_long_assign', u'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl MulAssign<u32> for Integer {
    fn mul_assign(&mut self, rhs: u32) {
        Integer::multiply_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&u32', u'Integer::multiply_c_long_assign', u'no', [],
// [u'deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl MulAssign<&u32> for Integer {
    fn mul_assign(&mut self, rhs: &u32) {
        Integer::multiply_c_long_assign(self, *rhs)
    }
}

// [u'Integer', u'i64', u'Integer::multiply_c_long_assign', u'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl MulAssign<i64> for Integer {
    fn mul_assign(&mut self, rhs: i64) {
        Integer::multiply_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&i64', u'Integer::multiply_c_long_assign', u'no', [],
// [u'deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl MulAssign<&i64> for Integer {
    fn mul_assign(&mut self, rhs: &i64) {
        Integer::multiply_c_long_assign(self, *rhs)
    }
}

// [u'Integer', u'u32', u'Integer::multiply_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl MulAssign<u32> for Integer {
    fn mul_assign(&mut self, rhs: u32) {
        Integer::multiply_assign(self, &Integer::from(rhs))
    }
}

// [u'Integer', u'&u32', u'Integer::multiply_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl MulAssign<&u32> for Integer {
    fn mul_assign(&mut self, rhs: &u32) {
        Integer::multiply_assign(self, &Integer::from(*rhs))
    }
}

// [u'Integer', u'i64', u'Integer::multiply_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl MulAssign<i64> for Integer {
    fn mul_assign(&mut self, rhs: i64) {
        Integer::multiply_assign(self, &Integer::from(rhs))
    }
}

// [u'Integer', u'&i64', u'Integer::multiply_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl MulAssign<&i64> for Integer {
    fn mul_assign(&mut self, rhs: &i64) {
        Integer::multiply_assign(self, &Integer::from(*rhs))
    }
}

// [u'Integer', u'u64', u'Integer::multiply_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}]]
impl MulAssign<u64> for Integer {
    fn mul_assign(&mut self, rhs: u64) {
        Integer::multiply_assign(self, &Integer::from(rhs))
    }
}

// [u'Integer', u'&u64', u'Integer::multiply_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
impl MulAssign<&u64> for Integer {
    fn mul_assign(&mut self, rhs: &u64) {
        Integer::multiply_assign(self, &Integer::from(*rhs))
    }
}

// [u'Integer', u'i128', u'Integer::multiply_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}]]
impl MulAssign<i128> for Integer {
    fn mul_assign(&mut self, rhs: i128) {
        Integer::multiply_assign(self, &Integer::from(rhs))
    }
}

// [u'Integer', u'&i128', u'Integer::multiply_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
impl MulAssign<&i128> for Integer {
    fn mul_assign(&mut self, rhs: &i128) {
        Integer::multiply_assign(self, &Integer::from(*rhs))
    }
}

// [u'Integer', u'u128', u'Integer::multiply_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}]]
impl MulAssign<u128> for Integer {
    fn mul_assign(&mut self, rhs: u128) {
        Integer::multiply_assign(self, &Integer::from(rhs))
    }
}

// [u'Integer', u'&u128', u'Integer::multiply_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
impl MulAssign<&u128> for Integer {
    fn mul_assign(&mut self, rhs: &u128) {
        Integer::multiply_assign(self, &Integer::from(*rhs))
    }
}
