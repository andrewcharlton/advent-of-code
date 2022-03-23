use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("couldn't read file");

    let target = first_invalid(&input, 25);
    println!("Part one: {}", target);
    println!("Part two: {}", contiguous_range(&input, target));
}

fn first_invalid(input: &str, preamble_size: usize) -> usize {
    let codes: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();

    for i in preamble_size..codes.len() {
        if !is_valid(&codes, i, preamble_size) {
            return *codes.get(i).unwrap();
        }
    }

    0
}

fn is_valid(codes: &Vec<usize>, i: usize, preamble_size: usize) -> bool {
    let target = *codes.get(i).unwrap();

    for j in i - preamble_size..i {
        let first = *codes.get(j).unwrap();
        for k in j + 1..i {
            if *codes.get(k).unwrap() + first == target {
                return true;
            }
        }
    }

    false
}

fn contiguous_range(input: &str, target: usize) -> usize {
    let codes: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();

    for i in 0..codes.len() {
        if let Some(j) = contiguous_range_from(&codes, target, i) {
            let mut min = *codes.get(i).unwrap();
            let mut max = *codes.get(i).unwrap();
            for k in i..j + 1 {
                let x = *codes.get(k).unwrap();
                if x < min {
                    min = x;
                }
                if x > max {
                    max = x;
                }
            }

            return min + max;
        }
    }

    0
}

fn contiguous_range_from(codes: &Vec<usize>, target: usize, i: usize) -> Option<usize> {
    let mut j = i;
    let mut sum = *codes.get(j).unwrap();

    loop {
        j += 1;
        sum += *codes.get(j).unwrap();

        if sum > target {
            return None;
        }

        if sum == target {
            return Some(j);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn first_invalid_test() {
        assert_eq!(first_invalid(INPUT, 5), 127);
    }

    #[test]
    fn contiguous_range_test() {
        assert_eq!(contiguous_range(INPUT, 127), 62);
    }
}
