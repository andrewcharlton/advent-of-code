use std::collections::HashMap;
use std::fs;

fn main() {
    println!("Part one: {}", unique_segments("input.txt"));
    println!("Part two: {}", sum("input.txt"));
}

fn unique_segments(filename: &str) -> usize {
    fs::read_to_string(filename)
        .expect("couldn't open file")
        .lines()
        .map(|line| line.split_once('|').unwrap().1)
        .map(|line| line.trim().split(' '))
        .flatten()
        .filter(|pattern| {
            let l = pattern.len();
            l == 2 || l == 3 || l == 4 || l == 7
        })
        .count()
}

fn sum(filename: &str) -> u64 {
    fs::read_to_string(filename)
        .expect("couldn't open file")
        .lines()
        .map(|line| {
            let (input, output) = line.split_once("|").unwrap();
            let values = decode(input);
            output.trim().split(" ").fold(0, |acc, code| {
                let code = sort_string(code);
                10 * acc + values.get(&code).unwrap()
            })
        })
        .sum()
}

fn decode(input: &str) -> HashMap<String, u64> {
    let mut map: HashMap<String, u64> = HashMap::new();
    let codes: Vec<&str> = input.trim().split(' ').collect();

    // Work out the easy ones first, with unique lengths.
    let mut one: &str = "";
    let mut four: &str = "";
    for code in &codes {
        match code.len() {
            2 => {
                one = code;
                map.insert(sort_string(code), 1)
            }
            3 => map.insert(sort_string(code), 7),
            4 => {
                four = code;
                map.insert(sort_string(code), 4)
            }
            7 => map.insert(sort_string(code), 8),
            _ => None,
        };
    }

    // Now we can eliminate the length 6:
    // 6 is the only length 6 one that doesn't contain 1.
    // 9 is the only length 9 one that contains 4.
    // 0 is the other one.
    let mut nine: &str = "";
    for code in &codes {
        if code.len() != 6 {
            continue;
        }
        if !contains(code, one) {
            map.insert(sort_string(code), 6);
        } else if contains(code, four) {
            nine = code;
            map.insert(sort_string(code), 9);
        } else {
            map.insert(sort_string(code), 0);
        }
    }

    // Finally, we can eliminate the length 5 codes.
    // 3 is the only one that contains 1.
    // 5 is contained within 9 (as is 3), 2 isn't.
    for code in &codes {
        if code.len() != 5 {
            continue;
        }

        if contains(code, one) {
            map.insert(sort_string(code), 3);
        } else if contains(nine, code) {
            map.insert(sort_string(code), 5);
        } else {
            map.insert(sort_string(code), 2);
        }
    }

    map
}

fn contains(code: &str, sub: &str) -> bool {
    sub.chars().find(|c| !code.contains(*c)).is_none()
}

fn sort_string(code: &str) -> String {
    let mut chars: Vec<char> = code.chars().collect();
    chars.sort();
    String::from_iter(chars)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(unique_segments("example.txt"), 26, "Part one");
        assert_eq!(sum("example.txt"), 61229, "Part two");
    }
}
