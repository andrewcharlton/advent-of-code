#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let validity: (usize, usize) = fs::read_to_string("input")
        .expect("unable to open file")
        .split("\n\n")
        .map(validate_passport)
        .fold((0, 0), |(acc_p, acc_v), (p, v)| {
            (
                if p { acc_p + 1 } else { acc_p },
                if v { acc_v + 1 } else { acc_v },
            )
        });

    println!("Part one: {}", validity.0);
    println!("Part two: {}", validity.1);
}

fn validate_passport(p: &str) -> (bool, bool) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<key>\w{3}):(?P<val>\S+)").unwrap();
    }

    let values: HashMap<&str, &str> = RE
        .captures_iter(p)
        .map(|cap| {
            (
                cap.name("key").unwrap().as_str(),
                cap.name("val").unwrap().as_str(),
            )
        })
        .collect();

    let mut validity: Vec<(bool, bool)> = Vec::new();

    validity.push(
        values
            .get("byr")
            .map_or((false, false), |val| (true, validate_num(val, 1920, 2002))),
    );

    validity.push(
        values
            .get("iyr")
            .map_or((false, false), |val| (true, validate_num(val, 2010, 2020))),
    );

    validity.push(
        values
            .get("eyr")
            .map_or((false, false), |val| (true, validate_num(val, 2020, 2030))),
    );

    validity.push(
        values
            .get("hgt")
            .map_or((false, false), |val| (true, validate_hgt(val))),
    );

    validity.push(
        values
            .get("hcl")
            .map_or((false, false), |val| (true, validate_hcl(val))),
    );

    validity.push(
        values
            .get("ecl")
            .map_or((false, false), |val| (true, validate_ecl(val))),
    );

    validity.push(
        values
            .get("pid")
            .map_or((false, false), |val| (true, validate_pid(val))),
    );

    validity
        .iter()
        .fold((true, true), |(acc_p, acc_v), &(p, v)| {
            (acc_p && p, acc_v && v)
        })
}

fn validate_num(val: &str, min: usize, max: usize) -> bool {
    val.parse::<usize>()
        .map(|v| v >= min && v <= max)
        .unwrap_or(false)
}

fn validate_hgt(val: &str) -> bool {
    if val.ends_with("cm") {
        return validate_num(val.strip_suffix("cm").unwrap(), 150, 193);
    }

    if val.ends_with("in") {
        return validate_num(val.strip_suffix("in").unwrap(), 59, 76);
    }

    false
}

fn validate_hcl(val: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }

    RE.is_match(val)
}

fn validate_ecl(val: &str) -> bool {
    val == "amb"
        || val == "blu"
        || val == "brn"
        || val == "gry"
        || val == "grn"
        || val == "hzl"
        || val == "oth"
}

fn validate_pid(val: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    }
    RE.is_match(val)
}
