use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Couldn't read file");
    let input: Vec<&str> = input.lines().collect();

    let answer: u64 = input.iter().map(|line| wrapping_required(line)).sum();
    println!("Part one: {}", answer);

    let answer: u64 = input.iter().map(|line| ribbon_required(line)).sum();
    println!("Part two: {}", answer);
}

fn wrapping_required(input: &str) -> u64 {
    let dimensions = parse_dimensions(input);

    let a = dimensions[0] * dimensions[1];
    let b = dimensions[0] * dimensions[2];
    let c = dimensions[1] * dimensions[2];

    2 * (a + b + c) + vec![a, b, c].iter().min().unwrap()
}

fn ribbon_required(input: &str) -> u64 {
    let mut dimensions = parse_dimensions(input);
    dimensions.sort();

    2 * (dimensions[0] + dimensions[1]) + dimensions[0] * dimensions[1] * dimensions[2]
}

fn parse_dimensions(input: &str) -> Vec<u64> {
    input.split('x').filter_map(|d| d.parse().ok()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrapping_required_test() {
        assert_eq!(wrapping_required("2x3x4"), 58);
        assert_eq!(wrapping_required("1x1x10"), 43);
    }

    #[test]
    fn ribbon_required_test() {
        assert_eq!(ribbon_required("2x3x4"), 34);
        assert_eq!(ribbon_required("1x1x10"), 14);
    }
}
