use crate::integer::Integer;
use core::ops::DivAssign;

// DivAssign          The division assignment operator /=.

// ['Integer', 'Integer', 'Integer::divide_assign', 'no', [], ['ref']]
impl DivAssign<Integer> for Integer {
    fn div_assign(&mut self, rhs: Integer) {
        Integer::divide_assign(self, &rhs)
    }
}

// ['Integer', '&Integer', 'Integer::divide_assign', 'no', [], []]
impl DivAssign<&Integer> for Integer {
    fn div_assign(&mut self, rhs: &Integer) {
        Integer::divide_assign(self, rhs)
    }
}

// ['Integer', 'i8', 'Integer::divide_c_long_assign', 'no', [], []]
impl DivAssign<i8> for Integer {
    fn div_assign(&mut self, rhs: i8) {
        Integer::divide_c_long_assign(self, rhs)
    }
}

// ['Integer', '&i8', 'Integer::divide_c_long_assign', 'no', [], ['deref']]
impl DivAssign<&i8> for Integer {
    fn div_assign(&mut self, rhs: &i8) {
        Integer::divide_c_long_assign(self, *rhs)
    }
}

// ['Integer', 'u8', 'Integer::divide_c_long_assign', 'no', [], []]
impl DivAssign<u8> for Integer {
    fn div_assign(&mut self, rhs: u8) {
        Integer::divide_c_long_assign(self, rhs)
    }
}

// ['Integer', '&u8', 'Integer::divide_c_long_assign', 'no', [], ['deref']]
impl DivAssign<&u8> for Integer {
    fn div_assign(&mut self, rhs: &u8) {
        Integer::divide_c_long_assign(self, *rhs)
    }
}

// ['Integer', 'i16', 'Integer::divide_c_long_assign', 'no', [], []]
impl DivAssign<i16> for Integer {
    fn div_assign(&mut self, rhs: i16) {
        Integer::divide_c_long_assign(self, rhs)
    }
}

// ['Integer', '&i16', 'Integer::divide_c_long_assign', 'no', [], ['deref']]
impl DivAssign<&i16> for Integer {
    fn div_assign(&mut self, rhs: &i16) {
        Integer::divide_c_long_assign(self, *rhs)
    }
}

// ['Integer', 'u16', 'Integer::divide_c_long_assign', 'no', [], []]
impl DivAssign<u16> for Integer {
    fn div_assign(&mut self, rhs: u16) {
        Integer::divide_c_long_assign(self, rhs)
    }
}

// ['Integer', '&u16', 'Integer::divide_c_long_assign', 'no', [], ['deref']]
impl DivAssign<&u16> for Integer {
    fn div_assign(&mut self, rhs: &u16) {
        Integer::divide_c_long_assign(self, *rhs)
    }
}

// ['Integer', 'i32', 'Integer::divide_c_long_assign', 'no', [], []]
impl DivAssign<i32> for Integer {
    fn div_assign(&mut self, rhs: i32) {
        Integer::divide_c_long_assign(self, rhs)
    }
}

// ['Integer', '&i32', 'Integer::divide_c_long_assign', 'no', [], ['deref']]
impl DivAssign<&i32> for Integer {
    fn div_assign(&mut self, rhs: &i32) {
        Integer::divide_c_long_assign(self, *rhs)
    }
}

// ['Integer', 'u32', 'Integer::divide_c_long_assign', 'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl DivAssign<u32> for Integer {
    fn div_assign(&mut self, rhs: u32) {
        Integer::divide_c_long_assign(self, rhs)
    }
}

// ['Integer', '&u32', 'Integer::divide_c_long_assign', 'no', [], ['deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl DivAssign<&u32> for Integer {
    fn div_assign(&mut self, rhs: &u32) {
        Integer::divide_c_long_assign(self, *rhs)
    }
}

// ['Integer', 'i64', 'Integer::divide_c_long_assign', 'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl DivAssign<i64> for Integer {
    fn div_assign(&mut self, rhs: i64) {
        Integer::divide_c_long_assign(self, rhs)
    }
}

// ['Integer', '&i64', 'Integer::divide_c_long_assign', 'no', [], ['deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl DivAssign<&i64> for Integer {
    fn div_assign(&mut self, rhs: &i64) {
        Integer::divide_c_long_assign(self, *rhs)
    }
}

// ['Integer', 'u32', 'Integer::divide_assign', 'no', [], ['ref', {'convert':
// 'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl DivAssign<u32> for Integer {
    fn div_assign(&mut self, rhs: u32) {
        Integer::divide_assign(self, &Integer::from(rhs))
    }
}

// ['Integer', '&u32', 'Integer::divide_assign', 'no', [], ['ref', {'convert':
// 'Integer'}, 'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl DivAssign<&u32> for Integer {
    fn div_assign(&mut self, rhs: &u32) {
        Integer::divide_assign(self, &Integer::from(*rhs))
    }
}

// ['Integer', 'i64', 'Integer::divide_assign', 'no', [], ['ref', {'convert':
// 'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl DivAssign<i64> for Integer {
    fn div_assign(&mut self, rhs: i64) {
        Integer::divide_assign(self, &Integer::from(rhs))
    }
}

// ['Integer', '&i64', 'Integer::divide_assign', 'no', [], ['ref', {'convert':
// 'Integer'}, 'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl DivAssign<&i64> for Integer {
    fn div_assign(&mut self, rhs: &i64) {
        Integer::divide_assign(self, &Integer::from(*rhs))
    }
}

// ['Integer', 'u64', 'Integer::divide_assign', 'no', [], ['ref', {'convert':
// 'Integer'}]]
impl DivAssign<u64> for Integer {
    fn div_assign(&mut self, rhs: u64) {
        Integer::divide_assign(self, &Integer::from(rhs))
    }
}

// ['Integer', '&u64', 'Integer::divide_assign', 'no', [], ['ref', {'convert':
// 'Integer'}, 'deref']]
impl DivAssign<&u64> for Integer {
    fn div_assign(&mut self, rhs: &u64) {
        Integer::divide_assign(self, &Integer::from(*rhs))
    }
}

// ['Integer', 'i128', 'Integer::divide_assign', 'no', [], ['ref', {'convert':
// 'Integer'}]]
impl DivAssign<i128> for Integer {
    fn div_assign(&mut self, rhs: i128) {
        Integer::divide_assign(self, &Integer::from(rhs))
    }
}

// ['Integer', '&i128', 'Integer::divide_assign', 'no', [], ['ref', {'convert':
// 'Integer'}, 'deref']]
impl DivAssign<&i128> for Integer {
    fn div_assign(&mut self, rhs: &i128) {
        Integer::divide_assign(self, &Integer::from(*rhs))
    }
}

// ['Integer', 'u128', 'Integer::divide_assign', 'no', [], ['ref', {'convert':
// 'Integer'}]]
impl DivAssign<u128> for Integer {
    fn div_assign(&mut self, rhs: u128) {
        Integer::divide_assign(self, &Integer::from(rhs))
    }
}

// ['Integer', '&u128', 'Integer::divide_assign', 'no', [], ['ref', {'convert':
// 'Integer'}, 'deref']]
impl DivAssign<&u128> for Integer {
    fn div_assign(&mut self, rhs: &u128) {
        Integer::divide_assign(self, &Integer::from(*rhs))
    }
}
