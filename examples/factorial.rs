use reckoner::Integer;

mod common;

fn main() {
    common::sequence_main("factorial", fact_iter())
}

fn fact_iter() -> impl Iterator<Item = Integer> {
    struct FactIter {
        multiplier: Integer,
        next_value: Integer,
    }

    impl Iterator for FactIter {
        type Item = Integer;

        fn next(&mut self) -> Option<Self::Item> {
            let next = self.next_value.clone();
            self.next_value *= &self.multiplier;
            self.multiplier += 1;

            Some(next)
        }
    }

    FactIter {
        multiplier: 1.into(),
        next_value: 1.into(),
    }
}
