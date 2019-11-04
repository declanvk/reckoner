use crate::error::{Result, RimathError};
use core::{
    cell::UnsafeCell,
    cmp::Ordering,
    convert::TryInto,
    fmt,
    mem::{self, MaybeUninit},
    ptr,
    str::FromStr,
};
use std::{
    alloc,
    ffi::CString,
    os::raw::{c_long, c_ulong},
};

pub(crate) mod comparison;
pub(crate) mod conversions;
pub(crate) mod ops;

/// Multiple precision integer value. Always heap allocated, not safe for
/// sharing across threads.
#[repr(transparent)]
pub struct Integer {
    raw: Box<UnsafeCell<imath_sys::mpz_t>>,
}

fn uninit_int() -> Box<MaybeUninit<imath_sys::mpz_t>> {
    // Replace with Box::new_uninit when it is stable (1.40 maybe?).
    let layout = alloc::Layout::new::<MaybeUninit<imath_sys::mpz_t>>();
    let ptr = unsafe { alloc::alloc(layout) };
    // This cast is safe bc the layout was specified for
    // MaybeUninit<imath_sys::mpz_t>
    unsafe { Box::from_raw(ptr.cast()) }
}

fn assume_int_valid(int: Box<MaybeUninit<imath_sys::mpz_t>>) -> Box<UnsafeCell<imath_sys::mpz_t>> {
    // This is safe bc MaybeUninit<imath_sys::mpz_t> and
    // UnsafeCell<imath_sys::mpz_t> are transparently represented as
    // a imath_sys::mpz_t
    unsafe { mem::transmute(int) }
}

impl Integer {
    /// Construct a new integer with a default value of zero.
    pub fn new() -> Self {
        Self::from_c_long(0)
    }

    pub(crate) fn copy_init(other: &Self) -> Self {
        let mut init = uninit_int();
        let other_raw = other.as_mut_ptr();

        {
            // This is safe bc init is entirely local. raw_mpz is also scoped to be less
            // than the lifetime of the value init
            let raw_mpz = init.as_mut_ptr();

            // This is safe bc a valid structure is provided to the unsafe methods. And the
            // src value is of the correct type?
            let res = unsafe { imath_sys::mp_int_init_copy(raw_mpz, other_raw) };

            // Accessing this is safe bc the MP_OK value is only ever used as an error
            // condition.
            if res != unsafe { imath_sys::MP_OK } {
                panic!("Value init failed! {:?}", res);
            }
        }

        Integer {
            raw: assume_int_valid(init),
        }
    }

    pub(crate) fn from_c_long(src: c_long) -> Self {
        let mut init = uninit_int();

        {
            // This is safe bc init is entirely local. raw_mpz is also scoped to be less
            // than the lifetime of the value init
            let raw_mpz = init.as_mut_ptr();

            // This is safe bc a valid structure is provided to the unsafe methods. And the
            // src value is of the correct type?
            let res = unsafe { imath_sys::mp_int_init_value(raw_mpz, src) };

            // Accessing this is safe bc the MP_OK value is only ever used as an error
            // condition.
            if res != unsafe { imath_sys::MP_OK } {
                panic!("Value init failed! {:?}", res);
            }
        }

        Integer {
            raw: assume_int_valid(init),
        }
    }

    pub(crate) fn from_string_repr(src: impl ToString) -> Result<Self> {
        let string_repr =
            CString::new(src.to_string()).map_err(|_| RimathError::IntegerReprContainedNul)?;
        let char_ptr = string_repr.into_raw();

        let mut init = uninit_int();

        {
            // This is safe bc init is entirely local. raw_mpz is also scoped to be less
            // than the lifetime of the value init
            let raw_mpz = init.as_mut_ptr();

            // This is safe bc a valid structure is provided to the unsafe methods. And the
            // src value is of the correct type?
            let res_init = unsafe { imath_sys::mp_int_init(raw_mpz) };

            // Accessing this is safe bc the MP_OK value is only ever used as an error
            // condition.
            if res_init != unsafe { imath_sys::MP_OK } {
                panic!("Init failed! {:?}", res_init);
            }

            // This is safe bc all the data provided to the function is correctly setup
            // (integer was allocated/initialized, char_ptr is 0-terminated).
            let res_read = unsafe { imath_sys::mp_int_read_string(raw_mpz, 10, char_ptr) };

            // Accessing this is safe bc the MP_OK value is only ever used as an error
            // condition.
            if res_read != unsafe { imath_sys::MP_OK } {
                return Err(RimathError::IntegerReprTruncated);
            }
        }

        // This is safe bc we produced the char_ptr earlier from a CString
        let _ = unsafe { CString::from_raw(char_ptr) };

        Ok(Integer {
            raw: assume_int_valid(init),
        })
    }

    pub(crate) fn as_mut_ptr(&self) -> *mut imath_sys::mpz_t {
        self.raw.get()
    }

    // Reports the minimum number of characters required to represent `z` as a
    // zero-terminated string in base-10.
    pub(crate) fn required_display_len(&self) -> usize {
        let self_raw = self.as_mut_ptr();

        // This is safe bc self has been initialized
        let len = unsafe { imath_sys::mp_int_string_len(self_raw, 10) };

        // The output of the call is an i32, check that it is gte zero.
        assert!(len >= 0);
        len as usize
    }

    pub(crate) fn to_cstring(&self) -> CString {
        let required_len = self.required_display_len();
        let self_raw = self.as_mut_ptr();

        let mut char_vec: Vec<u8> = Vec::with_capacity(required_len);
        let res = {
            let char_ptr = char_vec.as_mut_ptr();
            let cap = char_vec.capacity();

            debug_assert_eq!(required_len, cap);
            unsafe {
                imath_sys::mp_int_to_string(self_raw, 10, char_ptr as *mut _, required_len as i32)
            }
        };

        // Accessing this is safe bc the MP_OK value is only ever used as an error
        // condition.
        if res != unsafe { imath_sys::MP_OK } {
            panic!("Writing the value as a string failed! {:?}", res);
        }

        // Setting the length is safe bc we now that the `mp_int_to_string`
        // should have used the entire capacity to write to
        // string.
        unsafe {
            char_vec.set_len(required_len);
        }

        // At this point, char_vec is a zero-terminated vector containing a string
        // representation of the integer.
        let without_nul = &char_vec.as_slice()[..(required_len - 1)];

        CString::new(without_nul).expect("Failed to produce a valid CString")
    }

    /// Replaces the value of `other` with a copy of the value of `self`. No new
    /// memory is allocated unless `self` has more significant digits than
    /// `other` has allocated.
    pub fn copy_to(&self, other: &mut Self) {
        let self_raw = self.as_mut_ptr();
        let other_raw = other.as_mut_ptr();

        // This is safe bc self has been initialized with a value
        let res = unsafe { imath_sys::mp_int_copy(other_raw, self_raw) };

        if res != unsafe { imath_sys::MP_OK } {
            panic!("Copying the value failed! {:?}", res);
        }
    }

    /// Set value of integer to zero
    pub fn zero(&mut self) {
        let self_raw = self.as_mut_ptr();

        unsafe { imath_sys::mp_int_zero(self_raw) };
    }
}

macro_rules! integer_binops_fn {
    ($name:ident, $raw_fn:path, $c_long_name:ident, $c_long_fn:path) => {
        /// $name two integers
        pub fn $name(&self, other: &Self) -> Self {
            let self_raw = self.as_mut_ptr();
            let other_raw = other.as_mut_ptr();

            let result_int = Integer::new();
            let result_raw = result_int.as_mut_ptr();

            let op_res = unsafe { $raw_fn(self_raw, other_raw, result_raw) };

            if op_res != unsafe { imath_sys::MP_OK } {
                panic!("Operation failed! {:?}", op_res);
            }

            result_int
        }

        pub(crate) fn $c_long_name(&self, value: impl Into<c_long>) -> Self {
            let self_raw = self.as_mut_ptr();
            let result_int = Integer::new();
            let result_raw = result_int.as_mut_ptr();

            let op_res = unsafe { $c_long_fn(self_raw, value.into(), result_raw) };

            if op_res != unsafe { imath_sys::MP_OK } {
                panic!("Operation failed! {:?}", op_res);
            }

            result_int
        }
    };
}

impl Integer {
    integer_binops_fn!(
        add,
        imath_sys::mp_int_add,
        add_c_long,
        imath_sys::mp_int_add_value
    );

    integer_binops_fn!(
        subtract,
        imath_sys::mp_int_sub,
        subtract_c_long,
        imath_sys::mp_int_sub_value
    );

    integer_binops_fn!(
        multiply,
        imath_sys::mp_int_mul,
        multiply_c_long,
        imath_sys::mp_int_mul_value
    );

    /// Return the additive inverse
    pub fn negate(&self) -> Self {
        let self_raw = self.as_mut_ptr();
        let result_int = Integer::new();
        let result_raw = result_int.as_mut_ptr();

        let op_res = unsafe { imath_sys::mp_int_neg(self_raw, result_raw) };

        if op_res != unsafe { imath_sys::MP_OK } {
            panic!("Operation failed! {:?}", op_res);
        }

        result_int
    }

    /// Return the absolute value
    pub fn absolute_value(&self) -> Self {
        let self_raw = self.as_mut_ptr();
        let result_int = Integer::new();
        let result_raw = result_int.as_mut_ptr();

        let op_res = unsafe { imath_sys::mp_int_abs(self_raw, result_raw) };

        if op_res != unsafe { imath_sys::MP_OK } {
            panic!("Operation failed! {:?}", op_res);
        }

        result_int
    }

    fn mp_int_div(
        dividend: &Integer,
        divisor: &Integer,
        out_quotient: imath_sys::mp_int,
        out_remainder: imath_sys::mp_int,
    ) {
        assert!(!(out_quotient.is_null() && out_remainder.is_null()));

        let dividend_raw = dividend.as_mut_ptr();
        let divisor_raw = divisor.as_mut_ptr();

        let op_res = unsafe {
            imath_sys::mp_int_div(dividend_raw, divisor_raw, out_quotient, out_remainder)
        };

        if op_res != unsafe { imath_sys::MP_OK } {
            panic!("Operation failed! {:?}", op_res);
        }
    }

    /// Divide two integers and return quotient and remainder
    pub fn divide_full(&self, rhs: &Self) -> (Self, Self) {
        let quotient = Integer::new();
        let quotient_raw = quotient.as_mut_ptr();
        let remainder = Integer::new();
        let remainder_raw = remainder.as_mut_ptr();

        Integer::mp_int_div(self, rhs, quotient_raw, remainder_raw);

        (quotient, remainder)
    }

    /// Divide two integers and return only quotient
    pub fn divide(&self, rhs: &Self) -> Self {
        let quotient = Integer::new();
        let quotient_raw = quotient.as_mut_ptr();

        Integer::mp_int_div(self, rhs, quotient_raw, ptr::null_mut());

        quotient
    }

    /// Divide two integers and return only remainder
    pub fn remainder(&self, other: &Self) -> Self {
        let result = Integer::new();
        let result_raw = result.as_mut_ptr();

        Integer::mp_int_div(self, other, result_raw, ptr::null_mut());

        result
    }

    fn mp_int_div_value(
        dividend: &Integer,
        divisor: impl Into<c_long>,
        out_quotient: imath_sys::mp_int,
        out_remainder: *mut c_long,
    ) {
        let divident_raw = dividend.as_mut_ptr();

        let op_res = unsafe {
            imath_sys::mp_int_div_value(divident_raw, divisor.into(), out_quotient, out_remainder)
        };

        if op_res != unsafe { imath_sys::MP_OK } {
            panic!("Operation failed! {:?}", op_res);
        }
    }

    #[allow(dead_code)]
    pub(crate) fn divide_full_c_long<V>(&self, value: V) -> (Integer, V)
    where
        V: Into<c_long>,
        c_long: TryInto<V>,
    {
        let mut remainder: c_long = 0;
        let quotient = Integer::new();
        let quotient_raw = quotient.as_mut_ptr();

        Integer::mp_int_div_value(self, value.into(), quotient_raw, &mut remainder);

        // This is safe bc the modulo operation will always return within the range
        // [0, value].
        (
            quotient,
            remainder
                .try_into()
                .map_err(|_| RimathError::RemainedOutsideBounds)
                .unwrap(),
        )
    }

    pub(crate) fn divide_c_long(&self, value: impl Into<c_long>) -> Self {
        let quotient = Integer::new();
        let quotient_raw = quotient.as_mut_ptr();

        Integer::mp_int_div_value(self, value, quotient_raw, ptr::null_mut());

        quotient
    }

    pub(crate) fn remainder_c_long<V>(&self, value: V) -> V
    where
        V: Into<c_long>,
        c_long: TryInto<V>,
    {
        let mut result: c_long = 0;
        let result_raw = (&mut result) as *mut _;

        Integer::mp_int_div_value(self, value, ptr::null_mut(), result_raw);

        // This is safe bc the modulo operation will always return within the range
        // [0, value].
        result
            .try_into()
            .map_err(|_| RimathError::RemainedOutsideBounds)
            .unwrap()
    }

    /// Compare two integers
    pub fn compare(&self, rhs: &Self) -> Ordering {
        let self_raw = self.as_mut_ptr();
        let rhs_raw = rhs.as_mut_ptr();

        // This is safe bc both self & rhs have been initialized correctly
        let raw_cmp = unsafe { imath_sys::mp_int_compare(self_raw, rhs_raw) };

        raw_cmp.cmp(&0)
    }

    /// Compare the magnitude of two integers, not taking sign into account.
    pub fn compare_magnitude(&self, rhs: &Self) -> Ordering {
        let self_raw = self.as_mut_ptr();
        let rhs_raw = rhs.as_mut_ptr();

        // This is safe bc both self & rhs have been initialized correctly
        let raw_cmp = unsafe { imath_sys::mp_int_compare_unsigned(self_raw, rhs_raw) };

        raw_cmp.cmp(&0)
    }

    /// Compare an integer to zero.
    pub fn compare_zero(&self) -> Ordering {
        let self_raw = self.as_mut_ptr();

        // This is safe bc both self has been initialized correctly
        let raw_cmp = unsafe { imath_sys::mp_int_compare_zero(self_raw) };

        raw_cmp.cmp(&0)
    }

    pub(crate) fn compare_c_long(&self, value: impl Into<c_long>) -> Ordering {
        let self_raw = self.as_mut_ptr();
        let value = value.into();

        // This is safe bc both self has been initialized correctly
        let raw_cmp = unsafe { imath_sys::mp_int_compare_value(self_raw, value) };

        raw_cmp.cmp(&0)
    }

    #[allow(dead_code)]
    pub(crate) fn compare_c_ulong(&self, value: impl Into<c_ulong>) -> Ordering {
        let self_raw = self.as_mut_ptr();
        let value = value.into();

        // This is safe bc both self has been initialized correctly
        let raw_cmp = unsafe { imath_sys::mp_int_compare_uvalue(self_raw, value) };

        raw_cmp.cmp(&0)
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string_repr = self.to_cstring();

        f.write_str(string_repr.to_str().unwrap())
    }
}

impl fmt::Debug for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // This is safe bc self has been initialized
        let imath_sys::mpz_t {
            single,
            digits,
            alloc,
            used,
            sign,
        } = unsafe { *self.raw.get() };

        write!(
            f,
            "Integer {{ single: {:?}, digits: {:p}, alloc: {:?}, used: {:?}, sign: {:?} }}",
            single, digits, alloc, used, sign
        )
    }
}

impl Clone for Integer {
    fn clone(&self) -> Self {
        Integer::copy_init(self)
    }

    fn clone_from(&mut self, source: &Self) {
        source.copy_to(self);
    }
}

impl Default for Integer {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for Integer {
    type Err = RimathError;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        Integer::from_string_repr(s)
    }
}

impl Drop for Integer {
    fn drop(&mut self) {
        unsafe {
            let raw = self.as_mut_ptr();

            imath_sys::mp_int_clear(raw);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_default_integer() {
        let int = Integer::new();

        let string_repr = int.to_string();
        assert_eq!(&string_repr, "0");
    }

    #[test]
    fn create_integer_with_value() {
        let int = Integer::from_c_long(20000);

        let string_repr = int.to_string();
        assert_eq!(&string_repr, "20000");
    }

    #[test]
    fn parse_big_integer() {
        let int: Integer =
            "98712698346126837461287318238761234897612839471623487619872364981726348176234"
                .parse()
                .unwrap();
        #[allow(clippy::eq_op)]
        let zero = &int - &int;

        assert_eq!(zero, 0)
    }

    #[test]
    fn absolute_value_integer() {
        let neg_int: Integer = "-37129740".parse().unwrap();
        let pos_int: Integer = "37129740".parse().unwrap();

        assert_eq!(neg_int.absolute_value(), pos_int);
        assert_eq!(pos_int.absolute_value(), 37_129_740);
    }

    #[test]
    fn zero_integer() {
        let mut big_int: Integer =
            "98712698346126837461287318238761234897612839471623487619872364981726348176234"
                .parse()
                .unwrap();
        let mut small_int: Integer = (-4_565_234).into();

        big_int.zero();
        assert_eq!(big_int, 0);

        small_int.zero();
        assert_eq!(small_int, 0);
    }

    #[test]
    fn clone_integer() {
        let a: Integer =
            "98712698346126837461287318238761234897612839471623487619872364981726348176234"
                .parse()
                .unwrap();

        let b = a.clone();

        assert_eq!(a, b);
    }
}
