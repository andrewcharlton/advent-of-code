use std::collections::HashSet;
use std::fs;

fn main() {
    println!("Part one: {}", risk_level("input.txt"));
    println!("Part two: {}", basin_sizes("input.txt"));
}

struct Grid {
    points: Vec<Vec<u64>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn new(filename: &str) -> Grid {
        let points: Vec<Vec<u64>> = fs::read_to_string(filename)
            .expect("couldn't open file")
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap().into())
                    .collect()
            })
            .collect();

        let height = points.len();
        let width = points.get(0).unwrap().len();

        Grid {
            points,
            height,
            width,
        }
    }

    fn get(&self, x: usize, y: usize) -> u64 {
        *self.points.get(y).unwrap().get(x).unwrap()
    }

    fn neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbours: Vec<(usize, usize)> = Vec::new();

        if x > 0 {
            neighbours.push((x - 1, y));
        }
        if y > 0 {
            neighbours.push((x, y - 1));
        }
        if x < self.width - 1 {
            neighbours.push((x + 1, y));
        }
        if y < self.height - 1 {
            neighbours.push((x, y + 1));
        }

        neighbours
    }
}

fn risk_level(filename: &str) -> u64 {
    let grid = Grid::new(filename);
    let mut risk_sum: u64 = 0;

    for x in 0..grid.width {
        'outer: for y in 0..grid.height {
            let val = grid.get(x, y);

            for n in grid.neighbours(x, y) {
                if grid.get(n.0, n.1) <= val {
                    continue 'outer;
                }
            }

            risk_sum += 1 + val;
        }
    }

    risk_sum
}

fn basin_sizes(filename: &str) -> usize {
    let grid = Grid::new(filename);

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut sizes: Vec<usize> = Vec::new();

    for x in 0..grid.width {
        for y in 0..grid.height {
            if grid.get(x, y) == 9 {
                continue;
            }

            if visited.contains(&(x, y)) {
                continue;
            }

            let locs = basin_locations(&grid, x, y);
            sizes.push(locs.len());
            for (p, q) in locs {
                visited.insert((p, q));
            }
        }
    }

    sizes.sort();
    sizes.pop().unwrap() * sizes.pop().unwrap() * sizes.pop().unwrap()
}

fn basin_locations(grid: &Grid, x: usize, y: usize) -> HashSet<(usize, usize)> {
    let mut locs: HashSet<(usize, usize)> = HashSet::new();
    locs.insert((x, y));

    let mut neighbours = grid.neighbours(x, y);
    while neighbours.len() > 0 {
        let mut new_neighbours: Vec<(usize, usize)> = Vec::new();

        for (p, q) in neighbours {
            if grid.get(p, q) == 9 {
                continue;
            }

            if locs.contains(&(p, q)) {
                continue;
            }

            locs.insert((p, q));
            for n in grid.neighbours(p, q) {
                new_neighbours.push(n);
            }
        }

        neighbours = new_neighbours;
    }

    locs
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(risk_level("example.txt"), 15, "Part one");
        assert_eq!(basin_sizes("example.txt"), 1134, "Part two");
    }
}
