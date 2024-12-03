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
    section_score(input)
}

fn part_two(input: &str) -> i64 {
    input
        .split("do()")
        .map(|section| section.split("don't()").next().unwrap())
        .map(|section| section_score(section))
        .sum()
}

fn section_score(input: &str) -> i64 {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap());
    RE.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [a, b])| a.parse::<i64>().unwrap() * b.parse::<i64>().unwrap())
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const EXAMPLE2: &str = include_str!("../example2.txt");

    #[test]
    fn example() {
        assert_eq!(part_one(EXAMPLE), 161);
        assert_eq!(part_two(EXAMPLE2), 48);
    }
}
