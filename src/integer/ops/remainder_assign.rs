use super::helpers::c_long_remainder_assign;
use crate::integer::Integer;
use core::ops::RemAssign;

// RemAssign          The remainder assignment operator %=.

// ['Integer', 'Integer', 'Integer::remainder_assign', 'no', [], ['ref']]
impl RemAssign<Integer> for Integer {
    fn rem_assign(&mut self, rhs: Integer) {
        Integer::remainder_assign(self, &rhs)
    }
}

// ['Integer', '&Integer', 'Integer::remainder_assign', 'no', [], []]
impl RemAssign<&Integer> for Integer {
    fn rem_assign(&mut self, rhs: &Integer) {
        Integer::remainder_assign(self, rhs)
    }
}

// ['Integer', 'i8', 'Integer::remainder_c_long_assign', 'no', [], []]
impl RemAssign<i8> for Integer {
    fn rem_assign(&mut self, rhs: i8) {
        Integer::remainder_c_long_assign(self, rhs)
    }
}

// ['Integer', '&i8', 'Integer::remainder_c_long_assign', 'no', [], ['deref']]
impl RemAssign<&i8> for Integer {
    fn rem_assign(&mut self, rhs: &i8) {
        Integer::remainder_c_long_assign(self, *rhs)
    }
}

// ['i8', 'Integer', 'c_long_remainder_assign', 'no', [], ['ref']]
impl RemAssign<Integer> for i8 {
    fn rem_assign(&mut self, rhs: Integer) {
        c_long_remainder_assign(self, &rhs)
    }
}

// ['i8', '&Integer', 'c_long_remainder_assign', 'no', [], []]
impl RemAssign<&Integer> for i8 {
    fn rem_assign(&mut self, rhs: &Integer) {
        c_long_remainder_assign(self, rhs)
    }
}

// ['Integer', 'u8', 'Integer::remainder_c_long_assign', 'no', [], []]
impl RemAssign<u8> for Integer {
    fn rem_assign(&mut self, rhs: u8) {
        Integer::remainder_c_long_assign(self, rhs)
    }
}

// ['Integer', '&u8', 'Integer::remainder_c_long_assign', 'no', [], ['deref']]
impl RemAssign<&u8> for Integer {
    fn rem_assign(&mut self, rhs: &u8) {
        Integer::remainder_c_long_assign(self, *rhs)
    }
}

// ['u8', 'Integer', 'c_long_remainder_assign', 'no', [], ['ref']]
impl RemAssign<Integer> for u8 {
    fn rem_assign(&mut self, rhs: Integer) {
        c_long_remainder_assign(self, &rhs)
    }
}

// ['u8', '&Integer', 'c_long_remainder_assign', 'no', [], []]
impl RemAssign<&Integer> for u8 {
    fn rem_assign(&mut self, rhs: &Integer) {
        c_long_remainder_assign(self, rhs)
    }
}

// ['Integer', 'i16', 'Integer::remainder_c_long_assign', 'no', [], []]
impl RemAssign<i16> for Integer {
    fn rem_assign(&mut self, rhs: i16) {
        Integer::remainder_c_long_assign(self, rhs)
    }
}

// ['Integer', '&i16', 'Integer::remainder_c_long_assign', 'no', [], ['deref']]
impl RemAssign<&i16> for Integer {
    fn rem_assign(&mut self, rhs: &i16) {
        Integer::remainder_c_long_assign(self, *rhs)
    }
}

// ['i16', 'Integer', 'c_long_remainder_assign', 'no', [], ['ref']]
impl RemAssign<Integer> for i16 {
    fn rem_assign(&mut self, rhs: Integer) {
        c_long_remainder_assign(self, &rhs)
    }
}

// ['i16', '&Integer', 'c_long_remainder_assign', 'no', [], []]
impl RemAssign<&Integer> for i16 {
    fn rem_assign(&mut self, rhs: &Integer) {
        c_long_remainder_assign(self, rhs)
    }
}

// ['Integer', 'u16', 'Integer::remainder_c_long_assign', 'no', [], []]
impl RemAssign<u16> for Integer {
    fn rem_assign(&mut self, rhs: u16) {
        Integer::remainder_c_long_assign(self, rhs)
    }
}

// ['Integer', '&u16', 'Integer::remainder_c_long_assign', 'no', [], ['deref']]
impl RemAssign<&u16> for Integer {
    fn rem_assign(&mut self, rhs: &u16) {
        Integer::remainder_c_long_assign(self, *rhs)
    }
}

// ['u16', 'Integer', 'c_long_remainder_assign', 'no', [], ['ref']]
impl RemAssign<Integer> for u16 {
    fn rem_assign(&mut self, rhs: Integer) {
        c_long_remainder_assign(self, &rhs)
    }
}

// ['u16', '&Integer', 'c_long_remainder_assign', 'no', [], []]
impl RemAssign<&Integer> for u16 {
    fn rem_assign(&mut self, rhs: &Integer) {
        c_long_remainder_assign(self, rhs)
    }
}

// ['Integer', 'i32', 'Integer::remainder_c_long_assign', 'no', [], []]
impl RemAssign<i32> for Integer {
    fn rem_assign(&mut self, rhs: i32) {
        Integer::remainder_c_long_assign(self, rhs)
    }
}

// ['Integer', '&i32', 'Integer::remainder_c_long_assign', 'no', [], ['deref']]
impl RemAssign<&i32> for Integer {
    fn rem_assign(&mut self, rhs: &i32) {
        Integer::remainder_c_long_assign(self, *rhs)
    }
}

// ['i32', 'Integer', 'c_long_remainder_assign', 'no', [], ['ref']]
impl RemAssign<Integer> for i32 {
    fn rem_assign(&mut self, rhs: Integer) {
        c_long_remainder_assign(self, &rhs)
    }
}

// ['i32', '&Integer', 'c_long_remainder_assign', 'no', [], []]
impl RemAssign<&Integer> for i32 {
    fn rem_assign(&mut self, rhs: &Integer) {
        c_long_remainder_assign(self, rhs)
    }
}

// ['Integer', 'u32', 'Integer::remainder_c_long_assign', 'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl RemAssign<u32> for Integer {
    fn rem_assign(&mut self, rhs: u32) {
        Integer::remainder_c_long_assign(self, rhs)
    }
}

// ['Integer', '&u32', 'Integer::remainder_c_long_assign', 'no', [], ['deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl RemAssign<&u32> for Integer {
    fn rem_assign(&mut self, rhs: &u32) {
        Integer::remainder_c_long_assign(self, *rhs)
    }
}

// ['Integer', 'i64', 'Integer::remainder_c_long_assign', 'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl RemAssign<i64> for Integer {
    fn rem_assign(&mut self, rhs: i64) {
        Integer::remainder_c_long_assign(self, rhs)
    }
}

// ['Integer', '&i64', 'Integer::remainder_c_long_assign', 'no', [], ['deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl RemAssign<&i64> for Integer {
    fn rem_assign(&mut self, rhs: &i64) {
        Integer::remainder_c_long_assign(self, *rhs)
    }
}

// ['Integer', 'u32', 'Integer::remainder_assign', 'no', [], ['ref', {'convert':
// 'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl RemAssign<u32> for Integer {
    fn rem_assign(&mut self, rhs: u32) {
        Integer::remainder_assign(self, &Integer::from(rhs))
    }
}

// ['Integer', '&u32', 'Integer::remainder_assign', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl RemAssign<&u32> for Integer {
    fn rem_assign(&mut self, rhs: &u32) {
        Integer::remainder_assign(self, &Integer::from(*rhs))
    }
}

// ['Integer', 'i64', 'Integer::remainder_assign', 'no', [], ['ref', {'convert':
// 'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl RemAssign<i64> for Integer {
    fn rem_assign(&mut self, rhs: i64) {
        Integer::remainder_assign(self, &Integer::from(rhs))
    }
}

// ['Integer', '&i64', 'Integer::remainder_assign', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl RemAssign<&i64> for Integer {
    fn rem_assign(&mut self, rhs: &i64) {
        Integer::remainder_assign(self, &Integer::from(*rhs))
    }
}

// ['u32', 'Integer', 'c_long_remainder_assign', 'no', [], ['ref']]
impl RemAssign<Integer> for u32 {
    fn rem_assign(&mut self, rhs: Integer) {
        c_long_remainder_assign(self, &rhs)
    }
}

// ['u32', '&Integer', 'c_long_remainder_assign', 'no', [], []]
impl RemAssign<&Integer> for u32 {
    fn rem_assign(&mut self, rhs: &Integer) {
        c_long_remainder_assign(self, rhs)
    }
}

// ['i64', 'Integer', 'c_long_remainder_assign', 'no', [], ['ref']]
impl RemAssign<Integer> for i64 {
    fn rem_assign(&mut self, rhs: Integer) {
        c_long_remainder_assign(self, &rhs)
    }
}

// ['i64', '&Integer', 'c_long_remainder_assign', 'no', [], []]
impl RemAssign<&Integer> for i64 {
    fn rem_assign(&mut self, rhs: &Integer) {
        c_long_remainder_assign(self, rhs)
    }
}

// ['Integer', 'u64', 'Integer::remainder_assign', 'no', [], ['ref', {'convert':
// 'Integer'}]]
impl RemAssign<u64> for Integer {
    fn rem_assign(&mut self, rhs: u64) {
        Integer::remainder_assign(self, &Integer::from(rhs))
    }
}

// ['Integer', '&u64', 'Integer::remainder_assign', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
impl RemAssign<&u64> for Integer {
    fn rem_assign(&mut self, rhs: &u64) {
        Integer::remainder_assign(self, &Integer::from(*rhs))
    }
}

// ['u64', 'Integer', 'c_long_remainder_assign', 'no', [], ['ref']]
impl RemAssign<Integer> for u64 {
    fn rem_assign(&mut self, rhs: Integer) {
        c_long_remainder_assign(self, &rhs)
    }
}

// ['u64', '&Integer', 'c_long_remainder_assign', 'no', [], []]
impl RemAssign<&Integer> for u64 {
    fn rem_assign(&mut self, rhs: &Integer) {
        c_long_remainder_assign(self, rhs)
    }
}

// ['Integer', 'i128', 'Integer::remainder_assign', 'no', [], ['ref',
// {'convert': 'Integer'}]]
impl RemAssign<i128> for Integer {
    fn rem_assign(&mut self, rhs: i128) {
        Integer::remainder_assign(self, &Integer::from(rhs))
    }
}

// ['Integer', '&i128', 'Integer::remainder_assign', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
impl RemAssign<&i128> for Integer {
    fn rem_assign(&mut self, rhs: &i128) {
        Integer::remainder_assign(self, &Integer::from(*rhs))
    }
}

// ['i128', 'Integer', 'c_long_remainder_assign', 'no', [], ['ref']]
impl RemAssign<Integer> for i128 {
    fn rem_assign(&mut self, rhs: Integer) {
        c_long_remainder_assign(self, &rhs)
    }
}

// ['i128', '&Integer', 'c_long_remainder_assign', 'no', [], []]
impl RemAssign<&Integer> for i128 {
    fn rem_assign(&mut self, rhs: &Integer) {
        c_long_remainder_assign(self, rhs)
    }
}

// ['Integer', 'u128', 'Integer::remainder_assign', 'no', [], ['ref',
// {'convert': 'Integer'}]]
impl RemAssign<u128> for Integer {
    fn rem_assign(&mut self, rhs: u128) {
        Integer::remainder_assign(self, &Integer::from(rhs))
    }
}

// ['Integer', '&u128', 'Integer::remainder_assign', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
impl RemAssign<&u128> for Integer {
    fn rem_assign(&mut self, rhs: &u128) {
        Integer::remainder_assign(self, &Integer::from(*rhs))
    }
}

// ['u128', 'Integer', 'c_long_remainder_assign', 'no', [], ['ref']]
impl RemAssign<Integer> for u128 {
    fn rem_assign(&mut self, rhs: Integer) {
        c_long_remainder_assign(self, &rhs)
    }
}

// ['u128', '&Integer', 'c_long_remainder_assign', 'no', [], []]
impl RemAssign<&Integer> for u128 {
    fn rem_assign(&mut self, rhs: &Integer) {
        c_long_remainder_assign(self, rhs)
    }
}
