use crate::integer::Integer;
use core::ops::Div;

// Div          The division operator /.

// [u'Integer', u'Integer', u'Integer', u'Integer::divide_assign', u'lhs',
// [u'ref_mut'], [u'ref']]
impl Div<Integer> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: Integer) -> Self::Output {
        Integer::divide_assign(&mut self, &rhs);
        self
    }
}

// [u'Integer', u'&Integer', u'Integer', u'Integer::divide_assign', u'lhs',
// [u'ref_mut'], []]
impl Div<&Integer> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: &Integer) -> Self::Output {
        Integer::divide_assign(&mut self, rhs);
        self
    }
}

// [u'&Integer', u'Integer', u'Integer', u'Integer::divide', u'no', [],
// [u'ref']]
impl Div<Integer> for &Integer {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(self, &rhs)
    }
}

// [u'&Integer', u'&Integer', u'Integer', u'Integer::divide', u'no', [], []]
impl Div<&Integer> for &Integer {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(self, rhs)
    }
}

// [u'Integer', u'i8', u'Integer', u'Integer::divide_c_long_assign', u'lhs',
// [u'ref_mut'], []]
impl Div<i8> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: i8) -> Self::Output {
        Integer::divide_c_long_assign(&mut self, rhs);
        self
    }
}

// [u'Integer', u'&i8', u'Integer', u'Integer::divide_c_long_assign', u'lhs',
// [u'ref_mut'], [u'deref']]
impl Div<&i8> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: &i8) -> Self::Output {
        Integer::divide_c_long_assign(&mut self, *rhs);
        self
    }
}

// [u'&Integer', u'i8', u'Integer', u'Integer::divide_c_long', u'no', [], []]
impl Div<i8> for &Integer {
    type Output = Integer;

    fn div(self, rhs: i8) -> Self::Output {
        Integer::divide_c_long(self, rhs)
    }
}

// [u'&Integer', u'&i8', u'Integer', u'Integer::divide_c_long', u'no', [],
// [u'deref']]
impl Div<&i8> for &Integer {
    type Output = Integer;

    fn div(self, rhs: &i8) -> Self::Output {
        Integer::divide_c_long(self, *rhs)
    }
}

// [u'i8', u'Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}], [u'ref']]
impl Div<Integer> for i8 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), &rhs)
    }
}

// [u'i8', u'&Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}], []]
impl Div<&Integer> for i8 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), rhs)
    }
}

// [u'&i8', u'Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], [u'ref']]
impl Div<Integer> for &i8 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), &rhs)
    }
}

// [u'&i8', u'&Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], []]
impl Div<&Integer> for &i8 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), rhs)
    }
}

// [u'Integer', u'u8', u'Integer', u'Integer::divide_c_long_assign', u'lhs',
// [u'ref_mut'], []]
impl Div<u8> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: u8) -> Self::Output {
        Integer::divide_c_long_assign(&mut self, rhs);
        self
    }
}

// [u'Integer', u'&u8', u'Integer', u'Integer::divide_c_long_assign', u'lhs',
// [u'ref_mut'], [u'deref']]
impl Div<&u8> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: &u8) -> Self::Output {
        Integer::divide_c_long_assign(&mut self, *rhs);
        self
    }
}

// [u'&Integer', u'u8', u'Integer', u'Integer::divide_c_long', u'no', [], []]
impl Div<u8> for &Integer {
    type Output = Integer;

    fn div(self, rhs: u8) -> Self::Output {
        Integer::divide_c_long(self, rhs)
    }
}

// [u'&Integer', u'&u8', u'Integer', u'Integer::divide_c_long', u'no', [],
// [u'deref']]
impl Div<&u8> for &Integer {
    type Output = Integer;

    fn div(self, rhs: &u8) -> Self::Output {
        Integer::divide_c_long(self, *rhs)
    }
}

// [u'u8', u'Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}], [u'ref']]
impl Div<Integer> for u8 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), &rhs)
    }
}

// [u'u8', u'&Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}], []]
impl Div<&Integer> for u8 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), rhs)
    }
}

// [u'&u8', u'Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], [u'ref']]
impl Div<Integer> for &u8 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), &rhs)
    }
}

// [u'&u8', u'&Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], []]
impl Div<&Integer> for &u8 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), rhs)
    }
}

// [u'Integer', u'i16', u'Integer', u'Integer::divide_c_long_assign', u'lhs',
// [u'ref_mut'], []]
impl Div<i16> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: i16) -> Self::Output {
        Integer::divide_c_long_assign(&mut self, rhs);
        self
    }
}

// [u'Integer', u'&i16', u'Integer', u'Integer::divide_c_long_assign', u'lhs',
// [u'ref_mut'], [u'deref']]
impl Div<&i16> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: &i16) -> Self::Output {
        Integer::divide_c_long_assign(&mut self, *rhs);
        self
    }
}

// [u'&Integer', u'i16', u'Integer', u'Integer::divide_c_long', u'no', [], []]
impl Div<i16> for &Integer {
    type Output = Integer;

    fn div(self, rhs: i16) -> Self::Output {
        Integer::divide_c_long(self, rhs)
    }
}

// [u'&Integer', u'&i16', u'Integer', u'Integer::divide_c_long', u'no', [],
// [u'deref']]
impl Div<&i16> for &Integer {
    type Output = Integer;

    fn div(self, rhs: &i16) -> Self::Output {
        Integer::divide_c_long(self, *rhs)
    }
}

// [u'i16', u'Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}], [u'ref']]
impl Div<Integer> for i16 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), &rhs)
    }
}

// [u'i16', u'&Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}], []]
impl Div<&Integer> for i16 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), rhs)
    }
}

// [u'&i16', u'Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], [u'ref']]
impl Div<Integer> for &i16 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), &rhs)
    }
}

// [u'&i16', u'&Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], []]
impl Div<&Integer> for &i16 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), rhs)
    }
}

// [u'Integer', u'u16', u'Integer', u'Integer::divide_c_long_assign', u'lhs',
// [u'ref_mut'], []]
impl Div<u16> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: u16) -> Self::Output {
        Integer::divide_c_long_assign(&mut self, rhs);
        self
    }
}

// [u'Integer', u'&u16', u'Integer', u'Integer::divide_c_long_assign', u'lhs',
// [u'ref_mut'], [u'deref']]
impl Div<&u16> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: &u16) -> Self::Output {
        Integer::divide_c_long_assign(&mut self, *rhs);
        self
    }
}

// [u'&Integer', u'u16', u'Integer', u'Integer::divide_c_long', u'no', [], []]
impl Div<u16> for &Integer {
    type Output = Integer;

    fn div(self, rhs: u16) -> Self::Output {
        Integer::divide_c_long(self, rhs)
    }
}

// [u'&Integer', u'&u16', u'Integer', u'Integer::divide_c_long', u'no', [],
// [u'deref']]
impl Div<&u16> for &Integer {
    type Output = Integer;

    fn div(self, rhs: &u16) -> Self::Output {
        Integer::divide_c_long(self, *rhs)
    }
}

// [u'u16', u'Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}], [u'ref']]
impl Div<Integer> for u16 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), &rhs)
    }
}

// [u'u16', u'&Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}], []]
impl Div<&Integer> for u16 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), rhs)
    }
}

// [u'&u16', u'Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], [u'ref']]
impl Div<Integer> for &u16 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), &rhs)
    }
}

// [u'&u16', u'&Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], []]
impl Div<&Integer> for &u16 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), rhs)
    }
}

// [u'Integer', u'i32', u'Integer', u'Integer::divide_c_long_assign', u'lhs',
// [u'ref_mut'], []]
impl Div<i32> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: i32) -> Self::Output {
        Integer::divide_c_long_assign(&mut self, rhs);
        self
    }
}

// [u'Integer', u'&i32', u'Integer', u'Integer::divide_c_long_assign', u'lhs',
// [u'ref_mut'], [u'deref']]
impl Div<&i32> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: &i32) -> Self::Output {
        Integer::divide_c_long_assign(&mut self, *rhs);
        self
    }
}

// [u'&Integer', u'i32', u'Integer', u'Integer::divide_c_long', u'no', [], []]
impl Div<i32> for &Integer {
    type Output = Integer;

    fn div(self, rhs: i32) -> Self::Output {
        Integer::divide_c_long(self, rhs)
    }
}

// [u'&Integer', u'&i32', u'Integer', u'Integer::divide_c_long', u'no', [],
// [u'deref']]
impl Div<&i32> for &Integer {
    type Output = Integer;

    fn div(self, rhs: &i32) -> Self::Output {
        Integer::divide_c_long(self, *rhs)
    }
}

// [u'i32', u'Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}], [u'ref']]
impl Div<Integer> for i32 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), &rhs)
    }
}

// [u'i32', u'&Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}], []]
impl Div<&Integer> for i32 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), rhs)
    }
}

// [u'&i32', u'Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], [u'ref']]
impl Div<Integer> for &i32 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), &rhs)
    }
}

// [u'&i32', u'&Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], []]
impl Div<&Integer> for &i32 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), rhs)
    }
}

// [u'Integer', u'u32', u'Integer', u'Integer::divide_c_long_assign', u'lhs',
// [u'ref_mut'], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Div<u32> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: u32) -> Self::Output {
        Integer::divide_c_long_assign(&mut self, rhs);
        self
    }
}

// [u'Integer', u'&u32', u'Integer', u'Integer::divide_c_long_assign', u'lhs',
// [u'ref_mut'], [u'deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Div<&u32> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: &u32) -> Self::Output {
        Integer::divide_c_long_assign(&mut self, *rhs);
        self
    }
}

// [u'&Integer', u'u32', u'Integer', u'Integer::divide_c_long', u'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Div<u32> for &Integer {
    type Output = Integer;

    fn div(self, rhs: u32) -> Self::Output {
        Integer::divide_c_long(self, rhs)
    }
}

// [u'&Integer', u'&u32', u'Integer', u'Integer::divide_c_long', u'no', [],
// [u'deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Div<&u32> for &Integer {
    type Output = Integer;

    fn div(self, rhs: &u32) -> Self::Output {
        Integer::divide_c_long(self, *rhs)
    }
}

// [u'u32', u'Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}], [u'ref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Div<Integer> for u32 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), &rhs)
    }
}

// [u'u32', u'&Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Div<&Integer> for u32 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), rhs)
    }
}

// [u'&u32', u'Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], [u'ref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Div<Integer> for &u32 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), &rhs)
    }
}

// [u'&u32', u'&Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Div<&Integer> for &u32 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), rhs)
    }
}

// [u'Integer', u'i64', u'Integer', u'Integer::divide_c_long_assign', u'lhs',
// [u'ref_mut'], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Div<i64> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: i64) -> Self::Output {
        Integer::divide_c_long_assign(&mut self, rhs);
        self
    }
}

// [u'Integer', u'&i64', u'Integer', u'Integer::divide_c_long_assign', u'lhs',
// [u'ref_mut'], [u'deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Div<&i64> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: &i64) -> Self::Output {
        Integer::divide_c_long_assign(&mut self, *rhs);
        self
    }
}

// [u'&Integer', u'i64', u'Integer', u'Integer::divide_c_long', u'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Div<i64> for &Integer {
    type Output = Integer;

    fn div(self, rhs: i64) -> Self::Output {
        Integer::divide_c_long(self, rhs)
    }
}

// [u'&Integer', u'&i64', u'Integer', u'Integer::divide_c_long', u'no', [],
// [u'deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Div<&i64> for &Integer {
    type Output = Integer;

    fn div(self, rhs: &i64) -> Self::Output {
        Integer::divide_c_long(self, *rhs)
    }
}

// [u'i64', u'Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}], [u'ref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Div<Integer> for i64 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), &rhs)
    }
}

// [u'i64', u'&Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Div<&Integer> for i64 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), rhs)
    }
}

// [u'&i64', u'Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], [u'ref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Div<Integer> for &i64 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), &rhs)
    }
}

// [u'&i64', u'&Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Div<&Integer> for &i64 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), rhs)
    }
}

// [u'Integer', u'u32', u'Integer', u'Integer::divide_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Div<u32> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: u32) -> Self::Output {
        Integer::divide_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// [u'Integer', u'&u32', u'Integer', u'Integer::divide_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}, u'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Div<&u32> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: &u32) -> Self::Output {
        Integer::divide_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// [u'&Integer', u'u32', u'Integer', u'Integer::divide', u'no', [], [u'ref',
// {u'convert': u'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Div<u32> for &Integer {
    type Output = Integer;

    fn div(self, rhs: u32) -> Self::Output {
        Integer::divide(self, &Integer::from(rhs))
    }
}

// [u'&Integer', u'&u32', u'Integer', u'Integer::divide', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Div<&u32> for &Integer {
    type Output = Integer;

    fn div(self, rhs: &u32) -> Self::Output {
        Integer::divide(self, &Integer::from(*rhs))
    }
}

// [u'u32', u'Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}], [u'ref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Div<Integer> for u32 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), &rhs)
    }
}

// [u'u32', u'&Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Div<&Integer> for u32 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), rhs)
    }
}

// [u'&u32', u'Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], [u'ref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Div<Integer> for &u32 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), &rhs)
    }
}

// [u'&u32', u'&Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Div<&Integer> for &u32 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), rhs)
    }
}

// [u'Integer', u'i64', u'Integer', u'Integer::divide_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Div<i64> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: i64) -> Self::Output {
        Integer::divide_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// [u'Integer', u'&i64', u'Integer', u'Integer::divide_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}, u'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Div<&i64> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: &i64) -> Self::Output {
        Integer::divide_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// [u'&Integer', u'i64', u'Integer', u'Integer::divide', u'no', [], [u'ref',
// {u'convert': u'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Div<i64> for &Integer {
    type Output = Integer;

    fn div(self, rhs: i64) -> Self::Output {
        Integer::divide(self, &Integer::from(rhs))
    }
}

// [u'&Integer', u'&i64', u'Integer', u'Integer::divide', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Div<&i64> for &Integer {
    type Output = Integer;

    fn div(self, rhs: &i64) -> Self::Output {
        Integer::divide(self, &Integer::from(*rhs))
    }
}

// [u'i64', u'Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}], [u'ref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Div<Integer> for i64 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), &rhs)
    }
}

// [u'i64', u'&Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Div<&Integer> for i64 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), rhs)
    }
}

// [u'&i64', u'Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], [u'ref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Div<Integer> for &i64 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), &rhs)
    }
}

// [u'&i64', u'&Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Div<&Integer> for &i64 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), rhs)
    }
}

// [u'Integer', u'u64', u'Integer', u'Integer::divide_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}]]
impl Div<u64> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: u64) -> Self::Output {
        Integer::divide_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// [u'Integer', u'&u64', u'Integer', u'Integer::divide_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}, u'deref']]
impl Div<&u64> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: &u64) -> Self::Output {
        Integer::divide_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// [u'&Integer', u'u64', u'Integer', u'Integer::divide', u'no', [], [u'ref',
// {u'convert': u'Integer'}]]
impl Div<u64> for &Integer {
    type Output = Integer;

    fn div(self, rhs: u64) -> Self::Output {
        Integer::divide(self, &Integer::from(rhs))
    }
}

// [u'&Integer', u'&u64', u'Integer', u'Integer::divide', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
impl Div<&u64> for &Integer {
    type Output = Integer;

    fn div(self, rhs: &u64) -> Self::Output {
        Integer::divide(self, &Integer::from(*rhs))
    }
}

// [u'u64', u'Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}], [u'ref']]
impl Div<Integer> for u64 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), &rhs)
    }
}

// [u'u64', u'&Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}], []]
impl Div<&Integer> for u64 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), rhs)
    }
}

// [u'&u64', u'Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], [u'ref']]
impl Div<Integer> for &u64 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), &rhs)
    }
}

// [u'&u64', u'&Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], []]
impl Div<&Integer> for &u64 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), rhs)
    }
}

// [u'Integer', u'i128', u'Integer', u'Integer::divide_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}]]
impl Div<i128> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: i128) -> Self::Output {
        Integer::divide_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// [u'Integer', u'&i128', u'Integer', u'Integer::divide_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}, u'deref']]
impl Div<&i128> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: &i128) -> Self::Output {
        Integer::divide_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// [u'&Integer', u'i128', u'Integer', u'Integer::divide', u'no', [], [u'ref',
// {u'convert': u'Integer'}]]
impl Div<i128> for &Integer {
    type Output = Integer;

    fn div(self, rhs: i128) -> Self::Output {
        Integer::divide(self, &Integer::from(rhs))
    }
}

// [u'&Integer', u'&i128', u'Integer', u'Integer::divide', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
impl Div<&i128> for &Integer {
    type Output = Integer;

    fn div(self, rhs: &i128) -> Self::Output {
        Integer::divide(self, &Integer::from(*rhs))
    }
}

// [u'i128', u'Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}], [u'ref']]
impl Div<Integer> for i128 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), &rhs)
    }
}

// [u'i128', u'&Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}], []]
impl Div<&Integer> for i128 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), rhs)
    }
}

// [u'&i128', u'Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], [u'ref']]
impl Div<Integer> for &i128 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), &rhs)
    }
}

// [u'&i128', u'&Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], []]
impl Div<&Integer> for &i128 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), rhs)
    }
}

// [u'Integer', u'u128', u'Integer', u'Integer::divide_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}]]
impl Div<u128> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: u128) -> Self::Output {
        Integer::divide_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// [u'Integer', u'&u128', u'Integer', u'Integer::divide_assign', u'lhs',
// [u'ref_mut'], [u'ref', {u'convert': u'Integer'}, u'deref']]
impl Div<&u128> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: &u128) -> Self::Output {
        Integer::divide_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// [u'&Integer', u'u128', u'Integer', u'Integer::divide', u'no', [], [u'ref',
// {u'convert': u'Integer'}]]
impl Div<u128> for &Integer {
    type Output = Integer;

    fn div(self, rhs: u128) -> Self::Output {
        Integer::divide(self, &Integer::from(rhs))
    }
}

// [u'&Integer', u'&u128', u'Integer', u'Integer::divide', u'no', [], [u'ref',
// {u'convert': u'Integer'}, u'deref']]
impl Div<&u128> for &Integer {
    type Output = Integer;

    fn div(self, rhs: &u128) -> Self::Output {
        Integer::divide(self, &Integer::from(*rhs))
    }
}

// [u'u128', u'Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}], [u'ref']]
impl Div<Integer> for u128 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), &rhs)
    }
}

// [u'u128', u'&Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}], []]
impl Div<&Integer> for u128 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), rhs)
    }
}

// [u'&u128', u'Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], [u'ref']]
impl Div<Integer> for &u128 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), &rhs)
    }
}

// [u'&u128', u'&Integer', u'Integer', u'Integer::divide', u'no', [u'ref',
// {u'convert': u'Integer'}, u'deref'], []]
impl Div<&Integer> for &u128 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), rhs)
    }
}
