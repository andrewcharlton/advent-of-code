use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("couldn't read example file");
    println!("Part one: {}", solve(&input, true));
    println!("Part two: {}", solve(&input, false));
}

fn solve(s: &str, twice: bool) -> u64 {
    s.split(",")
        .map(|section| Range::parse(section).invalid_ids(twice))
        .flatten()
        .sum()
}

struct Range {
    min: u64,
    max: u64,
}

impl Range {
    fn parse(s: &str) -> Self {
        let (min, max) = s.split_once("-").unwrap();
        let min: u64 = min.trim().parse().unwrap();
        let max: u64 = max.trim().parse().unwrap();
        Range { min, max }
    }

    fn invalid_ids(&self, twice: bool) -> Vec<u64> {
        let mut result = Vec::new();

        for x in self.min..=self.max {
            if twice {
                if self.is_invalid_twice(x) {
                    result.push(x);
                }
            } else if self.is_invalid(x) {
                result.push(x);
            }
        }

        result
    }

    fn is_invalid_twice(&self, n: u64) -> bool {
        let digits = 1 + n.ilog10();
        if digits % 2 == 1 {
            return false;
        }

        let div = 10u64.pow(digits / 2);
        n / div == n % div
    }

    fn is_invalid(&self, n: u64) -> bool {
        let digits = 1 + n.ilog10();

        for d in 1..=(digits / 2) {
            if digits % d != 0 {
                continue;
            }

            let div = 10u64.pow(d);
            let repeat = n % div;
            let mut x = 0;
            for _ in 0..digits / d {
                x = x * div + repeat;
            }

            if x == n {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = fs::read_to_string("example.txt").expect("couldn't read example file");
        assert_eq!(solve(&input, true), 1227775554);
        assert_eq!(solve(&input, false), 4174379265);
    }

    #[test]
    fn example2() {
        let input = "998-1012";
        assert_eq!(solve(&input, false), 2009);
    }
}
