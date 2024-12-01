use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

fn part_one(input: &str) -> u64 {
    let (mut left, mut right) = parse_input(input);
    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

fn part_two(input: &str) -> u64 {
    let (mut left, right) = parse_input(input);
    left.sort();

    let mut counts: HashMap<u64, u64> = HashMap::new();
    for x in right {
        if let Some(v) = counts.get_mut(&x) {
            *v += 1;
        } else {
            counts.insert(x, 1);
        }
    }

    left.iter().map(|x| x * counts.get(&x).unwrap_or(&0)).sum()
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let left = parts.next().unwrap().parse::<u64>().unwrap();
            let right = parts.next().unwrap().parse::<u64>().unwrap();
            (left, right)
        })
        .unzip()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(part_one(EXAMPLE), 11);
        assert_eq!(part_two(EXAMPLE), 31);
    }
}
