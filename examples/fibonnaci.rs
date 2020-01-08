use core::mem;
use reckoner::Integer;

mod common;

fn main() {
    common::sequence_main("fibonacci", fib_iter())
}

fn fib_iter() -> impl Iterator<Item = Integer> {
    struct FibIter {
        x: Integer,
        y: Integer,
    }

    impl Iterator for FibIter {
        type Item = Integer;

        fn next(&mut self) -> Option<Integer> {
            let next_y = &self.x + &self.y;
            let next_x = mem::replace(&mut self.y, next_y);
            let next = mem::replace(&mut self.x, next_x);

            Some(next)
        }
    }

    FibIter {
        x: 0.into(),
        y: 1.into(),
    }
}
