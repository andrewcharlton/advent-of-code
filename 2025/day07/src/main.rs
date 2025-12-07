use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("couldn't read input file");
    let (part_one, part_two) = solve(&input);
    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}

fn solve(input: &str) -> (usize, usize) {
    let mut lines: Vec<&str> = input.lines().collect();

    let mut beams: HashMap<usize, usize> = lines
        .remove(0)
        .chars()
        .enumerate()
        .filter_map(|(i, c)| if c == 'S' { Some((i, 1)) } else { None })
        .collect();

    let mut splits = 0;

    for line in lines {
        let splitters: HashSet<usize> = line
            .chars()
            .enumerate()
            .filter_map(|(i, c)| if c == '^' { Some(i) } else { None })
            .collect();

        if splitters.len() == 0 {
            continue;
        }

        let mut new_beams: HashMap<usize, usize> = HashMap::new();

        for (pos, count) in beams.iter() {
            if splitters.contains(pos) {
                splits += 1;

                let n = new_beams.get(&(pos - 1)).unwrap_or(&0);
                new_beams.insert(pos - 1, n + count);

                let n = new_beams.get(&(pos + 1)).unwrap_or(&0);
                new_beams.insert(pos + 1, n + count);
            } else {
                let n = new_beams.get(&pos).unwrap_or(&0);
                new_beams.insert(*pos, n + count);
            }
        }

        beams = new_beams;
    }

    (splits, beams.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = fs::read_to_string("example.txt").expect("couldn't read example file");
        let (part_one, part_two) = solve(&input);
        assert_eq!(part_one, 21);
        assert_eq!(part_two, 40);
    }
}
