#[macro_use]
extern crate lazy_static;

use std::fs;

use regex::{Regex, RegexSet};

fn main() {
    let input = fs::read_to_string("./input.txt");
    match input {
        Ok(input) => {
            let solution = solve_naive_2(input);
            println!("{}", solution);
            let _ = fs::write("output.txt", format!("{}", solution));
        }
        Err(err) => {
            panic!("Couldn't read file ./input.txt: {}", err);
        }
    }
}

fn solve_naive(input: String) -> u64 {
    input
        .as_bytes()
        .iter()
        .filter(|&c| c.is_ascii_digit() || c == &b'\n')
        .fold(
            (0u64, None::<&u8>, None::<&u8>),
            |(acc, first, last), value| {
                if value == &b'\n' {
                    let to_add = match (first, last) {
                        (Some(&f), Some(&l)) => u64::from(f - 48) * 10 + u64::from(l - 48),
                        _ => panic!("unreachable"),
                    };
                    println!("{} {} {}", acc, first.unwrap() - 48, last.unwrap() - 48);
                    return (acc + to_add, None, None);
                }

                match (first, last) {
                    (None, _) => (acc, Some(value), Some(value)),
                    (Some(f), Some(_)) => (acc, Some(f), Some(value)),
                    _ => {
                        panic!("unreachable")
                    }
                }
            },
        )
        .0
}

const PATTERNS: [&str; 10] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "[0-9]",
];

fn replacer(pattern: &str) -> u64 {
    match pattern {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,

        _ => panic!("unreachable"),
    }
}

lazy_static! {
    static ref SET: RegexSet = RegexSet::new(PATTERNS).unwrap();
    static ref REGEXES: Vec<Regex> = SET
        .patterns()
        .iter()
        .map(|pat| Regex::new(pat).unwrap())
        .collect();
}

fn matcher(line: &str) -> u64 {
    let matches: Vec<_> = SET
        .matches(line)
        .into_iter()
        .map(|index| &REGEXES[index])
        .flat_map(|re| re.find_iter(line))
        .map(|ma| (ma.as_str(), ma.start()))
        .collect();

    let first = matches
        .iter()
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .map(|&(s, _)| replacer(s))
        .expect("Every line should have at least one match, something went wrong!");

    let second = matches
        .iter()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map_or(first, |&(s, _)| replacer(s));

    first * 10 + second
}

fn solve_naive_2(input: String) -> u64 {
    input.lines().map(matcher).sum()
}
