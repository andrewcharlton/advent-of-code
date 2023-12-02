use regex::Regex;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref GAME: Regex = Regex::new(r"Game (\d+):").unwrap();
    static ref RED: Regex = Regex::new(r"\s(\d+) red").unwrap();
    static ref BLUE: Regex = Regex::new(r"\s(\d+) blue").unwrap();
    static ref GREEN: Regex = Regex::new(r"\s(\d+) green").unwrap();
    static ref REGEXES: Vec<(&'static Regex, usize)> = vec![(&RED, 12), (&GREEN, 13), (&BLUE, 14)];
}

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", solve_possible(INPUT));
    println!("Part two: {}", solve_power(INPUT));
}

fn solve_possible(s: &str) -> usize {
    s.lines().filter_map(|line| game_possible(line)).sum()
}

fn solve_power(s: &str) -> usize {
    s.lines().map(|line| game_power(line)).sum()
}

fn game_possible(s: &str) -> Option<usize> {
    for (re, max) in REGEXES.iter() {
        if max_colour_match(re, s) > *max {
            return None;
        }
    }

    GAME.captures(s)
        .map(|cap| cap.get(1).unwrap().as_str().parse().unwrap())
}

fn game_power(s: &str) -> usize {
    REGEXES
        .iter()
        .map(|(re, _)| max_colour_match(re, s))
        .product()
}

fn max_colour_match(re: &Regex, s: &str) -> usize {
    re.captures_iter(s)
        .map(|cap| cap.get(1).unwrap().as_str().parse().unwrap())
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(solve_possible(EXAMPLE), 8);
        assert_eq!(solve_power(EXAMPLE), 2286);
    }
}
