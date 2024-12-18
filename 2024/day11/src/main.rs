use std::collections::HashMap;
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let now = Instant::now();

    println!("Part one: {}", solve(INPUT, 25));
    println!("Part two: {}", solve(INPUT, 75));

    let elapsed_time = now.elapsed();
    println!("Solved in {}µs", elapsed_time.as_micros());
}

fn solve(input: &str, blinks: usize) -> usize {
    let mut splitter = StoneSplitter::new();
    let stones = parse_input(input);

    stones
        .iter()
        .map(|stone| splitter.split(*stone, blinks))
        .sum()
}

struct StoneSplitter {
    memoized: HashMap<(usize, usize), usize>,
}

impl StoneSplitter {
    fn new() -> Self {
        StoneSplitter {
            memoized: HashMap::new(),
        }
    }

    fn split(&mut self, stone: usize, blinks: usize) -> usize {
        let key = (stone, blinks);
        if let Some(count) = self.memoized.get(&key) {
            return *count;
        }

        let count = self._split(stone, blinks);
        self.memoized.insert(key, count);
        count
    }

    fn _split(&mut self, stone: usize, blinks: usize) -> usize {
        if blinks == 0 {
            return 1;
        }
        if stone == 0 {
            return self.split(1, blinks - 1);
        }

        let digits = stone.ilog10() + 1;
        if digits % 2 == 0 {
            let div = 10usize.pow(digits / 2);
            return self.split(stone / div, blinks - 1) + self.split(stone % div, blinks - 1);
        }

        self.split(stone * 2024, blinks - 1)
    }
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|stone| stone.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(solve(EXAMPLE, 6), 22);
        assert_eq!(solve(EXAMPLE, 25), 55312);
    }
}
