use std::collections::{HashMap, HashSet};
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let now = Instant::now();

    let mut grid = Grid::new(INPUT);
    grid.solve();

    println!("Part one: {}", grid.cheats(2, 100));
    println!("Part two: {}", grid.cheats(20, 100));

    let elapsed_time = now.elapsed();
    println!("Solved in {}µs", elapsed_time.as_micros());
}

struct Grid {
    walls: HashSet<(i64, i64)>,
    start: (i64, i64),
    end: (i64, i64),

    visited: HashMap<(i64, i64), i64>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut walls = HashSet::new();
        let mut start = (0, 0);
        let mut end = (0, 0);

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        walls.insert((x.try_into().unwrap(), y.try_into().unwrap()));
                    }
                    'S' => {
                        start = (x.try_into().unwrap(), y.try_into().unwrap());
                    }
                    'E' => {
                        end = (x.try_into().unwrap(), y.try_into().unwrap());
                    }
                    _ => (),
                }
            }
        }

        Grid {
            walls,
            start,
            end,
            visited: HashMap::new(),
        }
    }

    fn solve(&mut self) {
        self.visited.insert((self.start.0, self.start.1), 0);

        let mut latest_positions: HashMap<(i64, i64), i64> = HashMap::new();
        latest_positions.insert((self.start.0, self.start.1), 0);

        while latest_positions.len() > 0 {
            let mut newer_positions = HashMap::new();

            for ((x, y), score) in latest_positions {
                let neighbours = vec![(x + 1, y), (x - 1, y), (x, y - 1), (x, y + 1)];

                for (x, y) in neighbours {
                    if self.walls.contains(&(x, y)) {
                        continue;
                    }

                    if self.visited.contains_key(&(x, y)) {
                        continue;
                    }

                    self.visited.insert((x, y), score + 1);
                    newer_positions.insert((x, y), score + 1);

                    if x == self.end.0 && y == self.end.1 {
                        return;
                    }
                }
            }

            latest_positions = newer_positions;
        }
    }

    fn cheats(&self, max_cheat_len: i64, min_saving: i64) -> usize {
        let all_points: Vec<(i64, i64)> = self.visited.clone().into_keys().collect();

        let mut count = 0;
        for i in 0..all_points.len() {
            let a = all_points.get(i).unwrap();
            let pts_a = self.visited.get(a).unwrap();

            for j in i + 1..all_points.len() {
                let b = all_points.get(j).unwrap();

                let abs_diff = (b.0 - a.0).abs() + (b.1 - a.1).abs();
                if abs_diff > max_cheat_len {
                    continue;
                }

                let pts_b = self.visited.get(b).unwrap();
                let saving = (pts_b - pts_a).abs() - abs_diff;
                if saving >= min_saving {
                    count += 1;
                }
            }
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
        let mut grid = Grid::new(EXAMPLE);
        grid.solve();

        assert_eq!(grid.cheats(2, 2), 44);
        assert_eq!(grid.cheats(2, 20), 5);
        assert_eq!(grid.cheats(20, 50), 285);
    }
}
