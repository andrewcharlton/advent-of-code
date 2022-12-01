use std::collections::HashSet;
use std::fs;

fn main() {
    let (first, last) = solve("input.txt");
    println!("Part one: {}", first);
    println!("Part two: {}", last);
}

fn solve(filename: &str) -> (u64, u64) {
    let file: &str = &fs::read_to_string(filename).expect("couldn't open file");

    let nums: Vec<u64> = file
        .lines()
        .next()
        .unwrap()
        .trim()
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let lines: Vec<&str> = file.lines().skip(1).collect();

    let mut board: Board = Board::new();
    let mut first = (nums.len(), 0);
    let mut last = (0, 0);
    for (i, line) in lines.iter().enumerate() {
        if i % 6 == 0 {
            println!("Creating new board");
            board = Board::new();
            continue;
        }

        println!("Adding row: {}", line);
        board.add_row(line);
        if i % 6 == 5 {
            let (x, score) = board.score(&nums);
            if x < first.0 {
                first = (x, score);
            }
            if x > last.0 {
                last = (x, score);
            }
        }
    }

    (first.1, last.1)
}

struct Board {
    rows: Vec<HashSet<u64>>,
    cols: Vec<HashSet<u64>>,
}

impl Board {
    fn new() -> Board {
        Board {
            rows: Vec::new(),
            cols: Vec::new(),
        }
    }

    fn add_row(&mut self, line: &str) {
        let nums: Vec<u64> = line
            .trim()
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect();

        nums.iter().enumerate().for_each(|(i, val)| {
            if let Some(col) = self.cols.get_mut(i) {
                col.insert(*val);
            } else {
                let mut col = HashSet::new();
                col.insert(*val);
                self.cols.insert(i, col);
            }
        });

        let row: HashSet<u64> = nums.into_iter().collect();
        self.rows.push(row);
    }

    fn score(&mut self, nums: &Vec<u64>) -> (usize, u64) {
        for (i, v) in nums.iter().enumerate() {
            // Remove the number from the rows and cols.
            let mut done = false;
            for row in &mut self.rows {
                row.remove(v);
                done = done || row.len() == 0;
            }
            for col in &mut self.cols {
                col.remove(v);
                done = done || col.len() == 0;
            }

            if !done {
                continue;
            }

            let sum: u64 = self.rows.iter().flatten().sum();
            println!(
                "Finished. Turn {}, Sum: {}, Number just drawn: {}",
                i, sum, v
            );
            return (i, sum * v);
        }

        panic!("Unreachable")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(solve("example.txt"), (4512, 1924), "Example");
    }
}
