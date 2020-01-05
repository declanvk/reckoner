use crate::integer::Integer;
use core::ops::MulAssign;

// MulAssign    The multiplication assignment operator *=.

// ['Integer', 'Integer', 'Integer::multiply_assign', 'no', [], ['ref']]
impl MulAssign<Integer> for Integer {
    fn mul_assign(&mut self, rhs: Integer) {
        Integer::multiply_assign(self, &rhs)
    }
}

// ['Integer', '&Integer', 'Integer::multiply_assign', 'no', [], []]
impl MulAssign<&Integer> for Integer {
    fn mul_assign(&mut self, rhs: &Integer) {
        Integer::multiply_assign(self, rhs)
    }
}

// ['Integer', 'i8', 'Integer::multiply_c_long_assign', 'no', [], []]
impl MulAssign<i8> for Integer {
    fn mul_assign(&mut self, rhs: i8) {
        Integer::multiply_c_long_assign(self, rhs)
    }
}

// ['Integer', '&i8', 'Integer::multiply_c_long_assign', 'no', [], ['deref']]
impl MulAssign<&i8> for Integer {
    fn mul_assign(&mut self, rhs: &i8) {
        Integer::multiply_c_long_assign(self, *rhs)
    }
}

// ['Integer', 'u8', 'Integer::multiply_c_long_assign', 'no', [], []]
impl MulAssign<u8> for Integer {
    fn mul_assign(&mut self, rhs: u8) {
        Integer::multiply_c_long_assign(self, rhs)
    }
}

// ['Integer', '&u8', 'Integer::multiply_c_long_assign', 'no', [], ['deref']]
impl MulAssign<&u8> for Integer {
    fn mul_assign(&mut self, rhs: &u8) {
        Integer::multiply_c_long_assign(self, *rhs)
    }
}

// ['Integer', 'i16', 'Integer::multiply_c_long_assign', 'no', [], []]
impl MulAssign<i16> for Integer {
    fn mul_assign(&mut self, rhs: i16) {
        Integer::multiply_c_long_assign(self, rhs)
    }
}

// ['Integer', '&i16', 'Integer::multiply_c_long_assign', 'no', [], ['deref']]
impl MulAssign<&i16> for Integer {
    fn mul_assign(&mut self, rhs: &i16) {
        Integer::multiply_c_long_assign(self, *rhs)
    }
}

// ['Integer', 'u16', 'Integer::multiply_c_long_assign', 'no', [], []]
impl MulAssign<u16> for Integer {
    fn mul_assign(&mut self, rhs: u16) {
        Integer::multiply_c_long_assign(self, rhs)
    }
}

// ['Integer', '&u16', 'Integer::multiply_c_long_assign', 'no', [], ['deref']]
impl MulAssign<&u16> for Integer {
    fn mul_assign(&mut self, rhs: &u16) {
        Integer::multiply_c_long_assign(self, *rhs)
    }
}

// ['Integer', 'i32', 'Integer::multiply_c_long_assign', 'no', [], []]
impl MulAssign<i32> for Integer {
    fn mul_assign(&mut self, rhs: i32) {
        Integer::multiply_c_long_assign(self, rhs)
    }
}

// ['Integer', '&i32', 'Integer::multiply_c_long_assign', 'no', [], ['deref']]
impl MulAssign<&i32> for Integer {
    fn mul_assign(&mut self, rhs: &i32) {
        Integer::multiply_c_long_assign(self, *rhs)
    }
}

// ['Integer', 'u32', 'Integer::multiply_c_long_assign', 'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl MulAssign<u32> for Integer {
    fn mul_assign(&mut self, rhs: u32) {
        Integer::multiply_c_long_assign(self, rhs)
    }
}

// ['Integer', '&u32', 'Integer::multiply_c_long_assign', 'no', [], ['deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl MulAssign<&u32> for Integer {
    fn mul_assign(&mut self, rhs: &u32) {
        Integer::multiply_c_long_assign(self, *rhs)
    }
}

// ['Integer', 'i64', 'Integer::multiply_c_long_assign', 'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl MulAssign<i64> for Integer {
    fn mul_assign(&mut self, rhs: i64) {
        Integer::multiply_c_long_assign(self, rhs)
    }
}

// ['Integer', '&i64', 'Integer::multiply_c_long_assign', 'no', [], ['deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl MulAssign<&i64> for Integer {
    fn mul_assign(&mut self, rhs: &i64) {
        Integer::multiply_c_long_assign(self, *rhs)
    }
}

// ['Integer', 'u32', 'Integer::multiply_assign', 'no', [], ['ref', {'convert':
// 'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl MulAssign<u32> for Integer {
    fn mul_assign(&mut self, rhs: u32) {
        Integer::multiply_assign(self, &Integer::from(rhs))
    }
}

// ['Integer', '&u32', 'Integer::multiply_assign', 'no', [], ['ref', {'convert':
// 'Integer'}, 'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl MulAssign<&u32> for Integer {
    fn mul_assign(&mut self, rhs: &u32) {
        Integer::multiply_assign(self, &Integer::from(*rhs))
    }
}

// ['Integer', 'i64', 'Integer::multiply_assign', 'no', [], ['ref', {'convert':
// 'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl MulAssign<i64> for Integer {
    fn mul_assign(&mut self, rhs: i64) {
        Integer::multiply_assign(self, &Integer::from(rhs))
    }
}

// ['Integer', '&i64', 'Integer::multiply_assign', 'no', [], ['ref', {'convert':
// 'Integer'}, 'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl MulAssign<&i64> for Integer {
    fn mul_assign(&mut self, rhs: &i64) {
        Integer::multiply_assign(self, &Integer::from(*rhs))
    }
}

// ['Integer', 'u64', 'Integer::multiply_assign', 'no', [], ['ref', {'convert':
// 'Integer'}]]
impl MulAssign<u64> for Integer {
    fn mul_assign(&mut self, rhs: u64) {
        Integer::multiply_assign(self, &Integer::from(rhs))
    }
}

// ['Integer', '&u64', 'Integer::multiply_assign', 'no', [], ['ref', {'convert':
// 'Integer'}, 'deref']]
impl MulAssign<&u64> for Integer {
    fn mul_assign(&mut self, rhs: &u64) {
        Integer::multiply_assign(self, &Integer::from(*rhs))
    }
}

// ['Integer', 'i128', 'Integer::multiply_assign', 'no', [], ['ref', {'convert':
// 'Integer'}]]
impl MulAssign<i128> for Integer {
    fn mul_assign(&mut self, rhs: i128) {
        Integer::multiply_assign(self, &Integer::from(rhs))
    }
}

// ['Integer', '&i128', 'Integer::multiply_assign', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
impl MulAssign<&i128> for Integer {
    fn mul_assign(&mut self, rhs: &i128) {
        Integer::multiply_assign(self, &Integer::from(*rhs))
    }
}

// ['Integer', 'u128', 'Integer::multiply_assign', 'no', [], ['ref', {'convert':
// 'Integer'}]]
impl MulAssign<u128> for Integer {
    fn mul_assign(&mut self, rhs: u128) {
        Integer::multiply_assign(self, &Integer::from(rhs))
    }
}

// ['Integer', '&u128', 'Integer::multiply_assign', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
impl MulAssign<&u128> for Integer {
    fn mul_assign(&mut self, rhs: &u128) {
        Integer::multiply_assign(self, &Integer::from(*rhs))
    }
}
