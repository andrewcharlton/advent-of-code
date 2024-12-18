use std::collections::HashSet;
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let now = Instant::now();

    println!("Part one: {}", solve(INPUT, false, false));
    println!("Part two: {}", solve(INPUT, true, false));

    let elapsed_time = now.elapsed();
    println!("Solved in {}µs", elapsed_time.as_micros());
}

fn solve(input: &str, double: bool, visualise: bool) -> i64 {
    let mut grid = Grid::new(input, double);
    grid.run(visualise)
}

struct Grid {
    walls: HashSet<(i64, i64)>,
    boxes: HashSet<(i64, i64)>,
    robot: (i64, i64),
    instructions: Vec<Direction>,
    double: bool,
}

impl Grid {
    fn new(input: &str, double: bool) -> Self {
        let (grid, instructions) = input.split_once("\n\n").unwrap();

        let mut walls = HashSet::new();
        let mut boxes = HashSet::new();
        let mut robot = (0, 0);

        for (y, line) in grid.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let mut x: i64 = x.try_into().unwrap();
                let y: i64 = y.try_into().unwrap();

                if double {
                    x = 2 * x;
                }

                match c {
                    '#' => {
                        walls.insert((x, y));
                        if double {
                            walls.insert((x + 1, y));
                        }
                    }
                    'O' => {
                        boxes.insert((x.try_into().unwrap(), y.try_into().unwrap()));
                    }
                    '@' => {
                        robot = (x.try_into().unwrap(), y.try_into().unwrap());
                    }
                    _ => {}
                }
            }
        }

        let mut instructions: Vec<Direction> = instructions
            .chars()
            .filter_map(|c| match c {
                '^' => Some(Direction::Up),
                'v' => Some(Direction::Down),
                '<' => Some(Direction::Left),
                '>' => Some(Direction::Right),
                _ => None,
            })
            .collect();
        instructions.reverse();

        Grid {
            walls,
            boxes,
            robot,
            instructions,
            double,
        }
    }

    fn run(&mut self, visualise: bool) -> i64 {
        let mut count = 0;

        if self.double {
            loop {
                if visualise {
                    self.visualise(count);
                    count += 1;
                }
                if !self.iterate_double() {
                    return self.score();
                }
                continue;
            }
        }

        loop {
            if visualise {
                self.visualise(count);
                count += 1;
            }
            if !self.iterate() {
                return self.score();
            }
            continue;
        }
    }

    fn iterate(&mut self) -> bool {
        let dir = self.instructions.pop();
        if dir.is_none() {
            return false;
        }
        let dir = dir.unwrap();

        let next_robot = (self.robot.0 + dir.x(), self.robot.1 + dir.y());
        if self.walls.contains(&next_robot) {
            // We hit a wall, nothing to do.
            return true;
        }

        if !self.boxes.contains(&next_robot) {
            // No box, so just move the robot.
            self.robot = next_robot;
            return true;
        }

        // We know we've hit a box, so we need to keep going until we find an empty space.
        let mut pos = next_robot.clone();
        loop {
            pos = (pos.0 + dir.x(), pos.1 + dir.y());
            if self.walls.contains(&pos) {
                // We've hit a wall, so we can't shunt the boxes along.
                return true;
            }

            if !self.boxes.contains(&pos) {
                // We've hit a gap, so we can move all the boxes along.
                // In reality, this can
                self.boxes.insert(pos);
                self.boxes.remove(&next_robot);
                self.robot = next_robot;
                return true;
            }

            // We've hit another box, so we need to just keep going until we hit a wall or
            // a free space.
        }
    }

    fn iterate_double(&mut self) -> bool {
        use Direction::{Down, Left, Right, Up};

        let dir = self.instructions.pop();
        if dir.is_none() {
            return false;
        }
        let dir = dir.unwrap();

        let next_robot = (self.robot.0 + dir.x(), self.robot.1 + dir.y());
        let can_move = match dir {
            Left => self.move_left(&next_robot),
            Right => self.move_right(&next_robot),
            Up => self.move_vertically(&next_robot, -1),
            Down => self.move_vertically(&next_robot, 1),
        };
        if can_move {
            self.robot = next_robot;
        }

        true
    }

    fn move_left(&mut self, robot_pos: &(i64, i64)) -> bool {
        let mut pos = robot_pos.clone();
        loop {
            if self.walls.contains(&pos) {
                // We've hit a wall, nothing more to do.
                return false;
            }

            if self.boxes.contains(&(pos.0 - 1, pos.1)) {
                // We've hit a box, so we need to check the next spot two to the left.
                pos = (pos.0 - 2, pos.1);
                continue;
            }

            // We're not at a wall or empty spot so we can shunt all the boxes along.
            for x in (pos.0..robot_pos.0).step_by(2) {
                self.boxes.remove(&(x + 1, robot_pos.1));
                self.boxes.insert((x, robot_pos.1));
            }
            return true;
        }
    }

    fn move_right(&mut self, robot_pos: &(i64, i64)) -> bool {
        let mut pos = robot_pos.clone();
        loop {
            if self.walls.contains(&pos) {
                // We've hit a wall, nothing more to do.
                return false;
            }

            if self.boxes.contains(&(pos.0, pos.1)) {
                // We've hit a box, so we need to check the next spot two to the right;
                pos = (pos.0 + 2, pos.1);
                continue;
            }

            // We're not at a wall or empty spot so we can shunt all the boxes along.
            for x in (robot_pos.0..pos.0).step_by(2) {
                self.boxes.remove(&(x, robot_pos.1));
                self.boxes.insert((x + 1, robot_pos.1));
            }
            return true;
        }
    }

    fn move_vertically(&mut self, robot_pos: &(i64, i64), step: i64) -> bool {
        let mut x_pos: HashSet<i64> = HashSet::new();
        x_pos.insert(robot_pos.0);

        let mut blocks_to_move: HashSet<(i64, i64)> = HashSet::new();

        let mut y = robot_pos.1;

        loop {
            // Have we hit a wall with any block?
            for x in &x_pos {
                if self.walls.contains(&(*x, y)) {
                    return false;
                }
            }

            let mut next_x_pos: HashSet<i64> = HashSet::new();

            // Are there any blocks above/below us?
            for x in &x_pos {
                if self.boxes.contains(&(x - 1, y)) {
                    next_x_pos.insert(x - 1);
                    next_x_pos.insert(*x);
                    blocks_to_move.insert((x - 1, y));
                }
                if self.boxes.contains(&(*x, y)) {
                    next_x_pos.insert(*x);
                    next_x_pos.insert(x + 1);
                    blocks_to_move.insert((*x, y));
                }
            }

            // If we have nothing above/below the blocks we need to move, shift them.
            if next_x_pos.len() == 0 {
                for block in &blocks_to_move {
                    self.boxes.remove(block);
                }
                for block in &blocks_to_move {
                    self.boxes.insert((block.0, block.1 + step));
                }

                return true;
            }

            y += step;
            x_pos = next_x_pos;
        }
    }

    fn score(&self) -> i64 {
        self.boxes.iter().map(|(x, y)| 100 * y + x).sum()
    }

    fn visualise(&self, count: usize) {
        println!("\nIteration {}\n", count);
        let max_x = *self.walls.iter().map(|(x, _)| x).max().unwrap();
        let max_y = *self.walls.iter().map(|(_, y)| y).max().unwrap();

        for y in 0..=max_y {
            let mut line: Vec<char> = Vec::new();

            for x in 0..max_x {
                if x == self.robot.0 && y == self.robot.1 {
                    match self.instructions.last() {
                        Some(Direction::Up) => {
                            line.push('^');
                        }
                        Some(Direction::Down) => {
                            line.push('v');
                        }
                        Some(Direction::Left) => {
                            line.push('<');
                        }
                        Some(Direction::Right) => {
                            line.push('>');
                        }
                        None => {
                            line.push('@');
                        }
                    }
                    continue;
                }

                if self.walls.contains(&(x, y)) {
                    line.push('#');
                    continue;
                }

                if !self.double {
                    if self.boxes.contains(&(x, y)) {
                        line.push('O');
                        continue;
                    }
                    line.push(' ');
                    continue;
                }

                if self.boxes.contains(&(x, y)) {
                    line.push('[');
                    continue;
                }
                if self.boxes.contains(&(x - 1, y)) {
                    line.push(']');
                    continue;
                }

                line.push(' ');
            }

            let line: String = line.iter().collect();
            println!("{}", line);
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn x(&self) -> i64 {
        match self {
            Direction::Left => -1,
            Direction::Right => 1,
            _ => 0,
        }
    }

    fn y(&self) -> i64 {
        match self {
            Direction::Up => -1,
            Direction::Down => 1,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE1: &str = include_str!("../example1.txt");
    const EXAMPLE2: &str = include_str!("../example2.txt");
    const EXAMPLE3: &str = include_str!("../example3.txt");

    #[test]
    fn example() {
        assert_eq!(solve(EXAMPLE1, false, false), 2028);
        assert_eq!(solve(EXAMPLE2, false, false), 10092);
        assert_eq!(solve(EXAMPLE3, true, true), 618);
        assert_eq!(solve(EXAMPLE2, true, true), 9021);
    }
}
