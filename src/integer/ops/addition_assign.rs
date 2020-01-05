use crate::integer::Integer;
use core::ops::AddAssign;

// AddAssign    The addition assignment operator +=.

// ['Integer', 'Integer', 'Integer::add_assign', 'no', [], ['ref']]
impl AddAssign<Integer> for Integer {
    fn add_assign(&mut self, rhs: Integer) {
        Integer::add_assign(self, &rhs)
    }
}

// ['Integer', '&Integer', 'Integer::add_assign', 'no', [], []]
impl AddAssign<&Integer> for Integer {
    fn add_assign(&mut self, rhs: &Integer) {
        Integer::add_assign(self, rhs)
    }
}

// ['Integer', 'i8', 'Integer::add_c_long_assign', 'no', [], []]
impl AddAssign<i8> for Integer {
    fn add_assign(&mut self, rhs: i8) {
        Integer::add_c_long_assign(self, rhs)
    }
}

// ['Integer', '&i8', 'Integer::add_c_long_assign', 'no', [], ['deref']]
impl AddAssign<&i8> for Integer {
    fn add_assign(&mut self, rhs: &i8) {
        Integer::add_c_long_assign(self, *rhs)
    }
}

// ['Integer', 'u8', 'Integer::add_c_long_assign', 'no', [], []]
impl AddAssign<u8> for Integer {
    fn add_assign(&mut self, rhs: u8) {
        Integer::add_c_long_assign(self, rhs)
    }
}

// ['Integer', '&u8', 'Integer::add_c_long_assign', 'no', [], ['deref']]
impl AddAssign<&u8> for Integer {
    fn add_assign(&mut self, rhs: &u8) {
        Integer::add_c_long_assign(self, *rhs)
    }
}

// ['Integer', 'i16', 'Integer::add_c_long_assign', 'no', [], []]
impl AddAssign<i16> for Integer {
    fn add_assign(&mut self, rhs: i16) {
        Integer::add_c_long_assign(self, rhs)
    }
}

// ['Integer', '&i16', 'Integer::add_c_long_assign', 'no', [], ['deref']]
impl AddAssign<&i16> for Integer {
    fn add_assign(&mut self, rhs: &i16) {
        Integer::add_c_long_assign(self, *rhs)
    }
}

// ['Integer', 'u16', 'Integer::add_c_long_assign', 'no', [], []]
impl AddAssign<u16> for Integer {
    fn add_assign(&mut self, rhs: u16) {
        Integer::add_c_long_assign(self, rhs)
    }
}

// ['Integer', '&u16', 'Integer::add_c_long_assign', 'no', [], ['deref']]
impl AddAssign<&u16> for Integer {
    fn add_assign(&mut self, rhs: &u16) {
        Integer::add_c_long_assign(self, *rhs)
    }
}

// ['Integer', 'i32', 'Integer::add_c_long_assign', 'no', [], []]
impl AddAssign<i32> for Integer {
    fn add_assign(&mut self, rhs: i32) {
        Integer::add_c_long_assign(self, rhs)
    }
}

// ['Integer', '&i32', 'Integer::add_c_long_assign', 'no', [], ['deref']]
impl AddAssign<&i32> for Integer {
    fn add_assign(&mut self, rhs: &i32) {
        Integer::add_c_long_assign(self, *rhs)
    }
}

// ['Integer', 'u32', 'Integer::add_c_long_assign', 'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl AddAssign<u32> for Integer {
    fn add_assign(&mut self, rhs: u32) {
        Integer::add_c_long_assign(self, rhs)
    }
}

// ['Integer', '&u32', 'Integer::add_c_long_assign', 'no', [], ['deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl AddAssign<&u32> for Integer {
    fn add_assign(&mut self, rhs: &u32) {
        Integer::add_c_long_assign(self, *rhs)
    }
}

// ['Integer', 'i64', 'Integer::add_c_long_assign', 'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl AddAssign<i64> for Integer {
    fn add_assign(&mut self, rhs: i64) {
        Integer::add_c_long_assign(self, rhs)
    }
}

// ['Integer', '&i64', 'Integer::add_c_long_assign', 'no', [], ['deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl AddAssign<&i64> for Integer {
    fn add_assign(&mut self, rhs: &i64) {
        Integer::add_c_long_assign(self, *rhs)
    }
}

// ['Integer', 'u32', 'Integer::add_assign', 'no', [], ['ref', {'convert':
// 'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl AddAssign<u32> for Integer {
    fn add_assign(&mut self, rhs: u32) {
        Integer::add_assign(self, &Integer::from(rhs))
    }
}

// ['Integer', '&u32', 'Integer::add_assign', 'no', [], ['ref', {'convert':
// 'Integer'}, 'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl AddAssign<&u32> for Integer {
    fn add_assign(&mut self, rhs: &u32) {
        Integer::add_assign(self, &Integer::from(*rhs))
    }
}

// ['Integer', 'i64', 'Integer::add_assign', 'no', [], ['ref', {'convert':
// 'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl AddAssign<i64> for Integer {
    fn add_assign(&mut self, rhs: i64) {
        Integer::add_assign(self, &Integer::from(rhs))
    }
}

// ['Integer', '&i64', 'Integer::add_assign', 'no', [], ['ref', {'convert':
// 'Integer'}, 'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl AddAssign<&i64> for Integer {
    fn add_assign(&mut self, rhs: &i64) {
        Integer::add_assign(self, &Integer::from(*rhs))
    }
}

// ['Integer', 'u64', 'Integer::add_assign', 'no', [], ['ref', {'convert':
// 'Integer'}]]
impl AddAssign<u64> for Integer {
    fn add_assign(&mut self, rhs: u64) {
        Integer::add_assign(self, &Integer::from(rhs))
    }
}

// ['Integer', '&u64', 'Integer::add_assign', 'no', [], ['ref', {'convert':
// 'Integer'}, 'deref']]
impl AddAssign<&u64> for Integer {
    fn add_assign(&mut self, rhs: &u64) {
        Integer::add_assign(self, &Integer::from(*rhs))
    }
}

// ['Integer', 'i128', 'Integer::add_assign', 'no', [], ['ref', {'convert':
// 'Integer'}]]
impl AddAssign<i128> for Integer {
    fn add_assign(&mut self, rhs: i128) {
        Integer::add_assign(self, &Integer::from(rhs))
    }
}

// ['Integer', '&i128', 'Integer::add_assign', 'no', [], ['ref', {'convert':
// 'Integer'}, 'deref']]
impl AddAssign<&i128> for Integer {
    fn add_assign(&mut self, rhs: &i128) {
        Integer::add_assign(self, &Integer::from(*rhs))
    }
}

// ['Integer', 'u128', 'Integer::add_assign', 'no', [], ['ref', {'convert':
// 'Integer'}]]
impl AddAssign<u128> for Integer {
    fn add_assign(&mut self, rhs: u128) {
        Integer::add_assign(self, &Integer::from(rhs))
    }
}

// ['Integer', '&u128', 'Integer::add_assign', 'no', [], ['ref', {'convert':
// 'Integer'}, 'deref']]
impl AddAssign<&u128> for Integer {
    fn add_assign(&mut self, rhs: &u128) {
        Integer::add_assign(self, &Integer::from(*rhs))
    }
}
