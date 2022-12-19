use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

fn part_one(input: &str) -> usize {
    let mut grid = Grid::new(input);

    let mut count = 0;
    loop {
        let p = grid.pour();
        if p.y == grid.lowest {
            return count;
        }

        count += 1;
        grid.filled.insert(p);
    }
}

fn part_two(input: &str) -> usize {
    let mut grid = Grid::new(input);
    grid.lowest += 1;

    let mut count = 0;
    loop {
        count += 1;

        let p = grid.pour();
        if p == (Point { x: 500, y: 0 }) {
            return count;
        }

        grid.filled.insert(p);
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

struct Grid {
    filled: HashSet<Point>,
    lowest: isize,
}

impl Grid {
    fn new(input: &str) -> Grid {
        let filled: HashSet<Point> = input
            .lines()
            .map(|line| parse_wall(line))
            .flatten()
            .collect();

        let lowest = filled.iter().map(|p| p.y).max().unwrap();

        Grid { filled, lowest }
    }

    fn pour(&mut self) -> Point {
        let mut sand_pos = Point { x: 500, y: 0 };

        'outer: loop {
            if sand_pos.y == self.lowest {
                return sand_pos;
            }

            let dirs = vec![0, -1, 1]; // straight down, left then right
            for d in dirs {
                let next_pos = Point {
                    x: sand_pos.x + d,
                    y: sand_pos.y + 1,
                };

                // This position is already filled, so we can't pour here.
                if self.filled.contains(&next_pos) {
                    continue;
                }

                // If we can pour here, do so.
                sand_pos = next_pos;
                continue 'outer;
            }

            // If we get all the way through and we couldn't advance, we mark the position as
            // filled
            return sand_pos;
        }
    }
}

fn parse_wall(line: &str) -> HashSet<Point> {
    let vertices: Vec<Point> = line
        .split(" -> ")
        .map(|p| {
            let (x, y) = p.split_once(",").unwrap();
            let x = x.parse::<isize>().unwrap();
            let y = y.parse::<isize>().unwrap();
            Point { x, y }
        })
        .collect();

    vertices
        .windows(2)
        .map(|window| {
            let (min_x, max_x) = min_max(window[0].x, window[1].x);
            let (min_y, max_y) = min_max(window[0].y, window[1].y);

            if min_x == max_x {
                (min_y..=max_y)
                    .map(|y| Point { x: min_x, y })
                    .collect::<Vec<Point>>()
            } else {
                (min_x..=max_x)
                    .map(|x| Point { x, y: min_y })
                    .collect::<Vec<Point>>()
            }
        })
        .flatten()
        .collect()
}

fn min_max(a: isize, b: isize) -> (isize, isize) {
    if a > b {
        (b, a)
    } else {
        (a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(part_one(EXAMPLE), 24);
        assert_eq!(part_two(EXAMPLE), 93);
    }
}
