use std::convert::TryFrom;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("couldn't read input");

    let ans = hundred_phases(&input);
    println!("Part one: {}", ans);
}

fn hundred_phases(input: &str) -> String {
    let mut digits: Vec<i64> = input
        .trim()
        .chars()
        .map(|c| i64::try_from(c.to_digit(10).unwrap()).unwrap())
        .collect();

    for _ in 0..100 {
        next_phase(&mut digits);
    }

    digits.iter().take(8).map(|d| d.to_string()).collect()
}

fn next_phase(digits: &mut Vec<i64>) {
    let previous = digits.clone();

    for (i, n) in digits.iter_mut().enumerate() {
        let m = Multipliers::new(i + 1);
        *n = (previous.iter().zip(m).map(|(a, b)| a * b).sum::<i64>() % 10).abs();
    }
}

struct Multipliers {
    phase: usize,
    n: usize,
    base: usize,
}

impl Multipliers {
    fn new(phase: usize) -> Multipliers {
        Multipliers {
            phase,
            n: 0,
            base: 0,
        }
    }
}

impl Iterator for Multipliers {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        self.n += 1;

        if self.n == self.phase {
            self.n = 0;
            self.base = (self.base + 1) % 4;
        }

        match self.base {
            0 | 2 => Some(0),
            1 => Some(1),
            3 => Some(-1),
            x => panic!("unknown base: {}", x),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multipliers_test() {
        let m = Multipliers::new(2);
        let vals: Vec<i64> = m.take(16).collect();

        assert_eq!(
            vals,
            vec![0, 1, 1, 0, 0, -1, -1, 0, 0, 1, 1, 0, 0, -1, -1, 0]
        );
    }

    #[test]
    fn next_phase_test() {
        let mut digits = vec![1, 2, 3, 4, 5, 6, 7, 8];
        next_phase(&mut digits);
        assert_eq!(digits, vec![4, 8, 2, 2, 6, 1, 5, 8], "phase 1");
        next_phase(&mut digits);
        assert_eq!(digits, vec![3, 4, 0, 4, 0, 4, 3, 8], "phase 2");
        next_phase(&mut digits);
        assert_eq!(digits, vec![0, 3, 4, 1, 5, 5, 1, 8], "phase 3");
        next_phase(&mut digits);
        assert_eq!(digits, vec![0, 1, 0, 2, 9, 4, 9, 8], "phase 4");
    }

    #[test]
    fn hundred_phases_test() {
        assert_eq!(
            hundred_phases(&"80871224585914546619083218645595"),
            "24176176",
        );
    }
}
