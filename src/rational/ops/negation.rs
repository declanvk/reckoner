// Neg          The unary negation operator -.

use crate::rational::Rational;
use core::ops::Neg;

impl Neg for Rational {
    type Output = Rational;

    fn neg(self) -> Self::Output {
        self.negate()
    }
}

impl Neg for &Rational {
    type Output = Rational;

    fn neg(self) -> Self::Output {
        self.negate()
    }
}
