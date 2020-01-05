use crate::{error::Error, integer::Integer, rational::Rational};

mod addition;
mod addition_assign;
mod division;
mod division_assign;
mod multiplication;
mod multiplication_assign;
mod negation;
mod subtraction;
mod subtraction_assign;

impl Rational {
    /// Add two rational values and return the result.
    pub fn add(&self, other: &Self) -> Self {
        let self_raw = self.as_raw();
        let other_raw = other.as_raw();

        let result_rat = Rational::new();
        let result_raw = result_rat.as_raw();

        // This is safe bc `self`, `other`, and `result` have all been initialized.
        let op_res = unsafe { imath_sys::mp_rat_add(self_raw, other_raw, result_raw) };

        imath_check_panic!(op_res, "Add operation failed!");

        result_rat
    }

    /// Add a rational value and an integer value and return the rational
    /// result.
    pub fn add_integer(&self, other: &Integer) -> Self {
        let self_raw = self.as_raw();
        let other_raw = other.as_raw();

        let result_rat = Rational::new();
        let result_raw = result_rat.as_raw();

        // This is safe bc `self`, `other`, and `result` have all been initialized.
        let op_res = unsafe { imath_sys::mp_rat_add_int(self_raw, other_raw, result_raw) };

        imath_check_panic!(op_res, "Add operation failed!");

        result_rat
    }

    /// Add two rationals and assign the result to self.
    pub fn add_assign(&mut self, other: &Self) {
        let self_raw = self.as_raw();
        let other_raw = other.as_raw();

        // This is safe bc `self` and `other` have all been initialized and the result
        // poiner is allowed to alias with either argument pointer.
        let op_res = unsafe { imath_sys::mp_rat_add(self_raw, other_raw, self_raw) };

        imath_check_panic!(op_res, "Add assign operation failed!");
    }

    /// Add a rational value and an integer value and assign the result to self.
    pub fn add_assign_integer(&mut self, other: &Integer) {
        let self_raw = self.as_raw();
        let other_raw = other.as_raw();

        // This is safe bc `self` and `other` have all been initialized and the result
        // poiner is allowed to alias with either argument pointer.
        let op_res = unsafe { imath_sys::mp_rat_add_int(self_raw, other_raw, self_raw) };

        imath_check_panic!(op_res, "Add assign operation failed!");
    }

    /// Subtract two rational values and return the result.
    pub fn subtract(&self, other: &Self) -> Self {
        let self_raw = self.as_raw();
        let other_raw = other.as_raw();

        let result_rat = Rational::new();
        let result_raw = result_rat.as_raw();

        // This is safe bc `self`, `other`, and `result` have all been initialized.
        let op_res = unsafe { imath_sys::mp_rat_sub(self_raw, other_raw, result_raw) };

        imath_check_panic!(op_res, "Subtract operation failed!");

        result_rat
    }

    /// Subtract a rational value and an integer value and return the rational
    /// result.
    pub fn subtract_integer(&self, other: &Integer) -> Self {
        let self_raw = self.as_raw();
        let other_raw = other.as_raw();

        let result_rat = Rational::new();
        let result_raw = result_rat.as_raw();

        // This is safe bc `self`, `other`, and `result` have all been initialized.
        let op_res = unsafe { imath_sys::mp_rat_sub_int(self_raw, other_raw, result_raw) };

        imath_check_panic!(op_res, "Subtract operation failed!");

        result_rat
    }

    /// Subtract two rationals and assign the result to self.
    pub fn subtract_assign(&mut self, other: &Self) {
        let self_raw = self.as_raw();
        let other_raw = other.as_raw();

        // This is safe bc `self` and `other` have all been initialized and the result
        // poiner is allowed to alias with either argument pointer.
        let op_res = unsafe { imath_sys::mp_rat_sub(self_raw, other_raw, self_raw) };

        imath_check_panic!(op_res, "Subtract assign operation failed!");
    }

    /// Subtract a rational value and an integer value and assign the result to
    /// self.
    pub fn subtract_assign_integer(&mut self, other: &Integer) {
        let self_raw = self.as_raw();
        let other_raw = other.as_raw();

        // This is safe bc `self` and `other` have all been initialized and the result
        // poiner is allowed to alias with either argument pointer.
        let op_res = unsafe { imath_sys::mp_rat_sub_int(self_raw, other_raw, self_raw) };

        imath_check_panic!(op_res, "Subtract assign operation failed!");
    }

    /// Multiply two rational values and return the result.
    pub fn multiply(&self, other: &Self) -> Self {
        let self_raw = self.as_raw();
        let other_raw = other.as_raw();

        let result_rat = Rational::new();
        let result_raw = result_rat.as_raw();

        // This is safe bc `self`, `other`, and `result` have all been initialized.
        let op_res = unsafe { imath_sys::mp_rat_mul(self_raw, other_raw, result_raw) };

        imath_check_panic!(op_res, "Multiply operation failed!");

        result_rat
    }

    /// Multiply a rational value and an integer value and return the rational
    /// result.
    pub fn multiply_integer(&self, other: &Integer) -> Self {
        let self_raw = self.as_raw();
        let other_raw = other.as_raw();

        let result_rat = Rational::new();
        let result_raw = result_rat.as_raw();

        // This is safe bc `self`, `other`, and `result` have all been initialized.
        let op_res = unsafe { imath_sys::mp_rat_mul_int(self_raw, other_raw, result_raw) };

        imath_check_panic!(op_res, "Multiply operation failed!");

        result_rat
    }

    /// Multiply two rationals and assign the result to self.
    pub fn multiply_assign(&mut self, other: &Self) {
        let self_raw = self.as_raw();
        let other_raw = other.as_raw();

        // This is safe bc `self` and `other` have all been initialized and the result
        // poiner is allowed to alias with either argument pointer.
        let op_res = unsafe { imath_sys::mp_rat_mul(self_raw, other_raw, self_raw) };

        imath_check_panic!(op_res, "Multiply assign operation failed!");
    }

    /// Multiply a rational value and an integer value and assign the result to
    /// self.
    pub fn multiply_assign_integer(&mut self, other: &Integer) {
        let self_raw = self.as_raw();
        let other_raw = other.as_raw();

        // This is safe bc `self` and `other` have all been initialized and the result
        // poiner is allowed to alias with either argument pointer.
        let op_res = unsafe { imath_sys::mp_rat_mul_int(self_raw, other_raw, self_raw) };

        imath_check_panic!(op_res, "Multiply assign operation failed!");
    }

    /// Divide two rational values and return the result.
    pub fn divide(&self, other: &Self) -> Self {
        let self_raw = self.as_raw();
        let other_raw = other.as_raw();

        let result_rat = Rational::new();
        let result_raw = result_rat.as_raw();

        // This is safe bc `self`, `other`, and `result` have all been initialized.
        let op_res = unsafe { imath_sys::mp_rat_div(self_raw, other_raw, result_raw) };

        imath_check_panic!(op_res, "Divide operation failed!");

        result_rat
    }

    /// Divide a rational value and an integer value and return the rational
    /// result.
    pub fn divide_integer(&self, other: &Integer) -> Self {
        let self_raw = self.as_raw();
        let other_raw = other.as_raw();

        let result_rat = Rational::new();
        let result_raw = result_rat.as_raw();

        // This is safe bc `self`, `other`, and `result` have all been initialized.
        let op_res = unsafe { imath_sys::mp_rat_div_int(self_raw, other_raw, result_raw) };

        imath_check_panic!(op_res, "Divide operation failed!");

        result_rat
    }

    /// Divide two rationals and assign the result to self.
    pub fn divide_assign(&mut self, other: &Self) {
        let self_raw = self.as_raw();
        let other_raw = other.as_raw();

        // This is safe bc `self` and `other` have all been initialized and the result
        // poiner is allowed to alias with either argument pointer.
        let op_res = unsafe { imath_sys::mp_rat_div(self_raw, other_raw, self_raw) };

        imath_check_panic!(op_res, "Divide assign operation failed!");
    }

    /// Divide a rational value and an integer value and assign the result to
    /// self.
    pub fn divide_assign_integer(&mut self, other: &Integer) {
        let self_raw = self.as_raw();
        let other_raw = other.as_raw();

        // This is safe bc `self` and `other` have all been initialized and the result
        // poiner is allowed to alias with either argument pointer.
        let op_res = unsafe { imath_sys::mp_rat_div_int(self_raw, other_raw, self_raw) };

        imath_check_panic!(op_res, "Divide assign operation failed!");
    }

    /// Return the absolute value
    pub fn absolute_value(&self) -> Self {
        let self_raw = self.as_raw();
        let result_int = Rational::new();
        let result_raw = result_int.as_raw();

        // This operation is safe bc `self`and `result` have all been initialized.
        // `result` does not necessarily need to be initialized.
        let op_res = unsafe { imath_sys::mp_rat_abs(self_raw, result_raw) };

        imath_check_panic!(op_res, "Operation failed!");

        result_int
    }

    /// Assign the absolute value to self
    pub fn absolute_value_assign(&mut self) {
        let self_raw = self.as_raw();

        let op_res = unsafe { imath_sys::mp_rat_abs(self_raw, self_raw) };

        imath_check_panic!(op_res, "Operation failed!");
    }

    /// Return the additive inverse
    pub fn negate(&self) -> Self {
        let self_raw = self.as_raw();
        let result = Rational::new();
        let result_raw = result.as_raw();

        // This operation is safe bc `self`and `result` have all been initialized.
        // `result` does not necessarily need to be initialized.
        let op_res = unsafe { imath_sys::mp_rat_neg(self_raw, result_raw) };

        imath_check_panic!(op_res, "Operation failed!");

        result
    }

    /// Assign the additive inverse to self
    pub fn negate_assign(&mut self) {
        let self_raw = self.as_raw();

        // This is safe bc `self` has been initialized and the result
        // pointer is allowed to alias with the argument pointer.
        let op_res = unsafe { imath_sys::mp_rat_neg(self_raw, self_raw) };

        imath_check_panic!(op_res, "Operation failed!");
    }

    /// Return the multiplicative inverse
    pub fn reciprocal(&self) -> Self {
        let self_raw = self.as_raw();
        let result = Rational::new();
        let result_raw = result.as_raw();

        // This operation is safe bc `self`and `result` have all been initialized.
        // `result` does not necessarily need to be initialized.
        let op_res = unsafe { imath_sys::mp_rat_recip(self_raw, result_raw) };

        imath_check_panic!(op_res, "Operation failed!");

        result
    }

    /// Assign the multiplicative inverse to self
    pub fn reciprocal_assign(&mut self) {
        let self_raw = self.as_raw();

        // This is safe bc `self` has been initialized and the result
        // pointer is allowed to alias with the argument pointer.
        let op_res = unsafe { imath_sys::mp_rat_recip(self_raw, self_raw) };

        imath_check_panic!(op_res, "Operation failed!");
    }
}

pub(crate) mod helpers {
    use super::*;

    #[inline]
    pub fn add_reuse_lhs(lhs: impl Into<Rational>, rhs: &Rational) -> Rational {
        let mut lhs = lhs.into();

        Rational::add_assign(&mut lhs, rhs);

        lhs
    }

    #[inline]
    pub fn add_reuse_rhs(lhs: &Rational, rhs: impl Into<Rational>) -> Rational {
        let mut rhs = rhs.into();

        Rational::add_assign(&mut rhs, lhs);

        rhs
    }

    #[inline]
    pub fn add_assign_into_rational(lhs: &mut Rational, rhs: impl Into<Integer>) {
        let rhs = rhs.into();

        lhs.add_assign_integer(&rhs);
    }

    #[inline]
    pub fn multiply_reuse_lhs(lhs: impl Into<Rational>, rhs: &Rational) -> Rational {
        let mut lhs = lhs.into();

        Rational::multiply_assign(&mut lhs, rhs);

        lhs
    }

    #[inline]
    pub fn multiply_reuse_rhs(lhs: &Rational, rhs: impl Into<Rational>) -> Rational {
        let mut rhs = rhs.into();

        Rational::multiply_assign(&mut rhs, lhs);

        rhs
    }

    #[inline]
    pub fn multiply_assign_into_rational(lhs: &mut Rational, rhs: impl Into<Integer>) {
        let rhs = rhs.into();

        lhs.multiply_assign_integer(&rhs);
    }

    #[inline]
    pub fn subtract_reuse_lhs(lhs: impl Into<Rational>, rhs: &Rational) -> Rational {
        let mut lhs = lhs.into();

        lhs.subtract_assign(rhs);

        lhs
    }

    #[inline]
    pub fn divide_reuse_lhs(lhs: impl Into<Rational>, rhs: &Rational) -> Rational {
        let mut lhs = lhs.into();

        lhs.divide_assign(rhs);

        lhs
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_rationals() {
        let a = Rational::from(11111);
        let b = Rational::from(33333);
        let c = a + b;

        assert_eq!(c, 44_444);
    }

    #[test]
    fn add_assign_rationals() {
        let mut a = Rational::from(11111);
        let b = Rational::from(33333);
        a += b;

        assert_eq!(a, 44_444);
    }

    #[test]
    fn subtract_rationals() {
        let a = Rational::from(12345);
        let b = Rational::from(1234);
        let c = a - b;

        assert_eq!(c, 11_111);
    }

    #[test]
    fn subtract_assign_rationals() {
        let mut a = Rational::from(12345);
        let b = Rational::from(1234);
        a -= b;

        assert_eq!(a, 11_111);
    }

    #[test]
    fn multiply_rationals() {
        let a = Rational::from(50505);
        let b = Rational::from(5050);
        let c = a * b;

        assert_eq!(c, 255_050_250);
    }

    #[test]
    fn multiply_assign_rationals() {
        let mut a = Rational::from(50505);
        let b = Rational::from(5050);

        a *= b;
        assert_eq!(a, 255_050_250);
    }

    #[test]
    fn divide_rationals() {
        let a: Rational = "52384129912341238437480192384".parse().unwrap();

        #[allow(clippy::eq_op)]
        let one = &a / &a;
        assert_eq!(one, 1);
        assert_eq!(
            &a / 10_000_000_000_000_000_000_000u128,
            Rational::from((
                52_384_129_912_341_238_437_480_192_384u128,
                10_000_000_000_000_000_000_000u128
            ))
        );

        let b: Rational = 1_234_567.into();
        assert_eq!(&b / 123_456, Rational::from((1_234_567, 123_456)));
        assert_eq!(&b / 3, Rational::from((1_234_567, 3)));
        assert_eq!(&b / -3, Rational::from((-1_234_567, 3)));
        assert_eq!(&b / -230, Rational::from((-1_234_567, 230)))
    }

    #[test]
    fn divide_assign_rationals() {
        let mut a: Rational = "52384129912341238437480192384".parse().unwrap();

        a /= a.clone();
        assert_eq!(a, 1);
    }

    #[test]
    fn negate_rationals() {
        let a: Rational = "52384129912341238437480192384".parse().unwrap();
        let b = -a;

        let string_repr = b.to_string();
        assert_eq!(&string_repr, "-52384129912341238437480192384/1");
    }

    #[test]
    fn absolute_value_rational() {
        let neg_int: Rational = "-37129740".parse().unwrap();
        let pos_int: Rational = "37129740".parse().unwrap();

        assert_eq!(neg_int.absolute_value(), pos_int);
        assert_eq!(pos_int.absolute_value(), 37_129_740);
    }
}
