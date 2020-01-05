use crate::integer::Integer;
use core::ops::Sub;

// Sub          The subtraction operator -.

// ['Integer', 'Integer', 'Integer', 'Integer::subtract_assign', 'lhs',
// ['ref_mut'], ['ref']]
impl Sub<Integer> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: Integer) -> Self::Output {
        Integer::subtract_assign(&mut self, &rhs);
        self
    }
}

// ['Integer', '&Integer', 'Integer', 'Integer::subtract_assign', 'lhs',
// ['ref_mut'], []]
impl Sub<&Integer> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: &Integer) -> Self::Output {
        Integer::subtract_assign(&mut self, rhs);
        self
    }
}

// ['&Integer', 'Integer', 'Integer', 'Integer::subtract', 'no', [], ['ref']]
impl Sub<Integer> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(self, &rhs)
    }
}

// ['&Integer', '&Integer', 'Integer', 'Integer::subtract', 'no', [], []]
impl Sub<&Integer> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(self, rhs)
    }
}

// ['Integer', 'i8', 'Integer', 'Integer::subtract_c_long_assign', 'lhs',
// ['ref_mut'], []]
impl Sub<i8> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: i8) -> Self::Output {
        Integer::subtract_c_long_assign(&mut self, rhs);
        self
    }
}

// ['Integer', '&i8', 'Integer', 'Integer::subtract_c_long_assign', 'lhs',
// ['ref_mut'], ['deref']]
impl Sub<&i8> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: &i8) -> Self::Output {
        Integer::subtract_c_long_assign(&mut self, *rhs);
        self
    }
}

// ['&Integer', 'i8', 'Integer', 'Integer::subtract_c_long', 'no', [], []]
impl Sub<i8> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: i8) -> Self::Output {
        Integer::subtract_c_long(self, rhs)
    }
}

// ['&Integer', '&i8', 'Integer', 'Integer::subtract_c_long', 'no', [],
// ['deref']]
impl Sub<&i8> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: &i8) -> Self::Output {
        Integer::subtract_c_long(self, *rhs)
    }
}

// ['i8', 'Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}], ['ref']]
impl Sub<Integer> for i8 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), &rhs)
    }
}

// ['i8', '&Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}], []]
impl Sub<&Integer> for i8 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), rhs)
    }
}

// ['&i8', 'Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], ['ref']]
impl Sub<Integer> for &i8 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), &rhs)
    }
}

// ['&i8', '&Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], []]
impl Sub<&Integer> for &i8 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), rhs)
    }
}

// ['Integer', 'u8', 'Integer', 'Integer::subtract_c_long_assign', 'lhs',
// ['ref_mut'], []]
impl Sub<u8> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: u8) -> Self::Output {
        Integer::subtract_c_long_assign(&mut self, rhs);
        self
    }
}

// ['Integer', '&u8', 'Integer', 'Integer::subtract_c_long_assign', 'lhs',
// ['ref_mut'], ['deref']]
impl Sub<&u8> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: &u8) -> Self::Output {
        Integer::subtract_c_long_assign(&mut self, *rhs);
        self
    }
}

// ['&Integer', 'u8', 'Integer', 'Integer::subtract_c_long', 'no', [], []]
impl Sub<u8> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: u8) -> Self::Output {
        Integer::subtract_c_long(self, rhs)
    }
}

// ['&Integer', '&u8', 'Integer', 'Integer::subtract_c_long', 'no', [],
// ['deref']]
impl Sub<&u8> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: &u8) -> Self::Output {
        Integer::subtract_c_long(self, *rhs)
    }
}

// ['u8', 'Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}], ['ref']]
impl Sub<Integer> for u8 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), &rhs)
    }
}

// ['u8', '&Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}], []]
impl Sub<&Integer> for u8 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), rhs)
    }
}

// ['&u8', 'Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], ['ref']]
impl Sub<Integer> for &u8 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), &rhs)
    }
}

// ['&u8', '&Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], []]
impl Sub<&Integer> for &u8 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), rhs)
    }
}

// ['Integer', 'i16', 'Integer', 'Integer::subtract_c_long_assign', 'lhs',
// ['ref_mut'], []]
impl Sub<i16> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: i16) -> Self::Output {
        Integer::subtract_c_long_assign(&mut self, rhs);
        self
    }
}

// ['Integer', '&i16', 'Integer', 'Integer::subtract_c_long_assign', 'lhs',
// ['ref_mut'], ['deref']]
impl Sub<&i16> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: &i16) -> Self::Output {
        Integer::subtract_c_long_assign(&mut self, *rhs);
        self
    }
}

// ['&Integer', 'i16', 'Integer', 'Integer::subtract_c_long', 'no', [], []]
impl Sub<i16> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: i16) -> Self::Output {
        Integer::subtract_c_long(self, rhs)
    }
}

// ['&Integer', '&i16', 'Integer', 'Integer::subtract_c_long', 'no', [],
// ['deref']]
impl Sub<&i16> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: &i16) -> Self::Output {
        Integer::subtract_c_long(self, *rhs)
    }
}

// ['i16', 'Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}], ['ref']]
impl Sub<Integer> for i16 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), &rhs)
    }
}

// ['i16', '&Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}], []]
impl Sub<&Integer> for i16 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), rhs)
    }
}

// ['&i16', 'Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], ['ref']]
impl Sub<Integer> for &i16 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), &rhs)
    }
}

// ['&i16', '&Integer', 'Integer', 'Integer::subtract', 'no', ['ref',
// {'convert': 'Integer'}, 'deref'], []]
impl Sub<&Integer> for &i16 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), rhs)
    }
}

// ['Integer', 'u16', 'Integer', 'Integer::subtract_c_long_assign', 'lhs',
// ['ref_mut'], []]
impl Sub<u16> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: u16) -> Self::Output {
        Integer::subtract_c_long_assign(&mut self, rhs);
        self
    }
}

// ['Integer', '&u16', 'Integer', 'Integer::subtract_c_long_assign', 'lhs',
// ['ref_mut'], ['deref']]
impl Sub<&u16> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: &u16) -> Self::Output {
        Integer::subtract_c_long_assign(&mut self, *rhs);
        self
    }
}

// ['&Integer', 'u16', 'Integer', 'Integer::subtract_c_long', 'no', [], []]
impl Sub<u16> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: u16) -> Self::Output {
        Integer::subtract_c_long(self, rhs)
    }
}

// ['&Integer', '&u16', 'Integer', 'Integer::subtract_c_long', 'no', [],
// ['deref']]
impl Sub<&u16> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: &u16) -> Self::Output {
        Integer::subtract_c_long(self, *rhs)
    }
}

// ['u16', 'Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}], ['ref']]
impl Sub<Integer> for u16 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), &rhs)
    }
}

// ['u16', '&Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}], []]
impl Sub<&Integer> for u16 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), rhs)
    }
}

// ['&u16', 'Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], ['ref']]
impl Sub<Integer> for &u16 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), &rhs)
    }
}

// ['&u16', '&Integer', 'Integer', 'Integer::subtract', 'no', ['ref',
// {'convert': 'Integer'}, 'deref'], []]
impl Sub<&Integer> for &u16 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), rhs)
    }
}

// ['Integer', 'i32', 'Integer', 'Integer::subtract_c_long_assign', 'lhs',
// ['ref_mut'], []]
impl Sub<i32> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: i32) -> Self::Output {
        Integer::subtract_c_long_assign(&mut self, rhs);
        self
    }
}

// ['Integer', '&i32', 'Integer', 'Integer::subtract_c_long_assign', 'lhs',
// ['ref_mut'], ['deref']]
impl Sub<&i32> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: &i32) -> Self::Output {
        Integer::subtract_c_long_assign(&mut self, *rhs);
        self
    }
}

// ['&Integer', 'i32', 'Integer', 'Integer::subtract_c_long', 'no', [], []]
impl Sub<i32> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: i32) -> Self::Output {
        Integer::subtract_c_long(self, rhs)
    }
}

// ['&Integer', '&i32', 'Integer', 'Integer::subtract_c_long', 'no', [],
// ['deref']]
impl Sub<&i32> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: &i32) -> Self::Output {
        Integer::subtract_c_long(self, *rhs)
    }
}

// ['i32', 'Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}], ['ref']]
impl Sub<Integer> for i32 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), &rhs)
    }
}

// ['i32', '&Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}], []]
impl Sub<&Integer> for i32 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), rhs)
    }
}

// ['&i32', 'Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], ['ref']]
impl Sub<Integer> for &i32 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), &rhs)
    }
}

// ['&i32', '&Integer', 'Integer', 'Integer::subtract', 'no', ['ref',
// {'convert': 'Integer'}, 'deref'], []]
impl Sub<&Integer> for &i32 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), rhs)
    }
}

// ['Integer', 'u32', 'Integer', 'Integer::subtract_c_long_assign', 'lhs',
// ['ref_mut'], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Sub<u32> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: u32) -> Self::Output {
        Integer::subtract_c_long_assign(&mut self, rhs);
        self
    }
}

// ['Integer', '&u32', 'Integer', 'Integer::subtract_c_long_assign', 'lhs',
// ['ref_mut'], ['deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Sub<&u32> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: &u32) -> Self::Output {
        Integer::subtract_c_long_assign(&mut self, *rhs);
        self
    }
}

// ['&Integer', 'u32', 'Integer', 'Integer::subtract_c_long', 'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Sub<u32> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: u32) -> Self::Output {
        Integer::subtract_c_long(self, rhs)
    }
}

// ['&Integer', '&u32', 'Integer', 'Integer::subtract_c_long', 'no', [],
// ['deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Sub<&u32> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: &u32) -> Self::Output {
        Integer::subtract_c_long(self, *rhs)
    }
}

// ['u32', 'Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}], ['ref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Sub<Integer> for u32 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), &rhs)
    }
}

// ['u32', '&Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Sub<&Integer> for u32 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), rhs)
    }
}

// ['&u32', 'Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], ['ref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Sub<Integer> for &u32 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), &rhs)
    }
}

// ['&u32', '&Integer', 'Integer', 'Integer::subtract', 'no', ['ref',
// {'convert': 'Integer'}, 'deref'], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Sub<&Integer> for &u32 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), rhs)
    }
}

// ['Integer', 'i64', 'Integer', 'Integer::subtract_c_long_assign', 'lhs',
// ['ref_mut'], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Sub<i64> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: i64) -> Self::Output {
        Integer::subtract_c_long_assign(&mut self, rhs);
        self
    }
}

// ['Integer', '&i64', 'Integer', 'Integer::subtract_c_long_assign', 'lhs',
// ['ref_mut'], ['deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Sub<&i64> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: &i64) -> Self::Output {
        Integer::subtract_c_long_assign(&mut self, *rhs);
        self
    }
}

// ['&Integer', 'i64', 'Integer', 'Integer::subtract_c_long', 'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Sub<i64> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: i64) -> Self::Output {
        Integer::subtract_c_long(self, rhs)
    }
}

// ['&Integer', '&i64', 'Integer', 'Integer::subtract_c_long', 'no', [],
// ['deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Sub<&i64> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: &i64) -> Self::Output {
        Integer::subtract_c_long(self, *rhs)
    }
}

// ['i64', 'Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}], ['ref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Sub<Integer> for i64 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), &rhs)
    }
}

// ['i64', '&Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Sub<&Integer> for i64 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), rhs)
    }
}

// ['&i64', 'Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], ['ref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Sub<Integer> for &i64 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), &rhs)
    }
}

// ['&i64', '&Integer', 'Integer', 'Integer::subtract', 'no', ['ref',
// {'convert': 'Integer'}, 'deref'], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Sub<&Integer> for &i64 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), rhs)
    }
}

// ['Integer', 'u32', 'Integer', 'Integer::subtract_assign', 'lhs', ['ref_mut'],
// ['ref', {'convert': 'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Sub<u32> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: u32) -> Self::Output {
        Integer::subtract_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Integer', '&u32', 'Integer', 'Integer::subtract_assign', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}, 'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Sub<&u32> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: &u32) -> Self::Output {
        Integer::subtract_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Integer', 'u32', 'Integer', 'Integer::subtract', 'no', [], ['ref',
// {'convert': 'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Sub<u32> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: u32) -> Self::Output {
        Integer::subtract(self, &Integer::from(rhs))
    }
}

// ['&Integer', '&u32', 'Integer', 'Integer::subtract', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Sub<&u32> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: &u32) -> Self::Output {
        Integer::subtract(self, &Integer::from(*rhs))
    }
}

// ['u32', 'Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}], ['ref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Sub<Integer> for u32 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), &rhs)
    }
}

// ['u32', '&Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Sub<&Integer> for u32 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), rhs)
    }
}

// ['&u32', 'Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], ['ref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Sub<Integer> for &u32 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), &rhs)
    }
}

// ['&u32', '&Integer', 'Integer', 'Integer::subtract', 'no', ['ref',
// {'convert': 'Integer'}, 'deref'], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Sub<&Integer> for &u32 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), rhs)
    }
}

// ['Integer', 'i64', 'Integer', 'Integer::subtract_assign', 'lhs', ['ref_mut'],
// ['ref', {'convert': 'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Sub<i64> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: i64) -> Self::Output {
        Integer::subtract_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Integer', '&i64', 'Integer', 'Integer::subtract_assign', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}, 'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Sub<&i64> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: &i64) -> Self::Output {
        Integer::subtract_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Integer', 'i64', 'Integer', 'Integer::subtract', 'no', [], ['ref',
// {'convert': 'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Sub<i64> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: i64) -> Self::Output {
        Integer::subtract(self, &Integer::from(rhs))
    }
}

// ['&Integer', '&i64', 'Integer', 'Integer::subtract', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Sub<&i64> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: &i64) -> Self::Output {
        Integer::subtract(self, &Integer::from(*rhs))
    }
}

// ['i64', 'Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}], ['ref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Sub<Integer> for i64 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), &rhs)
    }
}

// ['i64', '&Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Sub<&Integer> for i64 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), rhs)
    }
}

// ['&i64', 'Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], ['ref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Sub<Integer> for &i64 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), &rhs)
    }
}

// ['&i64', '&Integer', 'Integer', 'Integer::subtract', 'no', ['ref',
// {'convert': 'Integer'}, 'deref'], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Sub<&Integer> for &i64 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), rhs)
    }
}

// ['Integer', 'u64', 'Integer', 'Integer::subtract_assign', 'lhs', ['ref_mut'],
// ['ref', {'convert': 'Integer'}]]
impl Sub<u64> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: u64) -> Self::Output {
        Integer::subtract_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Integer', '&u64', 'Integer', 'Integer::subtract_assign', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}, 'deref']]
impl Sub<&u64> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: &u64) -> Self::Output {
        Integer::subtract_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Integer', 'u64', 'Integer', 'Integer::subtract', 'no', [], ['ref',
// {'convert': 'Integer'}]]
impl Sub<u64> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: u64) -> Self::Output {
        Integer::subtract(self, &Integer::from(rhs))
    }
}

// ['&Integer', '&u64', 'Integer', 'Integer::subtract', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
impl Sub<&u64> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: &u64) -> Self::Output {
        Integer::subtract(self, &Integer::from(*rhs))
    }
}

// ['u64', 'Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}], ['ref']]
impl Sub<Integer> for u64 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), &rhs)
    }
}

// ['u64', '&Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}], []]
impl Sub<&Integer> for u64 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), rhs)
    }
}

// ['&u64', 'Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], ['ref']]
impl Sub<Integer> for &u64 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), &rhs)
    }
}

// ['&u64', '&Integer', 'Integer', 'Integer::subtract', 'no', ['ref',
// {'convert': 'Integer'}, 'deref'], []]
impl Sub<&Integer> for &u64 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), rhs)
    }
}

// ['Integer', 'i128', 'Integer', 'Integer::subtract_assign', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}]]
impl Sub<i128> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: i128) -> Self::Output {
        Integer::subtract_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Integer', '&i128', 'Integer', 'Integer::subtract_assign', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}, 'deref']]
impl Sub<&i128> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: &i128) -> Self::Output {
        Integer::subtract_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Integer', 'i128', 'Integer', 'Integer::subtract', 'no', [], ['ref',
// {'convert': 'Integer'}]]
impl Sub<i128> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: i128) -> Self::Output {
        Integer::subtract(self, &Integer::from(rhs))
    }
}

// ['&Integer', '&i128', 'Integer', 'Integer::subtract', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
impl Sub<&i128> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: &i128) -> Self::Output {
        Integer::subtract(self, &Integer::from(*rhs))
    }
}

// ['i128', 'Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}], ['ref']]
impl Sub<Integer> for i128 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), &rhs)
    }
}

// ['i128', '&Integer', 'Integer', 'Integer::subtract', 'no', ['ref',
// {'convert': 'Integer'}], []]
impl Sub<&Integer> for i128 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), rhs)
    }
}

// ['&i128', 'Integer', 'Integer', 'Integer::subtract', 'no', ['ref',
// {'convert': 'Integer'}, 'deref'], ['ref']]
impl Sub<Integer> for &i128 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), &rhs)
    }
}

// ['&i128', '&Integer', 'Integer', 'Integer::subtract', 'no', ['ref',
// {'convert': 'Integer'}, 'deref'], []]
impl Sub<&Integer> for &i128 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), rhs)
    }
}

// ['Integer', 'u128', 'Integer', 'Integer::subtract_assign', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}]]
impl Sub<u128> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: u128) -> Self::Output {
        Integer::subtract_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Integer', '&u128', 'Integer', 'Integer::subtract_assign', 'lhs',
// ['ref_mut'], ['ref', {'convert': 'Integer'}, 'deref']]
impl Sub<&u128> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: &u128) -> Self::Output {
        Integer::subtract_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Integer', 'u128', 'Integer', 'Integer::subtract', 'no', [], ['ref',
// {'convert': 'Integer'}]]
impl Sub<u128> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: u128) -> Self::Output {
        Integer::subtract(self, &Integer::from(rhs))
    }
}

// ['&Integer', '&u128', 'Integer', 'Integer::subtract', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
impl Sub<&u128> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: &u128) -> Self::Output {
        Integer::subtract(self, &Integer::from(*rhs))
    }
}

// ['u128', 'Integer', 'Integer', 'Integer::subtract', 'no', ['ref', {'convert':
// 'Integer'}], ['ref']]
impl Sub<Integer> for u128 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), &rhs)
    }
}

// ['u128', '&Integer', 'Integer', 'Integer::subtract', 'no', ['ref',
// {'convert': 'Integer'}], []]
impl Sub<&Integer> for u128 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), rhs)
    }
}

// ['&u128', 'Integer', 'Integer', 'Integer::subtract', 'no', ['ref',
// {'convert': 'Integer'}, 'deref'], ['ref']]
impl Sub<Integer> for &u128 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), &rhs)
    }
}

// ['&u128', '&Integer', 'Integer', 'Integer::subtract', 'no', ['ref',
// {'convert': 'Integer'}, 'deref'], []]
impl Sub<&Integer> for &u128 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), rhs)
    }
}
