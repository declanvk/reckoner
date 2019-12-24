use crate::integer::Integer;
use core::ops::AddAssign;

// AddAssign    The addition assignment operator +=.

// [u'Integer', u'Integer', u'Integer::add_assign', u'no', [], [u'ref']]
impl AddAssign<Integer> for Integer {
    fn add_assign(&mut self, rhs: Integer) {
        Integer::add_assign(self, &rhs)
    }
}

// [u'Integer', u'&Integer', u'Integer::add_assign', u'no', [], []]
impl AddAssign<&Integer> for Integer {
    fn add_assign(&mut self, rhs: &Integer) {
        Integer::add_assign(self, rhs)
    }
}

// [u'Integer', u'i8', u'Integer::add_c_long_assign', u'no', [], []]
impl AddAssign<i8> for Integer {
    fn add_assign(&mut self, rhs: i8) {
        Integer::add_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&i8', u'Integer::add_c_long_assign', u'no', [], [u'deref']]
impl AddAssign<&i8> for Integer {
    fn add_assign(&mut self, rhs: &i8) {
        Integer::add_c_long_assign(self, *rhs)
    }
}

// [u'Integer', u'u8', u'Integer::add_c_long_assign', u'no', [], []]
impl AddAssign<u8> for Integer {
    fn add_assign(&mut self, rhs: u8) {
        Integer::add_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&u8', u'Integer::add_c_long_assign', u'no', [], [u'deref']]
impl AddAssign<&u8> for Integer {
    fn add_assign(&mut self, rhs: &u8) {
        Integer::add_c_long_assign(self, *rhs)
    }
}

// [u'Integer', u'i16', u'Integer::add_c_long_assign', u'no', [], []]
impl AddAssign<i16> for Integer {
    fn add_assign(&mut self, rhs: i16) {
        Integer::add_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&i16', u'Integer::add_c_long_assign', u'no', [], [u'deref']]
impl AddAssign<&i16> for Integer {
    fn add_assign(&mut self, rhs: &i16) {
        Integer::add_c_long_assign(self, *rhs)
    }
}

// [u'Integer', u'u16', u'Integer::add_c_long_assign', u'no', [], []]
impl AddAssign<u16> for Integer {
    fn add_assign(&mut self, rhs: u16) {
        Integer::add_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&u16', u'Integer::add_c_long_assign', u'no', [], [u'deref']]
impl AddAssign<&u16> for Integer {
    fn add_assign(&mut self, rhs: &u16) {
        Integer::add_c_long_assign(self, *rhs)
    }
}

// [u'Integer', u'i32', u'Integer::add_c_long_assign', u'no', [], []]
impl AddAssign<i32> for Integer {
    fn add_assign(&mut self, rhs: i32) {
        Integer::add_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&i32', u'Integer::add_c_long_assign', u'no', [], [u'deref']]
impl AddAssign<&i32> for Integer {
    fn add_assign(&mut self, rhs: &i32) {
        Integer::add_c_long_assign(self, *rhs)
    }
}

// [u'Integer', u'u32', u'Integer::add_c_long_assign', u'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl AddAssign<u32> for Integer {
    fn add_assign(&mut self, rhs: u32) {
        Integer::add_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&u32', u'Integer::add_c_long_assign', u'no', [], [u'deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl AddAssign<&u32> for Integer {
    fn add_assign(&mut self, rhs: &u32) {
        Integer::add_c_long_assign(self, *rhs)
    }
}

// [u'Integer', u'i64', u'Integer::add_c_long_assign', u'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl AddAssign<i64> for Integer {
    fn add_assign(&mut self, rhs: i64) {
        Integer::add_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&i64', u'Integer::add_c_long_assign', u'no', [], [u'deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl AddAssign<&i64> for Integer {
    fn add_assign(&mut self, rhs: &i64) {
        Integer::add_c_long_assign(self, *rhs)
    }
}

// [u'Integer', u'u32', u'Integer::add_assign', u'no', [], [u'ref', {u'convert':
// u'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl AddAssign<u32> for Integer {
    fn add_assign(&mut self, rhs: u32) {
        Integer::add_assign(self, &Integer::from(rhs))
    }
}

// [u'Integer', u'&u32', u'Integer::add_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl AddAssign<&u32> for Integer {
    fn add_assign(&mut self, rhs: &u32) {
        Integer::add_assign(self, &Integer::from(*rhs))
    }
}

// [u'Integer', u'i64', u'Integer::add_assign', u'no', [], [u'ref', {u'convert':
// u'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl AddAssign<i64> for Integer {
    fn add_assign(&mut self, rhs: i64) {
        Integer::add_assign(self, &Integer::from(rhs))
    }
}

// [u'Integer', u'&i64', u'Integer::add_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl AddAssign<&i64> for Integer {
    fn add_assign(&mut self, rhs: &i64) {
        Integer::add_assign(self, &Integer::from(*rhs))
    }
}

// [u'Integer', u'u64', u'Integer::add_assign', u'no', [], [u'ref', {u'convert':
// u'Integer'}]]
impl AddAssign<u64> for Integer {
    fn add_assign(&mut self, rhs: u64) {
        Integer::add_assign(self, &Integer::from(rhs))
    }
}

// [u'Integer', u'&u64', u'Integer::add_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
impl AddAssign<&u64> for Integer {
    fn add_assign(&mut self, rhs: &u64) {
        Integer::add_assign(self, &Integer::from(*rhs))
    }
}

// [u'Integer', u'i128', u'Integer::add_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}]]
impl AddAssign<i128> for Integer {
    fn add_assign(&mut self, rhs: i128) {
        Integer::add_assign(self, &Integer::from(rhs))
    }
}

// [u'Integer', u'&i128', u'Integer::add_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
impl AddAssign<&i128> for Integer {
    fn add_assign(&mut self, rhs: &i128) {
        Integer::add_assign(self, &Integer::from(*rhs))
    }
}

// [u'Integer', u'u128', u'Integer::add_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}]]
impl AddAssign<u128> for Integer {
    fn add_assign(&mut self, rhs: u128) {
        Integer::add_assign(self, &Integer::from(rhs))
    }
}

// [u'Integer', u'&u128', u'Integer::add_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
impl AddAssign<&u128> for Integer {
    fn add_assign(&mut self, rhs: &u128) {
        Integer::add_assign(self, &Integer::from(*rhs))
    }
}
