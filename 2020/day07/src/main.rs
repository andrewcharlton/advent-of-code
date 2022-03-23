#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;
// use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("couldn't read input");

    let rules: HashMap<&str, Vec<(usize, &str)>> = input.lines().map(parse_line).collect();

    let mut results: HashMap<&str, bool> = HashMap::new();
    for bag in rules.keys().clone() {
        contains_bag(&rules, &mut results, bag, "shiny gold");
    }
    let parents: usize = results.iter().filter(|&(_, &v)| v).count();
    println!("Part one: {}", parents);

    let mut results: HashMap<&str, usize> = HashMap::new();
    let bag_count = descendants(&rules, &mut results, "shiny gold") - 1; // -1 to remove the initial shiny gold
    println!("Part two: {}", bag_count);
}

fn parse_line(line: &str) -> (&str, Vec<(usize, &str)>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<n>\d+) (?P<colour>[\w\s]+) bag").unwrap();
    }

    let container: &str = line.split(" bags contain").nth(0).unwrap();
    let matches: Vec<(usize, &str)> = RE
        .captures_iter(line)
        .map(|cap| {
            (
                cap.name("n").unwrap().as_str().parse::<usize>().unwrap(),
                cap.name("colour").unwrap().as_str(),
            )
        })
        .collect();

    (container, matches)
}

fn contains_bag<'a>(
    rules: &HashMap<&str, Vec<(usize, &'a str)>>,
    results: &mut HashMap<&'a str, bool>,
    bag: &'a str,
    target: &str,
) -> bool {
    if let Some(&x) = results.get(bag) {
        return x;
    }

    if bag == target {
        return true;
    }

    let mut any_contains_target = false;
    if let Some(contains) = rules.get(bag) {
        for (_, bag) in contains {
            let bag_contains_target = contains_bag(rules, results, bag, target);
            any_contains_target |= bag_contains_target;
        }
    }

    results.insert(bag, any_contains_target);
    any_contains_target
}

fn descendants<'a>(
    rules: &HashMap<&str, Vec<(usize, &'a str)>>,
    results: &mut HashMap<&'a str, usize>,
    bag: &'a str,
) -> usize {
    if let Some(&x) = results.get(bag) {
        return x;
    }

    let mut count = 1;
    if let Some(bag_rules) = rules.get(bag) {
        for (n, bag) in bag_rules {
            count += n * descendants(rules, results, bag);
        }
    }

    results.insert(bag, count);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_test() {
        let line = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let expected = vec![(1, "bright white"), (2, "muted yellow")];
        assert_eq!(parse_line(line), ("light red", expected));
    }
}
