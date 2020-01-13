use crate::{integer::Integer, rational::Rational};
use core::cmp::Ordering;

macro_rules! impl_partial_eq {
    ($rhs:ty, $func:path) => {
        impl PartialEq<$rhs> for Rational {
            fn eq(&self, other: &$rhs) -> bool {
                $func(&self, other) == Ordering::Equal
            }
        }
    };
    ($rhs:ty, $func:path, into rhs) => {
        impl PartialEq<$rhs> for Rational {
            fn eq(&self, other: &$rhs) -> bool {
                $func(&self, &Rational::from(other)) == Ordering::Equal
            }
        }

        impl PartialEq<Rational> for $rhs {
            fn eq(&self, other: &Rational) -> bool {
                // This doesn't need to be reversed as it simply checks equality
                $func(&other, &Rational::from(self)) == Ordering::Equal
            }
        }

        impl PartialEq<($rhs, $rhs)> for Rational {
            fn eq(&self, other: &($rhs, $rhs)) -> bool {
                $func(&self, &Rational::from(other)) == Ordering::Equal
            }
        }

        impl PartialEq<Rational> for ($rhs, $rhs) {
            fn eq(&self, other: &Rational) -> bool {
                // This doesn't need to be reversed as it simply checks equality
                $func(&other, &Rational::from(self)) == Ordering::Equal
            }
        }
    };
}

impl_partial_eq!(Rational, Rational::compare);
impl_partial_eq!(Integer, Rational::compare, into rhs);
impl_partial_eq!(u8, Rational::compare, into rhs);
impl_partial_eq!(i8, Rational::compare, into rhs);
impl_partial_eq!(u16, Rational::compare, into rhs);
impl_partial_eq!(i16, Rational::compare, into rhs);
impl_partial_eq!(i32, Rational::compare, into rhs);
impl_partial_eq!(u32, Rational::compare, into rhs);
impl_partial_eq!(i64, Rational::compare, into rhs);
impl_partial_eq!(u64, Rational::compare, into rhs);
impl_partial_eq!(i128, Rational::compare, into rhs);
impl_partial_eq!(u128, Rational::compare, into rhs);

impl Eq for Rational {}

macro_rules! impl_partial_ord {
    ($rhs:ty, $func:path) => {
        impl PartialOrd<$rhs> for Rational {
            fn partial_cmp(&self, other: &$rhs) -> Option<Ordering> {
                Some($func(self, other))
            }
        }
    };
    ($rhs:ty, $func:path, into rhs) => {
        impl PartialOrd<$rhs> for Rational {
            fn partial_cmp(&self, other: &$rhs) -> Option<Ordering> {
                Some($func(self, &Rational::from(other)))
            }
        }

        impl PartialOrd<Rational> for $rhs {
            fn partial_cmp(&self, other: &Rational) -> Option<Ordering> {
                // This implies that:
                // (a cmp b) <=> (b cmp a).reverse()
                // I don't know if thats always true
                Some($func(other, &Rational::from(self)).reverse())
            }
        }

        impl PartialOrd<($rhs, $rhs)> for Rational {
            fn partial_cmp(&self, other: &($rhs, $rhs)) -> Option<Ordering> {
                Some($func(self, &Rational::from(other)))
            }
        }

        impl PartialOrd<Rational> for ($rhs, $rhs) {
            fn partial_cmp(&self, other: &Rational) -> Option<Ordering> {
                // This implies that:
                // (a cmp b) <=> (b cmp a).reverse()
                // I don't know if thats always true
                Some($func(other, &Rational::from(self)).reverse())
            }
        }
    };
}

impl_partial_ord!(Rational, Rational::compare);
impl_partial_ord!(Integer, Rational::compare, into rhs);
impl_partial_ord!(u8, Rational::compare, into rhs);
impl_partial_ord!(i8, Rational::compare, into rhs);
impl_partial_ord!(u16, Rational::compare, into rhs);
impl_partial_ord!(i16, Rational::compare, into rhs);
impl_partial_ord!(i32, Rational::compare, into rhs);
impl_partial_ord!(u32, Rational::compare, into rhs);
impl_partial_ord!(i64, Rational::compare, into rhs);
impl_partial_ord!(u64, Rational::compare, into rhs);
impl_partial_ord!(i128, Rational::compare, into rhs);
impl_partial_ord!(u128, Rational::compare, into rhs);

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> Ordering {
        Rational::compare(self, other)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn compare_rationals() {
        let a = Rational::from(50505);
        let b = Rational::from(5050);

        assert_eq!(a.cmp(&b), Ordering::Greater);
        assert_eq!(a.cmp(&a), Ordering::Equal);
        assert_eq!(b.cmp(&a), Ordering::Less);
    }

    #[test]
    fn compare_rationals_with_operands() {
        let a = Rational::from(12345);
        let b = Rational::from(1234);

        assert!(a != b);
        assert!(a > b);
        assert!(!(a < b));
        // assert!((&a - &b) > b);

        assert!(b != a);
        assert!(b < a);
        assert!(!(b > a));
        // assert!((&b - &a) < 0);
    }

    #[test]
    fn compare_big_rationals_with_small() {
        let a = Rational::from_string_repr("1234567890").unwrap();

        assert!(a > 0);
        assert!(a != 987_654_321);
        assert!(a < 9_999_999_990u128);

        assert!(9_999_999_990u128 > a);
        assert!(987_654_321 != a);
        assert!(0 < a);
    }
}
