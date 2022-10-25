use crate::{
    error::{Error, Result},
    integer::Integer,
};
use core::{
    cmp::Ordering, convert::TryFrom, ffi::c_long, fmt, mem, mem::MaybeUninit, ptr, ptr::NonNull,
    str::FromStr,
};
use std::{alloc, ffi::CString};

pub(crate) mod comparisons;
pub(crate) mod conversions;
pub mod ops;

/// Multiple precision rational value.
#[repr(transparent)]
pub struct Rational {
    // This value must be constructed from a Box and then when Drop, must be reconstructed so that
    // the Box Drop can free the memory used.
    raw: NonNull<creachadair_imath_sys::mpq_t>,
}

impl Rational {
    pub(crate) fn uninit() -> Box<MaybeUninit<creachadair_imath_sys::mpq_t>> {
        // Replace with Box::new_uninit when it is stable (1.41 maybe?).
        let layout = alloc::Layout::new::<MaybeUninit<creachadair_imath_sys::mpq_t>>();
        let ptr = unsafe { alloc::alloc(layout) };
        // This cast is safe bc the layout was specified for
        // MaybeUninit<creachadair_imath_sys::mpq_t>
        unsafe { Box::from_raw(ptr.cast()) }
    }

    /// Create a new rational with a default value of zero (0/1).
    ///
    /// # Example
    /// ```
    /// use reckoner::Rational;
    ///
    /// let r = Rational::new();
    ///
    /// assert_eq!(r, (0, 1));
    /// ```
    pub fn new() -> Self {
        let mut init = Rational::uninit();

        {
            let raw_mpq = init.as_mut_ptr();

            // This function call is safe because the mp_rat_init function only requires
            // that the mpq_t struct has been correctly allocated.
            let res = unsafe { creachadair_imath_sys::mp_rat_init(raw_mpq) };

            imath_check_panic!(res, "Rational init failed!");
        }

        // This cast is safe (from MaybeUninit<creachadair_imath_sys::mpq_t> to
        // creachadair_imath_sys::mpq_t) because the value is now initialized.
        unsafe { Rational::from_raw(Box::into_raw(init).cast()) }
    }

    pub(crate) fn copy_init(other: &Self) -> Self {
        let mut init = Rational::uninit();
        let other_raw = other.as_raw();

        {
            let raw_mpq = init.as_mut_ptr();

            // This function call is safe because the mpq_t struct has been correctly
            // allocated and `other_raw` has been successfully initialized.
            let res = unsafe { creachadair_imath_sys::mp_rat_init_copy(raw_mpq, other_raw) };

            imath_check_panic!(res, "Rational init failed!");
        }

        // This cast is safe (from MaybeUninit<creachadair_imath_sys::mpq_t> to
        // creachadair_imath_sys::mpq_t) because the value is now initialized.
        unsafe { Rational::from_raw(Box::into_raw(init).cast()) }
    }

    /// Construct a Rational from a raw non-null pointer to
    /// `creachadair_imath_sys::mpq_t`.
    ///
    /// # Safety
    ///
    /// This function must only every be called once for a given pointer, and
    /// the pointer must point to an initialized `creachadair_imath_sys::mpq_t`
    /// struct. The recommendation is to only use raw pointers from the
    /// `Rational::into_raw` function.
    ///
    /// In ths context, initialized means that the
    /// `creachadair_imath_sys::mpq_t` has been the argument of a call to
    /// `creachadair_imath_sys::mp_rat_init`.
    ///
    /// # Example
    /// ```
    /// use creachadair_imath_sys::{mp_rat_zero, MP_OK};
    /// use reckoner::Rational;
    ///
    /// let a = Rational::from((456, 123));
    ///
    /// assert_eq!(a, (456, 123));
    ///
    /// let a_raw = Rational::into_raw(a);
    ///
    /// unsafe { mp_rat_zero(a_raw) };
    ///
    /// let a = unsafe { Rational::from_raw(a_raw) };
    ///
    /// assert_eq!(a, (0, 1));
    /// ```
    pub unsafe fn from_raw(raw: *mut creachadair_imath_sys::mpq_t) -> Self {
        assert!(!raw.is_null());

        // This is safe bc the invariants of the function and because it was checked
        // that the pointer is not null.
        #[allow(unused_unsafe)]
        let raw = unsafe { NonNull::new_unchecked(raw) };

        Rational { raw }
    }

    /// Consumes the Rational, returning a wrapped raw pointer.
    ///
    /// # Example
    /// ```
    /// use creachadair_imath_sys::{mp_rat_add, MP_OK};
    /// use reckoner::Rational;
    ///
    /// let a = Rational::from((377, 500));
    /// let b = Rational::from((123, 500));
    /// let c = Rational::new();
    ///
    /// let a_raw = Rational::into_raw(a);
    /// let b_raw = Rational::into_raw(b);
    /// let c_raw = Rational::into_raw(c);
    ///
    /// let op_res = unsafe { mp_rat_add(a_raw, b_raw, c_raw) };
    ///
    /// if op_res != unsafe { MP_OK } {
    ///     panic!("Operation failed.")
    /// }
    ///
    /// let a = unsafe { Rational::from_raw(a_raw) };
    /// let b = unsafe { Rational::from_raw(b_raw) };
    /// let c = unsafe { Rational::from_raw(c_raw) };
    ///
    /// assert_eq!(a, (377, 500));
    /// assert_eq!(b, (123, 500));
    /// assert_eq!(c, (500, 500));
    /// ```
    pub fn into_raw(mut rational: Rational) -> *mut creachadair_imath_sys::mpq_t {
        let raw = mem::replace(&mut rational.raw, NonNull::dangling());

        // The destructor does not need to run, as we are intentionally leaking the
        // resources here.
        mem::forget(rational);

        raw.as_ptr()
    }

    // Internal use only
    pub(crate) fn as_raw(&self) -> *mut creachadair_imath_sys::mpq_t {
        self.raw.as_ptr()
    }

    /// Reduces `r` in-place to lowest terms and canonical form.
    ///
    /// Zero is represented as 0/1, one as 1/1, and signs are adjusted so that
    /// the sign of the value is carried by the numerator.
    ///
    /// # Example
    /// ```
    /// use reckoner::Rational;
    ///
    /// let mut a = Rational::from((24, 2));
    /// let mut b = Rational::from((24, 3));
    /// let mut c = Rational::from((24, 4));
    ///
    /// a.reduce();
    /// b.reduce();
    /// c.reduce();
    ///
    /// assert_eq!(a, (12, 1));
    /// assert_eq!(b, (8, 1));
    /// assert_eq!(c, (6, 1));
    /// ```
    pub fn reduce(&mut self) {
        let self_raw = self.as_raw();

        let res = unsafe { creachadair_imath_sys::mp_rat_reduce(self_raw) };

        imath_check_panic!(res, "Reducing rational value failed!");
    }

    /// Set value of integer to zero
    ///
    /// # Example
    /// ```
    /// use reckoner::Rational;
    ///
    /// let mut r = Rational::from((1234567u128, 1346172348u64));
    ///
    /// r.zero();
    ///
    /// assert_eq!(r, (0, 1));
    /// ```
    pub fn zero(&mut self) {
        let self_raw = self.as_raw();

        // This is safe bc `self` has been initialized.
        unsafe { creachadair_imath_sys::mp_rat_zero(self_raw) };
    }

    /// Returns true if the denominator is 1.
    ///
    /// # Example
    /// ```
    /// use reckoner::Rational;
    ///
    /// assert!(Rational::from((0, 1)).is_integer());
    /// assert!(Rational::from((1, 1)).is_integer());
    /// assert!(!Rational::from((3, 25)).is_integer());
    /// ```
    pub fn is_integer(&self) -> bool {
        let self_raw = self.as_raw();

        // This is safe bc `self` has been initialized.
        unsafe { creachadair_imath_sys::mp_rat_is_integer(self_raw) }
    }

    /// Compare two rationals
    ///
    /// # Example
    /// ```
    /// use core::cmp::Ordering;
    /// use reckoner::Rational;
    ///
    /// let a = Rational::from((123, 500));
    /// let b = Rational::from((377, 500));
    ///
    /// assert_eq!(a.compare(&b), Ordering::Less);
    /// assert_eq!(b.compare(&a), Ordering::Greater);
    /// assert_eq!(a.compare(&a), Ordering::Equal);
    /// ```
    pub fn compare(&self, rhs: &Self) -> Ordering {
        let self_raw = self.as_raw();
        let rhs_raw = rhs.as_raw();

        // This is safe bc both self & rhs have been initialized correctly
        let raw_cmp = unsafe { creachadair_imath_sys::mp_rat_compare(self_raw, rhs_raw) };

        raw_cmp.cmp(&0)
    }

    /// Compare the magnitude of two rationals, not taking sign into account.
    ///
    /// # Example
    /// ```
    /// use core::cmp::Ordering;
    /// use reckoner::Rational;
    ///
    /// let a = Rational::from((123, 500));
    /// let b = Rational::from((-377, 500));
    ///
    /// assert_eq!(a.compare(&b), Ordering::Greater);
    /// assert_eq!(a.compare_magnitude(&b), Ordering::Less);
    /// ```
    pub fn compare_magnitude(&self, rhs: &Self) -> Ordering {
        let self_raw = self.as_raw();
        let rhs_raw = rhs.as_raw();

        // This is safe bc both self & rhs have been initialized correctly
        let raw_cmp = unsafe { creachadair_imath_sys::mp_rat_compare_unsigned(self_raw, rhs_raw) };

        raw_cmp.cmp(&0)
    }

    /// Compare a rational to zero.
    ///
    /// # Example
    /// ```
    /// use core::cmp::Ordering;
    /// use reckoner::Rational;
    ///
    /// let a = Rational::from((123, 500));
    /// let b = Rational::from((-377, 500));
    ///
    /// assert_eq!(a.compare_zero(), Ordering::Greater);
    /// assert_eq!(b.compare_zero(), Ordering::Less);
    /// ```
    pub fn compare_zero(&self) -> Ordering {
        let self_raw = self.as_raw();

        // This is safe bc both self has been initialized correctly
        let raw_cmp = unsafe { creachadair_imath_sys::mp_rat_compare_zero(self_raw) };

        raw_cmp.cmp(&0)
    }

    /// Return a copy of the numerator of the rational value
    ///
    /// # Example
    /// ```
    /// use reckoner::Rational;
    ///
    /// let a = Rational::from((34256, 54587));
    ///
    /// assert_eq!(a.numerator(), 34256);
    /// ```
    pub fn numerator(&self) -> Integer {
        let mut numer = Integer::new();
        let self_raw = self.as_raw();

        {
            let raw_int = Integer::into_raw(numer);

            let res = unsafe { creachadair_imath_sys::mp_rat_numer(self_raw, raw_int) };

            imath_check_panic!(res, "Value init failed!");

            // This is safe because the `raw_int` ptr is created directly from an
            // `Integer::into_raw` call.
            numer = unsafe { Integer::from_raw(raw_int) };
        }

        numer
    }

    /// Return a copy of the denominator of the rational value
    ///
    /// # Example
    /// ```
    /// use reckoner::Rational;
    ///
    /// let a = Rational::from((34256, 54587));
    ///
    /// assert_eq!(a.denominator(), 54587);
    /// ```
    pub fn denominator(&self) -> Integer {
        let mut denom = Integer::new();
        let self_raw = self.as_raw();

        {
            let raw_int = Integer::into_raw(denom);

            // This function call is safe as self_raw and raw_int have been initialized.
            let res = unsafe { creachadair_imath_sys::mp_rat_denom(self_raw, raw_int) };

            imath_check_panic!(res, "Value init failed!");

            // This is safe because the `raw_int` ptr is created directly from an
            // `Integer::into_raw` call.
            denom = unsafe { Integer::from_raw(raw_int) };
        }

        denom
    }

    /// Replaces the value of `other` with a copy of the value of `self`. No new
    /// memory is allocated unless `self` has more significant digits than
    /// `other` has allocated.
    ///
    /// # Example
    /// ```
    /// use reckoner::Rational;
    ///
    /// let a = Rational::from((34256, 54587));
    /// let mut b = Rational::new();
    ///
    /// a.copy_to(&mut b);
    ///
    /// assert_eq!(a, b);
    /// assert_eq!(b, (34256, 54587));
    /// ```
    pub fn copy_to(&self, other: &mut Self) {
        let self_raw = self.as_raw();
        let other_raw = other.as_raw();

        // This is safe bc self has been initialized with a value
        let res = unsafe { creachadair_imath_sys::mp_rat_copy(self_raw, other_raw) };

        imath_check_panic!(res, "Copying the value failed!");
    }

    pub(crate) fn from_string_repr(src: impl ToString) -> Result<Self> {
        let string_repr =
            CString::new(src.to_string()).map_err(|_| Error::StringReprContainedNul)?;
        let char_ptr = string_repr.into_raw();

        let mut init = Rational::uninit();

        {
            let raw_mpq = init.as_mut_ptr();

            // This is safe bc a valid structure is provided to the unsafe methods. And the
            // src value is of the correct type?
            let res_init = unsafe { creachadair_imath_sys::mp_rat_init(raw_mpq) };

            imath_check_panic!(res_init, "Init failed!");

            // This is safe bc all the data provided to the function is correctly setup
            // (rational was allocated/initialized, char_ptr is 0-terminated).
            let res_read = unsafe {
                creachadair_imath_sys::mp_rat_read_ustring(raw_mpq, 10, char_ptr, ptr::null_mut())
            };

            // Accessing this is safe bc the MP_OK value is only ever used as an error
            // condition.
            if res_read != unsafe { creachadair_imath_sys::MP_OK } {
                return Err(Error::ReadStringTruncated);
            }
        }

        // This is safe bc we produced the char_ptr earlier from a CString
        let _ = unsafe { CString::from_raw(char_ptr) };

        Ok(
            // This cast is safe (from MaybeUninit<creachadair_imath_sys::mpq_t> to
            // creachadair_imath_sys::mpq_t) because the value is now initialized.
            unsafe { Rational::from_raw(Box::into_raw(init).cast()) },
        )
    }

    // Reports the minimum number of characters required to represent `self` as a
    // zero-terminated string in base-10.
    pub(crate) fn required_display_len(&self) -> usize {
        let self_raw = self.as_raw();

        // This is safe bc self_raw has been initialized and 10 is within the range
        // `[MP_MIN_RADIX, MP_MAX_RADIX]`
        let len = unsafe { creachadair_imath_sys::mp_rat_string_len(self_raw, 10) };

        // The output of the call is an i32, check that it is gte zero.
        assert!(len >= 0);
        len as usize
    }

    // Reports the minimum number of characters required to represent `z` as a
    // zero-terminated decimal string in base-10.
    pub(crate) fn required_decimal_display_len(&self, max_precision: u16) -> usize {
        let self_raw = self.as_raw();

        // This is safe bc self_raw has been initialized and 10 is within the range
        // `[MP_MIN_RADIX, MP_MAX_RADIX]`
        let len = unsafe {
            creachadair_imath_sys::mp_rat_decimal_len(self_raw, 10, max_precision.into())
        };

        // The output of the call is an i32, check that it is gte zero.
        assert!(len >= 0);
        len as usize
    }

    pub(crate) fn to_cstring(&self) -> CString {
        let required_len = self.required_display_len();
        let self_raw = self.as_raw();

        let mut char_vec: Vec<u8> = Vec::with_capacity(required_len);
        // Initialize all to zero
        char_vec.resize_with(required_len, Default::default);

        let res = {
            let char_ptr = char_vec.as_mut_ptr();
            let cap = char_vec.capacity();

            debug_assert_eq!(required_len, cap);
            unsafe {
                creachadair_imath_sys::mp_rat_to_string(
                    self_raw,
                    10,
                    char_ptr as *mut _,
                    required_len as i32,
                )
            }
        };

        imath_check_panic!(res, "Writing the value as a string failed!");

        // Setting the length is safe bc we now that the `mp_rat_to_string`
        // should have used the entire capacity to write to
        // string.
        unsafe {
            char_vec.set_len(required_len);
        }

        // At this point, char_vec is a zero-terminated (possibly with many zeros)
        // string containing a string representation of the rational value.
        let (non_zero_idx, _) = char_vec
            .iter()
            .enumerate()
            .rfind(|(_, c)| **c != 0)
            .unwrap();
        let without_nul = &char_vec.as_slice()[..=non_zero_idx];

        CString::new(without_nul).expect("Failed to produce a valid CString")
    }

    /// Converts the value of `self` to a string in base-10 decimal-point
    /// notation.  It generates `max_precision` digits of precision and takes a
    /// `RoundMode` argument that determines how the ratio will be converted to
    /// decimal.
    ///
    /// # Example
    /// ```
    /// use reckoner::{Rational, RoundMode};
    ///
    /// let r = Rational::from((1146408, 364913));
    ///
    /// assert_eq!(r.to_decimal(RoundMode::HalfUp, 6), "3.141593");
    /// assert_eq!(r.to_decimal(RoundMode::HalfUp, 5), "3.14159");
    /// assert_eq!(r.to_decimal(RoundMode::HalfUp, 4), "3.1416");
    /// assert_eq!(r.to_decimal(RoundMode::HalfUp, 3), "3.142");
    /// ```
    pub fn to_decimal(&self, rounding_mode: RoundMode, max_precision: u16) -> String {
        let required_len = self.required_decimal_display_len(max_precision);
        let self_raw = self.as_raw();

        let mut char_vec: Vec<u8> = Vec::with_capacity(required_len);
        // Initialize all to zero
        char_vec.resize_with(required_len, Default::default);

        let res = {
            let char_ptr = char_vec.as_mut_ptr();
            let cap = char_vec.capacity();

            debug_assert_eq!(required_len, cap);
            unsafe {
                creachadair_imath_sys::mp_rat_to_decimal(
                    self_raw,
                    10,
                    max_precision.into(),
                    rounding_mode.into(),
                    char_ptr as *mut _,
                    required_len as i32,
                )
            }
        };

        imath_check_panic!(res, "Writing the value as a string failed!");

        // Setting the length is safe bc we now that the `mp_rat_to_decimal`
        // should have used the entire capacity to write to
        // string.
        unsafe {
            char_vec.set_len(required_len);
        }

        // At this point, char_vec is a zero-terminated (possibly with many zeros)
        // string containing a decimal representation of the rational value.
        let (non_zero_idx, _) = char_vec
            .iter()
            .enumerate()
            .rfind(|(_, c)| **c != 0)
            .unwrap();
        let without_nul = &char_vec.as_slice()[..=non_zero_idx];

        CString::new(without_nul)
            .unwrap_or_else(|err| {
                panic!(
                    "Failed to produce a valid CString. {} {:?}\n{}",
                    non_zero_idx, char_vec, err
                )
            })
            .to_string_lossy()
            .into_owned()
    }

    pub(crate) fn set_value(&mut self, numer: impl Into<c_long>, denom: impl Into<c_long>) {
        let self_raw = self.as_raw();

        let res = unsafe {
            creachadair_imath_sys::mp_rat_set_value(self_raw, numer.into(), denom.into())
        };

        imath_check_panic!(res, "Setting the value failed!");
    }

    pub(crate) fn set_numerator(&mut self, numer: &Integer) {
        let self_raw = self.as_raw();
        let int_raw = numer.as_raw();
        // TODO: Use `&raw` when it hits stable
        // This is correct bc the `num` field is the first field in the `mpz_t` struct,
        // which is also `repr(C)`.
        let num_raw = self_raw.cast::<creachadair_imath_sys::mpz_t>();

        let res = unsafe { creachadair_imath_sys::mp_int_copy(int_raw, num_raw) };

        imath_check_panic!(res, "Setting the numerator failed!");
    }

    pub(crate) fn set_denominator(&mut self, denom: &Integer) {
        let self_raw = self.as_raw();
        let denom_raw = denom.as_raw();
        // TODO: Use `&raw` when it hits stable
        // This is safe because the layout of the `mpq_t` struct is repr(C) and just two
        // `mpz_t` in a row. The alignment of `mpz_t` is 8, and the size is 32, so the
        // offset for the `den` field should also be 32, equivalent to one `mpz_t`.
        let den_raw = unsafe { self_raw.cast::<creachadair_imath_sys::mpz_t>().offset(1) };

        // This is safe because both `self` and `denom` have been initialized, and the
        // `den_raw` pointer points correctly to the `den` field of the `mpq_t` struct.
        let res = unsafe { creachadair_imath_sys::mp_int_copy(denom_raw, den_raw) };

        imath_check_panic!(res, "Setting the denominator failed!");
    }

    pub(crate) fn try_into_c_long(&self) -> Result<(c_long, c_long)> {
        let self_raw = self.as_raw();

        let mut numerator: c_long = 0;
        let numerator_raw = (&mut numerator) as *mut _;

        let mut denominator: c_long = 0;
        let denominator_raw = (&mut denominator) as *mut _;

        // This is safe bc `self` has been initialized and
        // `numerator_raw`/`denominator_raw` point to actual integers.
        let res = unsafe {
            creachadair_imath_sys::mp_rat_to_ints(self_raw, numerator_raw, denominator_raw)
        };

        // Accessing this is safe bc the MP_OK value is only ever used as an error
        // condition.
        if res == unsafe { creachadair_imath_sys::MP_OK } {
            Ok((numerator, denominator))
        } else {
            Err(Error::ConversionOutsideRange)
        }
    }
}

/// Parse a rational value from a string having one of the following formats,
/// each with an optional leading sign flag:
///
/// - `n`, integer format, e.g. "123"
/// - `n/d`, ratio format, e.g., "-12/5"
/// - `z.ffff`, decimal format, e.g., "1.627"
///
/// After successfully parsing, the rational will be reduced by factoring out
/// common multiples of the numerator and denominators, as if `Rational::reduce`
/// was called on the value.
impl FromStr for Rational {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Rational::from_string_repr(s)
    }
}

impl fmt::Debug for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            // This is safe bc self has been initialized
            let num = self.numerator();
            let den = self.denominator();

            write!(f, "Rational {{ num: {:#?}, den: {:#?} }}", num, den)
        } else {
            fmt::Display::fmt(&self, f)
        }
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string_repr = self.to_cstring();

        f.write_str(string_repr.to_str().unwrap())
    }
}

impl Default for Rational {
    fn default() -> Self {
        Self::new()
    }
}

// This is safe because the `Rational` has exclusive ownership of its data.
unsafe impl Send for Rational {}

impl Clone for Rational {
    fn clone(&self) -> Self {
        Self::copy_init(self)
    }

    fn clone_from(&mut self, source: &Self) {
        source.copy_to(self);
    }
}

impl Drop for Rational {
    fn drop(&mut self) {
        unsafe {
            let raw = self.as_raw();

            // This will ensure that the memory holding the rational data (the digits?) is
            // not leaked.
            creachadair_imath_sys::mp_rat_clear(raw);

            // This will ensure that the memory that held the `creachadair_imath_sys::mpq_t`
            // is not leaked.
            drop(Box::from_raw(raw));
        }
    }
}

// Ratios usually must be rounded when they are being converted for output
/// as a decimal value.  There are four rounding modes currently
/// supported.
pub enum RoundMode {
    /// Truncates the value toward zero.
    ///
    /// For example, 12.009 to 2 decimal places becomes 12.00.
    Down,
    /// Rounds the value away from zero.
    ///
    /// For example, 12.001 to 2 decimal places becomes 12.01, but 12.000 to 2
    /// decimal places remains 12.00.
    Up,
    /// Rounds the value to nearest digit, half rounds upward.
    ///
    /// For example, 12.005 to 2 decimal places becomes 12.01, but 12.004 to 2
    /// decimal places becomes 12.00.
    HalfUp,
    /// Rounds the value to nearest digit, half goes toward zero.
    ///
    /// For example, 12.005 to 2 decimal places becomes 12.00, but 12.006 to 2
    /// decimal places becomes 12.01.
    HalfDown,
}

impl From<RoundMode> for creachadair_imath_sys::mp_round_mode {
    fn from(src: RoundMode) -> creachadair_imath_sys::mp_round_mode {
        match src {
            RoundMode::Down => creachadair_imath_sys::mp_round_mode_MP_ROUND_DOWN,
            RoundMode::Up => creachadair_imath_sys::mp_round_mode_MP_ROUND_UP,
            RoundMode::HalfDown => creachadair_imath_sys::mp_round_mode_MP_ROUND_HALF_DOWN,
            RoundMode::HalfUp => creachadair_imath_sys::mp_round_mode_MP_ROUND_HALF_UP,
        }
    }
}

impl TryFrom<creachadair_imath_sys::mp_round_mode> for RoundMode {
    type Error = Error;

    fn try_from(src: creachadair_imath_sys::mp_round_mode) -> Result<Self> {
        use RoundMode::*;
        match src {
            creachadair_imath_sys::mp_round_mode_MP_ROUND_DOWN => Ok(Down),
            creachadair_imath_sys::mp_round_mode_MP_ROUND_UP => Ok(Up),
            creachadair_imath_sys::mp_round_mode_MP_ROUND_HALF_DOWN => Ok(HalfDown),
            creachadair_imath_sys::mp_round_mode_MP_ROUND_HALF_UP => Ok(HalfUp),
            _ => Err(Error::UnknownRoundingMode),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_default_rational() {
        let rat = Rational::new();

        assert_eq!(rat.numerator(), Integer::from(0));
        assert_eq!(rat.denominator(), Integer::from(1));
    }

    #[test]
    fn parse_normal_rational() {
        let rat: Rational = "12345/67890".parse().unwrap();

        assert_eq!(rat.numerator(), Integer::from(823));
        assert_eq!(rat.denominator(), Integer::from(4526));
    }

    #[test]
    fn parse_integer_rational() {
        let rat: Rational = "12345".parse().unwrap();

        assert_eq!(rat.numerator(), Integer::from(12345));
        assert_eq!(rat.denominator(), Integer::from(1));
    }

    #[test]
    fn parse_decimal_rational() {
        let rat: Rational = "1.2345".parse().unwrap();

        assert_eq!(rat.numerator(), Integer::from(2469));
        assert_eq!(rat.denominator(), Integer::from(2000));
    }

    #[test]
    fn rational_to_decimal() {
        let mut rat = Rational::new();
        rat.set_value(2, 3);

        assert_eq!(rat.to_decimal(RoundMode::Down, 3), "0.666");
        assert_eq!(rat.to_decimal(RoundMode::Down, 15), "0.666666666666666");

        rat.set_value(2, 1);

        assert_eq!(rat.to_decimal(RoundMode::Down, 3), "2.000");
        assert_eq!(rat.to_decimal(RoundMode::Down, 15), "2.000000000000000");

        rat.zero();

        assert_eq!(rat.to_decimal(RoundMode::Down, 3), "0.000");
        assert_eq!(rat.to_decimal(RoundMode::Down, 15), "0.000000000000000");

        rat.set_value(9, 1000);
        assert_eq!(rat.to_decimal(RoundMode::Down, 2), "0.00");

        rat.set_value(1, 1000);
        assert_eq!(rat.to_decimal(RoundMode::Up, 2), "0.01");

        rat.set_value(5, 1000);
        assert_eq!(rat.to_decimal(RoundMode::HalfUp, 2), "0.01");
        rat.set_value(4, 1000);
        assert_eq!(rat.to_decimal(RoundMode::HalfUp, 2), "0.00");

        rat.set_value(5, 1000);
        assert_eq!(rat.to_decimal(RoundMode::HalfDown, 2), "0.00");
        rat.set_value(6, 1000);
        assert_eq!(rat.to_decimal(RoundMode::HalfDown, 2), "0.01");
    }

    #[test]
    fn reduce_rational() {
        let mut rat = Rational::new();

        rat.set_value(10, 20);
        rat.reduce();
        assert_eq!(rat.numerator(), Integer::from(1));
        assert_eq!(rat.denominator(), Integer::from(2));

        rat.set_value(0, 1234);
        rat.reduce();
        assert_eq!(rat.numerator(), Integer::from(0));
        assert_eq!(rat.denominator(), Integer::from(1));

        rat.set_value(13412, 13412);
        rat.reduce();
        assert_eq!(rat.numerator(), Integer::from(1));
        assert_eq!(rat.denominator(), Integer::from(1));
    }

    #[test]
    fn clone_rational() {
        let a: Rational =
            "98712698346126837461287318238761234897612839471623487619872364981726348176234"
                .parse()
                .unwrap();

        let b = a.clone();

        assert_eq!(a.numerator(), b.numerator());
        assert_eq!(a.denominator(), b.denominator());
    }

    #[test]
    fn send_not_sync_rational() {
        static_assertions::assert_impl_all!(Rational: Send);
        static_assertions::assert_not_impl_any!(Rational: Sync);
    }
}
