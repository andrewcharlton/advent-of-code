use std::collections::{HashMap, HashSet};
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let now = Instant::now();

    let mut keypad = Keypad::new();
    println!("Part one: {}", keypad.solve(INPUT, 3));
    println!("Part two: {}", keypad.solve(INPUT, 26));

    let elapsed_time = now.elapsed();
    println!("Solved in {}µs", elapsed_time.as_micros());
}

struct Keypad {
    keys: HashMap<char, (i64, i64)>,
    gaps: HashSet<(i64, i64)>,

    memoized: HashMap<(String, usize), usize>,
}

impl Keypad {
    fn new() -> Self {
        let keys = HashMap::from([
            ('7', (0, 0)),
            ('8', (1, 0)),
            ('9', (2, 0)),
            ('4', (0, 1)),
            ('5', (1, 1)),
            ('6', (2, 1)),
            ('1', (0, 2)),
            ('2', (1, 2)),
            ('3', (2, 2)),
            ('0', (1, 3)),
            ('^', (1, 3)),
            ('A', (2, 3)),
            ('<', (0, 4)),
            ('v', (1, 4)),
            ('>', (2, 4)),
        ]);
        let gaps = HashSet::from([(0, 3)]);

        Keypad {
            keys,
            gaps,
            memoized: HashMap::new(),
        }
    }

    fn solve(&mut self, input: &str, depth: usize) -> usize {
        input
            .lines()
            .map(|line| {
                let len = self.shortest_sequence(line, depth);
                let n: usize = line.strip_suffix('A').unwrap().parse().unwrap();
                n * len
            })
            .sum()
    }

    fn shortest_sequence(&mut self, code: &str, depth: usize) -> usize {
        if depth == 0 {
            return code.len();
        }

        if let Some(n) = self.memoized.get(&(code.to_string(), depth)) {
            return *n;
        }

        let n = self._shortest_sequence(&code, depth);
        self.memoized.insert((code.to_string(), depth), n);
        n
    }

    fn _shortest_sequence(&mut self, code: &str, depth: usize) -> usize {
        let mut chars: Vec<char> = code.chars().collect();
        chars.insert(0, 'A');

        let mut total = 0;

        // We can consider each sub-sequence individually because we always have to return back to
        // the 'A  button, so just total up the minimum length for each subsequence.
        for i in 1..chars.len() {
            let start = chars.get(i - 1).unwrap();
            let end = chars.get(i).unwrap();
            total += self
                .shortest_path_between_buttons(start, end)
                .iter()
                .map(|path| self.shortest_sequence(path, depth - 1))
                .min()
                .unwrap();
        }

        total
    }

    fn shortest_path_between_buttons(&self, start: &char, end: &char) -> Vec<String> {
        let start = self.keys.get(start).unwrap();
        let end = self.keys.get(end).unwrap();

        let x_dir = if end.0 < start.0 { '<' } else { '>' };
        let x_len = (end.0 - start.0).abs();

        let y_dir = if end.1 < start.1 { '^' } else { 'v' };
        let y_len = (end.1 - start.1).abs();

        let mut horizontal_first = String::new();
        for _ in 0..x_len {
            horizontal_first.push(x_dir);
        }
        for _ in 0..y_len {
            horizontal_first.push(y_dir);
        }
        horizontal_first.push('A');

        let mut vertical_first = String::new();
        for _ in 0..y_len {
            vertical_first.push(y_dir);
        }
        for _ in 0..x_len {
            vertical_first.push(x_dir);
        }
        vertical_first.push('A');

        // If we're only moving in one direction, then just return one, they're the same either
        // way.
        if start.0 == end.0 || start.1 == end.1 {
            return vec![vertical_first];
        }

        // If by moving horizontally first, then we end up at a gap, we have to go vertically
        // first.
        if self.gaps.contains(&(end.0, start.1)) {
            return vec![vertical_first];
        }

        // If by moving vertically first, then we end up at a gap, we have to go horizontally
        // first.
        if self.gaps.contains(&(start.0, end.1)) {
            return vec![horizontal_first];
        }

        vec![horizontal_first, vertical_first]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let mut keypad = Keypad::new();
        assert_eq!(keypad.shortest_sequence("029A", 3), 68);
        assert_eq!(keypad.shortest_sequence("980A", 3), 60);
        assert_eq!(keypad.shortest_sequence("179A", 3), 68);
        assert_eq!(keypad.shortest_sequence("456A", 3), 64);
        assert_eq!(keypad.shortest_sequence("379A", 3), 64);
        assert_eq!(keypad.solve(EXAMPLE, 3), 126384);
    }
}
