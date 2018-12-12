use std::collections::{HashSet, VecDeque};
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Couldn't read file");
    let lines: Vec<&str> = input.lines().collect();

    let mut pots = parse_initial_state(lines[0]);
    let rules = parse_rules(&lines[2..]);

    for _ in 0..20 {
        pots = iterate(pots, &rules);
    }

    println!("Part one: {}", pots.iter().sum::<i64>());

    let mut pots = parse_initial_state(lines[0]);
    let mut sum: i64 = pots.iter().sum();
    let mut diffs: VecDeque<i64> = VecDeque::new();
    let mut iteration = 0;
    while diffs.len() < 10 || diffs.iter().min().unwrap() != diffs.iter().max().unwrap() {
        pots = iterate(pots, &rules);
        let next_sum: i64 = pots.iter().sum();

        let diff = next_sum - sum;
        diffs.push_back(diff);
        if diffs.len() > 10 {
            diffs.pop_front();
        }

        sum = next_sum;
        iteration += 1;
    }

    let total = sum + (50000000000 - iteration) * diffs[0];
    println!("Part two: {}", total);
}

fn iterate(pots: HashSet<i64>, rules: &HashSet<Rule>) -> HashSet<i64> {
    let min = pots.iter().min().unwrap();
    let max = pots.iter().max().unwrap();

    let mut next = HashSet::new();
    for p in min - 4..max + 5 {
        if will_spread(&pots, rules, &p) {
            next.insert(p);
        }
    }

    next
}

fn will_spread(pots: &HashSet<i64>, rules: &HashSet<Rule>, p: &i64) -> bool {
    let spread = (
        pots.contains(&(p - 2)),
        pots.contains(&(p - 1)),
        pots.contains(&p),
        pots.contains(&(p + 1)),
        pots.contains(&(p + 2)),
    );

    rules.contains(&spread)
}

fn parse_initial_state(line: &str) -> HashSet<i64> {
    line.chars()
        .filter(|&c| c == '#' || c == '.')
        .enumerate()
        .filter(|&(_, c)| c == '#')
        .map(|(i, _)| i as i64)
        .collect()
}

type Rule = (bool, bool, bool, bool, bool);

fn parse_rules(lines: &[&str]) -> HashSet<Rule> {
    lines
        .iter()
        .filter_map(|line| {
            let chars: Vec<bool> = line
                .chars()
                .filter(|&c| c == '#' || c == '.')
                .map(|c| c == '#')
                .collect();
            if !chars[5] {
                return None;
            }

            Some((chars[0], chars[1], chars[2], chars[3], chars[4]))
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_initial_state_test() {
        let line = "initial state: #..##....";
        let mut expected: HashSet<i64> = HashSet::new();
        expected.insert(0);
        expected.insert(3);
        expected.insert(4);
        assert_eq!(expected, parse_initial_state(&line));
    }

    #[test]
    fn parse_rules_test() {
        let lines: Vec<&str> = ".#### => .\n##.## => #\n#.#.# => .\n.#.#. => #"
            .lines()
            .collect();

        let mut expected: HashSet<Rule> = HashSet::new();
        expected.insert((true, true, false, true, true));
        expected.insert((false, true, false, true, false));
        assert_eq!(expected, parse_rules(&lines));
    }
}
