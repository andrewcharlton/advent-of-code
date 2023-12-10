use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let grid = Grid::new(INPUT);
    println!("Part one: {}", grid.furthest_distance());
    println!("Part two: {}", grid.internal_area());
}

type Loc = (usize, usize);

struct Grid {
    loop_locations: HashMap<Loc, Pipe>,
    height: usize,
    width: usize,
}

impl Grid {
    fn new(input: &str) -> Grid {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len();
        let width = lines.get(0).unwrap().len();

        let (pipes, start) = parse_input(input);

        let mut loc = start.clone();
        let mut prev = (0, 0);

        let mut locations = HashMap::new();

        loop {
            let pipe = pipes.get(&loc).unwrap();
            let (a, b) = pipe.adjacent(loc);
            if prev == a {
                prev = loc;
                loc = b;
            } else {
                prev = loc;
                loc = a;
            }

            locations.insert(prev, *pipe);
            if loc == start {
                break;
            }
        }

        Grid {
            loop_locations: locations,
            height,
            width,
        }
    }

    fn furthest_distance(&self) -> usize {
        self.loop_locations.len() / 2
    }

    fn internal_area(&self) -> usize {
        use Pipe::*;

        let mut inside = 0;
        for y in 0..self.height {
            let mut boundaries_crossed = 0;
            let mut n = 0;
            let mut s = 0;

            for x in 0..self.width {
                if let Some(p) = self.loop_locations.get(&(x, y)) {
                    // We're working from left to right, and calculating how
                    // many boundaries each would need to cross to get to the
                    // outside.
                    // A | is a boundary all by itself.
                    // The others need both a north and south component, so we
                    // keep track of how many north and south there has been.
                    match p {
                        V => boundaries_crossed += 1,
                        NE | NW => n += 1,
                        SE | SW => s += 1,
                        _ => (),
                    }
                    continue;
                }

                let boundaries = boundaries_crossed + if n > s { s } else { n };
                if boundaries % 2 == 1 {
                    inside += 1;
                }
            }
        }

        inside
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Pipe {
    H,
    V,
    NE,
    NW,
    SE,
    SW,
    Start,
    Ground,
}

impl Pipe {
    fn adjacent(&self, loc: Loc) -> (Loc, Loc) {
        use Pipe::*;
        match self {
            H => ((loc.0 - 1, loc.1), (loc.0 + 1, loc.1)),
            V => ((loc.0, loc.1 - 1), (loc.0, loc.1 + 1)),
            NE => ((loc.0 + 1, loc.1), (loc.0, loc.1 - 1)),
            NW => ((loc.0 - 1, loc.1), (loc.0, loc.1 - 1)),
            SE => ((loc.0 + 1, loc.1), (loc.0, loc.1 + 1)),
            SW => ((loc.0 - 1, loc.1), (loc.0, loc.1 + 1)),
            Start => panic!("Start not used"),
            Ground => panic!("Ground not used"),
        }
    }
}

fn parse_input(input: &str) -> (HashMap<Loc, Pipe>, Loc) {
    use Pipe::*;

    let mut pipes: HashMap<Loc, Pipe> = input
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '|' => Some(((x, y), V)),
                '-' => Some(((x, y), H)),
                'L' => Some(((x, y), NE)),
                'J' => Some(((x, y), NW)),
                '7' => Some(((x, y), SW)),
                'F' => Some(((x, y), SE)),
                'S' => Some(((x, y), Start)),
                _ => None,
            })
        })
        .collect();

    let (start_x, start_y) = *pipes
        .iter()
        .find(|(_, pipe)| pipe == &&Start)
        .map(|(loc, _)| loc)
        .unwrap();

    // We need to work out what the start should have been, so we examine the four
    // adjacent pipes and work out which are pointing towards the start.
    let north = if start_y > 0 {
        *pipes.get(&(start_x, start_y - 1)).unwrap_or(&Ground)
    } else {
        Ground
    };
    let north = north == V || north == SE || north == SW;

    let east = *pipes.get(&(start_x + 1, start_y)).unwrap_or(&Ground);
    let east = east == H || east == NW || east == SW;

    let south = *pipes.get(&(start_x, start_y + 1)).unwrap_or(&Ground);
    let south = south == V || south == NE || south == NW;

    let west = if start_x > 0 {
        *pipes.get(&(start_x - 1, start_y)).unwrap_or(&Ground)
    } else {
        Ground
    };
    let west = west == H || west == NE || west == SE;

    let start_pipe = if north && east {
        NE
    } else if north && south {
        V
    } else if north && west {
        NW
    } else if east && south {
        SE
    } else if east && west {
        H
    } else if south && west {
        SW
    } else {
        panic!("Can't idenfity which way the start pipe goes");
    };
    pipes.insert((start_x, start_y), start_pipe);

    (pipes, (start_x, start_y))
}

#[cfg(test)]
mod test {
    use super::*;

    const PART1_EXAMPLE1: &str = include_str!("../pt1_example1.txt");
    const PART1_EXAMPLE2: &str = include_str!("../pt1_example2.txt");

    #[test]
    fn part_one() {
        assert_eq!(Grid::new(PART1_EXAMPLE1).furthest_distance(), 4);
        assert_eq!(Grid::new(PART1_EXAMPLE2).furthest_distance(), 8);
    }

    const PART2_EXAMPLE1: &str = include_str!("../pt2_example1.txt");
    const PART2_EXAMPLE2: &str = include_str!("../pt2_example2.txt");
    const PART2_EXAMPLE3: &str = include_str!("../pt2_example3.txt");

    #[test]
    fn part_two() {
        assert_eq!(Grid::new(PART2_EXAMPLE1).internal_area(), 4);
        assert_eq!(Grid::new(PART2_EXAMPLE2).internal_area(), 8);
        assert_eq!(Grid::new(PART2_EXAMPLE3).internal_area(), 10);
    }
}
