use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("couldn't read input");

    let diffs = diffs(&input);
    println!("Part one: {}", diffs[1] * diffs[3]);

    let arrangements = arrangements(&input);
    println!("Part two: {}", arrangements);
}

fn diffs(input: &str) -> [usize; 4] {
    let mut joltages: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();
    joltages.sort();
    joltages.insert(0, 0);

    let mut diffs: [usize; 4] = [0, 0, 0, 1];
    for i in 1..joltages.len() {
        let diff = joltages[i] - joltages[i - 1];
        diffs[diff] += 1;
    }

    diffs
}

fn arrangements(input: &str) -> usize {
    let joltages: HashSet<usize> = input.lines().map(|line| line.parse().unwrap()).collect();
    let target = joltages.iter().max().unwrap() + 3;
    let mut m: HashMap<usize, usize> = HashMap::new();
    arrangements_rec(&joltages, &mut m, 0, target)
}

fn arrangements_rec(
    joltages: &HashSet<usize>,
    m: &mut HashMap<usize, usize>,
    n: usize,
    target: usize,
) -> usize {
    if let Some(x) = m.get(&n) {
        return *x;
    }

    // Add up the number of ways
    let mut sum = 0;
    for i in 1..4 {
        if n + i == target {
            sum += 1;
        } else if joltages.contains(&(n + i)) {
            sum += arrangements_rec(&joltages, m, n + i, target);
        }
    }

    m.insert(n, sum);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "16
10
15
5
1
11
7
19
6
12
4";

    #[test]
    fn diffs_test() {
        assert_eq!(diffs(&INPUT), [0, 7, 0, 5]);
    }

    #[test]
    fn arrangements_test() {
        assert_eq!(arrangements(&INPUT), 8);
    }
}
