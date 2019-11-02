use crate::error::{Result, RimathError};
use core::{cmp::Ordering, fmt, mem::ManuallyDrop};
use std::{
    ffi::CString,
    os::raw::{c_long, c_ulong},
    str::FromStr,
};

pub(crate) mod comparison;
pub(crate) mod conversions;
pub(crate) mod ops;

/// Multiple precision integer value. Always heap allocated, not safe for
/// sharing across threads.
#[repr(transparent)]
pub struct Integer {
    raw: ManuallyDrop<*mut imath_sys::mpz_t>,
}

impl Integer {
    pub(crate) fn from_c_long(src: c_long) -> Self {
        // This is safe?
        let raw_mpz = unsafe { imath_sys::mp_int_alloc() };

        // This is safe bc a valid structure is provided to the unsafe methods. And the
        // src value is of the correct type?
        let res = unsafe { imath_sys::mp_int_init_value(raw_mpz, src) };

        // Accessing this is safe bc the MP_OK value is only ever used as an error
        // condition.
        if res != unsafe { imath_sys::MP_OK } {
            panic!("Value init failed! {:?}", res);
        }

        Integer {
            raw: ManuallyDrop::new(raw_mpz),
        }
    }

    pub(crate) fn from_string_repr(src: impl ToString) -> Result<Self> {
        let string_repr =
            CString::new(src.to_string()).map_err(|_| RimathError::IntegerReprContainedNul)?;
        let char_ptr = string_repr.into_raw();

        // This is safe?
        let raw_mpz = unsafe { imath_sys::mp_int_alloc() };

        // This is safe bc all the data provided to the function is correctly setup
        // (integer was allocated/initialized, char_ptr is 0-terminated).
        let res = unsafe { imath_sys::mp_int_read_string(raw_mpz, 10, char_ptr) };

        // Accessing this is safe bc the MP_OK value is only ever used as an error
        // condition.
        if res != unsafe { imath_sys::MP_OK } {
            return Err(RimathError::IntegerReprTruncated);
        }

        // This is safe bc we produced the char_ptr earlier from a CString
        let _ = unsafe { CString::from_raw(char_ptr) };

        Ok(Integer {
            raw: ManuallyDrop::new(raw_mpz),
        })
    }

    pub(crate) fn as_mut_ptr(&self) -> *mut imath_sys::mpz_t {
        *self.raw
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
}

impl Integer {
    /// Construct a new integer with a default value of zero.
    pub fn new() -> Self {
        Self::from_c_long(0)
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

    /// Compare an integer value to zero.
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
        } = unsafe { **self.raw };

        write!(
            f,
            "Integer {{ single: {:?}, digits: {:p}, alloc: {:?}, used: {:?}, sign: {:?} }}",
            single, digits, alloc, used, sign
        )
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

            imath_sys::mp_int_free(raw);
            ManuallyDrop::drop(&mut self.raw);
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
        const INT_STRING: &str =
            "98712698346126837461287318238761234897612839471623487619872364981726348176234";

        let int: Integer = INT_STRING.parse().unwrap();
        #[allow(clippy::eq_op)]
        let zero = &int - &int;

        assert_eq!(zero, 0)
    }
}
