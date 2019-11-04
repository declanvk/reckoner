// Neg          The unary negation operator -.

use crate::integer::Integer;
use core::ops::Neg;

impl Neg for Integer {
    type Output = Integer;

    fn neg(self) -> Self::Output {
        self.negate()
    }
}

impl Neg for &Integer {
    type Output = Integer;

    fn neg(self) -> Self::Output {
        self.negate()
    }
}
