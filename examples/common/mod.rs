use core::fmt::Display;
use std::env;

pub(crate) fn sequence_main_custom_display<T>(
    executable_name: &str,
    mut sequence_iter: impl Iterator<Item = T>,
    display: impl Fn(&T) -> String,
) {
    let limit = env::args().nth(1).expect("Missing limit parameter.");

    if limit == "--help" || limit == "-h" {
        println!(
            "usage: ./{} <{{positive number}}|--unlimited|--help|-h>",
            executable_name
        );
    } else if limit == "--unlimited" {
        for n in sequence_iter {
            println!("{}", display(&n));
        }
    } else {
        let limit_number: usize = limit
            .parse()
            .expect("Limit value was not a positive integer.");
        let n = sequence_iter
            .nth(limit_number)
            .expect("Could not produce nth integer.");

        println!("{}", display(&n));
    }
}

#[allow(dead_code)]
pub(crate) fn sequence_main<T: Display>(
    executable_name: &str,
    sequence_iter: impl Iterator<Item = T>,
) {
    sequence_main_custom_display(executable_name, sequence_iter, |item| format!("{}", item))
}
