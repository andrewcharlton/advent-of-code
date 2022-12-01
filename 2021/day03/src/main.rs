use std::fs;

fn main() {
    let file: String = fs::read_to_string("input.txt").expect("couldn't open file");
    let lines: Vec<&str> = file.lines().collect();

    let gamma = power_consumption(&lines, false);
    let epsilon = power_consumption(&lines, true);
    println!("Part one: {}", gamma * epsilon);

    let oxygen = life_support_rating(&lines, false);
    let co2 = life_support_rating(&lines, true);
    println!("Part two: {}", oxygen * co2);
}

fn power_consumption(lines: &Vec<&str>, least: bool) -> usize {
    let cols: usize = lines.get(0).unwrap().len();
    let mut n: usize = 0;

    for col in 0..cols {
        n *= 2;
        if most_common(lines, "", col, least) == '1' {
            n += 1
        };
    }

    n
}

fn life_support_rating(lines: &Vec<&str>, least: bool) -> usize {
    let cols: usize = lines.get(0).unwrap().len();
    let mut n: usize = 0;
    let mut prefix: String = String::from("");

    for col in 0..cols {
        n *= 2;
        let c = most_common(lines, &prefix, col, least);
        if c == '1' {
            n += 1
        };
        prefix.push(c);
    }

    n
}

fn most_common(lines: &Vec<&str>, prefix: &str, col: usize, least: bool) -> char {
    let counts: Vec<i64> = lines
        .iter()
        .filter(|line| line.starts_with(prefix))
        .map(|line| {
            let c = line.trim().chars().nth(col).unwrap();
            match c {
                '1' => 1,
                '0' => -1,
                _ => panic!("Unknown character"),
            }
        })
        .collect();

    let len = counts.len();
    let sum: i64 = counts.iter().sum();

    if len == 1 {
        if sum > 0 {
            return '1';
        } else {
            return '0';
        }
    };

    if (sum >= 0 && !least) || (sum < 0 && least) {
        '1'
    } else {
        '0'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let file: String = fs::read_to_string("example.txt").expect("couldn't open file");
        let lines: Vec<&str> = file.lines().collect();

        assert_eq!(power_consumption(&lines, false), 22, "gamma rate");
        assert_eq!(power_consumption(&lines, true), 9, "epsilon rate");

        assert_eq!(life_support_rating(&lines, false), 23, "oxygen");
        assert_eq!(life_support_rating(&lines, true), 10, "co2");
    }
}
