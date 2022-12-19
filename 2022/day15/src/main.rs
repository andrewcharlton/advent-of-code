use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

#[macro_use]
extern crate lazy_static;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", part_one(INPUT, 2000000));
    println!("Part two: {}", part_two(INPUT, 0, 4000000));
}

fn part_one(input: &str, row: i64) -> usize {
    let sensors: Vec<Sensor> = input.lines().map(|line| line.parse().unwrap()).collect();

    let beacons: HashSet<i64> = sensors
        .iter()
        .filter(|s| s.beacon_y == row)
        .map(|s| s.beacon_x)
        .collect();

    let not_present: HashSet<i64> = sensors
        .iter()
        .filter_map(|s| s.scanned_on_row(row))
        .map(|range| (range.0..=range.1))
        .flatten()
        .collect();

    not_present.difference(&beacons).count()
}

fn part_two(input: &str, min: i64, max: i64) -> i64 {
    let mut sensors: Vec<Sensor> = input.lines().map(|line| line.parse().unwrap()).collect();
    sensors.sort_by(|a, b| {
        let ord = a.x.cmp(&b.x);
        if ord == Ordering::Equal {
            a.y.cmp(&b.y)
        } else {
            ord
        }
    });

    for y in min..=max {
        let mut ranges: Vec<(i64, i64)> = sensors
            .iter()
            .filter_map(|line| line.scanned_on_row(y))
            .collect();

        // Sort them so that we have them in order left to right.
        ranges.sort_by(|a, b| a.0.cmp(&b.0));

        // Iterate through the ranges,
        let mut end = min - 1;
        for r in ranges {
            if r.0 > end && r.0 > min {
                // We've found the gap.
                println!("Found beacon: ({}, {})", end + 1, y);
                return (end + 1) * 4000000 + y;
            }

            if r.1 > end {
                end = r.1;
            }
        }
    }

    panic!("no row found!");
}

struct Sensor {
    x: i64,
    y: i64,
    beacon_x: i64,
    beacon_y: i64,
    radius: i64,
}

impl FromStr for Sensor {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"
            )
            .unwrap();
        }

        let caps = RE.captures(s).unwrap();
        let x: i64 = caps[1].parse()?;
        let y: i64 = caps[2].parse()?;
        let beacon_x: i64 = caps[3].parse()?;
        let beacon_y: i64 = caps[4].parse()?;

        let radius: i64 = (x - beacon_x).abs() + (y - beacon_y).abs();
        Ok(Sensor {
            x,
            y,
            beacon_x,
            beacon_y,
            radius,
        })
    }
}

impl Sensor {
    fn scanned_on_row(&self, y: i64) -> Option<(i64, i64)> {
        let d_y: i64 = (self.y - y).abs();
        if d_y > self.radius {
            return None;
        }

        let d_x = self.radius - d_y;
        Some((self.x - d_x, self.x + d_x))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(part_one(EXAMPLE, 10), 26);
        assert_eq!(part_two(EXAMPLE, 0, 20), 56000011);
    }
}
