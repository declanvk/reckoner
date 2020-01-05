use crate::integer::Integer;
use core::ops::Div;

// Div          The division operator /.

// ['Integer', 'Integer', 'Integer', 'Integer::divide_assign', 'lhs',
// ['ref_mut'], ['ref']]
impl Div<Integer> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: Integer) -> Self::Output {
        Integer::divide_assign(&mut self, &rhs);
        self
    }
}

// ['Integer', '&Integer', 'Integer', 'Integer::divide_assign', 'lhs',
// ['ref_mut'], []]
impl Div<&Integer> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: &Integer) -> Self::Output {
        Integer::divide_assign(&mut self, rhs);
        self
    }
}

// ['&Integer', 'Integer', 'Integer', 'Integer::divide', 'no', [], ['ref']]
impl Div<Integer> for &Integer {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(self, &rhs)
    }
}

// ['&Integer', '&Integer', 'Integer', 'Integer::divide', 'no', [], []]
impl Div<&Integer> for &Integer {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(self, rhs)
    }
}

// ['Integer', 'i8', 'Integer', 'Integer::divide_c_long_assign', 'lhs',
// ['ref_mut'], []]
impl Div<i8> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: i8) -> Self::Output {
        Integer::divide_c_long_assign(&mut self, rhs);
        self
    }
}

// ['Integer', '&i8', 'Integer', 'Integer::divide_c_long_assign', 'lhs',
// ['ref_mut'], ['deref']]
impl Div<&i8> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: &i8) -> Self::Output {
        Integer::divide_c_long_assign(&mut self, *rhs);
        self
    }
}

// ['&Integer', 'i8', 'Integer', 'Integer::divide_c_long', 'no', [], []]
impl Div<i8> for &Integer {
    type Output = Integer;

    fn div(self, rhs: i8) -> Self::Output {
        Integer::divide_c_long(self, rhs)
    }
}

// ['&Integer', '&i8', 'Integer', 'Integer::divide_c_long', 'no', [], ['deref']]
impl Div<&i8> for &Integer {
    type Output = Integer;

    fn div(self, rhs: &i8) -> Self::Output {
        Integer::divide_c_long(self, *rhs)
    }
}

// ['i8', 'Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}], ['ref']]
impl Div<Integer> for i8 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), &rhs)
    }
}

// ['i8', '&Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}], []]
impl Div<&Integer> for i8 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), rhs)
    }
}

// ['&i8', 'Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], ['ref']]
impl Div<Integer> for &i8 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), &rhs)
    }
}

// ['&i8', '&Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], []]
impl Div<&Integer> for &i8 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), rhs)
    }
}

// ['Integer', 'u8', 'Integer', 'Integer::divide_c_long_assign', 'lhs',
// ['ref_mut'], []]
impl Div<u8> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: u8) -> Self::Output {
        Integer::divide_c_long_assign(&mut self, rhs);
        self
    }
}

// ['Integer', '&u8', 'Integer', 'Integer::divide_c_long_assign', 'lhs',
// ['ref_mut'], ['deref']]
impl Div<&u8> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: &u8) -> Self::Output {
        Integer::divide_c_long_assign(&mut self, *rhs);
        self
    }
}

// ['&Integer', 'u8', 'Integer', 'Integer::divide_c_long', 'no', [], []]
impl Div<u8> for &Integer {
    type Output = Integer;

    fn div(self, rhs: u8) -> Self::Output {
        Integer::divide_c_long(self, rhs)
    }
}

// ['&Integer', '&u8', 'Integer', 'Integer::divide_c_long', 'no', [], ['deref']]
impl Div<&u8> for &Integer {
    type Output = Integer;

    fn div(self, rhs: &u8) -> Self::Output {
        Integer::divide_c_long(self, *rhs)
    }
}

// ['u8', 'Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}], ['ref']]
impl Div<Integer> for u8 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), &rhs)
    }
}

// ['u8', '&Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}], []]
impl Div<&Integer> for u8 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), rhs)
    }
}

// ['&u8', 'Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], ['ref']]
impl Div<Integer> for &u8 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), &rhs)
    }
}

// ['&u8', '&Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], []]
impl Div<&Integer> for &u8 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), rhs)
    }
}

// ['Integer', 'i16', 'Integer', 'Integer::divide_c_long_assign', 'lhs',
// ['ref_mut'], []]
impl Div<i16> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: i16) -> Self::Output {
        Integer::divide_c_long_assign(&mut self, rhs);
        self
    }
}

// ['Integer', '&i16', 'Integer', 'Integer::divide_c_long_assign', 'lhs',
// ['ref_mut'], ['deref']]
impl Div<&i16> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: &i16) -> Self::Output {
        Integer::divide_c_long_assign(&mut self, *rhs);
        self
    }
}

// ['&Integer', 'i16', 'Integer', 'Integer::divide_c_long', 'no', [], []]
impl Div<i16> for &Integer {
    type Output = Integer;

    fn div(self, rhs: i16) -> Self::Output {
        Integer::divide_c_long(self, rhs)
    }
}

// ['&Integer', '&i16', 'Integer', 'Integer::divide_c_long', 'no', [],
// ['deref']]
impl Div<&i16> for &Integer {
    type Output = Integer;

    fn div(self, rhs: &i16) -> Self::Output {
        Integer::divide_c_long(self, *rhs)
    }
}

// ['i16', 'Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}], ['ref']]
impl Div<Integer> for i16 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), &rhs)
    }
}

// ['i16', '&Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}], []]
impl Div<&Integer> for i16 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), rhs)
    }
}

// ['&i16', 'Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], ['ref']]
impl Div<Integer> for &i16 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), &rhs)
    }
}

// ['&i16', '&Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], []]
impl Div<&Integer> for &i16 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), rhs)
    }
}

// ['Integer', 'u16', 'Integer', 'Integer::divide_c_long_assign', 'lhs',
// ['ref_mut'], []]
impl Div<u16> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: u16) -> Self::Output {
        Integer::divide_c_long_assign(&mut self, rhs);
        self
    }
}

// ['Integer', '&u16', 'Integer', 'Integer::divide_c_long_assign', 'lhs',
// ['ref_mut'], ['deref']]
impl Div<&u16> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: &u16) -> Self::Output {
        Integer::divide_c_long_assign(&mut self, *rhs);
        self
    }
}

// ['&Integer', 'u16', 'Integer', 'Integer::divide_c_long', 'no', [], []]
impl Div<u16> for &Integer {
    type Output = Integer;

    fn div(self, rhs: u16) -> Self::Output {
        Integer::divide_c_long(self, rhs)
    }
}

// ['&Integer', '&u16', 'Integer', 'Integer::divide_c_long', 'no', [],
// ['deref']]
impl Div<&u16> for &Integer {
    type Output = Integer;

    fn div(self, rhs: &u16) -> Self::Output {
        Integer::divide_c_long(self, *rhs)
    }
}

// ['u16', 'Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}], ['ref']]
impl Div<Integer> for u16 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), &rhs)
    }
}

// ['u16', '&Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}], []]
impl Div<&Integer> for u16 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), rhs)
    }
}

// ['&u16', 'Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], ['ref']]
impl Div<Integer> for &u16 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), &rhs)
    }
}

// ['&u16', '&Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], []]
impl Div<&Integer> for &u16 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), rhs)
    }
}

// ['Integer', 'i32', 'Integer', 'Integer::divide_c_long_assign', 'lhs',
// ['ref_mut'], []]
impl Div<i32> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: i32) -> Self::Output {
        Integer::divide_c_long_assign(&mut self, rhs);
        self
    }
}

// ['Integer', '&i32', 'Integer', 'Integer::divide_c_long_assign', 'lhs',
// ['ref_mut'], ['deref']]
impl Div<&i32> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: &i32) -> Self::Output {
        Integer::divide_c_long_assign(&mut self, *rhs);
        self
    }
}

// ['&Integer', 'i32', 'Integer', 'Integer::divide_c_long', 'no', [], []]
impl Div<i32> for &Integer {
    type Output = Integer;

    fn div(self, rhs: i32) -> Self::Output {
        Integer::divide_c_long(self, rhs)
    }
}

// ['&Integer', '&i32', 'Integer', 'Integer::divide_c_long', 'no', [],
// ['deref']]
impl Div<&i32> for &Integer {
    type Output = Integer;

    fn div(self, rhs: &i32) -> Self::Output {
        Integer::divide_c_long(self, *rhs)
    }
}

// ['i32', 'Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}], ['ref']]
impl Div<Integer> for i32 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), &rhs)
    }
}

// ['i32', '&Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}], []]
impl Div<&Integer> for i32 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), rhs)
    }
}

// ['&i32', 'Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], ['ref']]
impl Div<Integer> for &i32 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), &rhs)
    }
}

// ['&i32', '&Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], []]
impl Div<&Integer> for &i32 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), rhs)
    }
}

// ['Integer', 'u32', 'Integer', 'Integer::divide_c_long_assign', 'lhs',
// ['ref_mut'], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Div<u32> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: u32) -> Self::Output {
        Integer::divide_c_long_assign(&mut self, rhs);
        self
    }
}

// ['Integer', '&u32', 'Integer', 'Integer::divide_c_long_assign', 'lhs',
// ['ref_mut'], ['deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Div<&u32> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: &u32) -> Self::Output {
        Integer::divide_c_long_assign(&mut self, *rhs);
        self
    }
}

// ['&Integer', 'u32', 'Integer', 'Integer::divide_c_long', 'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Div<u32> for &Integer {
    type Output = Integer;

    fn div(self, rhs: u32) -> Self::Output {
        Integer::divide_c_long(self, rhs)
    }
}

// ['&Integer', '&u32', 'Integer', 'Integer::divide_c_long', 'no', [],
// ['deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Div<&u32> for &Integer {
    type Output = Integer;

    fn div(self, rhs: &u32) -> Self::Output {
        Integer::divide_c_long(self, *rhs)
    }
}

// ['u32', 'Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}], ['ref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Div<Integer> for u32 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), &rhs)
    }
}

// ['u32', '&Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Div<&Integer> for u32 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), rhs)
    }
}

// ['&u32', 'Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], ['ref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Div<Integer> for &u32 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), &rhs)
    }
}

// ['&u32', '&Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Div<&Integer> for &u32 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), rhs)
    }
}

// ['Integer', 'i64', 'Integer', 'Integer::divide_c_long_assign', 'lhs',
// ['ref_mut'], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Div<i64> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: i64) -> Self::Output {
        Integer::divide_c_long_assign(&mut self, rhs);
        self
    }
}

// ['Integer', '&i64', 'Integer', 'Integer::divide_c_long_assign', 'lhs',
// ['ref_mut'], ['deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Div<&i64> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: &i64) -> Self::Output {
        Integer::divide_c_long_assign(&mut self, *rhs);
        self
    }
}

// ['&Integer', 'i64', 'Integer', 'Integer::divide_c_long', 'no', [], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Div<i64> for &Integer {
    type Output = Integer;

    fn div(self, rhs: i64) -> Self::Output {
        Integer::divide_c_long(self, rhs)
    }
}

// ['&Integer', '&i64', 'Integer', 'Integer::divide_c_long', 'no', [],
// ['deref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Div<&i64> for &Integer {
    type Output = Integer;

    fn div(self, rhs: &i64) -> Self::Output {
        Integer::divide_c_long(self, *rhs)
    }
}

// ['i64', 'Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}], ['ref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Div<Integer> for i64 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), &rhs)
    }
}

// ['i64', '&Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Div<&Integer> for i64 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), rhs)
    }
}

// ['&i64', 'Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], ['ref']]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Div<Integer> for &i64 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), &rhs)
    }
}

// ['&i64', '&Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], []]
#[cfg(all(target_pointer_width = "64", not(windows)))]
impl Div<&Integer> for &i64 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), rhs)
    }
}

// ['Integer', 'u32', 'Integer', 'Integer::divide_assign', 'lhs', ['ref_mut'],
// ['ref', {'convert': 'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Div<u32> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: u32) -> Self::Output {
        Integer::divide_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Integer', '&u32', 'Integer', 'Integer::divide_assign', 'lhs', ['ref_mut'],
// ['ref', {'convert': 'Integer'}, 'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Div<&u32> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: &u32) -> Self::Output {
        Integer::divide_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Integer', 'u32', 'Integer', 'Integer::divide', 'no', [], ['ref',
// {'convert': 'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Div<u32> for &Integer {
    type Output = Integer;

    fn div(self, rhs: u32) -> Self::Output {
        Integer::divide(self, &Integer::from(rhs))
    }
}

// ['&Integer', '&u32', 'Integer', 'Integer::divide', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Div<&u32> for &Integer {
    type Output = Integer;

    fn div(self, rhs: &u32) -> Self::Output {
        Integer::divide(self, &Integer::from(*rhs))
    }
}

// ['u32', 'Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}], ['ref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Div<Integer> for u32 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), &rhs)
    }
}

// ['u32', '&Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Div<&Integer> for u32 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), rhs)
    }
}

// ['&u32', 'Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], ['ref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Div<Integer> for &u32 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), &rhs)
    }
}

// ['&u32', '&Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Div<&Integer> for &u32 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), rhs)
    }
}

// ['Integer', 'i64', 'Integer', 'Integer::divide_assign', 'lhs', ['ref_mut'],
// ['ref', {'convert': 'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Div<i64> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: i64) -> Self::Output {
        Integer::divide_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Integer', '&i64', 'Integer', 'Integer::divide_assign', 'lhs', ['ref_mut'],
// ['ref', {'convert': 'Integer'}, 'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Div<&i64> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: &i64) -> Self::Output {
        Integer::divide_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Integer', 'i64', 'Integer', 'Integer::divide', 'no', [], ['ref',
// {'convert': 'Integer'}]]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Div<i64> for &Integer {
    type Output = Integer;

    fn div(self, rhs: i64) -> Self::Output {
        Integer::divide(self, &Integer::from(rhs))
    }
}

// ['&Integer', '&i64', 'Integer', 'Integer::divide', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Div<&i64> for &Integer {
    type Output = Integer;

    fn div(self, rhs: &i64) -> Self::Output {
        Integer::divide(self, &Integer::from(*rhs))
    }
}

// ['i64', 'Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}], ['ref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Div<Integer> for i64 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), &rhs)
    }
}

// ['i64', '&Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Div<&Integer> for i64 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), rhs)
    }
}

// ['&i64', 'Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], ['ref']]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Div<Integer> for &i64 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), &rhs)
    }
}

// ['&i64', '&Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], []]
#[cfg(not(all(target_pointer_width = "64", not(windows))))]
impl Div<&Integer> for &i64 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), rhs)
    }
}

// ['Integer', 'u64', 'Integer', 'Integer::divide_assign', 'lhs', ['ref_mut'],
// ['ref', {'convert': 'Integer'}]]
impl Div<u64> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: u64) -> Self::Output {
        Integer::divide_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Integer', '&u64', 'Integer', 'Integer::divide_assign', 'lhs', ['ref_mut'],
// ['ref', {'convert': 'Integer'}, 'deref']]
impl Div<&u64> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: &u64) -> Self::Output {
        Integer::divide_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Integer', 'u64', 'Integer', 'Integer::divide', 'no', [], ['ref',
// {'convert': 'Integer'}]]
impl Div<u64> for &Integer {
    type Output = Integer;

    fn div(self, rhs: u64) -> Self::Output {
        Integer::divide(self, &Integer::from(rhs))
    }
}

// ['&Integer', '&u64', 'Integer', 'Integer::divide', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
impl Div<&u64> for &Integer {
    type Output = Integer;

    fn div(self, rhs: &u64) -> Self::Output {
        Integer::divide(self, &Integer::from(*rhs))
    }
}

// ['u64', 'Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}], ['ref']]
impl Div<Integer> for u64 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), &rhs)
    }
}

// ['u64', '&Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}], []]
impl Div<&Integer> for u64 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), rhs)
    }
}

// ['&u64', 'Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], ['ref']]
impl Div<Integer> for &u64 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), &rhs)
    }
}

// ['&u64', '&Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], []]
impl Div<&Integer> for &u64 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), rhs)
    }
}

// ['Integer', 'i128', 'Integer', 'Integer::divide_assign', 'lhs', ['ref_mut'],
// ['ref', {'convert': 'Integer'}]]
impl Div<i128> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: i128) -> Self::Output {
        Integer::divide_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Integer', '&i128', 'Integer', 'Integer::divide_assign', 'lhs', ['ref_mut'],
// ['ref', {'convert': 'Integer'}, 'deref']]
impl Div<&i128> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: &i128) -> Self::Output {
        Integer::divide_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Integer', 'i128', 'Integer', 'Integer::divide', 'no', [], ['ref',
// {'convert': 'Integer'}]]
impl Div<i128> for &Integer {
    type Output = Integer;

    fn div(self, rhs: i128) -> Self::Output {
        Integer::divide(self, &Integer::from(rhs))
    }
}

// ['&Integer', '&i128', 'Integer', 'Integer::divide', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
impl Div<&i128> for &Integer {
    type Output = Integer;

    fn div(self, rhs: &i128) -> Self::Output {
        Integer::divide(self, &Integer::from(*rhs))
    }
}

// ['i128', 'Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}], ['ref']]
impl Div<Integer> for i128 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), &rhs)
    }
}

// ['i128', '&Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}], []]
impl Div<&Integer> for i128 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), rhs)
    }
}

// ['&i128', 'Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], ['ref']]
impl Div<Integer> for &i128 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), &rhs)
    }
}

// ['&i128', '&Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], []]
impl Div<&Integer> for &i128 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), rhs)
    }
}

// ['Integer', 'u128', 'Integer', 'Integer::divide_assign', 'lhs', ['ref_mut'],
// ['ref', {'convert': 'Integer'}]]
impl Div<u128> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: u128) -> Self::Output {
        Integer::divide_assign(&mut self, &Integer::from(rhs));
        self
    }
}

// ['Integer', '&u128', 'Integer', 'Integer::divide_assign', 'lhs', ['ref_mut'],
// ['ref', {'convert': 'Integer'}, 'deref']]
impl Div<&u128> for Integer {
    type Output = Integer;

    fn div(mut self, rhs: &u128) -> Self::Output {
        Integer::divide_assign(&mut self, &Integer::from(*rhs));
        self
    }
}

// ['&Integer', 'u128', 'Integer', 'Integer::divide', 'no', [], ['ref',
// {'convert': 'Integer'}]]
impl Div<u128> for &Integer {
    type Output = Integer;

    fn div(self, rhs: u128) -> Self::Output {
        Integer::divide(self, &Integer::from(rhs))
    }
}

// ['&Integer', '&u128', 'Integer', 'Integer::divide', 'no', [], ['ref',
// {'convert': 'Integer'}, 'deref']]
impl Div<&u128> for &Integer {
    type Output = Integer;

    fn div(self, rhs: &u128) -> Self::Output {
        Integer::divide(self, &Integer::from(*rhs))
    }
}

// ['u128', 'Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}], ['ref']]
impl Div<Integer> for u128 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), &rhs)
    }
}

// ['u128', '&Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}], []]
impl Div<&Integer> for u128 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(self), rhs)
    }
}

// ['&u128', 'Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], ['ref']]
impl Div<Integer> for &u128 {
    type Output = Integer;

    fn div(self, rhs: Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), &rhs)
    }
}

// ['&u128', '&Integer', 'Integer', 'Integer::divide', 'no', ['ref', {'convert':
// 'Integer'}, 'deref'], []]
impl Div<&Integer> for &u128 {
    type Output = Integer;

    fn div(self, rhs: &Integer) -> Self::Output {
        Integer::divide(&Integer::from(*self), rhs)
    }
}
