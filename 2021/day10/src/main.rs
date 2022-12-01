use std::fs;

fn main() {
    let (x, y) = solve("input.txt");
    println!("Part one: {}", x);
    println!("Part two: {}", y);
}

fn solve(filename: &str) -> (u64, u64) {
    let statuses: Vec<LineStatus> = fs::read_to_string(filename)
        .expect("couldn't open file")
        .lines()
        .map(|l| evaluate_line(l))
        .collect();

    let corrupt: u64 = statuses
        .iter()
        .map(|s| match s {
            LineStatus::Corrupt(x) => *x,
            _ => 0,
        })
        .sum();

    let mut incomplete: Vec<u64> = statuses
        .iter()
        .filter_map(|s| match s {
            LineStatus::Incomplete(x) => Some(*x),
            _ => None,
        })
        .collect();

    incomplete.sort();
    println!("{:?}", incomplete);
    (corrupt, *incomplete.get((incomplete.len()) / 2).unwrap())
}

#[derive(PartialEq, Debug)]
enum Chunk {
    Round,
    Square,
    Curly,
    Angle,
}

enum LineStatus {
    Ok,
    Corrupt(u64),
    Incomplete(u64),
}

fn evaluate_line(line: &str) -> LineStatus {
    let mut previous: Vec<Chunk> = Vec::new();

    let chars = line.chars();
    for c in chars {
        match c {
            '(' => previous.push(Chunk::Round),
            '[' => previous.push(Chunk::Square),
            '{' => previous.push(Chunk::Curly),
            '<' => previous.push(Chunk::Angle),
            ')' => {
                if previous.last() == Some(&Chunk::Round) {
                    previous.pop();
                } else {
                    return LineStatus::Corrupt(3);
                }
            }
            ']' => {
                if previous.last() == Some(&Chunk::Square) {
                    previous.pop();
                } else {
                    return LineStatus::Corrupt(57);
                }
            }
            '}' => {
                if previous.last() == Some(&Chunk::Curly) {
                    previous.pop();
                } else {
                    return LineStatus::Corrupt(1197);
                }
            }
            '>' => {
                if previous.last() == Some(&Chunk::Angle) {
                    previous.pop();
                } else {
                    return LineStatus::Corrupt(25137);
                }
            }
            _ => panic!("Unknown character {}", c),
        }
    }

    if previous.len() == 0 {
        return LineStatus::Ok;
    }

    let score: u64 = previous.iter().rev().fold(0, |acc, status| match status {
        Chunk::Round => 5 * acc + 1,
        Chunk::Square => 5 * acc + 2,
        Chunk::Curly => 5 * acc + 3,
        Chunk::Angle => 5 * acc + 4,
    });
    LineStatus::Incomplete(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let (a, b) = solve("example.txt");
        assert_eq!(a, 26397, "Part one");
        assert_eq!(b, 288957, "Part two");
    }
}
