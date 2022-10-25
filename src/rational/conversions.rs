#![allow(clippy::useless_conversion)]

use crate::{error::Error, integer::Integer, rational::Rational};
use core::convert::{TryFrom, TryInto};

impl From<Integer> for Rational {
    fn from(src: Integer) -> Self {
        let mut rat = Rational::new();

        rat.set_numerator(&src);

        rat
    }
}

impl From<&Integer> for Rational {
    fn from(src: &Integer) -> Self {
        let mut rat = Rational::new();

        rat.set_numerator(src);

        rat
    }
}

impl<A, B> From<(A, B)> for Rational
where
    A: Into<Integer>,
    B: Into<Integer>,
{
    fn from((numer, denom): (A, B)) -> Self {
        let mut rat = Rational::new();

        rat.set_numerator(&numer.into());
        rat.set_denominator(&denom.into());

        rat
    }
}

impl<'a, A, B> From<&'a (A, B)> for Rational
where
    &'a A: Into<Integer>,
    &'a B: Into<Integer>,
{
    fn from((numer, denom): &'a (A, B)) -> Self {
        let mut rat = Rational::new();

        rat.set_numerator(&numer.into());
        rat.set_denominator(&denom.into());

        rat
    }
}

impl From<&(Integer, Integer)> for Rational {
    fn from((numer, denom): &(Integer, Integer)) -> Self {
        let mut rat = Rational::new();

        rat.set_numerator(numer);
        rat.set_denominator(denom);

        rat
    }
}

impl TryFrom<Rational> for Integer {
    type Error = Error;

    fn try_from(src: Rational) -> Result<Self, Self::Error> {
        if src.is_integer() {
            Ok(src.numerator())
        } else {
            Err(Error::NotCanonicalInteger)
        }
    }
}

impl TryFrom<&Rational> for Integer {
    type Error = Error;

    fn try_from(src: &Rational) -> Result<Self, Self::Error> {
        if src.is_integer() {
            Ok(src.numerator())
        } else {
            Err(Error::NotCanonicalInteger)
        }
    }
}

impl From<i8> for Rational {
    fn from(src: i8) -> Self {
        let mut rat = Rational::new();

        rat.set_value(src, 1);

        rat
    }
}

impl From<&i8> for Rational {
    fn from(src: &i8) -> Self {
        let mut rat = Rational::new();

        rat.set_value(*src, 1);

        rat
    }
}

impl TryFrom<Rational> for (i8, i8) {
    type Error = Error;

    fn try_from(src: Rational) -> Result<Self, Self::Error> {
        src.try_into_c_long()
            .and_then(|(num, denom)| Ok((num.try_into()?, denom.try_into()?)))
    }
}

impl TryFrom<&Rational> for (i8, i8) {
    type Error = Error;

    fn try_from(src: &Rational) -> Result<Self, Self::Error> {
        src.try_into_c_long()
            .and_then(|(num, denom)| Ok((num.try_into()?, denom.try_into()?)))
    }
}

impl From<u8> for Rational {
    fn from(src: u8) -> Self {
        let mut rat = Rational::new();

        rat.set_value(src, 1);

        rat
    }
}

impl From<&u8> for Rational {
    fn from(src: &u8) -> Self {
        let mut rat = Rational::new();

        rat.set_value(*src, 1);

        rat
    }
}

impl TryFrom<Rational> for (u8, u8) {
    type Error = Error;

    fn try_from(src: Rational) -> Result<Self, Self::Error> {
        src.try_into_c_long()
            .and_then(|(num, denom)| Ok((num.try_into()?, denom.try_into()?)))
    }
}

impl TryFrom<&Rational> for (u8, u8) {
    type Error = Error;

    fn try_from(src: &Rational) -> Result<Self, Self::Error> {
        src.try_into_c_long()
            .and_then(|(num, denom)| Ok((num.try_into()?, denom.try_into()?)))
    }
}

impl From<i16> for Rational {
    fn from(src: i16) -> Self {
        let mut rat = Rational::new();

        rat.set_value(src, 1);

        rat
    }
}

impl From<&i16> for Rational {
    fn from(src: &i16) -> Self {
        let mut rat = Rational::new();

        rat.set_value(*src, 1);

        rat
    }
}

impl TryFrom<Rational> for (i16, i16) {
    type Error = Error;

    fn try_from(src: Rational) -> Result<Self, Self::Error> {
        src.try_into_c_long()
            .and_then(|(num, denom)| Ok((num.try_into()?, denom.try_into()?)))
    }
}

impl TryFrom<&Rational> for (i16, i16) {
    type Error = Error;

    fn try_from(src: &Rational) -> Result<Self, Self::Error> {
        src.try_into_c_long()
            .and_then(|(num, denom)| Ok((num.try_into()?, denom.try_into()?)))
    }
}

impl From<u16> for Rational {
    fn from(src: u16) -> Self {
        let mut rat = Rational::new();

        rat.set_value(src, 1);

        rat
    }
}

impl From<&u16> for Rational {
    fn from(src: &u16) -> Self {
        let mut rat = Rational::new();

        rat.set_value(*src, 1);

        rat
    }
}

impl TryFrom<Rational> for (u16, u16) {
    type Error = Error;

    fn try_from(src: Rational) -> Result<Self, Self::Error> {
        src.try_into_c_long()
            .and_then(|(num, denom)| Ok((num.try_into()?, denom.try_into()?)))
    }
}

impl TryFrom<&Rational> for (u16, u16) {
    type Error = Error;

    fn try_from(src: &Rational) -> Result<Self, Self::Error> {
        src.try_into_c_long()
            .and_then(|(num, denom)| Ok((num.try_into()?, denom.try_into()?)))
    }
}

impl From<i32> for Rational {
    fn from(src: i32) -> Self {
        let mut rat = Rational::new();

        rat.set_value(src, 1);

        rat
    }
}

impl From<&i32> for Rational {
    fn from(src: &i32) -> Self {
        let mut rat = Rational::new();

        rat.set_value(*src, 1);

        rat
    }
}

impl TryFrom<Rational> for (i32, i32) {
    type Error = Error;

    fn try_from(src: Rational) -> Result<Self, Self::Error> {
        src.try_into_c_long()
            .and_then(|(num, denom)| Ok((num.try_into()?, denom.try_into()?)))
    }
}

impl TryFrom<&Rational> for (i32, i32) {
    type Error = Error;

    fn try_from(src: &Rational) -> Result<Self, Self::Error> {
        src.try_into_c_long()
            .and_then(|(num, denom)| Ok((num.try_into()?, denom.try_into()?)))
    }
}

cfg_if::cfg_if! {
    if #[cfg(all(target_pointer_width = "64", not(windows)))] {
        impl From<u32> for Rational {
            fn from(src: u32) -> Self {
                let mut rat = Rational::new();
                rat.set_value(src, 1);
                rat
            }
        }

        impl From<&u32> for Rational {
            fn from(src: &u32) -> Self {
                let mut rat = Rational::new();
                rat.set_value(*src, 1);
                rat
            }
        }

        impl TryFrom<Rational> for (u32, u32) {
            type Error = Error;

            fn try_from(src: Rational) -> Result<Self, Self::Error> {
                src.try_into_c_long()
                    .and_then(|(num, denom)| Ok((num.try_into()?, denom.try_into()?)))
            }
        }

        impl TryFrom<&Rational> for (u32, u32) {
            type Error = Error;

            fn try_from(src: &Rational) -> Result<Self, Self::Error> {
                src.try_into_c_long()
                    .and_then(|(num, denom)| Ok((num.try_into()?, denom.try_into()?)))
            }
        }

        impl From<i64> for Rational {
            fn from(src: i64) -> Self {
                let mut rat = Rational::new();
                rat.set_value(src, 1);
                rat
            }
        }

        impl From<&i64> for Rational {
            fn from(src: &i64) -> Self {
                let mut rat = Rational::new();
                rat.set_value(*src, 1);
                rat
            }
        }

        impl TryFrom<Rational> for (i64, i64) {
            type Error = Error;

            fn try_from(src: Rational) -> Result<Self, Self::Error> {
                src.try_into_c_long()
                    .and_then(|(num, denom)| Ok((num.try_into()?, denom.try_into()?)))
            }
        }

        impl TryFrom<&Rational> for (i64, i64) {
            type Error = Error;

            fn try_from(src: &Rational) -> Result<Self, Self::Error> {
                src.try_into_c_long()
                    .and_then(|(num, denom)| Ok((num.try_into()?, denom.try_into()?)))
            }
        }
    } else {
        impl From<u32> for Rational {
            fn from(src: u32) -> Self {
                Self::from_string_repr(src).expect("Conversion from string failed")
            }
        }

        impl From<&u32> for Rational {
            fn from(src: &u32) -> Self {
                Self::from_string_repr(src).expect("Conversion from string failed")
            }
        }

        impl TryFrom<Rational> for (u32, u32) {
            type Error = Error;

            fn try_from(src: Rational) -> Result<Self, Self::Error> {
                let num = src.numerator().to_string().parse().map_err(|_| Error::ConversionOutsideRange)?;
                let den = src.denominator().to_string().parse().map_err(|_| Error::ConversionOutsideRange)?;

                Ok((num, den))
            }
        }

        impl TryFrom<&Rational> for (u32, u32) {
            type Error = Error;

            fn try_from(src: &Rational) -> Result<Self, Self::Error> {
                let num = src.numerator().to_string().parse().map_err(|_| Error::ConversionOutsideRange)?;
                let den = src.denominator().to_string().parse().map_err(|_| Error::ConversionOutsideRange)?;

                Ok((num, den))
            }
        }

        impl From<i64> for Rational {
            fn from(src: i64) -> Self {
                Self::from_string_repr(src).expect("Conversion from string failed")
            }
        }

        impl From<&i64> for Rational {
            fn from(src: &i64) -> Self {
                Self::from_string_repr(src).expect("Conversion from string failed")
            }
        }

        impl TryFrom<Rational> for (i64, i64) {
            type Error = Error;

            fn try_from(src: Rational) -> Result<Self, Self::Error> {
                let num = src.numerator().to_string().parse().map_err(|_| Error::ConversionOutsideRange)?;
                let den = src.denominator().to_string().parse().map_err(|_| Error::ConversionOutsideRange)?;

                Ok((num, den))
            }
        }

        impl TryFrom<&Rational> for (i64, i64) {
            type Error = Error;

            fn try_from(src: &Rational) -> Result<Self, Self::Error> {
                let num = src.numerator().to_string().parse().map_err(|_| Error::ConversionOutsideRange)?;
                let den = src.denominator().to_string().parse().map_err(|_| Error::ConversionOutsideRange)?;

                Ok((num, den))
            }
        }
    }
}

impl From<u64> for Rational {
    fn from(src: u64) -> Self {
        Self::from_string_repr(src).expect("Conversion from string failed")
    }
}

impl From<&u64> for Rational {
    fn from(src: &u64) -> Self {
        Self::from_string_repr(src).expect("Conversion from string failed")
    }
}

impl TryFrom<Rational> for (u64, u64) {
    type Error = Error;

    fn try_from(src: Rational) -> Result<Self, Self::Error> {
        let num = src
            .numerator()
            .to_string()
            .parse()
            .map_err(|_| Error::ConversionOutsideRange)?;
        let den = src
            .denominator()
            .to_string()
            .parse()
            .map_err(|_| Error::ConversionOutsideRange)?;

        Ok((num, den))
    }
}

impl TryFrom<&Rational> for (u64, u64) {
    type Error = Error;

    fn try_from(src: &Rational) -> Result<Self, Self::Error> {
        let num = src
            .numerator()
            .to_string()
            .parse()
            .map_err(|_| Error::ConversionOutsideRange)?;
        let den = src
            .denominator()
            .to_string()
            .parse()
            .map_err(|_| Error::ConversionOutsideRange)?;

        Ok((num, den))
    }
}

impl From<i128> for Rational {
    fn from(src: i128) -> Self {
        Self::from_string_repr(src).expect("Conversion from string failed")
    }
}

impl From<&i128> for Rational {
    fn from(src: &i128) -> Self {
        Self::from_string_repr(src).expect("Conversion from string failed")
    }
}

impl TryFrom<Rational> for (i128, i128) {
    type Error = Error;

    fn try_from(src: Rational) -> Result<Self, Self::Error> {
        let num = src
            .numerator()
            .to_string()
            .parse()
            .map_err(|_| Error::ConversionOutsideRange)?;
        let den = src
            .denominator()
            .to_string()
            .parse()
            .map_err(|_| Error::ConversionOutsideRange)?;

        Ok((num, den))
    }
}

impl TryFrom<&Rational> for (i128, i128) {
    type Error = Error;

    fn try_from(src: &Rational) -> Result<Self, Self::Error> {
        let num = src
            .numerator()
            .to_string()
            .parse()
            .map_err(|_| Error::ConversionOutsideRange)?;
        let den = src
            .denominator()
            .to_string()
            .parse()
            .map_err(|_| Error::ConversionOutsideRange)?;

        Ok((num, den))
    }
}

impl From<u128> for Rational {
    fn from(src: u128) -> Self {
        Self::from_string_repr(src).expect("Conversion from string failed")
    }
}

impl From<&u128> for Rational {
    fn from(src: &u128) -> Self {
        Self::from_string_repr(src).expect("Conversion from string failed")
    }
}

impl TryFrom<Rational> for (u128, u128) {
    type Error = Error;

    fn try_from(src: Rational) -> Result<Self, Self::Error> {
        let num = src
            .numerator()
            .to_string()
            .parse()
            .map_err(|_| Error::ConversionOutsideRange)?;
        let den = src
            .denominator()
            .to_string()
            .parse()
            .map_err(|_| Error::ConversionOutsideRange)?;

        Ok((num, den))
    }
}

impl TryFrom<&Rational> for (u128, u128) {
    type Error = Error;

    fn try_from(src: &Rational) -> Result<Self, Self::Error> {
        let num = src
            .numerator()
            .to_string()
            .parse()
            .map_err(|_| Error::ConversionOutsideRange)?;
        let den = src
            .denominator()
            .to_string()
            .parse()
            .map_err(|_| Error::ConversionOutsideRange)?;

        Ok((num, den))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use core::{convert::TryFrom, str::FromStr};

    #[test]
    fn conversion_to_primitive() {
        let valid_i8 = Rational::from_str("56").unwrap();
        let invalid_i8 = Rational::from_str("129").unwrap();

        assert_eq!(TryFrom::try_from(valid_i8), Ok((56i8, 1)));
        assert_eq!(
            TryFrom::try_from(invalid_i8),
            Err(Error::ConversionOutsideRange) as Result<(i8, i8), _>
        );

        let valid_i32 = Rational::from_str("-2147483648").unwrap();
        let invalid_i32 = Rational::from_str("-2147483649").unwrap();

        assert_eq!(TryFrom::try_from(valid_i32), Ok((-2_147_483_648i32, 1)));
        assert_eq!(
            TryFrom::try_from(invalid_i32),
            Err(Error::ConversionOutsideRange) as Result<(i32, i32), _>
        );

        let valid_i128 = Rational::from_str("170141183460469231731687303715884105727").unwrap();
        let invalid_i128 = Rational::from_str("170141183460469231731687303715884105728").unwrap();

        assert_eq!(
            TryFrom::try_from(valid_i128),
            Ok((170_141_183_460_469_231_731_687_303_715_884_105_727i128, 1))
        );
        assert_eq!(
            TryFrom::try_from(invalid_i128),
            Err(Error::ConversionOutsideRange) as Result<(i128, i128), _>
        );
    }
}
