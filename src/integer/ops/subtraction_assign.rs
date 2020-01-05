use crate::integer::Integer;
use core::ops::SubAssign;

// SubAssign          The subtraction assignment operator -=.

// ['Integer', 'Integer', 'Integer::subtract_assign', 'no', [], ['ref']]
impl SubAssign<Integer> for Integer {
    fn sub_assign(&mut self, rhs: Integer) {
        Integer::subtract_assign(self, &rhs)
    }
}

// ['Integer', '&Integer', 'Integer::subtract_assign', 'no', [], []]
impl SubAssign<&Integer> for Integer {
    fn sub_assign(&mut self, rhs: &Integer) {
        Integer::subtract_assign(self, rhs)
    }
}

// ['Integer', 'i8', 'Integer::subtract_c_long_assign', 'no', [], []]
impl SubAssign<i8> for Integer {
    fn sub_assign(&mut self, rhs: i8) {
        Integer::subtract_c_long_assign(self, rhs)
    }
}

// ['Integer', '&i8', 'Integer::subtract_c_long_assign', 'no', [], ['deref']]
impl SubAssign<&i8> for Integer {
    fn sub_assign(&mut self, rhs: &i8) {
        Integer::subtract_c_long_assign(self, *rhs)
    }
}

// ['Integer', 'u8', 'Integer::subtract_c_long_assign', 'no', [], []]
impl SubAssign<u8> for Integer {
    fn sub_assign(&mut self, rhs: u8) {
        Integer::subtract_c_long_assign(self, rhs)
    }
}

// ['Integer', '&u8', 'Integer::subtract_c_long_assign', 'no', [], ['deref']]
impl SubAssign<&u8> for Integer {
    fn sub_assign(&mut self, rhs: &u8) {
        Integer::subtract_c_long_assign(self, *rhs)
    }
}

// ['Integer', 'i16', 'Integer::subtract_c_long_assign', 'no', [], []]
impl SubAssign<i16> for Integer {
    fn sub_assign(&mut self, rhs: i16) {
        Integer::subtract_c_long_assign(self, rhs)
    }
}

// ['Integer', '&i16', 'Integer::subtract_c_long_assign', 'no', [], ['deref']]
impl SubAssign<&i16> for Integer {
    fn sub_assign(&mut self, rhs: &i16) {
        Integer::subtract_c_long_assign(self, *rhs)
    }
}

// ['Integer', 'u16', 'Integer::subtract_c_long_assign', 'no', [], []]
impl SubAssign<u16> for Integer {
    fn sub_assign(&mut self, rhs: u16) {
        Integer::subtract_c_long_assign(self, rhs)
    }
}

// ['Integer', '&u16', 'Integer::subtract_c_long_assign', 'no', [], ['deref']]
impl SubAssign<&u16> for Integer {
    fn sub_assign(&mut self, rhs: &u16) {
        Integer::subtract_c_long_assign(self, *rhs)
    }
}

// ['Integer', 'i32', 'Integer::subtract_c_long_assign', 'no', [], []]
impl SubAssign<i32> for Integer {
    fn sub_assign(&mut self, rhs: i32) {
        Integer::subtract_c_long_assign(self, rhs)
    }
}

// ['Integer', '&i32', 'Integer::subtract_c_long_assign', 'no', [], ['deref']]
impl SubAssign<&i32> for Integer {
    fn sub_assign(&mut self, rhs: &i32) {
        Integer::subtract_c_long_assign(self, *rhs)
    }
}

// ['Integer', 'u32', 'Integer::subtract_c_long_assign', 'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl SubAssign<u32> for Integer {
    fn sub_assign(&mut self, rhs: u32) {
        Integer::subtract_c_long_assign(self, rhs)
    }
}

// ['Integer', '&u32', 'Integer::subtract_c_long_assign', 'no', [], ['deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl SubAssign<&u32> for Integer {
    fn sub_assign(&mut self, rhs: &u32) {
        Integer::subtract_c_long_assign(self, *rhs)
    }
}

// ['Integer', 'i64', 'Integer::subtract_c_long_assign', 'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl SubAssign<i64> for Integer {
    fn sub_assign(&mut self, rhs: i64) {
        Integer::subtract_c_long_assign(self, rhs)
    }
}

// ['Integer', '&i64', 'Integer::subtract_c_long_assign', 'no', [], ['deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl SubAssign<&i64> for Integer {
    fn sub_assign(&mut self, rhs: &i64) {
        Integer::subtract_c_long_assign(self, *rhs)
    }
}

// ['Integer', 'u32', 'Integer::subtract_assign', 'no', [], ['ref', {'convert':
// 'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl SubAssign<u32> for Integer {
    fn sub_assign(&mut self, rhs: u32) {
        Integer::subtract_assign(self, &Integer::from(rhs))
    }
}

// ['Integer', '&u32', 'Integer::subtract_assign', 'no', [], ['ref', {'convert':
// 'Integer'}, 'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl SubAssign<&u32> for Integer {
    fn sub_assign(&mut self, rhs: &u32) {
        Integer::subtract_assign(self, &Integer::from(*rhs))
    }
}

// ['Integer', 'i64', 'Integer::subtract_assign', 'no', [], ['ref', {'convert':
// 'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl SubAssign<i64> for Integer {
    fn sub_assign(&mut self, rhs: i64) {
        Integer::subtract_assign(self, &Integer::from(rhs))
    }
}

// ['Integer', '&i64', 'Integer::subtract_assign', 'no', [], ['ref', {'convert':
// 'Integer'}, 'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl SubAssign<&i64> for Integer {
    fn sub_assign(&mut self, rhs: &i64) {
        Integer::subtract_assign(self, &Integer::from(*rhs))
    }
}

// ['Integer', 'u64', 'Integer::subtract_assign', 'no', [], ['ref', {'convert':
// 'Integer'}]]
impl SubAssign<u64> for Integer {
    fn sub_assign(&mut self, rhs: u64) {
        Integer::subtract_assign(self, &Integer::from(rhs))
    }
}

// ['Integer', '&u64', 'Integer::subtract_assign', 'no', [], ['ref', {'convert':
// 'Integer'}, 'deref']]
impl SubAssign<&u64> for Integer {
    fn sub_assign(&mut self, rhs: &u64) {
        Integer::subtract_assign(self, &Integer::from(*rhs))
    }
}

// ['Integer', 'i128', 'Integer::subtract_assign', 'no', [], ['ref', {'convert':
// 'Integer'}]]
impl SubAssign<i128> for Integer {
    fn sub_assign(&mut self, rhs: i128) {
        Integer::subtract_assign(self, &Integer::from(rhs))
    }
}

// ['Integer', '&i128', 'Integer::subtract_assign', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
impl SubAssign<&i128> for Integer {
    fn sub_assign(&mut self, rhs: &i128) {
        Integer::subtract_assign(self, &Integer::from(*rhs))
    }
}

// ['Integer', 'u128', 'Integer::subtract_assign', 'no', [], ['ref', {'convert':
// 'Integer'}]]
impl SubAssign<u128> for Integer {
    fn sub_assign(&mut self, rhs: u128) {
        Integer::subtract_assign(self, &Integer::from(rhs))
    }
}

// ['Integer', '&u128', 'Integer::subtract_assign', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
impl SubAssign<&u128> for Integer {
    fn sub_assign(&mut self, rhs: &u128) {
        Integer::subtract_assign(self, &Integer::from(*rhs))
    }
}
