use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

fn part_one(input: &str) -> usize {
    input.lines().map(|line| Card::parse(line).score()).sum()
}

fn part_two(input: &str) -> usize {
    let mut counts: HashMap<usize, usize> = HashMap::new();
    let mut total = 0;

    for line in input.lines() {
        let card = Card::parse(line);
        let score = card.matches();
        let copies = 1 + counts.get(&card.num).unwrap_or(&0);
        total += copies;

        for i in card.num + 1..=card.num + score {
            if let Some(x) = counts.get_mut(&i) {
                *x += copies;
            } else {
                counts.insert(i, copies);
            }
        }
    }

    total
}

struct Card {
    num: usize,
    winners: HashSet<u32>,
    candidates: HashSet<u32>,
}

impl Card {
    fn parse(s: &str) -> Card {
        let (game, nums) = s.split_once(":").unwrap();
        let (winners, candidates) = nums.split_once("|").unwrap();

        Card {
            num: game
                .strip_prefix("Card ")
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap(),
            winners: winners
                .trim()
                .split(" ")
                .filter(|d| !d.is_empty())
                .map(|d| d.parse::<u32>().unwrap())
                .collect(),
            candidates: candidates
                .trim()
                .split(" ")
                .filter(|d| !d.is_empty())
                .map(|d| d.trim().parse::<u32>().unwrap())
                .collect(),
        }
    }

    fn score(&self) -> usize {
        let m = self.matches();
        if m == 0 {
            0
        } else {
            1 << m - 1
        }
    }

    fn matches(&self) -> usize {
        self.winners.intersection(&self.candidates).count()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(part_one(EXAMPLE), 13);
        assert_eq!(part_two(EXAMPLE), 30);
    }
}
