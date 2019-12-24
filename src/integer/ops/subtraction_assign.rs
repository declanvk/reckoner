use crate::integer::Integer;
use core::ops::SubAssign;

// SubAssign          The subtraction assignment operator -=.

// [u'Integer', u'Integer', u'Integer::subtract_assign', u'no', [], [u'ref']]
impl SubAssign<Integer> for Integer {
    fn sub_assign(&mut self, rhs: Integer) {
        Integer::subtract_assign(self, &rhs)
    }
}

// [u'Integer', u'&Integer', u'Integer::subtract_assign', u'no', [], []]
impl SubAssign<&Integer> for Integer {
    fn sub_assign(&mut self, rhs: &Integer) {
        Integer::subtract_assign(self, rhs)
    }
}

// [u'Integer', u'i8', u'Integer::subtract_c_long_assign', u'no', [], []]
impl SubAssign<i8> for Integer {
    fn sub_assign(&mut self, rhs: i8) {
        Integer::subtract_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&i8', u'Integer::subtract_c_long_assign', u'no', [],
// [u'deref']]
impl SubAssign<&i8> for Integer {
    fn sub_assign(&mut self, rhs: &i8) {
        Integer::subtract_c_long_assign(self, *rhs)
    }
}

// [u'Integer', u'u8', u'Integer::subtract_c_long_assign', u'no', [], []]
impl SubAssign<u8> for Integer {
    fn sub_assign(&mut self, rhs: u8) {
        Integer::subtract_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&u8', u'Integer::subtract_c_long_assign', u'no', [],
// [u'deref']]
impl SubAssign<&u8> for Integer {
    fn sub_assign(&mut self, rhs: &u8) {
        Integer::subtract_c_long_assign(self, *rhs)
    }
}

// [u'Integer', u'i16', u'Integer::subtract_c_long_assign', u'no', [], []]
impl SubAssign<i16> for Integer {
    fn sub_assign(&mut self, rhs: i16) {
        Integer::subtract_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&i16', u'Integer::subtract_c_long_assign', u'no', [],
// [u'deref']]
impl SubAssign<&i16> for Integer {
    fn sub_assign(&mut self, rhs: &i16) {
        Integer::subtract_c_long_assign(self, *rhs)
    }
}

// [u'Integer', u'u16', u'Integer::subtract_c_long_assign', u'no', [], []]
impl SubAssign<u16> for Integer {
    fn sub_assign(&mut self, rhs: u16) {
        Integer::subtract_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&u16', u'Integer::subtract_c_long_assign', u'no', [],
// [u'deref']]
impl SubAssign<&u16> for Integer {
    fn sub_assign(&mut self, rhs: &u16) {
        Integer::subtract_c_long_assign(self, *rhs)
    }
}

// [u'Integer', u'i32', u'Integer::subtract_c_long_assign', u'no', [], []]
impl SubAssign<i32> for Integer {
    fn sub_assign(&mut self, rhs: i32) {
        Integer::subtract_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&i32', u'Integer::subtract_c_long_assign', u'no', [],
// [u'deref']]
impl SubAssign<&i32> for Integer {
    fn sub_assign(&mut self, rhs: &i32) {
        Integer::subtract_c_long_assign(self, *rhs)
    }
}

// [u'Integer', u'u32', u'Integer::subtract_c_long_assign', u'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl SubAssign<u32> for Integer {
    fn sub_assign(&mut self, rhs: u32) {
        Integer::subtract_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&u32', u'Integer::subtract_c_long_assign', u'no', [],
// [u'deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl SubAssign<&u32> for Integer {
    fn sub_assign(&mut self, rhs: &u32) {
        Integer::subtract_c_long_assign(self, *rhs)
    }
}

// [u'Integer', u'i64', u'Integer::subtract_c_long_assign', u'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl SubAssign<i64> for Integer {
    fn sub_assign(&mut self, rhs: i64) {
        Integer::subtract_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&i64', u'Integer::subtract_c_long_assign', u'no', [],
// [u'deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl SubAssign<&i64> for Integer {
    fn sub_assign(&mut self, rhs: &i64) {
        Integer::subtract_c_long_assign(self, *rhs)
    }
}

// [u'Integer', u'u32', u'Integer::subtract_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl SubAssign<u32> for Integer {
    fn sub_assign(&mut self, rhs: u32) {
        Integer::subtract_assign(self, &Integer::from(rhs))
    }
}

// [u'Integer', u'&u32', u'Integer::subtract_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl SubAssign<&u32> for Integer {
    fn sub_assign(&mut self, rhs: &u32) {
        Integer::subtract_assign(self, &Integer::from(*rhs))
    }
}

// [u'Integer', u'i64', u'Integer::subtract_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl SubAssign<i64> for Integer {
    fn sub_assign(&mut self, rhs: i64) {
        Integer::subtract_assign(self, &Integer::from(rhs))
    }
}

// [u'Integer', u'&i64', u'Integer::subtract_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl SubAssign<&i64> for Integer {
    fn sub_assign(&mut self, rhs: &i64) {
        Integer::subtract_assign(self, &Integer::from(*rhs))
    }
}

// [u'Integer', u'u64', u'Integer::subtract_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}]]
impl SubAssign<u64> for Integer {
    fn sub_assign(&mut self, rhs: u64) {
        Integer::subtract_assign(self, &Integer::from(rhs))
    }
}

// [u'Integer', u'&u64', u'Integer::subtract_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
impl SubAssign<&u64> for Integer {
    fn sub_assign(&mut self, rhs: &u64) {
        Integer::subtract_assign(self, &Integer::from(*rhs))
    }
}

// [u'Integer', u'i128', u'Integer::subtract_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}]]
impl SubAssign<i128> for Integer {
    fn sub_assign(&mut self, rhs: i128) {
        Integer::subtract_assign(self, &Integer::from(rhs))
    }
}

// [u'Integer', u'&i128', u'Integer::subtract_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
impl SubAssign<&i128> for Integer {
    fn sub_assign(&mut self, rhs: &i128) {
        Integer::subtract_assign(self, &Integer::from(*rhs))
    }
}

// [u'Integer', u'u128', u'Integer::subtract_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}]]
impl SubAssign<u128> for Integer {
    fn sub_assign(&mut self, rhs: u128) {
        Integer::subtract_assign(self, &Integer::from(rhs))
    }
}

// [u'Integer', u'&u128', u'Integer::subtract_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
impl SubAssign<&u128> for Integer {
    fn sub_assign(&mut self, rhs: &u128) {
        Integer::subtract_assign(self, &Integer::from(*rhs))
    }
}
