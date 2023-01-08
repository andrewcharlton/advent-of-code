use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Write};
use std::num::ParseIntError;
use std::str::FromStr;

#[macro_use]
extern crate lazy_static;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

fn part_one(input: &str) -> usize {
    0
}

fn part_two(input: &str) -> usize {
    0
}

fn parse_input(input: &str) -> HashMap<Location, Room> {
    input
        .lines()
        .map(|line| line.parse::<Room>().unwrap())
        .map(|room| (room.loc.clone(), room))
        .collect()
}

const A: u32 = 'A' as u32;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Location(u32);

impl FromStr for Location {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(());
        }

        Ok(Location(
            s.chars().fold(0, |acc, c| acc * 26 + (c as u32 - A)),
        ))
    }
}

impl Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(char::from_u32((self.0 / 26) + A).unwrap())?;
        f.write_char(char::from_u32((self.0 % 26) + A).unwrap())
    }
}

struct Room {
    loc: Location,
    flow_rate: usize,
    tunnels: Vec<Location>,
}

impl FromStr for Room {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref re: Regex = Regex::new(
                r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels lead to valves ([A-Z\s,]+)"
            )
            .unwrap();
        }
        let caps = re.captures(s).unwrap();

        let loc: Location = caps[1].parse().unwrap();
        let flow_rate: usize = caps[2].parse()?;
        let tunnels: Vec<Location> = caps[3]
            .split(", ")
            .map(|loc| loc.parse().unwrap())
            .collect();

        return Ok(Room {
            loc,
            flow_rate,
            tunnels,
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(part_one(EXAMPLE), 0);
        assert_eq!(part_two(EXAMPLE), 0);
    }
}
