use std::collections::HashMap;
use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", part_one(INPUT));
}

fn part_one(input: &str) -> usize {
    let mut grid = Grid::parse(input);

    while !grid.advance() {}
    grid.n
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

struct Grid {
    n: usize,
    end: Point,
    heights: HashMap<Point, u32>,
    visited: HashMap<Point, usize>,
    last_visited: HashSet<Point>,
}

impl Grid {
    fn parse(input: &str) -> Grid {
        let mut start: Option<Point> = None;
        let mut end: Option<Point> = None;
        let mut heights: HashMap<Point, u32> = HashMap::new();

        input.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, char)| match char {
                'S' => {
                    start = Some(Point { x, y });
                    heights.insert(Point { x, y }, 'a' as u32);
                }
                'E' => {
                    end = Some(Point { x, y });
                    heights.insert(Point { x, y }, 'z' as u32);
                }
                _ => {
                    heights.insert(Point { x, y }, char as u32);
                }
            })
        });

        let start = start.unwrap();
        let end = end.unwrap();

        let mut last_visited: HashSet<Point> = HashSet::new();
        last_visited.insert(start.clone());

        Grid {
            n: 0,
            end,
            heights,
            visited: HashMap::new(),
            last_visited,
        }
    }

    fn advance(&mut self) -> bool {
        let visited: HashSet<Point> = self
            .last_visited
            .iter()
            .map(|p| {
                let mut potentials: HashSet<Point> = HashSet::new();

                if let Some(x) = p.x.checked_sub(1) {
                    potentials.insert(Point { x, y: p.y });
                };
                potentials.insert(Point { x: p.x + 1, y: p.y });

                if let Some(y) = p.y.checked_sub(1) {
                    potentials.insert(Point { x: p.x, y });
                };
                potentials.insert(Point { x: p.x, y: p.y + 1 });

                let current_height = self.heights.get(p).unwrap();
                potentials
                    .into_iter()
                    .filter(|p| {
                        if self.visited.contains_key(p) {
                            return false;
                        }

                        let height = self.heights.get(p);
                        if height.is_none() {
                            return false;
                        }
                        if *height.unwrap() > current_height + 1 {
                            return false;
                        }

                        true
                    })
                    .collect::<Vec<Point>>()
            })
            .flatten()
            .collect();

        self.n += 1;
        for p in &visited {
            self.visited.insert(p.clone(), self.n);
        }

        self.last_visited = visited;
        self.visited.contains_key(&self.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(part_one(EXAMPLE), 31);
    }
}
