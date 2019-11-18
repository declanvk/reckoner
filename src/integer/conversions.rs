use crate::{error::Error, integer::Integer};
use core::convert::{TryFrom, TryInto};

impl From<i8> for Integer {
    fn from(src: i8) -> Self {
        Self::from_c_long(src)
    }
}

impl From<&i8> for Integer {
    fn from(src: &i8) -> Self {
        Self::from_c_long(*src)
    }
}

impl TryFrom<Integer> for i8 {
    type Error = Error;

    fn try_from(src: Integer) -> Result<Self, Self::Error> {
        src.try_into_c_long()
            .and_then(|value| value.try_into().map_err(From::from))
    }
}

impl TryFrom<&Integer> for i8 {
    type Error = Error;

    fn try_from(src: &Integer) -> Result<Self, Self::Error> {
        src.try_into_c_long()
            .and_then(|value| value.try_into().map_err(From::from))
    }
}

impl From<u8> for Integer {
    fn from(src: u8) -> Self {
        Self::from_c_long(src)
    }
}

impl From<&u8> for Integer {
    fn from(src: &u8) -> Self {
        Self::from_c_long(*src)
    }
}

impl TryFrom<Integer> for u8 {
    type Error = Error;

    fn try_from(src: Integer) -> Result<Self, Self::Error> {
        src.try_into_c_long()
            .and_then(|value| value.try_into().map_err(From::from))
    }
}

impl TryFrom<&Integer> for u8 {
    type Error = Error;

    fn try_from(src: &Integer) -> Result<Self, Self::Error> {
        src.try_into_c_long()
            .and_then(|value| value.try_into().map_err(From::from))
    }
}

impl From<i16> for Integer {
    fn from(src: i16) -> Self {
        Self::from_c_long(src)
    }
}

impl From<&i16> for Integer {
    fn from(src: &i16) -> Self {
        Self::from_c_long(*src)
    }
}

impl TryFrom<Integer> for i16 {
    type Error = Error;

    fn try_from(src: Integer) -> Result<Self, Self::Error> {
        src.try_into_c_long()
            .and_then(|value| value.try_into().map_err(From::from))
    }
}

impl TryFrom<&Integer> for i16 {
    type Error = Error;

    fn try_from(src: &Integer) -> Result<Self, Self::Error> {
        src.try_into_c_long()
            .and_then(|value| value.try_into().map_err(From::from))
    }
}

impl From<u16> for Integer {
    fn from(src: u16) -> Self {
        Self::from_c_long(src)
    }
}

impl From<&u16> for Integer {
    fn from(src: &u16) -> Self {
        Self::from_c_long(*src)
    }
}

impl TryFrom<Integer> for u16 {
    type Error = Error;

    fn try_from(src: Integer) -> Result<Self, Self::Error> {
        src.try_into_c_long()
            .and_then(|value| value.try_into().map_err(From::from))
    }
}

impl TryFrom<&Integer> for u16 {
    type Error = Error;

    fn try_from(src: &Integer) -> Result<Self, Self::Error> {
        src.try_into_c_long()
            .and_then(|value| value.try_into().map_err(From::from))
    }
}

impl From<i32> for Integer {
    fn from(src: i32) -> Self {
        Self::from_c_long(src)
    }
}

impl From<&i32> for Integer {
    fn from(src: &i32) -> Self {
        Self::from_c_long(*src)
    }
}

impl TryFrom<Integer> for i32 {
    type Error = Error;

    fn try_from(src: Integer) -> Result<Self, Self::Error> {
        src.try_into_c_long()
            .and_then(|value| value.try_into().map_err(From::from))
    }
}

impl TryFrom<&Integer> for i32 {
    type Error = Error;

    fn try_from(src: &Integer) -> Result<Self, Self::Error> {
        src.try_into_c_long()
            .and_then(|value| value.try_into().map_err(From::from))
    }
}

cfg_if::cfg_if! {
    if #[cfg(all(target_pointer_width = "64", not(windows)))] {
        impl From<u32> for Integer {
            fn from(src: u32) -> Self {
                Self::from_c_long(src)
            }
        }

        impl From<&u32> for Integer {
            fn from(src: &u32) -> Self {
                Self::from_c_long(*src)
            }
        }

        impl TryFrom<Integer> for u32 {
            type Error = Error;
            fn try_from(src: Integer) -> Result<Self, Self::Error> {
                src.try_into_c_long()
                    .and_then(|value| value.try_into().map_err(From::from))
            }
        }

        impl TryFrom<&Integer> for u32 {
            type Error = Error;
            fn try_from(src: &Integer) -> Result<Self, Self::Error> {
                src.try_into_c_long()
                    .and_then(|value| value.try_into().map_err(From::from))
            }
        }

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

        impl TryFrom<Integer> for i64 {
            type Error = Error;
            fn try_from(src: Integer) -> Result<Self, Self::Error> {
                src.try_into_c_long()
                    .and_then(|value| value.try_into().map_err(From::from))
            }
        }

        impl TryFrom<&Integer> for i64 {
            type Error = Error;
            fn try_from(src: &Integer) -> Result<Self, Self::Error> {
                src.try_into_c_long()
                    .and_then(|value| value.try_into().map_err(From::from))
            }
        }
    } else {
        impl From<u32> for Integer {
            fn from(src: u32) -> Self {
                Self::from_string_repr(src).expect("Conversion from string failed")
            }
        }

        impl From<&u32> for Integer {
            fn from(src: &u32) -> Self {
                Self::from_string_repr(src).expect("Conversion from string failed")
            }
        }

        impl TryFrom<Integer> for u32 {
            type Error = Error;
            fn try_from(src: Integer) -> Result<Self, Self::Error> {
                src.to_string().parse().map_err(|_| Error::ConversionOutsideRange)
            }
        }
        impl TryFrom<&Integer> for u32 {
            type Error = Error;
            fn try_from(src: &Integer) -> Result<Self, Self::Error> {
                src.to_string().parse().map_err(|_| Error::ConversionOutsideRange)
            }
        }


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

        impl TryFrom<Integer> for i64 {
            type Error = Error;
            fn try_from(src: Integer) -> Result<Self, Self::Error> {
                src.to_string().parse().map_err(|_| Error::ConversionOutsideRange)
            }
        }
        impl TryFrom<&Integer> for i64 {
            type Error = Error;
            fn try_from(src: &Integer) -> Result<Self, Self::Error> {
                src.to_string().parse().map_err(|_| Error::ConversionOutsideRange)
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

impl TryFrom<Integer> for u64 {
    type Error = Error;

    fn try_from(src: Integer) -> Result<Self, Self::Error> {
        src.to_string()
            .parse()
            .map_err(|_| Error::ConversionOutsideRange)
    }
}

impl TryFrom<&Integer> for u64 {
    type Error = Error;

    fn try_from(src: &Integer) -> Result<Self, Self::Error> {
        src.to_string()
            .parse()
            .map_err(|_| Error::ConversionOutsideRange)
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

impl TryFrom<Integer> for i128 {
    type Error = Error;

    fn try_from(src: Integer) -> Result<Self, Self::Error> {
        src.to_string()
            .parse()
            .map_err(|_| Error::ConversionOutsideRange)
    }
}

impl TryFrom<&Integer> for i128 {
    type Error = Error;

    fn try_from(src: &Integer) -> Result<Self, Self::Error> {
        src.to_string()
            .parse()
            .map_err(|_| Error::ConversionOutsideRange)
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

impl TryFrom<Integer> for u128 {
    type Error = Error;

    fn try_from(src: Integer) -> Result<Self, Self::Error> {
        src.to_string()
            .parse()
            .map_err(|_| Error::ConversionOutsideRange)
    }
}

impl TryFrom<&Integer> for u128 {
    type Error = Error;

    fn try_from(src: &Integer) -> Result<Self, Self::Error> {
        src.to_string()
            .parse()
            .map_err(|_| Error::ConversionOutsideRange)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use core::{convert::TryFrom, str::FromStr};

    #[test]
    fn conversion_to_primitive() {
        let valid_i8 = Integer::from_str("56").unwrap();
        let invalid_i8 = Integer::from_str("129").unwrap();

        assert_eq!(TryFrom::try_from(valid_i8), Ok(56i8));
        assert_eq!(
            TryFrom::try_from(invalid_i8),
            Err(Error::ConversionOutsideRange) as Result<i8, _>
        );

        let valid_i32 = Integer::from_str("-2147483648").unwrap();
        let invalid_i32 = Integer::from_str("-2147483649").unwrap();

        assert_eq!(TryFrom::try_from(valid_i32), Ok(-2_147_483_648i32));
        assert_eq!(
            TryFrom::try_from(invalid_i32),
            Err(Error::ConversionOutsideRange) as Result<i32, _>
        );

        let valid_i128 = Integer::from_str("170141183460469231731687303715884105727").unwrap();
        let invalid_i128 = Integer::from_str("170141183460469231731687303715884105728").unwrap();

        assert_eq!(
            TryFrom::try_from(valid_i128),
            Ok(170_141_183_460_469_231_731_687_303_715_884_105_727i128)
        );
        assert_eq!(
            TryFrom::try_from(invalid_i128),
            Err(Error::ConversionOutsideRange) as Result<i128, _>
        );
    }
}
