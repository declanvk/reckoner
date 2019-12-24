use super::helpers::c_long_remainder_assign;
use crate::integer::Integer;
use core::ops::RemAssign;

// RemAssign          The remainder assignment operator %=.

// [u'Integer', u'Integer', u'Integer::remainder_assign', u'no', [], [u'ref']]
impl RemAssign<Integer> for Integer {
    fn rem_assign(&mut self, rhs: Integer) {
        Integer::remainder_assign(self, &rhs)
    }
}

// [u'Integer', u'&Integer', u'Integer::remainder_assign', u'no', [], []]
impl RemAssign<&Integer> for Integer {
    fn rem_assign(&mut self, rhs: &Integer) {
        Integer::remainder_assign(self, rhs)
    }
}

// [u'Integer', u'i8', u'Integer::remainder_c_long_assign', u'no', [], []]
impl RemAssign<i8> for Integer {
    fn rem_assign(&mut self, rhs: i8) {
        Integer::remainder_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&i8', u'Integer::remainder_c_long_assign', u'no', [],
// [u'deref']]
impl RemAssign<&i8> for Integer {
    fn rem_assign(&mut self, rhs: &i8) {
        Integer::remainder_c_long_assign(self, *rhs)
    }
}

// [u'i8', u'Integer', u'c_long_remainder_assign', u'no', [], [u'ref']]
impl RemAssign<Integer> for i8 {
    fn rem_assign(&mut self, rhs: Integer) {
        c_long_remainder_assign(self, &rhs)
    }
}

// [u'i8', u'&Integer', u'c_long_remainder_assign', u'no', [], []]
impl RemAssign<&Integer> for i8 {
    fn rem_assign(&mut self, rhs: &Integer) {
        c_long_remainder_assign(self, rhs)
    }
}

// [u'Integer', u'u8', u'Integer::remainder_c_long_assign', u'no', [], []]
impl RemAssign<u8> for Integer {
    fn rem_assign(&mut self, rhs: u8) {
        Integer::remainder_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&u8', u'Integer::remainder_c_long_assign', u'no', [],
// [u'deref']]
impl RemAssign<&u8> for Integer {
    fn rem_assign(&mut self, rhs: &u8) {
        Integer::remainder_c_long_assign(self, *rhs)
    }
}

// [u'u8', u'Integer', u'c_long_remainder_assign', u'no', [], [u'ref']]
impl RemAssign<Integer> for u8 {
    fn rem_assign(&mut self, rhs: Integer) {
        c_long_remainder_assign(self, &rhs)
    }
}

// [u'u8', u'&Integer', u'c_long_remainder_assign', u'no', [], []]
impl RemAssign<&Integer> for u8 {
    fn rem_assign(&mut self, rhs: &Integer) {
        c_long_remainder_assign(self, rhs)
    }
}

// [u'Integer', u'i16', u'Integer::remainder_c_long_assign', u'no', [], []]
impl RemAssign<i16> for Integer {
    fn rem_assign(&mut self, rhs: i16) {
        Integer::remainder_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&i16', u'Integer::remainder_c_long_assign', u'no', [],
// [u'deref']]
impl RemAssign<&i16> for Integer {
    fn rem_assign(&mut self, rhs: &i16) {
        Integer::remainder_c_long_assign(self, *rhs)
    }
}

// [u'i16', u'Integer', u'c_long_remainder_assign', u'no', [], [u'ref']]
impl RemAssign<Integer> for i16 {
    fn rem_assign(&mut self, rhs: Integer) {
        c_long_remainder_assign(self, &rhs)
    }
}

// [u'i16', u'&Integer', u'c_long_remainder_assign', u'no', [], []]
impl RemAssign<&Integer> for i16 {
    fn rem_assign(&mut self, rhs: &Integer) {
        c_long_remainder_assign(self, rhs)
    }
}

// [u'Integer', u'u16', u'Integer::remainder_c_long_assign', u'no', [], []]
impl RemAssign<u16> for Integer {
    fn rem_assign(&mut self, rhs: u16) {
        Integer::remainder_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&u16', u'Integer::remainder_c_long_assign', u'no', [],
// [u'deref']]
impl RemAssign<&u16> for Integer {
    fn rem_assign(&mut self, rhs: &u16) {
        Integer::remainder_c_long_assign(self, *rhs)
    }
}

// [u'u16', u'Integer', u'c_long_remainder_assign', u'no', [], [u'ref']]
impl RemAssign<Integer> for u16 {
    fn rem_assign(&mut self, rhs: Integer) {
        c_long_remainder_assign(self, &rhs)
    }
}

// [u'u16', u'&Integer', u'c_long_remainder_assign', u'no', [], []]
impl RemAssign<&Integer> for u16 {
    fn rem_assign(&mut self, rhs: &Integer) {
        c_long_remainder_assign(self, rhs)
    }
}

// [u'Integer', u'i32', u'Integer::remainder_c_long_assign', u'no', [], []]
impl RemAssign<i32> for Integer {
    fn rem_assign(&mut self, rhs: i32) {
        Integer::remainder_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&i32', u'Integer::remainder_c_long_assign', u'no', [],
// [u'deref']]
impl RemAssign<&i32> for Integer {
    fn rem_assign(&mut self, rhs: &i32) {
        Integer::remainder_c_long_assign(self, *rhs)
    }
}

// [u'i32', u'Integer', u'c_long_remainder_assign', u'no', [], [u'ref']]
impl RemAssign<Integer> for i32 {
    fn rem_assign(&mut self, rhs: Integer) {
        c_long_remainder_assign(self, &rhs)
    }
}

// [u'i32', u'&Integer', u'c_long_remainder_assign', u'no', [], []]
impl RemAssign<&Integer> for i32 {
    fn rem_assign(&mut self, rhs: &Integer) {
        c_long_remainder_assign(self, rhs)
    }
}

// [u'Integer', u'u32', u'Integer::remainder_c_long_assign', u'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl RemAssign<u32> for Integer {
    fn rem_assign(&mut self, rhs: u32) {
        Integer::remainder_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&u32', u'Integer::remainder_c_long_assign', u'no', [],
// [u'deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl RemAssign<&u32> for Integer {
    fn rem_assign(&mut self, rhs: &u32) {
        Integer::remainder_c_long_assign(self, *rhs)
    }
}

// [u'Integer', u'i64', u'Integer::remainder_c_long_assign', u'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl RemAssign<i64> for Integer {
    fn rem_assign(&mut self, rhs: i64) {
        Integer::remainder_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&i64', u'Integer::remainder_c_long_assign', u'no', [],
// [u'deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl RemAssign<&i64> for Integer {
    fn rem_assign(&mut self, rhs: &i64) {
        Integer::remainder_c_long_assign(self, *rhs)
    }
}

// [u'Integer', u'u32', u'Integer::remainder_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl RemAssign<u32> for Integer {
    fn rem_assign(&mut self, rhs: u32) {
        Integer::remainder_assign(self, &Integer::from(rhs))
    }
}

// [u'Integer', u'&u32', u'Integer::remainder_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl RemAssign<&u32> for Integer {
    fn rem_assign(&mut self, rhs: &u32) {
        Integer::remainder_assign(self, &Integer::from(*rhs))
    }
}

// [u'Integer', u'i64', u'Integer::remainder_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl RemAssign<i64> for Integer {
    fn rem_assign(&mut self, rhs: i64) {
        Integer::remainder_assign(self, &Integer::from(rhs))
    }
}

// [u'Integer', u'&i64', u'Integer::remainder_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl RemAssign<&i64> for Integer {
    fn rem_assign(&mut self, rhs: &i64) {
        Integer::remainder_assign(self, &Integer::from(*rhs))
    }
}

// [u'u32', u'Integer', u'c_long_remainder_assign', u'no', [], [u'ref']]
impl RemAssign<Integer> for u32 {
    fn rem_assign(&mut self, rhs: Integer) {
        c_long_remainder_assign(self, &rhs)
    }
}

// [u'u32', u'&Integer', u'c_long_remainder_assign', u'no', [], []]
impl RemAssign<&Integer> for u32 {
    fn rem_assign(&mut self, rhs: &Integer) {
        c_long_remainder_assign(self, rhs)
    }
}

// [u'i64', u'Integer', u'c_long_remainder_assign', u'no', [], [u'ref']]
impl RemAssign<Integer> for i64 {
    fn rem_assign(&mut self, rhs: Integer) {
        c_long_remainder_assign(self, &rhs)
    }
}

// [u'i64', u'&Integer', u'c_long_remainder_assign', u'no', [], []]
impl RemAssign<&Integer> for i64 {
    fn rem_assign(&mut self, rhs: &Integer) {
        c_long_remainder_assign(self, rhs)
    }
}

// [u'Integer', u'u64', u'Integer::remainder_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}]]
impl RemAssign<u64> for Integer {
    fn rem_assign(&mut self, rhs: u64) {
        Integer::remainder_assign(self, &Integer::from(rhs))
    }
}

// [u'Integer', u'&u64', u'Integer::remainder_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
impl RemAssign<&u64> for Integer {
    fn rem_assign(&mut self, rhs: &u64) {
        Integer::remainder_assign(self, &Integer::from(*rhs))
    }
}

// [u'u64', u'Integer', u'c_long_remainder_assign', u'no', [], [u'ref']]
impl RemAssign<Integer> for u64 {
    fn rem_assign(&mut self, rhs: Integer) {
        c_long_remainder_assign(self, &rhs)
    }
}

// [u'u64', u'&Integer', u'c_long_remainder_assign', u'no', [], []]
impl RemAssign<&Integer> for u64 {
    fn rem_assign(&mut self, rhs: &Integer) {
        c_long_remainder_assign(self, rhs)
    }
}

// [u'Integer', u'i128', u'Integer::remainder_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}]]
impl RemAssign<i128> for Integer {
    fn rem_assign(&mut self, rhs: i128) {
        Integer::remainder_assign(self, &Integer::from(rhs))
    }
}

// [u'Integer', u'&i128', u'Integer::remainder_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
impl RemAssign<&i128> for Integer {
    fn rem_assign(&mut self, rhs: &i128) {
        Integer::remainder_assign(self, &Integer::from(*rhs))
    }
}

// [u'i128', u'Integer', u'c_long_remainder_assign', u'no', [], [u'ref']]
impl RemAssign<Integer> for i128 {
    fn rem_assign(&mut self, rhs: Integer) {
        c_long_remainder_assign(self, &rhs)
    }
}

// [u'i128', u'&Integer', u'c_long_remainder_assign', u'no', [], []]
impl RemAssign<&Integer> for i128 {
    fn rem_assign(&mut self, rhs: &Integer) {
        c_long_remainder_assign(self, rhs)
    }
}

// [u'Integer', u'u128', u'Integer::remainder_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}]]
impl RemAssign<u128> for Integer {
    fn rem_assign(&mut self, rhs: u128) {
        Integer::remainder_assign(self, &Integer::from(rhs))
    }
}

// [u'Integer', u'&u128', u'Integer::remainder_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
impl RemAssign<&u128> for Integer {
    fn rem_assign(&mut self, rhs: &u128) {
        Integer::remainder_assign(self, &Integer::from(*rhs))
    }
}

// [u'u128', u'Integer', u'c_long_remainder_assign', u'no', [], [u'ref']]
impl RemAssign<Integer> for u128 {
    fn rem_assign(&mut self, rhs: Integer) {
        c_long_remainder_assign(self, &rhs)
    }
}

// [u'u128', u'&Integer', u'c_long_remainder_assign', u'no', [], []]
impl RemAssign<&Integer> for u128 {
    fn rem_assign(&mut self, rhs: &Integer) {
        c_long_remainder_assign(self, rhs)
    }
}
