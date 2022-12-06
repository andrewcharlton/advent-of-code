use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", solve(INPUT, 4));
    println!("Part two: {}", solve(INPUT, 14));
}

fn solve(s: &str, n: usize) -> usize {
    let chars: Vec<char> = s.chars().collect();
    chars
        .windows(n)
        .enumerate()
        .find(|(_, chars)| chars.iter().copied().collect::<HashSet<char>>().len() == n)
        .map(|(i, _)| i + n)
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_examples() {
        assert_eq!(solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
        assert_eq!(solve("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(solve("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    #[test]
    fn part_two_examples() {
        assert_eq!(solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(solve("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(solve("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }
}
