use crate::integer::Integer;
use core::ops::Sub;

// Sub          The subtraction operator -.

// [u'Integer', u'Integer', u'Integer', u'Integer::subtract_assign', u'lhs',
// [u'ref_mut'], [u'ref']]
impl Sub<Integer> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: Integer) -> Self::Output {
        Integer::subtract_assign(&mut self, &rhs);
        self
    }
}

// [u'Integer', u'&Integer', u'Integer', u'Integer::subtract_assign', u'lhs',
// [u'ref_mut'], []]
impl Sub<&Integer> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: &Integer) -> Self::Output {
        Integer::subtract_assign(&mut self, rhs);
        self
    }
}

// [u'&Integer', u'Integer', u'Integer', u'Integer::subtract', u'no', [],
// [u'ref']]
impl Sub<Integer> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(self, &rhs)
    }
}

// [u'&Integer', u'&Integer', u'Integer', u'Integer::subtract', u'no', [], []]
impl Sub<&Integer> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(self, rhs)
    }
}

// [u'Integer', u'i8', u'Integer', u'Integer::subtract_c_long_assign', u'lhs',
// [u'ref_mut'], []]
impl Sub<i8> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: i8) -> Self::Output {
        Integer::subtract_c_long_assign(&mut self, rhs);
        self
    }
}

// [u'Integer', u'&i8', u'Integer', u'Integer::subtract_c_long_assign', u'lhs',
// [u'ref_mut'], [u'deref']]
impl Sub<&i8> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: &i8) -> Self::Output {
        Integer::subtract_c_long_assign(&mut self, *rhs);
        self
    }
}

// [u'&Integer', u'i8', u'Integer', u'Integer::subtract_c_long', u'no', [], []]
impl Sub<i8> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: i8) -> Self::Output {
        Integer::subtract_c_long(self, rhs)
    }
}

// [u'&Integer', u'&i8', u'Integer', u'Integer::subtract_c_long', u'no', [],
// [u'deref']]
impl Sub<&i8> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: &i8) -> Self::Output {
        Integer::subtract_c_long(self, *rhs)
    }
}

// [u'i8', u'Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}], [u'ref']]
impl Sub<Integer> for i8 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), &rhs)
    }
}

// [u'i8', u'&Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}], []]
impl Sub<&Integer> for i8 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), rhs)
    }
}

// [u'&i8', u'Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], [u'ref']]
impl Sub<Integer> for &i8 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), &rhs)
    }
}

// [u'&i8', u'&Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], []]
impl Sub<&Integer> for &i8 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), rhs)
    }
}

// [u'Integer', u'u8', u'Integer', u'Integer::subtract_c_long_assign', u'lhs',
// [u'ref_mut'], []]
impl Sub<u8> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: u8) -> Self::Output {
        Integer::subtract_c_long_assign(&mut self, rhs);
        self
    }
}

// [u'Integer', u'&u8', u'Integer', u'Integer::subtract_c_long_assign', u'lhs',
// [u'ref_mut'], [u'deref']]
impl Sub<&u8> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: &u8) -> Self::Output {
        Integer::subtract_c_long_assign(&mut self, *rhs);
        self
    }
}

// [u'&Integer', u'u8', u'Integer', u'Integer::subtract_c_long', u'no', [], []]
impl Sub<u8> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: u8) -> Self::Output {
        Integer::subtract_c_long(self, rhs)
    }
}

// [u'&Integer', u'&u8', u'Integer', u'Integer::subtract_c_long', u'no', [],
// [u'deref']]
impl Sub<&u8> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: &u8) -> Self::Output {
        Integer::subtract_c_long(self, *rhs)
    }
}

// [u'u8', u'Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}], [u'ref']]
impl Sub<Integer> for u8 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), &rhs)
    }
}

// [u'u8', u'&Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}], []]
impl Sub<&Integer> for u8 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), rhs)
    }
}

// [u'&u8', u'Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], [u'ref']]
impl Sub<Integer> for &u8 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), &rhs)
    }
}

// [u'&u8', u'&Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], []]
impl Sub<&Integer> for &u8 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), rhs)
    }
}

// [u'Integer', u'i16', u'Integer', u'Integer::subtract_c_long_assign', u'lhs',
// [u'ref_mut'], []]
impl Sub<i16> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: i16) -> Self::Output {
        Integer::subtract_c_long_assign(&mut self, rhs);
        self
    }
}

// [u'Integer', u'&i16', u'Integer', u'Integer::subtract_c_long_assign', u'lhs',
// [u'ref_mut'], [u'deref']]
impl Sub<&i16> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: &i16) -> Self::Output {
        Integer::subtract_c_long_assign(&mut self, *rhs);
        self
    }
}

// [u'&Integer', u'i16', u'Integer', u'Integer::subtract_c_long', u'no', [], []]
impl Sub<i16> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: i16) -> Self::Output {
        Integer::subtract_c_long(self, rhs)
    }
}

// [u'&Integer', u'&i16', u'Integer', u'Integer::subtract_c_long', u'no', [],
// [u'deref']]
impl Sub<&i16> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: &i16) -> Self::Output {
        Integer::subtract_c_long(self, *rhs)
    }
}

// [u'i16', u'Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}], [u'ref']]
impl Sub<Integer> for i16 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), &rhs)
    }
}

// [u'i16', u'&Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}], []]
impl Sub<&Integer> for i16 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), rhs)
    }
}

// [u'&i16', u'Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], [u'ref']]
impl Sub<Integer> for &i16 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), &rhs)
    }
}

// [u'&i16', u'&Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], []]
impl Sub<&Integer> for &i16 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), rhs)
    }
}

// [u'Integer', u'u16', u'Integer', u'Integer::subtract_c_long_assign', u'lhs',
// [u'ref_mut'], []]
impl Sub<u16> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: u16) -> Self::Output {
        Integer::subtract_c_long_assign(&mut self, rhs);
        self
    }
}

// [u'Integer', u'&u16', u'Integer', u'Integer::subtract_c_long_assign', u'lhs',
// [u'ref_mut'], [u'deref']]
impl Sub<&u16> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: &u16) -> Self::Output {
        Integer::subtract_c_long_assign(&mut self, *rhs);
        self
    }
}

// [u'&Integer', u'u16', u'Integer', u'Integer::subtract_c_long', u'no', [], []]
impl Sub<u16> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: u16) -> Self::Output {
        Integer::subtract_c_long(self, rhs)
    }
}

// [u'&Integer', u'&u16', u'Integer', u'Integer::subtract_c_long', u'no', [],
// [u'deref']]
impl Sub<&u16> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: &u16) -> Self::Output {
        Integer::subtract_c_long(self, *rhs)
    }
}

// [u'u16', u'Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}], [u'ref']]
impl Sub<Integer> for u16 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), &rhs)
    }
}

// [u'u16', u'&Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}], []]
impl Sub<&Integer> for u16 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), rhs)
    }
}

// [u'&u16', u'Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], [u'ref']]
impl Sub<Integer> for &u16 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), &rhs)
    }
}

// [u'&u16', u'&Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], []]
impl Sub<&Integer> for &u16 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), rhs)
    }
}

// [u'Integer', u'i32', u'Integer', u'Integer::subtract_c_long_assign', u'lhs',
// [u'ref_mut'], []]
impl Sub<i32> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: i32) -> Self::Output {
        Integer::subtract_c_long_assign(&mut self, rhs);
        self
    }
}

// [u'Integer', u'&i32', u'Integer', u'Integer::subtract_c_long_assign', u'lhs',
// [u'ref_mut'], [u'deref']]
impl Sub<&i32> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: &i32) -> Self::Output {
        Integer::subtract_c_long_assign(&mut self, *rhs);
        self
    }
}

// [u'&Integer', u'i32', u'Integer', u'Integer::subtract_c_long', u'no', [], []]
impl Sub<i32> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: i32) -> Self::Output {
        Integer::subtract_c_long(self, rhs)
    }
}

// [u'&Integer', u'&i32', u'Integer', u'Integer::subtract_c_long', u'no', [],
// [u'deref']]
impl Sub<&i32> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: &i32) -> Self::Output {
        Integer::subtract_c_long(self, *rhs)
    }
}

// [u'i32', u'Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}], [u'ref']]
impl Sub<Integer> for i32 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), &rhs)
    }
}

// [u'i32', u'&Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}], []]
impl Sub<&Integer> for i32 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), rhs)
    }
}

// [u'&i32', u'Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], [u'ref']]
impl Sub<Integer> for &i32 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), &rhs)
    }
}

// [u'&i32', u'&Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], []]
impl Sub<&Integer> for &i32 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), rhs)
    }
}

// [u'Integer', u'u32', u'Integer', u'Integer::subtract_c_long_assign', u'lhs',
// [u'ref_mut'], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Sub<u32> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: u32) -> Self::Output {
        Integer::subtract_c_long_assign(&mut self, rhs);
        self
    }
}

// [u'Integer', u'&u32', u'Integer', u'Integer::subtract_c_long_assign', u'lhs',
// [u'ref_mut'], [u'deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Sub<&u32> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: &u32) -> Self::Output {
        Integer::subtract_c_long_assign(&mut self, *rhs);
        self
    }
}

// [u'&Integer', u'u32', u'Integer', u'Integer::subtract_c_long', u'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Sub<u32> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: u32) -> Self::Output {
        Integer::subtract_c_long(self, rhs)
    }
}

// [u'&Integer', u'&u32', u'Integer', u'Integer::subtract_c_long', u'no', [],
// [u'deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Sub<&u32> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: &u32) -> Self::Output {
        Integer::subtract_c_long(self, *rhs)
    }
}

// [u'u32', u'Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}], [u'ref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Sub<Integer> for u32 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), &rhs)
    }
}

// [u'u32', u'&Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Sub<&Integer> for u32 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), rhs)
    }
}

// [u'&u32', u'Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], [u'ref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Sub<Integer> for &u32 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), &rhs)
    }
}

// [u'&u32', u'&Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Sub<&Integer> for &u32 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), rhs)
    }
}

// [u'Integer', u'i64', u'Integer', u'Integer::subtract_c_long_assign', u'lhs',
// [u'ref_mut'], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Sub<i64> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: i64) -> Self::Output {
        Integer::subtract_c_long_assign(&mut self, rhs);
        self
    }
}

// [u'Integer', u'&i64', u'Integer', u'Integer::subtract_c_long_assign', u'lhs',
// [u'ref_mut'], [u'deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Sub<&i64> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: &i64) -> Self::Output {
        Integer::subtract_c_long_assign(&mut self, *rhs);
        self
    }
}

// [u'&Integer', u'i64', u'Integer', u'Integer::subtract_c_long', u'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Sub<i64> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: i64) -> Self::Output {
        Integer::subtract_c_long(self, rhs)
    }
}

// [u'&Integer', u'&i64', u'Integer', u'Integer::subtract_c_long', u'no', [],
// [u'deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Sub<&i64> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: &i64) -> Self::Output {
        Integer::subtract_c_long(self, *rhs)
    }
}

// [u'i64', u'Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}], [u'ref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Sub<Integer> for i64 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), &rhs)
    }
}

// [u'i64', u'&Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Sub<&Integer> for i64 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), rhs)
    }
}

// [u'&i64', u'Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], [u'ref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Sub<Integer> for &i64 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), &rhs)
    }
}

// [u'&i64', u'&Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Sub<&Integer> for &i64 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), rhs)
    }
}

// [u'Integer', u'u32', u'Integer', u'Integer::subtract_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Sub<u32> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: u32) -> Self::Output {
        Integer::subtract_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// [u'Integer', u'&u32', u'Integer', u'Integer::subtract_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}, u'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Sub<&u32> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: &u32) -> Self::Output {
        Integer::subtract_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// [u'&Integer', u'u32', u'Integer', u'Integer::subtract', u'no', [], [u'ref',
// {u'convert': u'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Sub<u32> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: u32) -> Self::Output {
        Integer::subtract(self, &Integer::from(rhs))
    }
}

// [u'&Integer', u'&u32', u'Integer', u'Integer::subtract', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Sub<&u32> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: &u32) -> Self::Output {
        Integer::subtract(self, &Integer::from(*rhs))
    }
}

// [u'u32', u'Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}], [u'ref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Sub<Integer> for u32 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), &rhs)
    }
}

// [u'u32', u'&Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Sub<&Integer> for u32 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), rhs)
    }
}

// [u'&u32', u'Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], [u'ref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Sub<Integer> for &u32 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), &rhs)
    }
}

// [u'&u32', u'&Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Sub<&Integer> for &u32 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), rhs)
    }
}

// [u'Integer', u'i64', u'Integer', u'Integer::subtract_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Sub<i64> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: i64) -> Self::Output {
        Integer::subtract_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// [u'Integer', u'&i64', u'Integer', u'Integer::subtract_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}, u'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Sub<&i64> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: &i64) -> Self::Output {
        Integer::subtract_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// [u'&Integer', u'i64', u'Integer', u'Integer::subtract', u'no', [], [u'ref',
// {u'convert': u'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Sub<i64> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: i64) -> Self::Output {
        Integer::subtract(self, &Integer::from(rhs))
    }
}

// [u'&Integer', u'&i64', u'Integer', u'Integer::subtract', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Sub<&i64> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: &i64) -> Self::Output {
        Integer::subtract(self, &Integer::from(*rhs))
    }
}

// [u'i64', u'Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}], [u'ref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Sub<Integer> for i64 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), &rhs)
    }
}

// [u'i64', u'&Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Sub<&Integer> for i64 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), rhs)
    }
}

// [u'&i64', u'Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], [u'ref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Sub<Integer> for &i64 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), &rhs)
    }
}

// [u'&i64', u'&Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Sub<&Integer> for &i64 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), rhs)
    }
}

// [u'Integer', u'u64', u'Integer', u'Integer::subtract_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}]]
impl Sub<u64> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: u64) -> Self::Output {
        Integer::subtract_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// [u'Integer', u'&u64', u'Integer', u'Integer::subtract_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}, u'deref']]
impl Sub<&u64> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: &u64) -> Self::Output {
        Integer::subtract_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// [u'&Integer', u'u64', u'Integer', u'Integer::subtract', u'no', [], [u'ref',
// {u'convert': u'Integer'}]]
impl Sub<u64> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: u64) -> Self::Output {
        Integer::subtract(self, &Integer::from(rhs))
    }
}

// [u'&Integer', u'&u64', u'Integer', u'Integer::subtract', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
impl Sub<&u64> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: &u64) -> Self::Output {
        Integer::subtract(self, &Integer::from(*rhs))
    }
}

// [u'u64', u'Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}], [u'ref']]
impl Sub<Integer> for u64 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), &rhs)
    }
}

// [u'u64', u'&Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}], []]
impl Sub<&Integer> for u64 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), rhs)
    }
}

// [u'&u64', u'Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], [u'ref']]
impl Sub<Integer> for &u64 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), &rhs)
    }
}

// [u'&u64', u'&Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], []]
impl Sub<&Integer> for &u64 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), rhs)
    }
}

// [u'Integer', u'i128', u'Integer', u'Integer::subtract_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}]]
impl Sub<i128> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: i128) -> Self::Output {
        Integer::subtract_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// [u'Integer', u'&i128', u'Integer', u'Integer::subtract_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}, u'deref']]
impl Sub<&i128> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: &i128) -> Self::Output {
        Integer::subtract_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// [u'&Integer', u'i128', u'Integer', u'Integer::subtract', u'no', [], [u'ref',
// {u'convert': u'Integer'}]]
impl Sub<i128> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: i128) -> Self::Output {
        Integer::subtract(self, &Integer::from(rhs))
    }
}

// [u'&Integer', u'&i128', u'Integer', u'Integer::subtract', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
impl Sub<&i128> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: &i128) -> Self::Output {
        Integer::subtract(self, &Integer::from(*rhs))
    }
}

// [u'i128', u'Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}], [u'ref']]
impl Sub<Integer> for i128 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), &rhs)
    }
}

// [u'i128', u'&Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}], []]
impl Sub<&Integer> for i128 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), rhs)
    }
}

// [u'&i128', u'Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], [u'ref']]
impl Sub<Integer> for &i128 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), &rhs)
    }
}

// [u'&i128', u'&Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], []]
impl Sub<&Integer> for &i128 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), rhs)
    }
}

// [u'Integer', u'u128', u'Integer', u'Integer::subtract_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}]]
impl Sub<u128> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: u128) -> Self::Output {
        Integer::subtract_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// [u'Integer', u'&u128', u'Integer', u'Integer::subtract_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}, u'deref']]
impl Sub<&u128> for Integer {
    type Output = Integer;

    fn sub(mut self, rhs: &u128) -> Self::Output {
        Integer::subtract_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// [u'&Integer', u'u128', u'Integer', u'Integer::subtract', u'no', [], [u'ref',
// {u'convert': u'Integer'}]]
impl Sub<u128> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: u128) -> Self::Output {
        Integer::subtract(self, &Integer::from(rhs))
    }
}

// [u'&Integer', u'&u128', u'Integer', u'Integer::subtract', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
impl Sub<&u128> for &Integer {
    type Output = Integer;

    fn sub(self, rhs: &u128) -> Self::Output {
        Integer::subtract(self, &Integer::from(*rhs))
    }
}

// [u'u128', u'Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}], [u'ref']]
impl Sub<Integer> for u128 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), &rhs)
    }
}

// [u'u128', u'&Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}], []]
impl Sub<&Integer> for u128 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(self), rhs)
    }
}

// [u'&u128', u'Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], [u'ref']]
impl Sub<Integer> for &u128 {
    type Output = Integer;

    fn sub(self, rhs: Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), &rhs)
    }
}

// [u'&u128', u'&Integer', u'Integer', u'Integer::subtract', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], []]
impl Sub<&Integer> for &u128 {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        Integer::subtract(&Integer::from(*self), rhs)
    }
}
