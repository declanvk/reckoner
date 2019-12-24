use crate::integer::Integer;
use core::ops::DivAssign;

// DivAssign          The division assignment operator /=.

// [u'Integer', u'Integer', u'Integer::divide_assign', u'no', [], [u'ref']]
impl DivAssign<Integer> for Integer {
    fn div_assign(&mut self, rhs: Integer) {
        Integer::divide_assign(self, &rhs)
    }
}

// [u'Integer', u'&Integer', u'Integer::divide_assign', u'no', [], []]
impl DivAssign<&Integer> for Integer {
    fn div_assign(&mut self, rhs: &Integer) {
        Integer::divide_assign(self, rhs)
    }
}

// [u'Integer', u'i8', u'Integer::divide_c_long_assign', u'no', [], []]
impl DivAssign<i8> for Integer {
    fn div_assign(&mut self, rhs: i8) {
        Integer::divide_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&i8', u'Integer::divide_c_long_assign', u'no', [], [u'deref']]
impl DivAssign<&i8> for Integer {
    fn div_assign(&mut self, rhs: &i8) {
        Integer::divide_c_long_assign(self, *rhs)
    }
}

// [u'Integer', u'u8', u'Integer::divide_c_long_assign', u'no', [], []]
impl DivAssign<u8> for Integer {
    fn div_assign(&mut self, rhs: u8) {
        Integer::divide_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&u8', u'Integer::divide_c_long_assign', u'no', [], [u'deref']]
impl DivAssign<&u8> for Integer {
    fn div_assign(&mut self, rhs: &u8) {
        Integer::divide_c_long_assign(self, *rhs)
    }
}

// [u'Integer', u'i16', u'Integer::divide_c_long_assign', u'no', [], []]
impl DivAssign<i16> for Integer {
    fn div_assign(&mut self, rhs: i16) {
        Integer::divide_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&i16', u'Integer::divide_c_long_assign', u'no', [],
// [u'deref']]
impl DivAssign<&i16> for Integer {
    fn div_assign(&mut self, rhs: &i16) {
        Integer::divide_c_long_assign(self, *rhs)
    }
}

// [u'Integer', u'u16', u'Integer::divide_c_long_assign', u'no', [], []]
impl DivAssign<u16> for Integer {
    fn div_assign(&mut self, rhs: u16) {
        Integer::divide_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&u16', u'Integer::divide_c_long_assign', u'no', [],
// [u'deref']]
impl DivAssign<&u16> for Integer {
    fn div_assign(&mut self, rhs: &u16) {
        Integer::divide_c_long_assign(self, *rhs)
    }
}

// [u'Integer', u'i32', u'Integer::divide_c_long_assign', u'no', [], []]
impl DivAssign<i32> for Integer {
    fn div_assign(&mut self, rhs: i32) {
        Integer::divide_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&i32', u'Integer::divide_c_long_assign', u'no', [],
// [u'deref']]
impl DivAssign<&i32> for Integer {
    fn div_assign(&mut self, rhs: &i32) {
        Integer::divide_c_long_assign(self, *rhs)
    }
}

// [u'Integer', u'u32', u'Integer::divide_c_long_assign', u'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl DivAssign<u32> for Integer {
    fn div_assign(&mut self, rhs: u32) {
        Integer::divide_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&u32', u'Integer::divide_c_long_assign', u'no', [],
// [u'deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl DivAssign<&u32> for Integer {
    fn div_assign(&mut self, rhs: &u32) {
        Integer::divide_c_long_assign(self, *rhs)
    }
}

// [u'Integer', u'i64', u'Integer::divide_c_long_assign', u'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl DivAssign<i64> for Integer {
    fn div_assign(&mut self, rhs: i64) {
        Integer::divide_c_long_assign(self, rhs)
    }
}

// [u'Integer', u'&i64', u'Integer::divide_c_long_assign', u'no', [],
// [u'deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl DivAssign<&i64> for Integer {
    fn div_assign(&mut self, rhs: &i64) {
        Integer::divide_c_long_assign(self, *rhs)
    }
}

// [u'Integer', u'u32', u'Integer::divide_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl DivAssign<u32> for Integer {
    fn div_assign(&mut self, rhs: u32) {
        Integer::divide_assign(self, &Integer::from(rhs))
    }
}

// [u'Integer', u'&u32', u'Integer::divide_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl DivAssign<&u32> for Integer {
    fn div_assign(&mut self, rhs: &u32) {
        Integer::divide_assign(self, &Integer::from(*rhs))
    }
}

// [u'Integer', u'i64', u'Integer::divide_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl DivAssign<i64> for Integer {
    fn div_assign(&mut self, rhs: i64) {
        Integer::divide_assign(self, &Integer::from(rhs))
    }
}

// [u'Integer', u'&i64', u'Integer::divide_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl DivAssign<&i64> for Integer {
    fn div_assign(&mut self, rhs: &i64) {
        Integer::divide_assign(self, &Integer::from(*rhs))
    }
}

// [u'Integer', u'u64', u'Integer::divide_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}]]
impl DivAssign<u64> for Integer {
    fn div_assign(&mut self, rhs: u64) {
        Integer::divide_assign(self, &Integer::from(rhs))
    }
}

// [u'Integer', u'&u64', u'Integer::divide_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
impl DivAssign<&u64> for Integer {
    fn div_assign(&mut self, rhs: &u64) {
        Integer::divide_assign(self, &Integer::from(*rhs))
    }
}

// [u'Integer', u'i128', u'Integer::divide_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}]]
impl DivAssign<i128> for Integer {
    fn div_assign(&mut self, rhs: i128) {
        Integer::divide_assign(self, &Integer::from(rhs))
    }
}

// [u'Integer', u'&i128', u'Integer::divide_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
impl DivAssign<&i128> for Integer {
    fn div_assign(&mut self, rhs: &i128) {
        Integer::divide_assign(self, &Integer::from(*rhs))
    }
}

// [u'Integer', u'u128', u'Integer::divide_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}]]
impl DivAssign<u128> for Integer {
    fn div_assign(&mut self, rhs: u128) {
        Integer::divide_assign(self, &Integer::from(rhs))
    }
}

// [u'Integer', u'&u128', u'Integer::divide_assign', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
impl DivAssign<&u128> for Integer {
    fn div_assign(&mut self, rhs: &u128) {
        Integer::divide_assign(self, &Integer::from(*rhs))
    }
}
