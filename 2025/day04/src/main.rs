use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("couldn't read input file");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &str) -> usize {
    let rolls = parse_input(input);
    find_removal_candidates(&rolls).len()
}

fn part_two(input: &str) -> usize {
    let mut rolls = parse_input(input);
    let mut total = 0;

    loop {
        let to_remove = find_removal_candidates(&rolls);
        if to_remove.len() == 0 {
            break;
        }

        total += to_remove.len();

        for pos in to_remove {
            rolls.remove(&pos);
        }
    }

    total
}

fn find_removal_candidates(rolls: &HashSet<(i64, i64)>) -> Vec<(i64, i64)> {
    rolls
        .iter()
        .filter_map(|(x, y)| {
            let mut neighbours = 0;
            for x_delta in -1..=1 {
                for y_delta in -1..=1 {
                    if rolls.contains(&(x + x_delta, y + y_delta)) {
                        neighbours += 1;
                    }
                }
            }

            if neighbours < 5 {
                Some((*x, *y))
            } else {
                None
            }
        })
        .collect()
}

fn parse_input(s: &str) -> HashSet<(i64, i64)> {
    s.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '@' {
                    Some((x as i64, y as i64))
                } else {
                    None
                }
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = fs::read_to_string("example.txt").expect("couldn't read example file");
        assert_eq!(part_one(&input), 13);
        assert_eq!(part_two(&input), 43);
    }
}
