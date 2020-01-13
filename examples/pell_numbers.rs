use core::mem;
use reckoner::{Integer, Rational, RoundMode};

mod common;

fn main() {
    common::sequence_main_custom_display("sqrt_2_approximations", sqrt_2_approximations(), |rat| {
        rat.to_decimal(RoundMode::HalfUp, 100)
    });
}

struct PellLucasNumbersIter {
    g: fn(&Integer, &Integer) -> Integer,
    first: Integer,
    second: Integer,
}

impl Iterator for PellLucasNumbersIter {
    type Item = Integer;

    fn next(&mut self) -> Option<Self::Item> {
        let next_second = (self.g)(&self.first, &self.second);
        let next_first = mem::replace(&mut self.second, next_second);
        let next = mem::replace(&mut self.first, next_first);

        Some(next)
    }
}

fn generator(first: &Integer, second: &Integer) -> Integer {
    2 * second + first
}

fn pell_numbers_iter() -> impl Iterator<Item = Integer> {
    PellLucasNumbersIter {
        g: generator,
        first: 0.into(),
        second: 1.into(),
    }
}

fn pell_lucas_numbers_iter() -> impl Iterator<Item = Integer> {
    PellLucasNumbersIter {
        g: generator,
        first: 1.into(),
        second: 1.into(),
    }
}

fn sqrt_2_approximations() -> impl Iterator<Item = Rational> {
    pell_lucas_numbers_iter()
        .zip(pell_numbers_iter())
        .skip(1)
        .map(Rational::from)
}
