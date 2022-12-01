use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    println!("Part one: {}", unique_paths("input.txt", false));
    println!("Part two: {}", unique_paths("input.txt", true));
}

fn unique_paths(filename: &str, second_visit: bool) -> usize {
    let file: &str = &fs::read_to_string(filename).expect("couldn't open file");
    let pairs: Vec<(&str, &str)> = file
        .lines()
        .map(|line| line.trim().split_once("-").unwrap())
        .collect();

    let mut paths: HashMap<&str, Vec<&str>> = HashMap::new();
    for (a, b) in pairs {
        if let Some(list) = paths.get_mut(a) {
            list.push(b);
        } else {
            let list = vec![b];
            paths.insert(a, list);
        }

        if let Some(list) = paths.get_mut(b) {
            list.push(a);
        } else {
            let list = vec![a];
            paths.insert(b, list);
        }
    }

    let mut visited: HashSet<&str> = HashSet::new();
    visited.insert("start");

    return unique_paths_rec(&paths, "start", &visited, second_visit, false, "start");
}

fn unique_paths_rec(
    paths: &HashMap<&str, Vec<&str>>,
    current: &str,
    visited: &HashSet<&str>,
    second_visit: bool,
    second_visit_taken: bool,
    journey: &str,
) -> usize {
    if current == "end" {
        return 1;
    }

    let mut sum = 0;
    for &dest in paths.get(current).unwrap_or(&vec![]) {
        if dest == "start" {
            continue;
        }

        if dest.to_lowercase() != dest {
            let mut journey = journey.to_owned();
            journey.push_str(",");
            journey.push_str(dest);
            sum += unique_paths_rec(
                paths,
                dest,
                &visited,
                second_visit,
                second_visit_taken,
                &journey,
            );
            continue;
        }

        if visited.contains(dest) && (!second_visit || second_visit_taken) {
            continue;
        }
        let taken = second_visit_taken || visited.contains(dest);

        let mut visited = visited.clone();
        visited.insert(dest);

        let mut journey = journey.to_owned();
        journey.push_str(",");
        journey.push_str(dest);
        sum += unique_paths_rec(paths, dest, &visited, second_visit, taken, &journey);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_visit() {
        assert_eq!(unique_paths("example1.txt", false), 10, "example 1");
        assert_eq!(unique_paths("example2.txt", false), 19, "example 2");
        assert_eq!(unique_paths("example3.txt", false), 226, "example 3");
    }

    #[test]
    fn two_visits() {
        assert_eq!(unique_paths("example1.txt", true), 36, "example 1");
        assert_eq!(unique_paths("example2.txt", true), 103, "example 2");
        assert_eq!(unique_paths("example3.txt", true), 3509, "example 3");
    }
}
