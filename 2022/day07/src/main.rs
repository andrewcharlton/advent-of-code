use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", part_one(INPUT, 100000));
    println!("Part two: {}", part_two(INPUT, 70000000, 30000000));
}

fn part_one(s: &str, limit: usize) -> usize {
    dir_sizes(s).values().filter(|size| **size <= limit).sum()
}

fn part_two(s: &str, max: usize, needed: usize) -> usize {
    let sizes = dir_sizes(s);

    // Work out the current usage
    let current = sizes.get("").unwrap();
    let need_to_release = current + needed - max;

    sizes
        .values()
        .filter(|size| **size >= need_to_release)
        .fold(max, |acc, x| if *x < acc { *x } else { acc })
}

fn dir_sizes(s: &str) -> HashMap<String, usize> {
    let mut path: Vec<String> = Vec::new();
    let mut dirs: HashMap<String, usize> = HashMap::new();

    for line in s.lines() {
        let line = line.trim();

        if line == "$ cd /" {
            path.clear();
        } else if line == "$ cd .." {
            path.pop();
        } else if line.starts_with("$ cd ") {
            path.push(line.strip_prefix("$ cd ").unwrap().to_string());
        } else if line.starts_with("$") {
            // Do nothing
        } else if line.starts_with("dir") {
            // Do nothing
        } else {
            // Assume anything else is a file
            let parts = line.split_once(" ").unwrap();
            let size = parts.0.parse().unwrap();

            // We add the size of this to the current dir, and all it's parents.
            for i in 0..=path.len() {
                let p = path[..i].join("/");
                dirs.entry(p).and_modify(|e| *e += size).or_insert(size);
            }
        }
    }

    dirs
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(part_one(EXAMPLE, 100000), 95437);
        assert_eq!(part_two(EXAMPLE, 70000000, 30000000), 24933642);
    }

    #[test]
    fn parse_input() {
        let dirs = dir_sizes(EXAMPLE);

        let mut exp_dirs: HashMap<String, usize> = HashMap::new();
        exp_dirs.insert("".to_string(), 48381165);
        exp_dirs.insert("a".to_string(), 94853);
        exp_dirs.insert("a/e".to_string(), 584);
        exp_dirs.insert("d".to_string(), 24933642);
        assert_eq!(dirs, exp_dirs);
    }
}
