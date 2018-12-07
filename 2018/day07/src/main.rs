extern crate regex;

use regex::Regex;
use std::collections::HashSet;
use std::fs;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref re: Regex = Regex::new(r"Step ([A-Z]).*step ([A-Z]) can begin").unwrap();
}

fn main() {
    let input = fs::read_to_string("input").expect("Couldn't read file");

    println!("Part one: {}", running_order(&input));
}

fn running_order(input: &str) -> String {
    let mut pairs: Vec<(&str, &str)> = input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            (
                caps.get(0).map(|c| c.as_str()).unwrap_or(&""),
                caps.get(1).map(|c| c.as_str()).unwrap_or(&""),
            )
        })
        .collect();

    let mut steps: HashSet<&str> = HashSet::new();
    for (a, b) in pairs.iter() {
        steps.insert(a);
        steps.insert(b);
    }

    let mut order: Vec<&str> = Vec::new();

    while pairs.len() > 0 {
        let waiting_on: HashSet<&str> = HashSet::new();
        for (a, _) in pairs {
            waiting_on.insert(a);
        }

        let candidate = steps
            .iter()
            .filter(|&a| !waiting_on.contains(a))
            .min()
            .unwrap();

        order.push(candidate);
        steps.remove(candidate);
        pairs = pairs.into_iter().filter(|(a, _)| a != candidate).collect();
    }

    order.into_iter().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn running_order_test() {
        let input = "Step C must be finished before step A can begin.
                        Step C must be finished before step F can begin.
                        Step A must be finished before step B can begin.
                        Step A must be finished before step D can begin.
                        Step B must be finished before step E can begin.
                        Step D must be finished before step E can begin.
                        Step F must be finished before step E can begin.";

        assert_eq!(running_order(&input), String::from("CABDFE"));
    }
}
