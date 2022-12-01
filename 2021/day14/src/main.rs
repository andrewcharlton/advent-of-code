use std::collections::HashMap;
use std::fs;

fn main() {
    println!("Part one: {}", count_range("input.txt", 10));
    println!("Part two: {}", count_range("input.txt", 40));
}

type Pair = (char, char);

fn count_range(filename: &str, iterations: usize) -> usize {
    let (mut counts, map, final_char) = parse_input(filename);

    // Calculate how many of each pair we have in the final outcome.
    for _ in 0..iterations {
        let mut next_counts: HashMap<Pair, usize> = HashMap::new();
        for (pair, count) in &counts {
            for p in map.get(&pair).unwrap() {
                if let Some(c) = next_counts.get_mut(p) {
                    *c += count;
                } else {
                    next_counts.insert(*p, *count);
                }
            }
        }

        counts = next_counts;
    }

    // Count the number of characters we have.
    let mut char_counts: HashMap<char, usize> = HashMap::new();
    char_counts.insert(final_char, 1);
    for (pair, count) in &counts {
        if let Some(c) = char_counts.get_mut(&pair.0) {
            *c += count;
        } else {
            char_counts.insert(pair.0, *count);
        }
    }

    let mut most = 0;
    let mut least = usize::MAX;
    for (_, count) in char_counts {
        if count > most {
            most = count;
        }
        if count < least {
            least = count;
        }
    }

    most - least
}

fn parse_input(filename: &str) -> (HashMap<Pair, usize>, HashMap<Pair, Vec<Pair>>, char) {
    let file = fs::read_to_string(filename).expect("couldn't open file");
    let mut lines = file.lines();

    let chars: Vec<char> = lines.next().unwrap().chars().collect();
    let mut pairs_count: HashMap<Pair, usize> = HashMap::new();
    for i in 0..chars.len() - 1 {
        let pair = (*chars.get(i).unwrap(), *chars.get(i + 1).unwrap());
        if let Some(x) = pairs_count.get_mut(&pair) {
            *x += 1;
        } else {
            pairs_count.insert(pair, 1);
        }
    }
    let final_char = *chars.last().unwrap();

    lines.next();
    let map: HashMap<Pair, Vec<Pair>> = lines
        .map(|line| {
            let (pair, insert) = line.split_once(" -> ").unwrap();
            let a = pair.chars().nth(0).unwrap();
            let b = pair.chars().nth(1).unwrap();
            let insert = insert.chars().nth(0).unwrap();
            ((a, b), vec![(a, insert), (insert, b)])
        })
        .collect();

    (pairs_count, map, final_char)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(count_range("example.txt", 10), 1588, "10 iterations");
        assert_eq!(
            count_range("example.txt", 40),
            2188189693529,
            "40 iterations"
        );
    }
}
