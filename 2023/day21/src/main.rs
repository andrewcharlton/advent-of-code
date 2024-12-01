use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", solve(INPUT, 64));

    let mut steps = 65;
    for _ in 0..10 {
        println!("For {} steps: {}", steps, solve(INPUT, steps));
        steps += 131;
    }
}

fn solve(input: &str, steps: usize, print: bool) -> usize {
    let grid = Grid::parse(input);
    let steps_mod_2 = steps % 2;

    if print {
        println!("Grid size: {} x {}", grid.width, grid.height);
    }

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert(grid.start);
    let mut total = 1 - steps_mod_2;

    let mut current: HashSet<(usize, usize)> = HashSet::new();
    current.insert(grid.start);

    for i in 1..=steps {
        let next_steps: HashSet<(usize, usize)> = current
            .iter()
            .flat_map(|(x, y)| grid.adjacent(*x, *y))
            .filter(|pos| !visited.contains(pos))
            .collect();

        if i % 2 == steps_mod_2 {
            total += next_steps.len();
        }

        visited.extend(next_steps.clone());
        current = next_steps;
    }

    for y in 0..grid.height {
        let mut line = String::new();
        for x in 0..grid.width {
            if visited.contains(&(x, y)) {
                line.push('•');
            } else if grid.rocks.contains(&(x, y)) {
                line.push('x');
            } else {
                line.push(' ');
            }
        }
        println!("{}", line);
    }

    total
}

struct Grid {
    width: usize,
    height: usize,
    rocks: HashSet<(usize, usize)>,
    start: (usize, usize),
}

impl Grid {
    fn parse(s: &str) -> Self {
        let lines: Vec<&str> = s.lines().collect();
        let height = lines.len();
        let width = lines.get(0).unwrap().len();

        let mut rocks = HashSet::new();
        let (mut start_x, mut start_y) = (0, 0);

        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        rocks.insert((x, y));
                    }
                    'S' => {
                        start_x = x;
                        start_y = y;
                    }
                    _ => {}
                }
            }
        }

        Grid {
            width,
            height,
            rocks,
            start: (start_x, start_y),
        }
    }

    fn adjacent(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut adjacent = Vec::new();

        if x > 0 && !self.rocks.contains(&(x - 1, y)) {
            adjacent.push((x - 1, y));
        }
        if x < self.width - 1 && !self.rocks.contains(&(x + 1, y)) {
            adjacent.push((x + 1, y));
        }
        if y > 0 && !self.rocks.contains(&(x, y - 1)) {
            adjacent.push((x, y - 1));
        }
        if y < self.height - 1 && !self.rocks.contains(&(x, y + 1)) {
            adjacent.push((x, y + 1));
        }

        adjacent
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(solve(EXAMPLE, 6), 16);
    }
}
