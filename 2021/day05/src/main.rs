#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::str::FromStr;
use std::fs;
use std::collections::HashMap;

fn main() {
    println!("Part one: {}", intersection_count("input.txt", false));
    println!("Part two: {}", intersection_count("input.txt", true));
}


#[derive(PartialEq, Debug)]
struct Segment {
    x0: i16,
    y0: i16,
    x1: i16,
    y1: i16,
}

impl FromStr for Segment {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
        }

        let caps = RE.captures(s).unwrap();
        let x0: i16 = caps.get(1).unwrap().as_str().parse().unwrap();
        let y0: i16 = caps.get(2).unwrap().as_str().parse().unwrap();
        let x1: i16 = caps.get(3).unwrap().as_str().parse().unwrap();
        let y1: i16 = caps.get(4).unwrap().as_str().parse().unwrap();

        Ok(Segment{x0, y0, x1, y1})
    }
}


fn intersection_count(filename: &str, diagonals: bool) -> usize {
    let segments: Vec<Segment> = fs::read_to_string(filename)
        .expect("unable to read file")
        .lines()
        .into_iter()
        .map(|line| line.parse::<Segment>().unwrap())
        .filter(|seg| diagonals || seg.x0 == seg.x1 || seg.y0 == seg.y1)
        .collect();

    let mut intersections: HashMap<(i16, i16), usize> = HashMap::new();
    segments.iter().for_each(|segment| {
        let dx = if segment.x0 == segment.x1 { 0 } else if segment.x0 > segment.x1 { -1 } else { 1 };
        let dy = if segment.y0 == segment.y1 { 0 } else if segment.y0 > segment.y1 { -1 } else { 1 };

        let mut x = segment.x0;
        let mut y = segment.y0;

        while x != segment.x1 || y != segment.y1 {
                let count = intersections.entry((x,y)).or_insert(0);
                *count += 1;
                x += dx;
                y += dy;
        }

        // Insert final point
                let count = intersections.entry((x,y)).or_insert(0);
                *count += 1;
    });

    intersections.values().filter(|&count| count > &1).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_segment() {
        let line = "12,25 -> 19,11".parse::<Segment>().unwrap();
        let exp = Segment{x0: 12, y0: 25, x1: 19, y1: 11};
        assert_eq!(line, exp, "failed to parse correctly");
    }

    #[test]
    fn intersections() {
        assert_eq!(5, intersection_count("example.txt", false), "Wrong answer for part one");
        assert_eq!(12, intersection_count("example.txt", true), "Wrong answer for part two");
    }
}
