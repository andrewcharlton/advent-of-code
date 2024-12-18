use once_cell::sync::Lazy;
use regex::Regex;
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let now = Instant::now();

    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));

    let elapsed_time = now.elapsed();
    println!("Solved in {}µs", elapsed_time.as_micros());
}

fn part_one(input: &str) -> i64 {
    input
        .split("\n\n")
        .filter_map(|lines| {
            let machine = Machine::new(lines);
            machine.solve()
        })
        .sum()
}

fn part_two(input: &str) -> i64 {
    input
        .split("\n\n")
        .filter_map(|lines| {
            let mut machine = Machine::new(lines);
            machine.adjust_target(10000000000000);
            machine.solve()
        })
        .sum()
}

#[derive(Debug)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    target: (i64, i64),
}

impl Machine {
    fn new(input: &str) -> Self {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"X[=\+](\d+), Y[=\+](\d+)").unwrap());
        let mut caps = RE.captures_iter(input);

        let a = caps.next().unwrap();
        let a: (i64, i64) = (
            a.get(1).unwrap().as_str().parse().unwrap(),
            a.get(2).unwrap().as_str().parse().unwrap(),
        );

        let b = caps.next().unwrap();
        let b: (i64, i64) = (
            b.get(1).unwrap().as_str().parse().unwrap(),
            b.get(2).unwrap().as_str().parse().unwrap(),
        );

        let target = caps.next().unwrap();
        let target: (i64, i64) = (
            target.get(1).unwrap().as_str().parse().unwrap(),
            target.get(2).unwrap().as_str().parse().unwrap(),
        );

        Self { a, b, target }
    }

    fn adjust_target(&mut self, v: i64) {
        self.target.0 += v;
        self.target.1 += v;
    }

    fn solve(&self) -> Option<i64> {
        let div = self.a.0 * self.b.1 - self.a.1 * self.b.0;
        if div != 0 {
            // The two vectors are not co-linear, so there is at most one solution.
            let num = self.target.0 * self.b.1 - self.target.1 * self.b.0;

            if num % div != 0 {
                return None;
            }
            let a = num / div;

            let num = self.target.0 - a * self.a.0;
            if num % self.b.0 != 0 {
                return None;
            }
            let b = num / self.b.0;

            if a >= 0 && b >= 0 {
                return Some(b + 3 * a);
            }
            return None;
        }

        // If they are co-linear, we need to do things bit differently. Thankfully it doesn't look
        // like there are any cases like this, so we'll ignore.
        println!("{:?} is co-linear", self);
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(part_one(EXAMPLE), 480);
        assert_eq!(part_two(EXAMPLE), 875318608908);
    }
}
