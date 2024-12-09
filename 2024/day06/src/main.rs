use std::collections::HashSet;
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let now = Instant::now();

    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));

    let elapsed_time = now.elapsed();
    println!("Solved in {}µs", elapsed_time.as_micros());
}

fn part_one(input: &str) -> usize {
    let mut grid = Grid::new(input);
    grid.trace_route();
    grid.unique_locations().len()
}

fn part_two(input: &str) -> usize {
    let original_grid = Grid::new(input);

    let mut grid = original_grid.clone();
    grid.trace_route();
    let unique_locations = grid.unique_locations();

    let mut count = 0;

    for pos in unique_locations {
        if pos == original_grid.guard_pos {
            continue;
        }

        let log = pos == Pos{x:3, y:6};
        if log {
        println!("Trying obstacle at {:?}", pos);
        }
        let mut grid_clone = original_grid.clone();
        grid_clone.obstacles.insert(pos);
        if grid_clone.trace_route() {
            count += 1;
        } else {
            if log {
            println!("Visited: {:?}", grid_clone.unique_locations());
            }
        }
    }

    count
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn turn_right(&self) -> Dir {
        match self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    max_x: usize,
    max_y: usize,
    obstacles: HashSet<Pos>,

    guard_pos: Pos,
    guard_dir: Dir,

    visited: HashSet<(Pos, Dir)>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut obstacles = HashSet::new();
        let mut guard_pos = Pos { x: 0, y: 0 };
        let mut max_y = 0;
        let mut max_x = 0;

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        obstacles.insert(Pos { x, y });
                    }
                    '^' => {
                        guard_pos = Pos { x, y };
                    }
                    _ => (),
                }
            }

            max_y = max_y.max(y);
            max_x = max_x.max(line.len());
        }

        let mut visited = HashSet::new();
        visited.insert((guard_pos, Dir::N));

        Grid {
            max_x,
            max_y,
            obstacles,
            guard_pos,
            guard_dir: Dir::N,
            visited,
        }
    }

    fn trace_route(&mut self) -> bool {
        loop {
            if let Some(already_visited) = self.move_guard() {
                if already_visited {
                    return true;
                }
                continue;
            }

            return false;
        }
    }

    // Returns None if the move takes us outside of the grid.
    // Otherwise returns whether or not we've been to this position before.
    fn move_guard(&mut self) -> Option<bool> {
        if let Some(next_pos) = self.next_pos(self.guard_pos, self.guard_dir) {
            if self.obstacles.contains(&next_pos) {
                self.guard_dir = self.guard_dir.turn_right();
            } else {
                self.guard_pos = next_pos;
            }

            return Some(!self.visited.insert((self.guard_pos, self.guard_dir)));
        }

        None
    }

    fn next_pos(&self, pos: Pos, dir: Dir) -> Option<Pos> {
        match dir {
            Dir::N => {
                if pos.y == 0 {
                    return None;
                }
                return Some(Pos {
                    x: pos.x,
                    y: pos.y - 1,
                });
            }
            Dir::E => {
                if pos.x == self.max_x {
                    return None;
                }
                return Some(Pos {
                    x: pos.x + 1,
                    y: pos.y,
                });
            }
            Dir::S => {
                if pos.y == self.max_y {
                    return None;
                }
                return Some(Pos {
                    x: pos.x,
                    y: pos.y + 1,
                });
            }
            Dir::W => {
                if pos.x == 0 {
                    return None;
                }
                return Some(Pos {
                    x: pos.x - 1,
                    y: pos.y,
                });
            }
        }
    }

    fn unique_locations(&mut self) -> HashSet<Pos> {
        self.visited.iter().map(|(pos, _)| pos.clone()).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(part_one(EXAMPLE), 41);
        assert_eq!(part_two(EXAMPLE), 6);
    }
}
