use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("couldn't read input file");
    println!("Part one: {}", solve(&input, 2));
    println!("Part two: {}", solve(&input, 12));
}

fn solve(s: &str, battery_size: usize) -> u64 {
    s.lines().map(|line| max_joltage(line, battery_size)).sum()
}

fn max_joltage(s: &str, battery_size: usize) -> u64 {
    let digits: Vec<u32> = s.trim().chars().map(|c| c.to_digit(10).unwrap()).collect();

    let mut sum: u64 = 0;
    let mut start_pos = 0;

    for i in 0..battery_size {
        let (v, pos) = max_digit(&digits, start_pos, digits.len() - battery_size + 1 + i);
        sum = sum * 10 + (v as u64);
        start_pos = pos + 1;
    }

    sum
}

fn max_digit(digits: &[u32], start: usize, end: usize) -> (u32, usize) {
    let mut max: u32 = 0;
    let mut max_pos: usize = 0;

    for (i, v) in digits.iter().enumerate() {
        if i >= end {
            break;
        }
        if i < start {
            continue;
        }

        if *v > max {
            max = *v;
            max_pos = i;
        }
    }

    (max, max_pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = fs::read_to_string("example.txt").expect("couldn't read example file");
        assert_eq!(solve(&input, 2), 357);
        assert_eq!(solve(&input, 12), 3121910778619);
    }
}
