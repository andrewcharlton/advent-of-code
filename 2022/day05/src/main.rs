use regex::Regex;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

#[macro_use]
extern crate lazy_static;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", solve(INPUT, false));
    println!("Part two: {}", solve(INPUT, true));
}

fn solve(input: &str, batch: bool) -> String {
    let mut crates: Crates = input.parse().expect("unable to parse input");

    input
        .lines()
        .filter(|line| line.starts_with("move"))
        .map(|line| line.parse::<Move>().unwrap())
        .for_each(|m| crates.move_crates(m, batch));

    crates.top()
}

struct Crates {
    stacks: HashMap<usize, Vec<char>>,
}

impl FromStr for Crates {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut assignment: Vec<&str> = s
            .lines()
            .filter(|line| *line != "" && !line.starts_with("move"))
            .collect();

        // Map of position to stack number
        let columns: HashMap<usize, usize> = assignment
            .pop()
            .unwrap()
            .chars()
            .enumerate()
            .filter(|(_, c)| c.is_digit(10))
            .map(|(i, c)| (i, c.to_digit(10).unwrap() as usize))
            .collect();

        let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();
        assignment.iter().rev().for_each(|line| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '[' && *c != ']' && *c != ' ')
                .for_each(|(i, c)| {
                    let n = columns.get(&i).unwrap();
                    stacks
                        .entry(*n)
                        .and_modify(|stack| stack.push(c))
                        .or_insert(vec![c]);
                })
        });

        Ok(Crates { stacks })
    }
}

impl Crates {
    fn move_crates(&mut self, m: Move, batch: bool) {
        let mut moving: Vec<char> = Vec::new();

        let src_stack = self.stacks.get_mut(&m.src).unwrap();
        for _ in 0..m.n {
            moving.push(src_stack.pop().unwrap());
        }

        if !batch {
            moving.reverse();
        }

        let dst_stack = self.stacks.get_mut(&m.dst).unwrap();
        for _ in 0..m.n {
            dst_stack.push(moving.pop().unwrap());
        }
    }

    fn top(&self) -> String {
        let mut ordered_crates: Vec<usize> = self.stacks.keys().copied().collect();
        ordered_crates.sort();

        ordered_crates
            .iter()
            .map(|k| self.stacks.get(&k).unwrap().last().unwrap())
            .collect()
    }
}

struct Move {
    n: usize,
    src: usize,
    dst: usize,
}

impl FromStr for Move {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref re: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        }
        let caps = re.captures(s).unwrap();

        let n: usize = caps.get(1).unwrap().as_str().parse().unwrap();
        let src: usize = caps.get(2).unwrap().as_str().parse().unwrap();
        let dst: usize = caps.get(3).unwrap().as_str().parse().unwrap();
        Ok(Move { n, src, dst })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn parse_crates() {
        let crates: Crates = EXAMPLE.parse().unwrap();
        assert_eq!(crates.stacks.get(&1).unwrap(), &vec!['Z', 'N']);
        assert_eq!(crates.stacks.get(&2).unwrap(), &vec!['M', 'C', 'D']);
        assert_eq!(crates.stacks.get(&3).unwrap(), &vec!['P']);
    }

    #[test]
    fn example() {
        assert_eq!(solve(EXAMPLE, false), "CMZ");
        assert_eq!(solve(EXAMPLE, true), "MCD");
    }
}
