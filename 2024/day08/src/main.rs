use std::time::Instant;
use std::collections::{HashMap,HashSet};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let now = Instant::now();

    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));

    let elapsed_time = now.elapsed();
    println!("Solved in {}µs", elapsed_time.as_micros());
}

fn part_one(input: &str) -> usize {
    let (height, width, antennas) = parse_input(input);
    let mut locations: HashSet<(i64,i64)> = HashSet::new();

    for (_, v) in antennas {
        for i in 0..v.len()-1 {
            for j in i+1..v.len() {
                let a = v.get(i).unwrap();
                let b = v.get(j).unwrap();
                
                // b + (a -> b)
                let x = 2*b.0 - a.0;
                let y = 2*b.1 - a.1;
                if x >= 0 && x < width && y >= 0 && y < height {
                    locations.insert((x, y));
                }
                
                // a + (b -> a)
                let x = 2*a.0 - b.0;
                let y = 2*a.1 - b.1;
                if x >= 0 && x < width && y >= 0 && y < height {
                    locations.insert((x, y));
                }
            }
        }
    }

    locations.len()
}


fn part_two(input: &str) -> usize {
    let (height, width, antennas) = parse_input(input);
    let mut locations: HashSet<(i64,i64)> = HashSet::new();

    for (_, v) in antennas {
        for i in 0..v.len()-1 {
            for j in i+1..v.len() {
                let a = v.get(i).unwrap();
                let b = v.get(j).unwrap();

                let x_diff = b.0 - a.0;
                let y_diff = b.1 - a.1;

                let mut x = a.0;
                let mut y = a.1;
                while x >= 0 && x < width && y >= 0 && y < height {
                    locations.insert((x, y));
                    x += x_diff;
                    y += y_diff;
                }

                let mut x = a.0;
                let mut y = a.1;
                while x >= 0 && x < width && y >= 0 && y < height {
                    locations.insert((x, y));
                    x -= x_diff;
                    y -= y_diff;
                }
            }
        }
    }

    locations.len()
}

fn parse_input(input: &str) -> (i64, i64, HashMap<char, Vec<(i64, i64)>>) {
    let mut antennas: HashMap<char, Vec<(i64,i64)>> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            let x: i64 = x.try_into().unwrap();
            let y: i64 = y.try_into().unwrap();

            if let Some(pos) = antennas.get_mut(&c) {
                pos.push((x,y));
            } else {
                let pos = vec![(x,y)];
                antennas.insert(c, pos);
            }
        }
    }

    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len().try_into().unwrap();
    let width = lines.get(0).unwrap().len().try_into().unwrap();

    (height, width, antennas)
}


#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(part_one(EXAMPLE), 14);
        assert_eq!(part_two(EXAMPLE), 34);
    }
}
