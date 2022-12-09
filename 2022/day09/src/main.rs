use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", unique_tail_positions(INPUT, 2));
    println!("Part one: {}", unique_tail_positions(INPUT, 10));
}

fn unique_tail_positions(input: &str, knots: usize) -> usize {
    let mut rope = Rope::new(knots);
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
    n: usize,
    knots: Vec<(i32, i32)>,
    visited: HashSet<(i32, i32)>,
}

impl Rope {
    fn new(n: usize) -> Rope {
        let mut visited = HashSet::new();
        visited.insert((0, 0));

        let mut knots: Vec<(i32, i32)> = Vec::new();
        for _ in 0..n {
            knots.push((0, 0));
        }

        Rope { n, knots, visited }
    }

    fn move_n(&mut self, dir: char, n: usize) {
        for _ in 0..n {
            self.move_once(dir);
        }
    }

    fn move_once(&mut self, dir: char) {
        // Move the head first
        let mut head = self.knots.get_mut(0).unwrap();
        match dir {
            'L' => head.0 -= 1,
            'R' => head.0 += 1,
            'U' => head.1 += 1,
            'D' => head.1 -= 1,
            _ => panic!("unknown direction: {}", dir),
        }

        for i in 1..self.n {
            let prev = self.knots.get(i - 1).unwrap().clone();
            let mut curr = self.knots.get_mut(i).unwrap();

            let diff = (prev.0 - curr.0, prev.1 - curr.1);
            if diff.0.abs() > 1 || diff.1.abs() > 1 {
                curr.0 += unit(diff.0);
                curr.1 += unit(diff.1);
            }
        }

        let tail = self.knots.last().unwrap();
        self.visited.insert(*tail);
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
    const EXAMPLE2: &str = include_str!("../example2.txt");

    #[test]
    fn example() {
        assert_eq!(unique_tail_positions(EXAMPLE, 2), 13);
        assert_eq!(unique_tail_positions(EXAMPLE, 10), 1);
        assert_eq!(unique_tail_positions(EXAMPLE2, 10), 36);
    }
}
