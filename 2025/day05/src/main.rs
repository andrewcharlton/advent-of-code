use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("couldn't read input file");
    let (part_one, part_two) = solve(&input);
    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}

fn solve(input: &str) -> (usize, u64) {
    let (mut ranges, ingredients) = parse_input(input);
    let ranges = merge_ranges(&mut ranges);

    let part_one = ingredients
        .iter()
        .filter(|ingredient| is_fresh(&ranges, **ingredient))
        .count();

    let part_two: u64 = ranges.iter().map(|(min, max)| *max + 1 - *min).sum();

    (part_one, part_two)
}

fn merge_ranges(ranges: &mut Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    ranges.sort();

    let mut merged = Vec::new();
    let mut min: Option<u64> = None;
    let mut max: Option<u64> = None;

    for (x, y) in ranges {
        if min.is_none() && max.is_none() {
            min = Some(*x);
            max = Some(*y);
            continue;
        }

        if *x <= max.unwrap() {
            // The ranges overlap, so extend it, if needed.
            if *y >= max.unwrap() {
                max = Some(*y);
            }
            continue;
        }

        // Otherwise, we're no longer overlapping so commit this range and start a new one.
        merged.push((min.unwrap(), max.unwrap()));
        min = Some(*x);
        max = Some(*y);
    }
    merged.push((min.unwrap(), max.unwrap()));

    merged
}

fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();

    let ranges = ranges
        .lines()
        .map(|line| {
            let (min, max) = line.split_once("-").unwrap();
            let min: u64 = min.parse().unwrap();
            let max: u64 = max.parse().unwrap();
            (min, max)
        })
        .collect();

    let ingredients = ingredients
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    (ranges, ingredients)
}

fn is_fresh(ranges: &Vec<(u64, u64)>, ingredient: u64) -> bool {
    for (min, max) in ranges {
        if ingredient >= *min && ingredient <= *max {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = fs::read_to_string("example.txt").expect("couldn't read example file");
        let (part_one, part_two) = solve(&input);
        assert_eq!(part_one, 3);
        assert_eq!(part_two, 14);
    }
}
