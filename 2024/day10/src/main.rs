use std::collections::{HashMap, HashSet};
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let now = Instant::now();

    let mut mapper = Mapper::new(INPUT);
    println!("Part one: {}", mapper.trailhead_score_sum());
    println!("Part two: {}", mapper.trailhead_rating_sum());

    let elapsed_time = now.elapsed();
    println!("Solved in {}µs", elapsed_time.as_micros());
}

struct Mapper {
    grid: HashMap<(i64, i64), u32>,
    memoized: HashMap<(i64, i64), usize>,
}

impl Mapper {
    fn new(input: &str) -> Mapper {
        let grid: HashMap<(i64, i64), u32> = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                let line: HashMap<(i64, i64), u32> = line
                    .chars()
                    .enumerate()
                    .map(|(x, c)| {
                        (
                            (i64::try_from(x).unwrap(), i64::try_from(y).unwrap()),
                            c.to_digit(10).unwrap(),
                        )
                    })
                    .collect();
                line
            })
            .flatten()
            .collect();

        Mapper {
            grid,
            memoized: HashMap::new(),
        }
    }

    fn trailhead_score_sum(&mut self) -> usize {
        self.grid
            .iter()
            .filter_map(|(&pos, &n)| {
                if n == 0 {
                    Some(self.trailhead_score(pos))
                } else {
                    None
                }
            })
            .sum()
    }

    fn trailhead_score(&self, pos: (i64, i64)) -> usize {
        let mut positions = HashSet::new();
        positions.insert(pos);

        for height in 1u32..=9 {
            let mut next_positions = HashSet::new();
            for p in positions {
                for neighbour in vec![
                    (p.0 - 1, p.1),
                    (p.0 + 1, p.1),
                    (p.0, p.1 - 1),
                    (p.0, p.1 + 1),
                ] {
                    if let Some(n) = self.grid.get(&neighbour) {
                        if *n == height {
                            next_positions.insert(neighbour);
                        }
                    }
                }
            }

            positions = next_positions;
        }

        positions.len()
    }

    fn trailhead_rating_sum(&mut self) -> usize {
        let mut sum = 0;
        for (&pos, &d) in self.grid.clone().iter() {
            if d == 0 {
                let score = self.trailhead_rating(pos, d);
                sum += score;
            }
        }
        sum
    }

    fn trailhead_rating(&mut self, pos: (i64, i64), n: u32) -> usize {
        if let Some(count) = self.memoized.get(&pos) {
            return *count;
        }

        let count = self._trailhead_rating(pos, n);
        self.memoized.insert(pos, count);
        count
    }

    fn _trailhead_rating(&mut self, pos: (i64, i64), n: u32) -> usize {
        if n == 9 {
            return 1;
        }

        let mut count = 0;
        for neighbour in vec![
            (pos.0 - 1, pos.1),
            (pos.0 + 1, pos.1),
            (pos.0, pos.1 - 1),
            (pos.0, pos.1 + 1),
        ] {
            if let Some(height) = self.grid.get(&neighbour) {
                if *height == n + 1 {
                    count += self.trailhead_rating(neighbour, n + 1);
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
        let mut mapper = Mapper::new(EXAMPLE);
        assert_eq!(mapper.trailhead_score_sum(), 36);
        assert_eq!(mapper.trailhead_rating_sum(), 81);
    }
}
