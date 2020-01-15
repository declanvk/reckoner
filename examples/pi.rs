use reckoner::{Integer, Rational, RoundMode};
use std::env;

// Compute the value of PI to a selected degree of precision.
fn main() {
    let args: Vec<_> = env::args().collect();

    if let Some([iterations, precision]) = args.get(1..=2) {
        let iterations = iterations
            .parse()
            .expect("Unable to parse 'iterations' value as a 32-bit unsigned integer.");
        let precision = precision
            .parse()
            .expect("Unable to parse 'precision' value as a 16-bit unsigned integer.");

        let pi = compute_pi_approx(iterations);

        println!("{}", pi.to_decimal(RoundMode::HalfUp, precision));
    } else {
        help();
    }
}

fn factorial(v: &Integer) -> Integer {
    let mut accum = 1.into();
    let mut f = v.clone();

    while f > 0 {
        accum *= &f;
        f -= 1;
    }

    accum
}

// Product of all odd integer up to the given value.
fn odd_factorial(v: &Integer) -> Integer {
    let mut accum = 1.into();
    let mut f = if v % 2 == 0 { v - 1 } else { v.clone() };

    while f > 0 {
        accum *= &f;
        f -= 2;
    }

    accum
}

// Compute a rational approximation of PI based on https://en.wikipedia.org/wiki/Approximations_of_%CF%80#Other_classical_formulae
//
// ```
// \frac{\pi}{2}
//     = \sum_{k=0}^\infty\frac{k!}{(2k+1)!!}
//     = \sum_{k=0}^{\infty} \cfrac {2^k k!^2}{(2k + 1)!}
//     = 1+\frac{1}{3}\left(1+\frac{2}{5}\left(1+\frac{3}{7}\left(1+\cdots\right)\right)\right)
// ```
fn compute_pi_approx(iterations: u32) -> Rational {
    2 * (0..iterations)
        .map(Integer::from)
        .map(|n| {
            let numerator = factorial(&n);
            let denominator = odd_factorial(&(2 * n + 1));

            (numerator, denominator).into()
        })
        .sum::<Rational>()
}

fn help() {
    eprintln!("Usage: ./pi.rs <iterations> <precision>");
}
