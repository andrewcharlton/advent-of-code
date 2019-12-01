use std::cmp::max;
use std::fs;

fn main() {
    let masses: Vec<i64> = fs::read_to_string("input")
        .expect("couldn't open file")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let fuel_sum: i64 = masses.iter().map(|&mass| fuel_needed(mass)).sum();
    println!("Part one: {}", fuel_sum);

    let fuel_sum: i64 = masses.iter().map(|&mass| fuel_needed_total(mass)).sum();
    println!("Part two: {}", fuel_sum);
}

fn fuel_needed(mass: i64) -> i64 {
    max(mass / 3 - 2, 0)
}

fn fuel_needed_total(mass: i64) -> i64 {
    match fuel_needed(mass) {
        0 => 0,
        fuel => fuel + fuel_needed_total(fuel),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuel_needed_test() {
        assert_eq!(fuel_needed(14), 2);
        assert_eq!(fuel_needed(12), 2);
        assert_eq!(fuel_needed(1969), 654);
        assert_eq!(fuel_needed(100756), 33583);
        assert_eq!(fuel_needed(5), 0);
    }

    #[test]
    fn fuel_needed_rec_test() {
        assert_eq!(fuel_needed_total(14), 2);
        assert_eq!(fuel_needed_total(1969), 966);
        assert_eq!(fuel_needed_total(100756), 50346);
    }
}
