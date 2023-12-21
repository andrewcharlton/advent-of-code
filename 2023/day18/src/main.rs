use regex::Regex;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let plan = Plan::parse(INPUT);
    println!("Part one: {}", plan.interior_area());
}

struct Plan {
    trenches: HashMap<(i64, i64), String>,
    verts: HashMap<(i64, i64), bool>,
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

impl Plan {
    fn parse(s: &str) -> Self {
        let mut trenches = HashMap::new();
        let mut verts = HashMap::new();
        let (mut min_x, mut max_x, mut min_y, mut max_y) = (0, 0, 0, 0);
        let (mut x, mut y) = (0, 0);

        let re = Regex::new(r"([RLUD]) (\d+) \((.*)\)").unwrap();
        for line in s.lines() {
            let caps = re.captures(line).unwrap();
            let dir = caps.get(1).unwrap().as_str();
            let n: usize = caps.get(2).unwrap().as_str().parse().unwrap();
            let color = caps.get(3).unwrap().as_str().to_string();

            match dir {
                "U" => {
                    verts.insert((x, y), true);
                    for _ in 0..n {
                        y -= 1;
                        trenches.insert((x, y), color.clone());
                        verts.insert((x, y), true);
                    }
                    min_y = min_y.min(y);
                }
                "D" => {
                    verts.insert((x, y), false);
                    for _ in 0..n {
                        y += 1;
                        trenches.insert((x, y), color.clone());
                        verts.insert((x, y), false);
                    }
                    max_y = max_y.max(y);
                }
                "L" => {
                    for _ in 0..n {
                        x -= 1;
                        trenches.insert((x, y), color.clone());
                    }
                    min_x = min_x.min(x);
                }
                "R" => {
                    for _ in 0..n {
                        x += 1;
                        trenches.insert((x, y), color.clone());
                    }
                    max_x = max_x.max(x);
                }
                _ => panic!("Unknown direction: {}", dir),
            }
        }

        Plan {
            trenches,
            verts,
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    fn interior_area(&self) -> usize {
        let mut count = 0;

        for y in self.min_y..=self.max_y {
            let mut section_start: Option<bool> = None;
            let mut inside = false;

            let mut line = String::new();

            for x in self.min_x..=self.max_x {
                let is_up = self.verts.get(&(x, y));
                match (is_up, section_start) {
                    (Some(is_up), Some(start_was_up)) => {
                        count += 1; // We're on a dug-out trench so count it.
                        section_start = None;
                        if *is_up == start_was_up {
                            // We've gone in one direction so flip the inside status.
                            inside = !inside;
                        }

                        if *is_up {
                            line.push('↑');
                        } else {
                            line.push('↓');
                        }
                    }

                    (Some(is_up), None) => {
                        // We've started a new section.
                        count += 1;
                        section_start = Some(*is_up);
                        if *is_up {
                            line.push('↑');
                        } else {
                            line.push('↓');
                        }
                    }

                    (None, Some(_)) => {
                        // We're either in the middle of a section or finished a section of length
                        // 1.
                        if self.trenches.get(&(x, y)).is_some() {
                            // We're in the middle of a section.
                            count += 1;
                            line.push('-');
                        } else {
                            // We've finished a section.
                            section_start = None;
                            inside = !inside;

                            if inside {
                                count += 1;
                                line.push('*');
                            } else {
                                line.push(' ');
                            }
                        }
                    }

                    (None, None) => {
                        if inside {
                            line.push('*');
                            count += 1;
                        } else {
                            line.push(' ');
                        }
                    }
                }
            }

            println!("{}", line);
        }

        count
    }
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
