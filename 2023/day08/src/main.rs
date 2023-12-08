use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", solve(INPUT, "AAA"));
    println!("Part two: {}", solve_many(INPUT));
}

#[derive(Debug)]
enum Instruction {
    LEFT,
    RIGHT,
}

fn solve(s: &str, start: &str) -> usize {
    use Instruction::*;

    let (instructions, maps) = parse_input(s);

    let mut pos = start.to_owned();
    for (i, instruction) in instructions.iter().cycle().enumerate() {
        if pos.ends_with("Z") {
            return i;
        }

        let map = maps.get(&pos).unwrap();
        match instruction {
            LEFT => pos = map.0.clone(),
            RIGHT => pos = map.1.clone(),
        }
    }

    0
}

fn solve_many(s: &str) -> usize {
    let (_, maps) = parse_input(s);

    // This is not a great general solution, it's only due to the fact that the cycles are
    // very simple and just repeat every n instructions that we can get away with just finding
    // the lowest common multiple of all cycle lengths.
    // If there had been offsets, or cycles of varying lengths this would be a lot harder.
    maps.keys()
        .filter_map(|key| {
            if key.ends_with("A") {
                Some(key.to_owned())
            } else {
                None
            }
        })
        .map(|key| {
            let cycle_len = solve(s, &key);
            println!("{}: {}", key, cycle_len);
            cycle_len
        })
        .reduce(|acc, x| (acc * x) / gcd(acc, x))
        .unwrap()
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn parse_input(s: &str) -> (Vec<Instruction>, HashMap<String, (String, String)>) {
    let (instructions, maps) = s.split_once("\n\n").unwrap();

    let instructions = instructions
        .chars()
        .map(|c| match c {
            'L' => Instruction::LEFT,
            'R' => Instruction::RIGHT,
            _ => panic!("unknown instruction"),
        })
        .collect();

    let maps = maps
        .lines()
        .map(|line| {
            let (src, dst) = line.split_once(" = ").unwrap();
            let dst = dst.strip_prefix("(").unwrap().strip_suffix(")").unwrap();
            let (left, right) = dst.split_once(", ").unwrap();
            (
                src.trim().to_owned(),
                (left.trim().to_owned(), right.trim().to_owned()),
            )
        })
        .collect();

    (instructions, maps)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const EXAMPLE2: &str = include_str!("../example2.txt");
    const EXAMPLE3: &str = include_str!("../example3.txt");

    #[test]
    fn example() {
        assert_eq!(solve(EXAMPLE, "AAA"), 2);
        assert_eq!(solve(EXAMPLE2, "AAA"), 6);
        assert_eq!(solve_many(EXAMPLE3), 6);
    }
}
