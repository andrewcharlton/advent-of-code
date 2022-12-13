use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two:\n{}", part_two(INPUT).join("\n"));
}

fn part_one(input: &str) -> isize {
    parse_input(input)
        .iter()
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|(i, n)| {
            println!("{} x {}", i + 1, n); // i+1 because the cycle's are 1-indexed.
            ((i + 1) as isize) * n
        })
        .sum()
}

fn part_two(input: &str) -> Vec<String> {
    parse_input(input)
        .chunks(40)
        .filter_map(|chunk| {
            let visible: HashSet<isize> = chunk
                .iter()
                .enumerate()
                .filter_map(|(i, &s)| {
                    let i: isize = i as isize;
                    if s >= i - 1 && s <= i + 1 {
                        Some(i)
                    } else {
                        None
                    }
                })
                .collect();

            if visible.len() == 0 {
                return None;
            }

            let mut s: String = String::new();
            for i in 0..40 {
                if visible.contains(&i) {
                    s.push('#');
                } else {
                    s.push(' ');
                }
            }
            Some(s)
        })
        .collect()
}

fn parse_input(input: &str) -> Vec<isize> {
    let changes: Vec<isize> = input
        .lines()
        .map(|line| match line {
            "noop" => vec![0],
            s if s.starts_with("addx ") => {
                let n = s.strip_prefix("addx ").unwrap().parse::<isize>().unwrap();
                vec![0, n]
            }
            _ => panic!("invalid line: {}", line),
        })
        .flatten()
        .scan(1, |state, n| {
            *state = *state + n;
            Some(*state)
        })
        .collect();

    let mut crt = vec![1];
    crt.extend(changes);
    crt
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const EXAMPLE2: &str = include_str!("../example2.txt");
    const RESULT: &str = include_str!("../part2_example_result.txt");

    #[test]
    fn test_parsing() {
        assert_eq!(parse_input(EXAMPLE), vec![1, 1, 1, 4, 4, -1]);
    }

    #[test]
    fn test_example() {
        assert_eq!(part_one(EXAMPLE2), 13140);

        let p2_result = part_two(EXAMPLE2).join("\n");
        assert_eq!(p2_result.trim(), RESULT.trim());
    }
}
