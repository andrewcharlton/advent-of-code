use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    use Dir::*;

    let grid = parse_input(INPUT);
    println!("Part one: {}", count_energised(&grid, 0, 0, E));
    println!("Part two: {}", max_energised(&grid));
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Dir {
    N,
    E,
    S,
    W,
}

fn max_energised(grid: &Grid) -> usize {
    use Dir::*;

    let mut highest = 0;
    for x in 0..grid.width {
        highest = highest.max(count_energised(grid, x, 0, S));
        highest = highest.max(count_energised(grid, x, grid.height - 1, N));
    }
    for y in 0..grid.height {
        highest = highest.max(count_energised(grid, 0, y, E));
        highest = highest.max(count_energised(grid, grid.width - 1, y, W));
    }
    highest
}

fn count_energised(grid: &Grid, start_x: usize, start_y: usize, start_dir: Dir) -> usize {
    use Dir::*;

    let mut energised: HashSet<(usize, usize, Dir)> = HashSet::new();
    let mut current: HashSet<(usize, usize, Dir)> = HashSet::new();

    for dir in grid.next_dir(start_x, start_y, start_dir) {
        current.insert((start_x, start_y, dir));
    }

    loop {
        if current.len() == 0 {
            break;
        }

        let mut next_steps: HashSet<(usize, usize, Dir)> = HashSet::new();

        for (x, y, dir) in current {
            if energised.contains(&(x, y, dir)) {
                continue;
            }
            energised.insert((x, y, dir));

            // Check that we can move first
            if x == 0 && dir == W {
                continue;
            }
            if x >= grid.width - 1 && dir == E {
                continue;
            }
            if y == 0 && dir == N {
                continue;
            }
            if y >= grid.height - 1 && dir == S {
                continue;
            }

            // Then move
            let (x, y) = match dir {
                N => (x, y - 1),
                E => (x + 1, y),
                S => (x, y + 1),
                W => (x - 1, y),
            };
            for dir in grid.next_dir(x, y, dir) {
                next_steps.insert((x, y, dir));
            }
        }

        current = next_steps;
    }

    energised
        .into_iter()
        .map(|(x, y, _)| (x, y))
        .collect::<HashSet<(usize, usize)>>()
        .len()
}

struct Grid {
    height: usize,
    width: usize,
    mirrors: HashMap<(usize, usize), char>,
}

impl Grid {
    fn next_dir(&self, x: usize, y: usize, dir: Dir) -> Vec<Dir> {
        use Dir::*;

        if let Some(mirror) = self.mirrors.get(&(x, y)) {
            match mirror {
                '|' => {
                    if dir == W || dir == E {
                        return vec![N, S];
                    } else {
                        return vec![dir];
                    };
                }
                '-' => {
                    if dir == N || dir == S {
                        return vec![E, W];
                    } else {
                        return vec![dir];
                    };
                }
                '/' => {
                    match dir {
                        N => return vec![E],
                        E => return vec![N],
                        S => return vec![W],
                        W => return vec![S],
                    };
                }
                '\\' => {
                    match dir {
                        N => return vec![W],
                        E => return vec![S],
                        S => return vec![E],
                        W => return vec![N],
                    };
                }
                _ => panic!("Unknown mirror '{}'", mirror),
            }
        }

        vec![dir]
    }
}

fn parse_input(s: &str) -> Grid {
    let lines: Vec<&str> = s.lines().collect();
    let height = lines.len();
    let width = lines.get(0).unwrap().len();

    let mirrors = lines
        .into_iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| if c == '.' { None } else { Some(((x, y), c)) })
        })
        .collect();

    Grid {
        height,
        width,
        mirrors,
    }
}
