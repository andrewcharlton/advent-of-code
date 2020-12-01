#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::fs;

fn main() {
    let valid: Vec<(bool, bool)> = fs::read_to_string("input")
        .expect("unable to open file")
        .lines()
        .map(is_valid)
        .collect();

    println!("Part one: {}", valid.iter().filter(|(a, _)| *a).count());
    println!("Part two: {}", valid.iter().filter(|(_, b)| *b).count());
}

fn is_valid(line: &str) -> (bool, bool) {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<c>\w): (?P<word>\w+)$").unwrap();
    }

    let caps = RE.captures(line).unwrap();

    let min: usize = caps.name("min").unwrap().as_str().parse().unwrap();
    let max: usize = caps.name("max").unwrap().as_str().parse().unwrap();
    let chr: char = caps.name("c").unwrap().as_str().parse().unwrap();
    let word: &str = caps.name("word").unwrap().as_str();

    let count = word.chars().filter(|&c| c == chr).count();

    let first = word.chars().nth(min - 1).unwrap() == chr;
    let second = word.chars().nth(max - 1).unwrap() == chr;

    (count >= min && count <= max, first ^ second)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_test() {
        assert_eq!(is_valid("1-3 a: abcde"), (true, true));
        assert_eq!(is_valid("1-3 b: cdefg"), (false, false));
        assert_eq!(is_valid("2-9 c: ccccccccc"), (true, false));
    }
}
