use core::{fmt, mem::ManuallyDrop};
use std::{ffi::CString, os::raw::c_long};

pub(crate) mod ops;

#[repr(transparent)]
pub struct Integer {
    raw: ManuallyDrop<*mut imath_sys::mpz_t>,
}

impl Integer {
    #[allow(dead_code)]
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
        // This operation is safe bc we correctly initialized the raw structure.
        Integer {
            raw: ManuallyDrop::new(raw_mpz),
        }
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
        // $name two integers
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

        #[allow(dead_code)]
        pub(crate) fn $c_long_name(&self, value: c_long) -> Self {
            let self_raw = self.as_mut_ptr();
            let result_int = Integer::new();
            let result_raw = result_int.as_mut_ptr();

            let op_res = unsafe { $c_long_fn(self_raw, value, result_raw) };

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
        sub,
        imath_sys::mp_int_sub,
        sub_c_long,
        imath_sys::mp_int_sub_value
    );

    integer_binops_fn!(
        mul,
        imath_sys::mp_int_mul,
        mul_c_long,
        imath_sys::mp_int_mul_value
    );
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
    fn add_integers() {
        let a = Integer::from_c_long(11111);
        let b = Integer::from_c_long(33333);
        let c = a.add(&b);

        let string_repr = c.to_string();
        assert_eq!(&string_repr, "44444");
    }

    #[test]
    fn subtract_integers() {
        let a = Integer::from_c_long(12345);
        let b = Integer::from_c_long(1234);
        let c = a.sub(&b);

        let string_repr = c.to_string();
        assert_eq!(&string_repr, "11111");
    }

    #[test]
    fn multiply_integers() {
        let a = Integer::from_c_long(50505);
        let b = Integer::from_c_long(5050);
        let c = a.mul(&b);

        let string_repr = c.to_string();
        assert_eq!(&string_repr, "255050250");
    }
}
