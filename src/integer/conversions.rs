use crate::integer::Integer;

impl From<i8> for Integer {
    fn from(src: i8) -> Self {
        Self::from_c_long(src as _)
    }
}

impl From<&i8> for Integer {
    fn from(src: &i8) -> Self {
        Self::from_c_long(*src as _)
    }
}

impl From<u8> for Integer {
    fn from(src: u8) -> Self {
        Self::from_c_long(src as _)
    }
}

impl From<&u8> for Integer {
    fn from(src: &u8) -> Self {
        Self::from_c_long(*src as _)
    }
}

impl From<i16> for Integer {
    fn from(src: i16) -> Self {
        Self::from_c_long(src as _)
    }
}

impl From<&i16> for Integer {
    fn from(src: &i16) -> Self {
        Self::from_c_long(*src as _)
    }
}

impl From<u16> for Integer {
    fn from(src: u16) -> Self {
        Self::from_c_long(src as _)
    }
}

impl From<&u16> for Integer {
    fn from(src: &u16) -> Self {
        Self::from_c_long(*src as _)
    }
}

impl From<i32> for Integer {
    fn from(src: i32) -> Self {
        Self::from_c_long(src as _)
    }
}

impl From<&i32> for Integer {
    fn from(src: &i32) -> Self {
        Self::from_c_long(*src as _)
    }
}

impl From<u32> for Integer {
    fn from(src: u32) -> Self {
        Self::from_c_long(src as _)
    }
}

impl From<&u32> for Integer {
    fn from(src: &u32) -> Self {
        Self::from_c_long(*src as _)
    }
}

cfg_if::cfg_if! {
    if #[cfg(all(target_pointer_width = "64", not(windows)))] {
        impl From<i64> for Integer {
            fn from(src: i64) -> Self {
                Self::from_c_long(src)
            }
        }

        impl From<&i64> for Integer {
            fn from(src: &i64) -> Self {
                Self::from_c_long(*src)
            }
        }
    } else {
        impl From<i64> for Integer {
            fn from(src: i64) -> Self {
                Self::from_string_repr(src).expect("Conversion from string failed")
            }
        }

        impl From<&i64> for Integer {
            fn from(src: &i64) -> Self {
                Self::from_string_repr(*src).expect("Conversion from string failed")
            }
        }
    }
}

impl From<u64> for Integer {
    fn from(src: u64) -> Self {
        Self::from_string_repr(src).expect("Conversion from string failed")
    }
}

impl From<&u64> for Integer {
    fn from(src: &u64) -> Self {
        Self::from_string_repr(*src).expect("Conversion from string failed")
    }
}

impl From<i128> for Integer {
    fn from(src: i128) -> Self {
        Self::from_string_repr(src).expect("Conversion from string failed")
    }
}

impl From<&i128> for Integer {
    fn from(src: &i128) -> Self {
        Self::from_string_repr(*src).expect("Conversion from string failed")
    }
}

impl From<u128> for Integer {
    fn from(src: u128) -> Self {
        Self::from_string_repr(src).expect("Conversion from string failed")
    }
}

impl From<&u128> for Integer {
    fn from(src: &u128) -> Self {
        Self::from_string_repr(*src).expect("Conversion from string failed")
    }
}
