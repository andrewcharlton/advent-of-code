use std::collections::{HashMap, HashSet};
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let now = Instant::now();

    println!("Part one: {}", part_one(INPUT, 70, 1024));
    println!("Part two: {}", part_two(INPUT, 70));

    let elapsed_time = now.elapsed();
    println!("Solved in {}µs", elapsed_time.as_micros());
}

fn part_one(input: &str, size: i64, time: usize) -> usize {
    let grid = Grid::new(input, size, size, time);
    grid.best_route().unwrap()
}

fn part_two(input: &str, size: i64) -> &str {
    let max = input.lines().count();

    let (mut i, mut j) = (0, max);
    while i != j {
        let n = (i + j) / 2;
        let grid = Grid::new(input, size, size, n);
        if let Some(_) = grid.best_route() {
            if i == n {
                j = n;
            }
            i = n;
        } else {
            if j == n {
                i = n;
            }
            j = n;
        }
    }

    let lines: Vec<&str> = input.lines().collect();
    lines.get(i).unwrap()
}

struct Grid {
    corrupted: HashSet<(i64, i64)>,
    width: i64,
    height: i64,
}

impl Grid {
    fn new(input: &str, width: i64, height: i64, time: usize) -> Grid {
        let corrupted = input
            .lines()
            .take(time)
            .map(|line| {
                let (x, y) = line.split_once(",").unwrap();
                let x = x.parse().unwrap();
                let y = y.parse().unwrap();
                (x, y)
            })
            .collect();

        Grid {
            corrupted,
            width,
            height,
        }
    }

    fn best_route(&self) -> Option<usize> {
        let mut best: Option<usize> = None;
        let mut visited: HashMap<(i64, i64), usize> = HashMap::new();
        visited.insert((0, 0), 0);

        let mut latest_positions: HashMap<(i64, i64), usize> = HashMap::new();
        latest_positions.insert((0, 0), 0);

        while latest_positions.len() > 0 {
            let mut newer_positions = HashMap::new();

            for ((x, y), score) in latest_positions {
                let neighbours = vec![(x + 1, y), (x - 1, y), (x, y - 1), (x, y + 1)];

                for (x, y) in neighbours {
                    if x < 0 || y < 0 || x > self.width || y > self.height {
                        continue;
                    }

                    if self.corrupted.contains(&(x, y)) {
                        continue;
                    }

                    if let Some(previous_score) = visited.get(&(x, y)) {
                        if *previous_score < score + 1 {
                            continue;
                        }
                    }

                    if let Some(best_score) = best {
                        if best_score < score {
                            continue;
                        }
                    }

                    visited.insert((x, y), score + 1);
                    newer_positions.insert((x, y), score + 1);

                    if x == self.width && y == self.height {
                        if let Some(previous) = best {
                            if score < previous {
                                best = Some(score + 1);
                            }
                        } else {
                            best = Some(score + 1);
                        }
                    }
                }
            }

            latest_positions = newer_positions;
        }

        best
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(part_one(EXAMPLE, 6, 12), 22);
        assert_eq!(part_two(EXAMPLE, 6), "6,1");
    }
}
