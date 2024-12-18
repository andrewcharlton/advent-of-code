use std::collections::{HashMap, HashSet};
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
    let (score, _) = grid.find_route(grid.start_pos, Dir::E, grid.end_pos);
    score
}

fn part_two(input: &str) -> usize {
    let mut grid = Grid::new(input);
    grid.tiles_on_best_route()
}

struct Grid {
    walls: HashSet<(i64, i64)>,

    start_pos: (i64, i64),
    end_pos: (i64, i64),
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut walls = HashSet::new();
        let mut start_pos = (0, 0);
        let mut end_pos = (0, 0);

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        walls.insert((x.try_into().unwrap(), y.try_into().unwrap()));
                    }
                    'S' => {
                        start_pos = (x.try_into().unwrap(), y.try_into().unwrap());
                    }
                    'E' => {
                        end_pos = (x.try_into().unwrap(), y.try_into().unwrap());
                    }
                    _ => (),
                }
            }
        }

        Grid {
            walls,
            start_pos,
            end_pos,
        }
    }

    fn find_route(
        &mut self,
        start: (i64, i64),
        start_dir: Dir,
        end: (i64, i64),
    ) -> (usize, HashMap<(i64, i64, Dir), usize>) {
        let mut best: Option<usize> = None;
        let mut visited: HashMap<(i64, i64, Dir), usize> = HashMap::new();
        visited.insert((start.0, start.1, start_dir), 0);

        let mut latest_positions: HashMap<(i64, i64, Dir), usize> = HashMap::new();
        latest_positions.insert((start.0, start.1, start_dir), 0);

        while latest_positions.len() > 0 {
            let mut newer_positions = HashMap::new();

            for ((x, y, dir), score) in latest_positions {
                let neighbours = vec![
                    (x + dir.x(), y + dir.y(), dir, score + 1),
                    (x, y, dir.left(), score + 1000),
                    (x, y, dir.right(), score + 1000),
                ];

                for (x, y, dir, score) in neighbours {
                    if self.walls.contains(&(x, y)) {
                        continue;
                    }

                    if let Some(previous_score) = visited.get(&(x, y, dir)) {
                        if *previous_score < score {
                            continue;
                        }
                    }

                    if let Some(best_score) = best {
                        if best_score < score {
                            continue;
                        }
                    }

                    visited.insert((x, y, dir), score);
                    newer_positions.insert((x, y, dir), score);

                    if x == end.0 && y == end.1 {
                        if let Some(previous) = best {
                            if score < previous {
                                best = Some(score);
                            }
                        } else {
                            best = Some(score);
                        }
                    }
                }
            }

            latest_positions = newer_positions;
        }

        (best.unwrap(), visited)
    }

    fn tiles_on_best_route(&mut self) -> usize {
        let (best_score, visited_forwards) = self.find_route(self.start_pos, Dir::E, self.end_pos);

        let end_dirs: Vec<Dir> = visited_forwards
            .iter()
            .filter_map(|((x, y, dir), score)| {
                if *x == self.end_pos.0 && *y == self.end_pos.1 && *score == best_score {
                    Some(*dir)
                } else {
                    None
                }
            })
            .collect();

        let mut on_path: HashSet<(i64, i64)> = HashSet::new();

        for dir in end_dirs {
            let (backwards_score, visited_backwards) =
                self.find_route(self.end_pos, dir.rev(), self.start_pos);
            if backwards_score != best_score {
                println!(
                    "Backwards score does not match: {} != {}",
                    best_score, backwards_score
                );
            }

            for ((x, y, dir), score) in visited_forwards.clone() {
                if let Some(backwards_score) = visited_backwards.get(&(x, y, dir.rev())) {
                    if score + backwards_score == best_score {
                        on_path.insert((x, y));
                    }
                }
            }
        }

        on_path.len()
    }
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug, Ord, PartialOrd)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn left(&self) -> Self {
        match self {
            Dir::N => Dir::W,
            Dir::E => Dir::N,
            Dir::S => Dir::E,
            Dir::W => Dir::S,
        }
    }

    fn right(&self) -> Self {
        match self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
        }
    }

    fn rev(&self) -> Self {
        match self {
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::E => Dir::W,
            Dir::W => Dir::E,
        }
    }

    fn x(&self) -> i64 {
        match self {
            Dir::E => 1,
            Dir::W => -1,
            _ => 0,
        }
    }

    fn y(&self) -> i64 {
        match self {
            Dir::N => -1,
            Dir::S => 1,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE1: &str = include_str!("../example1.txt");
    const EXAMPLE2: &str = include_str!("../example2.txt");

    #[test]
    fn example() {
        assert_eq!(part_one(EXAMPLE1), 7036);
        assert_eq!(part_one(EXAMPLE2), 11048);

        assert_eq!(part_two(EXAMPLE1), 45);
        assert_eq!(part_two(EXAMPLE2), 64);
    }
}
