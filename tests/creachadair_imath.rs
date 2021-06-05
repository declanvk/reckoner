use reckoner::Integer;
use std::{convert::TryFrom, fs::File, io::Read, mem::MaybeUninit, path::Path};

#[derive(Debug)]
struct TestCase<'i, const I: usize, const O: usize> {
    operator: &'i str,
    arguments: [&'i str; I],
    outputs: [&'i str; O],
}

impl<'i, const I: usize, const O: usize> TestCase<'i, I, O> {
    fn parse(input: &'i str) -> Self {
        let mut segments = input.split(':');
        let operator = segments.next().expect("unable to split operator");
        let mut arguments = array_from_iter(
            segments
                .next()
                .expect("unable to split arguments")
                .split(","),
        );
        let outputs = array_from_iter(
            segments
                .next()
                .expect("unable to split outputss")
                .split(","),
        );

        // resolve back references of the form `=n` where `n` is the referenced index
        for idx in 0..arguments.len() {
            if arguments[idx].starts_with('=') {
                let backref = arguments[idx]
                    .trim_start_matches('=')
                    .parse::<usize>()
                    .expect("unable to parse backref index");

                // the backref indices are 1-based, so need to adjust to array indexing.
                arguments[idx] = arguments[backref - 1];
            }
        }

        TestCase {
            operator,
            arguments,
            outputs,
        }
    }
}

fn array_from_iter<T, const N: usize>(mut iter: impl Iterator<Item = T>) -> [T; N] {
    // TODO: replace with `MaybeUninit::uninit_array` when stable
    // SAFETY: An uninitialized `[MaybeUninit<_>; LEN]` is valid.
    let mut arr = unsafe { MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init() };

    for idx in 0..N {
        let item = iter.next().expect("Unable to extract all N items");
        unsafe { arr[idx].as_mut_ptr().write(item) }
    }

    // TODO: replace with `MaybeUninit::array_assume_init` when stable
    // SAFETY:
    // * Earlier blocks guarantee all N elements are initialized
    // * `MaybeUninit<T>` and T are guaranteed to have the same layout
    // * MaybeUnint does not drop, so there are no double-frees
    // And thus the conversion is safe
    unsafe { (&arr as *const _ as *const [T; N]).read() }
}

fn read_file(path: impl AsRef<Path>) -> String {
    let mut file = File::open(path.as_ref()).expect("unable to open file for read");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("unable to read file to string");

    contents
}

fn filter_comments_and_whitespace(input: &&str) -> bool {
    let trimmed = input.trim_start();

    return !(trimmed.starts_with('#') || trimmed.is_empty());
}

macro_rules! tc_path {
    ($s:expr) => {
        concat!("creachadair-imath-sys/vendor/imath/tests/", $s)
    };
}

fn parse_integer(s: &str) -> Integer {
    let (s, radix) = if let Some(s) = s.strip_prefix("#") {
        let radix = match s.chars().next() {
            Some('x') | Some('X') => 16,
            Some('d') | Some('D') => 10,
            Some('o') | Some('O') => 8,
            Some('b') | Some('B') => 2,
            x => panic!("unable to parse radix symbol: '{:?}'", x),
        };

        (s.get(1..).expect("unable to slice remainder"), radix)
    } else {
        (s, 10)
    };

    Integer::from_string(s, radix).unwrap_or_else(|err| panic!("unable to parse: {:?}. {}", s, err))
}

#[test]
fn check_test_data_add() {
    let file_contents = read_file(tc_path!("add.tc"));
    let test_cases = file_contents
        .lines()
        .filter(filter_comments_and_whitespace)
        .map(TestCase::<3, 1>::parse);

    for tc in test_cases {
        match tc.operator {
            "add" => {
                let lhs = parse_integer(tc.arguments[0]);
                let rhs = parse_integer(tc.arguments[1]);
                let result = lhs + rhs;
                let expected = parse_integer(tc.outputs[0]);

                assert_eq!(result, expected);
            }
            "addv" => {
                let lhs = parse_integer(tc.arguments[0]);
                let rhs_large = parse_integer(tc.arguments[1]);
                let rhs = i32::try_from(rhs_large).expect("unable to parse as small int");
                let result = lhs + rhs;
                let expected = parse_integer(tc.outputs[0]);

                assert_eq!(result, expected);
            }
            x => panic!("unknown operator: {:?}", x),
        }
    }
}

#[test]
fn check_test_data_sub() {
    let file_contents = read_file(tc_path!("sub.tc"));
    let test_cases = file_contents
        .lines()
        .filter(filter_comments_and_whitespace)
        .map(TestCase::<3, 1>::parse);

    for tc in test_cases {
        match tc.operator {
            "sub" => {
                let lhs = parse_integer(tc.arguments[0]);
                let rhs = parse_integer(tc.arguments[1]);
                let result = lhs - rhs;
                let expected = parse_integer(tc.outputs[0]);

                assert_eq!(result, expected);
            }
            "subv" => {
                let lhs = parse_integer(tc.arguments[0]);
                let rhs_large = parse_integer(tc.arguments[1]);
                let rhs = i32::try_from(rhs_large).expect("unable to parse as small int");
                let result = lhs - rhs;
                let expected = parse_integer(tc.outputs[0]);

                assert_eq!(result, expected);
            }
            x => panic!("unknown operator: {:?}", x),
        }
    }
}

#[test]
fn check_test_data_mul() {
    let file_contents = read_file(tc_path!("mul.tc"));
    let test_cases = file_contents
        .lines()
        .filter(filter_comments_and_whitespace)
        .map(TestCase::<3, 1>::parse);

    for tc in test_cases {
        match tc.operator {
            "mul" => {
                let lhs = parse_integer(tc.arguments[0]);
                let rhs = parse_integer(tc.arguments[1]);
                let result = lhs * rhs;
                let expected = parse_integer(tc.outputs[0]);

                assert_eq!(result, expected);
            }
            "mulv" => {
                let lhs = parse_integer(tc.arguments[0]);
                let rhs_large = parse_integer(tc.arguments[1]);
                let rhs = i32::try_from(rhs_large).expect("unable to parse as small int");
                let result = lhs * rhs;
                let expected = parse_integer(tc.outputs[0]);

                assert_eq!(result, expected);
            }
            "mulp2" => {} // ignore mulp2 tests as not currently exposed in `reckoner` API.
            x => panic!("unknown operator: {:?}", x),
        }
    }
}

#[test]
fn check_test_data_neg() {
    let file_contents = read_file(tc_path!("neg.tc"));
    let test_cases = file_contents
        .lines()
        .filter(filter_comments_and_whitespace)
        .map(TestCase::<2, 1>::parse);

    for tc in test_cases {
        match tc.operator {
            "neg" => {
                let input = parse_integer(tc.arguments[0]);
                let output = -input;

                let expected = parse_integer(tc.outputs[0]);

                assert_eq!(output, expected);
            }
            x => panic!("unknown operator: {:?}", x),
        }
    }
}
