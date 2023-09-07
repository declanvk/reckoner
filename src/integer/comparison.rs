use crate::integer::Integer;
use core::cmp::Ordering;

macro_rules! impl_partial_eq {
    ($rhs:ty, $func:path) => {
        impl PartialEq<$rhs> for Integer {
            fn eq(&self, other: &$rhs) -> bool {
                $func(&self, other) == Ordering::Equal
            }
        }
    };
    ($rhs:ty, $func:path, deref rhs) => {
        impl PartialEq<$rhs> for Integer {
            fn eq(&self, other: &$rhs) -> bool {
                $func(&self, *other) == Ordering::Equal
            }
        }

        impl PartialEq<Integer> for $rhs {
            fn eq(&self, other: &Integer) -> bool {
                // This doesn't need to be reversed as it simply checks equality
                $func(&other, *self) == Ordering::Equal
            }
        }
    };
    ($rhs:ty, $func:path, into rhs) => {
        impl PartialEq<$rhs> for Integer {
            fn eq(&self, other: &$rhs) -> bool {
                $func(&self, &Integer::from(other)) == Ordering::Equal
            }
        }

        impl PartialEq<Integer> for $rhs {
            fn eq(&self, other: &Integer) -> bool {
                // This doesn't need to be reversed as it simply checks equality
                $func(&other, &Integer::from(self)) == Ordering::Equal
            }
        }
    };
}

impl_partial_eq!(Integer, Integer::compare);
impl_partial_eq!(u8, Integer::compare_c_long, deref rhs);
impl_partial_eq!(i8, Integer::compare_c_long, deref rhs);
impl_partial_eq!(u16, Integer::compare_c_long, deref rhs);
impl_partial_eq!(i16, Integer::compare_c_long, deref rhs);
impl_partial_eq!(i32, Integer::compare_c_long, deref rhs);
cfg_if::cfg_if! {
    if #[cfg(all(target_pointer_width = "64", not(windows)))] {
        impl_partial_eq!(u32, Integer::compare_c_long, deref rhs);
        impl_partial_eq!(i64, Integer::compare_c_long, deref rhs);
    } else {
        impl_partial_eq!(u32, Integer::compare, into rhs);
        impl_partial_eq!(i64, Integer::compare, into rhs);
    }
}
impl_partial_eq!(u64, Integer::compare, into rhs);
impl_partial_eq!(i128, Integer::compare, into rhs);
impl_partial_eq!(u128, Integer::compare, into rhs);

impl Eq for Integer {}

macro_rules! impl_partial_ord {
    ($rhs:ty, $func:path) => {
        impl PartialOrd<$rhs> for Integer {
            fn partial_cmp(&self, other: &$rhs) -> Option<Ordering> {
                Some($func(self, other))
            }
        }
    };
    ($rhs:ty, $func:path, deref rhs) => {
        impl PartialOrd<$rhs> for Integer {
            fn partial_cmp(&self, other: &$rhs) -> Option<Ordering> {
                Some($func(self, *other))
            }
        }

        impl PartialOrd<Integer> for $rhs {
            fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
                // This implies that:
                // (a cmp b) <=> (b cmp a).reverse()
                // I don't know if thats always true
                Some($func(other, *self).reverse())
            }
        }
    };
    ($rhs:ty, $func:path, into rhs) => {
        impl PartialOrd<$rhs> for Integer {
            fn partial_cmp(&self, other: &$rhs) -> Option<Ordering> {
                Some($func(self, &Integer::from(other)))
            }
        }

        impl PartialOrd<Integer> for $rhs {
            fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
                // This implies that:
                // (a cmp b) <=> (b cmp a).reverse()
                // I don't know if thats always true
                Some($func(other, &Integer::from(self)).reverse())
            }
        }
    };
}

// impl_partial_ord!(Integer, Integer::compare);
// manual impl to satify clippy::incorrect_partial_ord_impl_on_ord_type lint
impl PartialOrd for Integer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl_partial_ord!(u8, Integer::compare_c_long, deref rhs);
impl_partial_ord!(i8, Integer::compare_c_long, deref rhs);
impl_partial_ord!(u16, Integer::compare_c_long, deref rhs);
impl_partial_ord!(i16, Integer::compare_c_long, deref rhs);
impl_partial_ord!(i32, Integer::compare_c_long, deref rhs);
cfg_if::cfg_if! {
    if #[cfg(all(target_pointer_width = "64", not(windows)))] {
        impl_partial_ord!(u32, Integer::compare_c_long, deref rhs);
        impl_partial_ord!(i64, Integer::compare_c_long, deref rhs);
    } else {
        impl_partial_ord!(u32, Integer::compare, into rhs);
        impl_partial_ord!(i64, Integer::compare, into rhs);
    }
}
impl_partial_ord!(u64, Integer::compare, into rhs);
impl_partial_ord!(i128, Integer::compare, into rhs);
impl_partial_ord!(u128, Integer::compare, into rhs);

impl Ord for Integer {
    fn cmp(&self, other: &Self) -> Ordering {
        Integer::compare(self, other)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn compare_integers() {
        let a = Integer::from_c_long(50505);
        let b = Integer::from_c_long(5050);

        assert_eq!(a.cmp(&b), Ordering::Greater);
        assert_eq!(a.cmp(&a), Ordering::Equal);
        assert_eq!(b.cmp(&a), Ordering::Less);
    }

    #[test]
    fn compare_integers_with_operands() {
        let a = Integer::from_c_long(12345);
        let b = Integer::from_c_long(1234);

        assert!(a != b);
        assert!(a > b);
        assert!(!(a < b));
        assert!((&a - &b) > b);

        assert!(b != a);
        assert!(b < a);
        assert!(!(b > a));
        assert!((&b - &a) < 0);
    }

    #[test]
    fn compare_big_integers_with_small() {
        let a = Integer::from_string_repr("1234567890", 10).unwrap();

        assert!(a > 0);
        assert!(a != 987_654_321);
        assert!(a < 9_999_999_990u128);

        assert!(9_999_999_990u128 > a);
        assert!(987_654_321 != a);
        assert!(0 < a);
    }
}
