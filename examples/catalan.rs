use core::mem;
use reckoner::Integer;

mod common;

fn main() {
    common::sequence_main("catalan", catalan_iter());
}

fn catalan_iter() -> impl Iterator<Item = Integer> {
    struct CatalanIter {
        next: Integer,
        counter: Integer,
    }

    impl Iterator for CatalanIter {
        type Item = Integer;

        fn next(&mut self) -> Option<Self::Item> {
            let next_next = (2 * (2 * &self.counter + 1) * &self.next) / (&self.counter + 2);
            let next = mem::replace(&mut self.next, next_next);
            self.counter += 1;

            Some(next)
        }
    }

    CatalanIter {
        next: 1.into(),
        counter: 0.into(),
    }
}
