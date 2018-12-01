use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part one: {}", line_sum(&input, line_diff));
    println!("Part two: {}", line_sum(&input, line_divisor));
}

fn line_sum(input: &String, f: fn(Vec<u32>) -> u32) -> u32 {
    input
        .lines()
        .map(parse_line)
        .fold(0, |acc, line| acc + f(line))
}

fn line_diff(nums: Vec<u32>) -> u32 {
    nums.iter().max().unwrap() - nums.iter().min().unwrap()
}

fn line_divisor(nums: Vec<u32>) -> u32 {
    for (i, x) in nums.iter().enumerate() {
        for (j, y) in nums.iter().enumerate() {
            if i != j && x % y == 0 {
                return x / y;
            }
        }
    }
    0
}

fn parse_line(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .filter_map(|word| word.parse::<u32>().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_test() {
        assert_eq!(parse_line("5806	6444	1281	38"), vec![5806, 6444, 1281, 38]);
    }

    #[test]
    fn line_diff_sum_test() {
        assert_eq!(
            line_sum(&String::from("5\t1\t9\t5\n7\t5\t3\n2\t4\t6\t8"), line_diff),
            18
        );
    }

    #[test]
    fn line_divisor_sum_test() {
        assert_eq!(
            line_sum(
                &String::from("5\t9\t2\t8\n9\t4\t7\t3\n3\t8\t6\t5"),
                line_divisor
            ),
            9
        );
    }

}
