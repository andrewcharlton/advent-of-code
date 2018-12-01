use std::collections::HashSet;
use std::fs;

fn main() {
    let changes = fs::read_to_string("input").expect("Couldn't open file");
    let changes = parse_file(changes);

    println!("Part one: {}", file_sum(&changes));
    println!("Part two: {}", repeat_sum(&changes));
}

fn parse_file(contents: String) -> Vec<i64> {
    contents.lines().map(|line| line.parse().unwrap()).collect()
}

fn file_sum(changes: &Vec<i64>) -> i64 {
    changes.iter().fold(0, |acc, x| acc + x)
}

fn repeat_sum(changes: &Vec<i64>) -> i64 {
    let mut results_seen = HashSet::new();

    let mut acc = 0;
    loop {
        for v in changes.iter() {
            if results_seen.contains(&acc) {
                return acc;
            };
            results_seen.insert(acc);
            acc = acc + v;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_file_test() {
        assert_eq!(
            parse_file(String::from("+1\n-2\n+3\n-5")),
            vec![1, -2, 3, -5]
        );
    }

    #[test]
    fn file_sum_test() {
        assert_eq!(file_sum(&vec![1, -2, 3, 1]), 3);
        assert_eq!(file_sum(&vec![1, 1, 1]), 3);
        assert_eq!(file_sum(&vec![1, 1, -2]), 0);
        assert_eq!(file_sum(&vec![-1, -2, -3]), -6);
    }

    #[test]
    fn repeat_sum_test() {
        assert_eq!(repeat_sum(&vec![1, -2, 3, 1, 1, -2]), 2);
        assert_eq!(repeat_sum(&vec![1, -1]), 0);
        assert_eq!(repeat_sum(&vec![-6, 3, 8, 5, -6]), 5);
    }
}
