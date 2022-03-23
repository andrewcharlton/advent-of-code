use std::collections::HashSet;
use std::fs;

fn main() {
    let input: String = fs::read_to_string("input").expect("couldn't open file");
    let groups: Vec<&str> = input.split("\n\n").collect();

    let unique: usize = groups.iter().map(|&group| unique_answers(group)).sum();
    println!("Part one: {}", unique);

    let joint: usize = groups.iter().map(|&group| joint_answers(group)).sum();
    println!("Part two: {}", joint);
}

fn unique_answers(group: &str) -> usize {
    group
        .chars()
        .filter(|&c| c != '\n')
        .collect::<HashSet<char>>()
        .len()
}

fn joint_answers(group: &str) -> usize {
    let mut lines: Vec<&str> = group.lines().collect();

    let mut answered: HashSet<char> = lines.pop().unwrap().chars().collect();
    for line in lines {
        answered.retain(|&c| line.contains(c))
    }

    answered.len()
}
