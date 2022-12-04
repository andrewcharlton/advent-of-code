use std::num::ParseIntError;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let pairs: Vec<Pair> = INPUT
        .lines()
        .map(|line| line.parse::<Pair>().unwrap())
        .collect();

    let part_one = pairs
        .iter()
        .filter(|pair| pair.0.fully_contains(&pair.1) || pair.1.fully_contains(&pair.0))
        .count();
    println!("Part one: {}", part_one);

    let part_two = pairs
        .iter()
        .filter(|pair| pair.0.overlaps(&pair.1) || pair.1.overlaps(&pair.0))
        .count();
    println!("Part two: {}", part_two);
}

struct Pair(Range, Range);

impl FromStr for Pair {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        let x = x.parse::<Range>()?;
        let y = y.parse::<Range>()?;
        Ok(Pair(x, y))
    }
}

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn fully_contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        (self.start >= other.start && self.start <= other.end)
            || (self.end >= other.start && self.end <= other.end)
    }
}

impl FromStr for Range {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once('-').unwrap();
        let x = x.parse::<u32>()?;
        let y = y.parse::<u32>()?;
        Ok(Range { start: x, end: y })
    }
}
