use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let now = Instant::now();

    let mut towels = Towels::new(INPUT);

    println!("Part one: {}", towels.possible_designs());
    println!("Part two: {}", towels.total_combinations());

    let elapsed_time = now.elapsed();
    println!("Solved in {}µs", elapsed_time.as_micros());
}

struct Towels {
    patterns: HashSet<String>,
    min_pattern_length: usize,
    max_pattern_length: usize,

    designs: Vec<String>,

    seen: HashMap<String, usize>,
}

impl Towels {
    fn new(input: &str) -> Self {
        let (patterns, designs) = input.split_once("\n\n").unwrap();

        let patterns: HashSet<String> = patterns.split(",").map(|s| s.trim().to_owned()).collect();
        let designs: Vec<String> = designs.lines().map(|s| s.to_owned()).collect();

        let (min_pattern_length, max_pattern_length) = patterns
            .iter()
            .map(|p| p.len())
            .fold((usize::MAX, 0), |acc, x| (min(acc.0, x), max(acc.1, x)));

        Towels {
            patterns,
            min_pattern_length,
            max_pattern_length,
            designs,
            seen: HashMap::new(),
        }
    }

    fn possible_designs(&mut self) -> usize {
        let mut count = 0;
        for design in self.designs.clone() {
            if self.combinations(&design) > 0 {
                count += 1;
            }
        }

        count
    }

    fn total_combinations(&mut self) -> usize {
        let mut count = 0;
        for design in self.designs.clone() {
            count += self.combinations(&design);
        }
        count
    }

    fn combinations(&mut self, s: &str) -> usize {
        if let Some(count) = self.seen.get(s) {
            return *count;
        }

        let count = self._combinations(s);
        self.seen.insert(s.to_string(), count);
        count
    }

    fn _combinations(&mut self, s: &str) -> usize {
        if s.len() == 0 {
            return 1;
        }

        let mut count = 0;
        for i in self.min_pattern_length..=self.max_pattern_length {
            if i > s.len() {
                break;
            }

            let (prefix, remainder) = s.split_at(i);
            if !self.patterns.contains(prefix) {
                continue;
            }

            count += self.combinations(remainder);
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
        let mut towels = Towels::new(EXAMPLE);
        assert_eq!(towels.possible_designs(), 6);
        assert_eq!(towels.total_combinations(), 16);
    }
}
