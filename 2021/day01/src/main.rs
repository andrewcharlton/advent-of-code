use std::fs;

fn main() {
    let lines: Vec<i32> = fs::read_to_string("input")
        .expect("couldn't open file")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    println!("Part one: {}", count_increases(&lines, 1));
    println!("Part two: {}", count_increases(&lines, 3));
}

// count_increases counts the number
fn count_increases(measurements: &[i32], offset: usize) -> usize {
    measurements
        .iter()
        .zip(measurements.iter().skip(offset))
        .filter(|(a, b)| a < b)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let values = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_increases(&values, 1), 7, "part one");
        assert_eq!(count_increases(&values, 3), 5, "part two");
    }
}
