extern crate regex;
#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

lazy_static! {
    static ref re: Regex = Regex::new(r"<\s*(-?\d+),\s*(-?\d+)>.+<\s*(-?\d+),\s*(-?\d+)>").unwrap();
}

fn main() {
    let input = fs::read_to_string("input").expect("Couldn't read file");
    let points: Vec<Point> = input.lines().map(|line| Point::parse(line)).collect();

    let (min_t, max_t) = candidate_time_range(&points);

    let mut scores = HashMap::new();
    for t in min_t..max_t {
        scores.insert(t, fitness(&points.iter().map(|p| p.advance(t)).collect()));
    }

    let t = *scores
        .iter()
        .max_by_key(|&(_, s)| s)
        .map(|(i, _)| i)
        .unwrap();

    let points: Vec<Point> = points.iter().map(|p| p.advance(t)).collect();
    print_points(&points);
    println!("{}", t);
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Point {
    x: i64,
    y: i64,
    v_x: i64,
    v_y: i64,
}

impl Point {
    fn parse(input: &str) -> Point {
        let caps = re.captures(input).expect("Couldnt match input");
        let caps: Vec<i64> = caps
            .iter()
            .skip(1)
            .filter_map(|m| m.unwrap().as_str().parse::<i64>().ok())
            .collect();

        Point {
            x: *caps.get(0).unwrap(),
            y: *caps.get(1).unwrap(),
            v_x: *caps.get(2).unwrap(),
            v_y: *caps.get(3).unwrap(),
        }
    }

    fn advance(&self, t: i64) -> Point {
        Point {
            x: self.x + t * self.v_x,
            y: self.y + t * self.v_y,
            ..*self
        }
    }

    fn intersection_time(&self, other: &Point) -> Result<i64, &str> {
        let d = (self.v_x - other.v_x) * (self.v_x - other.v_x)
            + (self.v_y - other.v_y) * (self.v_y - other.v_y);
        if d == 0 {
            return Err("Moving parallel");
        }

        let n = (self.x - other.x) * (self.v_x - other.v_x)
            + (self.y - other.y) * (self.v_y - other.v_y);
        Ok(-n / d)
    }
}

fn candidate_time_range(points: &Vec<Point>) -> (i64, i64) {
    let mut times = HashSet::new();
    for (i, p) in points.iter().enumerate() {
        for q in points.iter().skip(i) {
            let t = p.intersection_time(&q);
            if t.is_ok() {
                times.insert(t.unwrap());
            }
        }
    }

    (*times.iter().min().unwrap(), *times.iter().max().unwrap())
}

fn fitness(points: &Vec<Point>) -> usize {
    let mut f = 0;
    for p in points.iter() {
        for q in points.iter() {
            if (p.x - q.x).abs() + (p.y - q.y).abs() == 1 {
                f += 1;
            }
        }
    }

    f
}

fn print_points(points: &Vec<Point>) {
    let mut coords = HashSet::new();
    for p in points.iter() {
        coords.insert((p.x, p.y));
    }

    let min_x = *coords.iter().map(|(x, _)| x).min().unwrap();
    let max_x = *coords.iter().map(|(x, _)| x).max().unwrap();
    let min_y = *coords.iter().map(|(_, y)| y).min().unwrap();
    let max_y = *coords.iter().map(|(_, y)| y).max().unwrap();

    for y in min_y..max_y + 1 {
        let mut chars: Vec<char> = Vec::new();
        for x in min_x..max_x + 1 {
            if coords.contains(&(x, y)) {
                chars.push('#');
            } else {
                chars.push(' ');
            }
        }
        let output: String = chars.iter().collect();
        println!("{}", output);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_test() {
        let input = "position=< 7,  0> velocity=<-1,  0>";
        assert_eq!(
            Point::parse(&input),
            Point {
                x: 7,
                y: 0,
                v_x: -1,
                v_y: 0
            }
        );
    }
}
