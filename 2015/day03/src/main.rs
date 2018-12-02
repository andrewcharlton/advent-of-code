use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Couldn't read file");

    println!("Part one: {}", houses_visited(&input).len());

    let (s, r): (Vec<(usize, char)>, Vec<(usize, char)>) =
        input.chars().enumerate().partition(|(i, _)| i % 2 == 0);

    let s: String = s.iter().map(|(_, x)| x.clone()).collect();
    let r: String = r.iter().map(|(_, x)| x.clone()).collect();

    let s = houses_visited(&s);
    let r = houses_visited(&r);
    let answer: Vec<&(i64, i64)> = s.union(&r).collect();
    println!("Part two: {}", answer.len());
}

fn houses_visited(input: &str) -> HashSet<(i64, i64)> {
    let mut visited = HashSet::new();
    let mut location = (0, 0);
    visited.insert(location);

    for c in input.chars() {
        location = match c {
            '^' => (location.0, location.1 + 1),
            'v' => (location.0, location.1 - 1),
            '<' => (location.0 - 1, location.1),
            '>' => (location.0 + 1, location.1),
            _ => location,
        };
        visited.insert(location);
    }

    visited
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn houses_visited_test() {
        assert_eq!(houses_visited(">").len(), 2);
        assert_eq!(houses_visited("^>v<").len(), 4);
        assert_eq!(houses_visited("v^v^v^v^v^").len(), 2);
    }
}
