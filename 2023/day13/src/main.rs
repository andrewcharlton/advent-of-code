const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", solve(INPUT, 0));
    println!("Part two: {}", solve(INPUT, 1));
}

fn solve(input: &str, diff_allowed: usize) -> usize {
    input
        .split("\n\n")
        .enumerate()
        .filter_map(|(i, pattern)| {
            let ans = solve_pattern(pattern, diff_allowed);
            if ans.is_none() {
                println!("Pattern {} has no solution", i + 1);
                println!("{}", pattern);
                println!("--\n{}\n", transpose(pattern));
            }
            ans
        })
        .sum()
}

fn solve_pattern(pattern: &str, diff_allowed: usize) -> Option<usize> {
    if let Some(rows) = find_mirror_line(pattern.lines().collect(), diff_allowed) {
        return Some(100 * rows);
    }

    let pattern = transpose(pattern);
    if let Some(columns) = find_mirror_line(pattern.lines().collect(), diff_allowed) {
        return Some(columns);
    }

    None
}

fn find_mirror_line(lines: Vec<&str>, diff_allowed: usize) -> Option<usize> {
    'outer: for i in 0..lines.len() - 1 {
        let mut total_diff = 0;

        let mut d = 0;
        loop {
            if d > i || d + i + 1 >= lines.len() {
                if total_diff == diff_allowed {
                    return Some(i + 1);
                }
                continue 'outer;
            }

            total_diff += diff(lines.get(i - d).unwrap(), lines.get(i + d + 1).unwrap());
            if total_diff > diff_allowed {
                continue 'outer;
            }

            d += 1;
        }
    }

    None
}

fn transpose(s: &str) -> String {
    let mut output: Vec<String> = Vec::new();

    for line in s.lines() {
        for (i, c) in line.chars().enumerate() {
            if let Some(v) = output.get_mut(i) {
                v.push(c);
            } else {
                output.push(String::from(c));
            }
        }
    }

    output.join("\n")
}

fn diff(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).filter(|(a, b)| a != b).count()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example_pt1() {
        let (pattern_0, pattern_1) = EXAMPLE.split_once("\n\n").unwrap();

        assert_eq!(solve_pattern(pattern_0, 0), Some(5));
        assert_eq!(solve_pattern(pattern_1, 0), Some(400));
    }

    #[test]
    fn example_pt2() {
        let (pattern_0, pattern_1) = EXAMPLE.split_once("\n\n").unwrap();

        assert_eq!(solve_pattern(pattern_0, 1), Some(300));
        assert_eq!(solve_pattern(pattern_1, 1), Some(100));
    }

    #[test]
    fn test_transpose() {
        let original = "123\n456\n789";
        let exp = "147\n258\n369";
        assert_eq!(transpose(original), exp);
    }

    #[test]
    fn test_diff() {
        let a = "123456789";
        let b = "123!5678!";
        assert_eq!(diff(a, b), 2);
    }
}
