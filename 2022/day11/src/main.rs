use regex::Regex;
use std::num::ParseIntError;
use std::str::FromStr;

#[macro_use]
extern crate lazy_static;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", part_one(INPUT, 20));
    println!("Part two: {}", part_two(INPUT, 10000));
}

fn part_one(input: &str, rounds: usize) -> usize {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|chunk| chunk.parse().unwrap())
        .collect();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let m = monkeys.get_mut(i).unwrap();
            m.examine().iter().for_each(|(id, worry)| {
                monkeys[*id].items.push(*worry);
            });
        }
    }

    monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));
    monkeys[0].inspections * monkeys[1].inspections
}

fn part_two(input: &str, rounds: usize) -> usize {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|chunk| chunk.parse().unwrap())
        .collect();

    let divisor = monkeys
        .iter()
        .map(|m| m.divide_by)
        .reduce(|acc, x| acc * x / gcd(acc, x))
        .unwrap();

    println!("Divisor: {}", divisor);

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let m = monkeys.get_mut(i).unwrap();
            m.examine_mod(divisor).iter().for_each(|(id, worry)| {
                monkeys[*id].items.push(*worry);
            });
        }
    }

    monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));
    println!("{:?}", monkeys);
    monkeys[0].inspections * monkeys[1].inspections
}

fn gcd(a: i64, b: i64) -> i64 {
    if a % b == 0 {
        return b;
    }
    gcd(b, a % b)
}

#[derive(Debug, Eq, PartialEq)]
enum Op {
    Add(i64),
    Mult(i64),
    Sq,
}

#[derive(Debug, Eq, PartialEq)]
struct Monkey {
    id: usize,
    items: Vec<i64>,
    op: Op,
    divide_by: i64,
    dst_true: usize,
    dst_false: usize,
    inspections: usize,
}

impl FromStr for Monkey {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"Monkey (\d+):\s*
  Starting items: (\d+(?:,\s+\d+)*)\s*
  Operation: new = old (\*|\+) (old|\d+)\s*
  Test: divisible by (\d+)\s*
    If true: throw to monkey (\d+)\s*
    If false: throw to monkey (\d+)"
            )
            .unwrap();
        }

        let caps = RE.captures(s).unwrap();

        let id: usize = caps[1].parse()?;
        let items: Vec<i64> = caps[2]
            .split(", ")
            .map(|item| item.parse::<i64>().unwrap())
            .collect();

        let operator: &str = &caps[3];
        let operand: &str = &caps[4];
        let op: Op = match (operator, operand) {
            ("*", "old") => Op::Sq,
            ("*", _) => {
                let n = operand.parse::<i64>().unwrap();
                Op::Mult(n)
            }
            ("+", _) => {
                let n = operand.parse::<i64>().unwrap();
                Op::Add(n)
            }
            (_, _) => panic!("unknown op: {} {}", operator, operand),
        };

        let divide_by: i64 = caps[5].parse()?;
        let dst_true: usize = caps[6].parse()?;
        let dst_false: usize = caps[7].parse()?;

        Ok(Monkey {
            id,
            items,
            op,
            divide_by,
            dst_true,
            dst_false,
            inspections: 0,
        })
    }
}

impl Monkey {
    fn examine(&mut self) -> Vec<(usize, i64)> {
        let items: Vec<(usize, i64)> = self
            .items
            .drain(0..)
            .map(|item| {
                let worry: i64 = match self.op {
                    Op::Add(x) => (item + x) / 3,
                    Op::Mult(x) => (item * x) / 3,
                    Op::Sq => (item * item) / 3,
                };

                if worry % self.divide_by == 0 {
                    (self.dst_true, worry)
                } else {
                    (self.dst_false, worry)
                }
            })
            .collect();

        self.inspections += items.len();
        items
    }

    fn examine_mod(&mut self, n: i64) -> Vec<(usize, i64)> {
        let items: Vec<(usize, i64)> = self
            .items
            .drain(0..)
            .map(|item| {
                let worry: i64 = match self.op {
                    Op::Add(x) => (item + x) % n,
                    Op::Mult(x) => (item * x) % n,
                    Op::Sq => (item * item) % n,
                };

                if worry % self.divide_by == 0 {
                    (self.dst_true, worry)
                } else {
                    (self.dst_false, worry)
                }
            })
            .collect();

        self.inspections += items.len();
        items
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_parsing() {
        let first = EXAMPLE.split_once("\n\n").unwrap().0;
        let m: Monkey = first.parse().unwrap();
        let exp = Monkey {
            id: 0,
            items: vec![79, 98],
            op: Op::Mult(19),
            divide_by: 23,
            dst_true: 2,
            dst_false: 3,
            inspections: 0,
        };
        assert_eq!(exp, m);
    }

    #[test]
    fn item_examination() {
        let first = EXAMPLE.split_once("\n\n").unwrap().0;
        let mut m: Monkey = first.parse().unwrap();
        let items = m.examine();

        let exp: Vec<(usize, i64)> = vec![(3, 500), (3, 620)];
        assert_eq!(items, exp);
        assert_eq!(2, m.inspections);
        assert_eq!(0, m.items.len());
    }

    #[test]
    fn example() {
        assert_eq!(part_one(EXAMPLE, 20), 10605);
        assert_eq!(part_two(EXAMPLE, 10000), 2713310158);
    }
}
