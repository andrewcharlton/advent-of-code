use regex::Regex;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let plan = Plan::parse(INPUT);
    println!("Part one: {}", plan.interior_area());
}

#[derive(Debug, Eq, PartialEq)]
enum Dir {
    U,
    D,
    L,
    R,
}

struct Section {
    dir: Dir,
    x0: i64,
    x1: i64,
    y0: i64,
    y1: i64,
}

struct Plan {
    sections: Vec<Section>,
    chunk_dividers: Vec<i64>,
    min_y: i64,
    max_y: i64,
}

impl Plan {
    fn parse(s: &str, use_colors: bool) -> Self {
        use Dir::*;

        let mut sections: Vec<Section> = Vec::new();
        let mut chunk_dividers: HashSet<i64> = HashSet::new();
        let (mut min_y, mut max_y) = (0, 0);
        let (mut x, mut y) = (0, 0);

        let re = Regex::new(r"([RLUD]) (\d+) \((.*)\)").unwrap();
        for line in s.lines() {
            let (dir, n) = if use_colors {
                parse_color(line)
            } else {
                parse_instruction(line)
            };

            match dir {
                U => {
                    sections.push(Section {
                        dir: U,
                        x0: x,
                        x1: x,
                        y0: y,
                        y1: y + n,
                    });
                    y += n;
                }
                D => {
                    sections.push(Section {
                        dir: U,
                        x0: x,
                        x1: x,
                        y0: y - n,
                        y1: y,
                    });
                    y -= n;
                    max_y = max_y.max(y);
                }
                L => {
                    sections.push(Section {
                        dir: L,
                        x0: x - n,
                        x1: x,
                        y0: y,
                        y1: y,
                    });
                    x -= n;
                    chunk_dividers.insert(y);
                }
                R => {
                    sections.push(Section {
                        dir: R,
                        x0: x,
                        x1: x + n,
                        y0: y,
                        y1: y,
                    });
                    x += n;
                    chunk_dividers.insert(y);
                }
            }
        }

        sections.sort_unstable_by(|a, b| a.x0.cmp(&b.x0));
        let chunk_dividers: Vec<i64> = chunk_dividers.into_iter().collect();

        Plan {
            sections,
            chunk_dividers,
            min_y,
            max_y,
        }
    }

    fn interior_area(&self) -> usize {}

    fn line_area(&self, y: i64) -> usize {
        let mut area = 0;
        let mut inside = false;
        let mut last_vert: Option<Dir> = None;

        for section in self.sections {
            if section.y0 > y || section.y1 < y {
                continue;
            }

            if section.dir == Dir::

            if section.dir == Dir::L || section.dir == Dir::R {
                continue;
            }
        }

        area
    }
}

fn parse_color(line: &str) -> (Dir, i64) {
    use Dir::*;

    let parts: Vec<&str> = line.split(" ").collect();
    let n = parts.get(1).unwrap().parse().unwrap();
    let dir = *parts.get(0).unwrap();
    let dir = match dir {
        "U" => U,
        "D" => D,
        "R" => R,
        "L" => L,
        _ => panic!("unrecognised direction: {}", dir),
    };

    (dir, n)
}

fn parse_instruction(line: &str) -> (Dir, i64) {
    use Dir::*;

    let parts: Vec<&str> = line.split(" ").collect();
    let n = parts.get(1).unwrap().parse().unwrap();
    let dir = *parts.get(0).unwrap();
    let dir = match dir {
        "U" => U,
        "D" => D,
        "R" => R,
        "L" => L,
        _ => panic!("unrecognised direction: {}", dir),
    };

    (dir, n)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let plan = Plan::parse(EXAMPLE);
        println!("Verticals: {:?}", plan.verts);
        assert_eq!(plan.interior_area(), 62);
    }
}
