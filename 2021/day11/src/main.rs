use std::collections::HashSet;
use std::fs;

fn main() {
    let mut grid = Grid::new("input.txt");
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += grid.step();
    }
    println!("Part one: {}", flashes);

    let mut count = 101;
    while grid.step() != grid.height * grid.width {
        count += 1;
    }
    println!("Part two: {}", count);
}

struct Grid {
    octopi: Vec<Vec<u8>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn new(filename: &str) -> Grid {
        let octopi: Vec<Vec<u8>> = fs::read_to_string(filename)
            .expect("couldn't open file")
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                    .collect()
            })
            .collect();

        let height = octopi.len().try_into().unwrap();
        let width = octopi.get(0).unwrap().len().try_into().unwrap();
        Grid {
            octopi,
            height,
            width,
        }
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        *self.octopi.get(y).unwrap().get(x).unwrap()
    }

    fn inc(&mut self, x: usize, y: usize) {
        let octopus = self.octopi.get_mut(y).unwrap().get_mut(x).unwrap();
        *octopus += 1;
    }

    fn reset(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let octopus = self.octopi.get_mut(y).unwrap().get_mut(x).unwrap();
                if *octopus > 9 {
                    *octopus = 0;
                }
            }
        }
    }

    fn step(&mut self) -> usize {
        let mut flashed: HashSet<(usize, usize)> = HashSet::new();
        let mut flashes: usize = 0;

        // first, increment all octopi by one
        for x in 0..self.width {
            for y in 0..self.height {
                self.inc(x, y);
            }
        }

        let mut has_flashed = true;
        while has_flashed {
            has_flashed = false;

            for x in 0..self.width {
                for y in 0..self.width {
                    if self.get(x, y) <= 9 || flashed.contains(&(x, y)) {
                        continue;
                    }

                    has_flashed = true;
                    flashed.insert((x, y));
                    flashes += 1;

                    let min_x = if x > 0 { x - 1 } else { x };
                    let max_x = if x < self.width - 1 { x + 1 } else { x };
                    let min_y = if y > 0 { y - 1 } else { y };
                    let max_y = if y < self.height - 1 { y + 1 } else { y };

                    for p in min_x..=max_x {
                        for q in min_y..=max_y {
                            self.inc(p, q);
                        }
                    }
                }
            }
        }

        self.reset();
        flashes
    }

    fn string(&self) -> String {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                s.push_str(&self.get(x, y).to_string());
            }
            s.push('\n');
        }

        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flash_count() {
        let mut grid = Grid::new("example.txt");
        let mut flashes = 0;
        for _ in 0..10 {
            flashes += grid.step();
        }
        assert_eq!(flashes, 204, "After 10 steps");

        for _ in 10..100 {
            flashes += grid.step();
        }
        assert_eq!(flashes, 1656, "After 100 steps");
    }

    #[test]
    fn synchro_flash() {
        let mut grid = Grid::new("example.txt");
        let mut count = 1;
        while grid.step() != grid.height * grid.width {
            count += 1;
        }
        assert_eq!(count, 195, "Synchronised flash");
    }
}
