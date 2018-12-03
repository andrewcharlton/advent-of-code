extern crate regex;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs;

lazy_static! {
    static ref RE: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
}

fn main() {
    let input = fs::read_to_string("input").expect("Couldn't read file");
    let mut claims: Vec<Claim> = input.lines().map(|line| Claim::new(&line)).collect();
    claims.sort_unstable();

    println!("Part one: {}", calc_overlap(&claims));
    println!("Part two: {}", find_non_overlapping(&claims));
}

fn calc_overlap(claims: &Vec<Claim>) -> usize {
    let mut overlaps: HashSet<(u16, u16)> = HashSet::new();

    for i in 0..claims.len() - 1 {
        for j in i + 1..claims.len() {
            // Claims are sorted, based on their left coordinate so we can exit early
            if claims[j].left > claims[i].right {
                break;
            }

            let overlap = claims[i].intersection(&claims[j]);
            if overlap.is_some() {
                for &x in overlap.unwrap().coords().iter() {
                    overlaps.insert(x);
                }
            }
        }
    }

    overlaps.len()
}

fn find_non_overlapping(claims: &Vec<Claim>) -> u16 {
    for i in 0..claims.len() - 1 {
        let mut has_overlap = false;
        for j in 1..claims.len() {
            if i == j {
                continue;
            }

            let overlap = claims[i].intersection(&claims[j]);
            if overlap.is_some() {
                has_overlap = true;
                break;
            }
        }
        if !has_overlap {
            return claims[i].id;
        }
    }

    0
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Claim {
    left: u16,
    right: u16,
    top: u16,
    bottom: u16,
    id: u16,
}

impl Claim {
    fn new(input: &str) -> Claim {
        let caps: Vec<u16> = RE
            .captures(input)
            .unwrap()
            .iter()
            .skip(1)
            .map(|cap| cap.unwrap().as_str().parse().unwrap())
            .collect();

        Claim {
            left: caps[1],
            right: caps[1] + caps[3],
            top: caps[2],
            bottom: caps[2] + caps[4],
            id: caps[0],
        }
    }

    fn intersection(&self, other: &Claim) -> Option<Claim> {
        let left = max(self.left, other.left);
        let right = min(self.right, other.right);
        let top = max(self.top, other.top);
        let bottom = min(self.bottom, other.bottom);

        if left >= right || top >= bottom {
            return None;
        }

        Some(Claim {
            id: 0,
            left,
            right,
            top,
            bottom,
        })
    }

    fn coords(&self) -> Vec<(u16, u16)> {
        let mut coords = Vec::new();

        for i in self.left..self.right {
            for j in self.top..self.bottom {
                coords.push((i, j));
            }
        }

        coords
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn claim_parsing() {
        assert_eq!(
            Claim::new("#123 @ 3,2: 5x4"),
            Claim {
                id: 123,
                left: 3,
                top: 2,
                right: 8,
                bottom: 6
            }
        );
    }

    #[test]
    fn claim_intersection() {
        let a = Claim {
            id: 0,
            left: 2,
            top: 2,
            right: 10,
            bottom: 10,
        };
        let b = Claim {
            id: 0,
            left: 5,
            top: 7,
            right: 8,
            bottom: 12,
        };
        assert_eq!(
            a.intersection(&b).unwrap(),
            Claim {
                id: 0,
                left: 5,
                top: 7,
                right: 8,
                bottom: 10
            }
        );
    }
}
