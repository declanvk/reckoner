use core::mem::MaybeUninit;

#[test]
fn add_two_numbers() {
    let mut a: MaybeUninit<imath_sys::mpz_t> = MaybeUninit::uninit();
    let mut b: MaybeUninit<imath_sys::mpz_t> = MaybeUninit::uninit();
    let mut c: MaybeUninit<imath_sys::mpz_t> = MaybeUninit::uninit();

    let mut expected: MaybeUninit<imath_sys::mpz_t> = MaybeUninit::uninit();

    unsafe {
        imath_sys::mp_int_init_value(a.as_mut_ptr(), 1024);
        imath_sys::mp_int_init_value(b.as_mut_ptr(), 4201);
        imath_sys::mp_int_init_value(c.as_mut_ptr(), 0);

        imath_sys::mp_int_init_value(expected.as_mut_ptr(), 5225);
    }

    let res = unsafe { imath_sys::mp_int_add(a.as_mut_ptr(), b.as_mut_ptr(), c.as_mut_ptr()) };

    if res != unsafe { imath_sys::MP_OK } {
        panic!();
    }

    let cmp = unsafe { imath_sys::mp_int_compare(c.as_mut_ptr(), expected.as_mut_ptr()) };

    assert_eq!(cmp, 0);

    unsafe {
        imath_sys::mp_int_clear(a.as_mut_ptr());
        imath_sys::mp_int_clear(b.as_mut_ptr());
        imath_sys::mp_int_clear(c.as_mut_ptr());
        imath_sys::mp_int_clear(expected.as_mut_ptr());
    }
}
