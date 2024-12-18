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
    get_regions(input).iter().map(|region| region.len() * perimeter(region)).sum()
}

fn part_two(input: &str) -> usize {
    get_regions(input).iter().map(|region| region.len() * sides(region)).sum()
}

fn get_regions(input: &str) -> Vec<HashSet<(i64, i64)>> {
    let mut positions: HashMap<(i64, i64), char> = input
        .lines()
        .enumerate()
        .map(|(y, line)| line.chars().enumerate().map(move |(x, c)| ((x.try_into().unwrap(), y.try_into().unwrap()), c)))
        .flatten()
        .collect();

    let lines: Vec<&str> = input.lines().collect();
    let height: i64 = lines.len().try_into().unwrap();
    let width: i64 = lines.get(0).unwrap().len().try_into().unwrap();

    let mut regions: Vec<HashSet<(i64, i64)>> = Vec::new();

    for x in 0..width {
        for y in 0..height {
            if !positions.contains_key(&(x,y)) {
                continue;
            }

            let current_char = positions.remove(&(x,y)).unwrap();
            let mut region: HashSet<(i64, i64)> = HashSet::new();
            region.insert((x,y));
            let mut last_added: HashSet<(i64, i64)> = HashSet::new();
            last_added.insert((x,y));

            while last_added.len() > 0 {
                let mut new_positions: HashSet<(i64,i64)> = HashSet::new();
                for pos in last_added {

                    for p in vec![(pos.0-1,pos.1), (pos.0+1, pos.1), (pos.0, pos.1-1), (pos.0, pos.1+1)] {
                        if let Some(c) = positions.get(&p) {
                            if *c == current_char {
                                region.insert(p);
                                new_positions.insert(p);
                                positions.remove(&p);
                            }
                        }
                    }
                }

                last_added = new_positions;
            }

            regions.push(region);
        }
    }

    regions
}

fn perimeter(region: &HashSet<(i64, i64)>) -> usize {
    region.iter().map(|(x, y)| {
        let neighbours: Vec<(i64,i64)> = vec![(x-1,*y), (x+1, *y), (*x,y-1), (*x,y+1)];
        neighbours.iter().filter(|pos| !region.contains(pos)).count()
    }).sum()
}

fn sides(region: &HashSet<(i64, i64)>) -> usize {
    let mut left: Vec<(i64, i64)> = Vec::new();
    let mut right: Vec<(i64, i64)> = Vec::new();
    let mut top: Vec<(i64, i64)> = Vec::new();
    let mut bottom: Vec<(i64, i64)> = Vec::new();

    for &(x, y) in region {
        if !region.contains(&(x-1, y)) {
            left.push((x,y));
        }
        if !region.contains(&(x+1,y)) {
            right.push((x,y));
        }

        if !region.contains(&(x,y-1)) {
            top.push((x, y));
        }
        if !region.contains(&(x,y+1)) {
            bottom.push((x,y+1));
        }
    }


    left.sort();
    right.sort();
    top.sort_by_key(|(x,y)| (*y, *x));
    bottom.sort_by_key(|(x,y)| (*y, *x));

    let mut left_sides = 1;
    for i in 1..left.len() {
        let p0 = left.get(i-1).unwrap();
        let p1 = left.get(i).unwrap();

        if p0.0 != p1.0 || p1.1 - p0.1 != 1 {
            left_sides += 1;
        }
    }
    let mut right_sides = 1;
    for i in 1..right.len() {
        let p0 = right.get(i-1).unwrap();
        let p1 = right.get(i).unwrap();

        if p0.0 != p1.0 || p1.1 - p0.1 != 1 {
            right_sides += 1;
        }
    }

    let mut top_sides = 1;
    for i in 1..top.len() {
        let p0 = top.get(i-1).unwrap();
        let p1 = top.get(i).unwrap();

        if p0.1 != p1.1 || p1.0 - p0.0 != 1 {
            top_sides += 1;
        }
    }
    let mut bottom_sides = 1;
    for i in 1..bottom.len() {
        let p0 = bottom.get(i-1).unwrap();
        let p1 = bottom.get(i).unwrap();

        if p0.1 != p1.1 || p1.0 - p0.0 != 1 {
            bottom_sides += 1;
        }
    }

    left_sides + right_sides + top_sides + bottom_sides
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(part_one(EXAMPLE), 1930);
        assert_eq!(part_two(EXAMPLE), 1206);
    }
}
