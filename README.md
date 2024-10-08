# Reckoner

A high level arbitrary precision integer and rational arithmetic library wrapping [`imath`](https://github.com/creachadair/imath/).

## Example

The following example computes an approximation of pi using the [Newton / Euler Convergence Transformation](https://en.wikipedia.org/wiki/Approximations_of_%CF%80#Other_classical_formulae).

````rust
use reckoner::{Integer, Rational};

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
````

See [`examples/`](https://github.com/declanvk/reckoner/tree/main/examples) for more.

## Crates

The MSRV for both crates is `1.70.0`.

### `reckoner`

[![crates.io](https://img.shields.io/crates/d/reckoner)](https://crates.io/crates/reckoner) [![docs.rs](https://docs.rs/reckoner/badge.svg)](https://docs.rs/reckoner)

A high level arbitrary precision arithmetic library supporting integer and rational numbers.

### `creachadair-imath-sys`

[![crates.io](https://img.shields.io/crates/d/creachadair-imath-sys)](https://crates.io/crates/creachadair-imath-sys) [![docs.rs](https://docs.rs/creachadair-imath-sys/badge.svg)](https://docs.rs/creachadair-imath-sys)

FFI bindings for [`imath`](https://github.com/creachadair/imath/).

## Documentation

[Documentation for `reckoner` from `main` branch](https://declanvk.github.io/reckoner/reckoner/index.html)

[Documentation for `creachadair-imath-sys` from `main` branch](https://declanvk.github.io/reckoner/creachadair_imath_sys/index.html)

## Contributing

Download the crate using the command

```bash
git clone --recurse-submodules https://github.com/declanvk/reckoner
```

so that you also get the submodule sources, which are required to compile the `creachadair-imath-sys` crate. If you already cloned the project and forgot `--recurse-submodules`, you can combine the `git submodule init` and `git submodule update` steps by running `git submodule update --init`.