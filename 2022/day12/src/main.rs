use std::collections::HashMap;
use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

fn part_one(input: &str) -> usize {
    let mut grid = Grid::parse(input);

    while !grid.descend() {}
    grid.n
}

fn part_two(input: &str) -> usize {
    let mut grid = Grid::parse(input);
    while !grid.descend() {}

    *grid
        .visited
        .iter()
        .fold((&grid.start, &grid.n), |best, curr| {
            if curr.1 > &best.1 || grid.heights[curr.0] != 'a' as u32 {
                return best;
            }
            curr
        })
        .1
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

struct Grid {
    n: usize,
    start: Point,
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
        last_visited.insert(end.clone());

        Grid {
            n: 0,
            start,
            heights,
            visited: HashMap::new(),
            last_visited,
        }
    }

    fn descend(&mut self) -> bool {
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
                        if *height.unwrap() < current_height - 1 {
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
        self.visited.contains_key(&self.start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(part_one(EXAMPLE), 31);
        assert_eq!(part_two(EXAMPLE), 29);
    }
}
