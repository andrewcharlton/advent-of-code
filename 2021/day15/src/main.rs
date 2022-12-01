use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let cavern = parse_input("input.txt");
    println!("Part one: {}", minimal_path(&cavern));

    let expanded_cavern = expand_cavern("input.txt");
    println!("Part two: {}", minimal_path(&expanded_cavern));
}

type Coordinate = (usize, usize);

fn minimal_path(cavern: &Vec<Vec<usize>>) -> usize {
    let mut visited: HashSet<Coordinate> = HashSet::new();
    let mut scores: HashMap<Coordinate, usize> = HashMap::new();
    scores.insert((0, 0), 0);

    let height = cavern.len();
    let width = cavern.first().unwrap().len();

    loop {
        // Find the location with the lowest current risk.
        let mut least = usize::MAX;
        let mut next_point: Coordinate = (0, 0);
        for (&pt, &score) in &scores {
            if score < least {
                least = score;
                next_point = pt;
            }
        }
        visited.insert(next_point);
        scores.remove(&next_point);

        // If we've found our way to the end, return.
        if next_point.0 == width - 1 && next_point.1 == height - 1 {
            return least;
        }

        // Find its valid neighbours
        let mut neighbours: Vec<Coordinate> = Vec::new();
        if next_point.0 > 0 {
            neighbours.push((next_point.0 - 1, next_point.1));
        }
        if next_point.0 < width - 1 {
            neighbours.push((next_point.0 + 1, next_point.1));
        }
        if next_point.1 > 0 {
            neighbours.push((next_point.0, next_point.1 - 1));
        }
        if next_point.1 < height - 1 {
            neighbours.push((next_point.0, next_point.1 + 1));
        }

        // Update the neighbours with
        for n in neighbours {
            if visited.contains(&n) {
                continue;
            }

            let risk = least + cavern.get(n.1).unwrap().get(n.0).unwrap();
            if let Some(x) = scores.get_mut(&n) {
                if risk < *x {
                    *x = risk;
                }
            } else {
                scores.insert(n, risk);
            }
        }
    }
}

fn parse_input(filename: &str) -> Vec<Vec<usize>> {
    let file = fs::read_to_string(filename).expect("couldn't open file");

    file.lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect()
}

fn expand_cavern(filename: &str) -> Vec<Vec<usize>> {
    let original = parse_input(filename);

    let mut expanded: Vec<Vec<usize>> = Vec::new();
    for y in 0..5 {
        for line in &original {
            let mut expanded_line = Vec::new();
            for x in 0..5 {
                let new_line: Vec<usize> = line.iter().map(|v| (v + x + y - 1) % 9 + 1).collect();
                expanded_line.extend_from_slice(&new_line);
            }
            expanded.push(expanded_line);
        }
    }

    expanded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let cavern = parse_input("example.txt");
        assert_eq!(minimal_path(&cavern), 40, "Part one");

        let expanded_cavern = expand_cavern("example.txt");
        assert_eq!(minimal_path(&expanded_cavern), 315, "Part two");
    }
}
