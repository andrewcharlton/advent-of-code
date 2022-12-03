use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}

fn part_one() -> u32 {
    INPUT
        .lines()
        .map(|line| {
            let n = line.len() / 2;
            let a = line[..n].to_owned();
            let b = line[n..].to_owned();
            priority(intersection(vec![&a, &b]))
        })
        .sum()
}

fn part_two() -> u32 {
    let lines: Vec<&str> = INPUT.lines().collect();

    lines
        .chunks_exact(3)
        .map(|chunk| priority(intersection(chunk.to_vec())))
        .sum()
}

fn intersection(lines: Vec<&str>) -> char {
    lines
        .into_iter()
        .map(|line| line.chars().collect::<HashSet<char>>())
        .reduce(|acc, set| acc.intersection(&set).copied().collect())
        .unwrap()
        .iter()
        .next()
        .unwrap()
        .clone()
}

fn priority(c: char) -> u32 {
    let c = c as u32;
    match c {
        // Lower case
        97..=122 => c - 96,

        // Upper case
        65..=90 => c - 38,

        _ => panic!("unknown character code {}", c),
    }
}
