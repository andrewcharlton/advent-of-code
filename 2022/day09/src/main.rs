use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", part_one(INPUT));
}

fn part_one(input: &str) -> usize {
    let mut rope = Rope::new();
    let moves = parse_moves(input);
    for (dir, n) in moves {
        rope.move_n(dir, n);
    }

    rope.visited.len()
}

fn parse_moves(input: &str) -> Vec<(char, usize)> {
    input
        .lines()
        .map(|line| {
            let (dir, n) = line.split_once(" ").unwrap();
            (dir.chars().next().unwrap(), n.parse::<usize>().unwrap())
        })
        .collect()
}

struct Rope {
    head: (i32, i32),
    tail: (i32, i32),
    visited: HashSet<(i32, i32)>,
}

impl Rope {
    fn new() -> Rope {
        let mut visited = HashSet::new();
        visited.insert((0, 0));

        Rope {
            head: (0, 0),
            tail: (0, 0),
            visited,
        }
    }

    fn move_n(&mut self, dir: char, n: usize) {
        for i in 0..n {
            self.move_once(dir);
        }
    }

    fn move_once(&mut self, dir: char) {
        // Move the head first
        match dir {
            'L' => self.head.0 -= 1,
            'R' => self.head.0 += 1,
            'U' => self.head.1 += 1,
            'D' => self.head.1 -= 1,
            _ => panic!("unknown direction: {}", dir),
        }

        let diff = (self.head.0 - self.tail.0, self.head.1 - self.tail.1);

        // We need to move if we are more than 1 away in any direction
        if diff.0.abs() > 1 || diff.1.abs() > 1 {
            self.tail.0 += unit(diff.0);
            self.tail.1 += unit(diff.1);
        }
        println!(
            "Head: ({}, {}), Tail: ({}, {})",
            self.head.0, self.head.1, self.tail.0, self.tail.1
        );
        self.visited.insert(self.tail);
    }
}

fn unit(distance: i32) -> i32 {
    match distance {
        0 => 0,
        x if x > 0 => 1,
        _ => -1,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(part_one(EXAMPLE), 13);
    }
}
