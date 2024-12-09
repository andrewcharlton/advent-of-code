use std::collections::{HashMap, HashSet};
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let now = Instant::now();

    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));

    let elapsed_time = now.elapsed();
    println!("Solved in {}µs", elapsed_time.as_micros());
}

fn part_one(input: &str) -> u64 {
    let (orderings, updates) = parse_input(input);

    updates
        .iter()
        .filter(|update| is_in_order(update, &orderings))
        .map(|update| *update.get(update.len() / 2).unwrap())
        .sum()
}

fn part_two(input: &str) -> u64 {
    let (orderings, updates) = parse_input(input);

    updates
        .iter()
        .filter(|update| !is_in_order(update, &orderings))
        .map(|update| sort_update(update, &orderings))
        .map(|update| *update.get(update.len() / 2).unwrap())
        .sum()
}

fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<Vec<u64>>) {
    let parts = input.split_once("\n\n").unwrap();

    let orderings = parts
        .0
        .lines()
        .map(|line| {
            let nums = line.split_once("|").unwrap();
            let a = nums.0.parse().unwrap();
            let b = nums.1.parse().unwrap();
            (a, b)
        })
        .collect();

    let updates = parts
        .1
        .lines()
        .map(|line| line.split(",").map(|num| num.parse().unwrap()).collect())
        .collect();

    (orderings, updates)
}

fn is_in_order(update: &Vec<u64>, orderings: &Vec<(u64, u64)>) -> bool {
    for ordering in orderings {
        let first = find(update, ordering.0);
        if first.is_none() {
            continue;
        }

        let second = find(update, ordering.1);
        if second.is_none() {
            continue;
        }

        if first.unwrap() > second.unwrap() {
            return false;
        }
    }

    true
}

fn sort_update(update: &Vec<u64>, orderings: &Vec<(u64, u64)>) -> Vec<u64> {
    let mut before: HashMap<u64, HashSet<u64>> = HashMap::new();
    for ordering in orderings {
        if find(update, ordering.0).is_none() || find(update, ordering.1).is_none() {
            continue;
        }

        if let Some(s) = before.get_mut(&ordering.0) {
            s.insert(ordering.1);
        } else {
            let mut s = HashSet::new();
            s.insert(ordering.1);
            before.insert(ordering.0, s);
        }
    }

    // It looks like the order is completely explicit and there's no need to imply
    // anything. i.e. if we have a < b < c then we will be told that a < b, b < c
    // and a < c, there's no need to work that out.
    // For the more general solution where we do have to work the implicit orderings
    // out, we could iterate through the map, extending it by everything after each
    let mut update = update.clone();
    update.sort_by(|a, b| {
        let a = before.get(a).map(|s| s.len()).unwrap_or(0);
        let b = before.get(b).map(|s| s.len()).unwrap_or(0);
        a.cmp(&b)
    });

    update
}

fn find(update: &Vec<u64>, key: u64) -> Option<usize> {
    update
        .iter()
        .enumerate()
        .find(|(_, x)| **x == key)
        .map(|(i, _)| i)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(part_one(EXAMPLE), 143);
        assert_eq!(part_two(EXAMPLE), 123);
    }
}
