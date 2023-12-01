const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", calibration_sum(INPUT));
    println!("Part two: {}", calibration_sum_words(INPUT));
}

fn calibration_sum(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            10 * digits.first().unwrap() + digits.last().unwrap()
        })
        .sum()
}

fn calibration_sum_words(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            // Replace the words, but we need to be careful of possible overlaps so we leave some
            // additional letters in play.
            let line = line
                .replace("one", "o1e") // the o can be the end of two, and the e can start eight
                .replace("two", "t2o") // the t can end eight and the o can start one
                .replace("three", "t3e") // the t can end eight and the e can start eight
                .replace("four", "4") // nothing finishes with f or starts with r
                .replace("five", "5e") // nothing finishes with f byt e starts eight
                .replace("six", "6") // nothing finishes with s or starts with x
                .replace("seven", "7n") // nothing finishes with s, but nine starts with n
                .replace("eight", "e8t") // nine ends with e and two/three start with t
                .replace("nine", "n9e"); // seven ends in n, and eight starts with e

            let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            10 * digits.first().unwrap() + digits.last().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const EXAMPLE2: &str = include_str!("../example2.txt");

    #[test]
    fn example() {
        assert_eq!(calibration_sum(EXAMPLE), 142);
        assert_eq!(calibration_sum_words(EXAMPLE2), 281);
    }
}
