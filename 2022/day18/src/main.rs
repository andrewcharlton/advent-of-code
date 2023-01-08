use itertools::Itertools;
use std::num::ParseIntError;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

fn part_one(input: &str) -> usize {
    let cubes: Vec<Cube> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut count: usize = 0;

    let mut cubes_sorted_for_x: Vec<Cube> = cubes.clone();
    cubes_sorted_for_x.sort_by_key(|cube| (cube.y, cube.z, cube.x));
    for (_, group) in &cubes_sorted_for_x
        .into_iter()
        .group_by(|cube| (cube.y, cube.z))
    {
        let xs: Vec<u8> = group.into_iter().map(|cube| cube.x).collect();
        count += count_consecutive(xs);
    }

    let mut cubes_sorted_for_y: Vec<Cube> = cubes.clone();
    cubes_sorted_for_y.sort_by_key(|cube| (cube.x, cube.z, cube.y));
    for (_, group) in &cubes_sorted_for_y
        .into_iter()
        .group_by(|cube| (cube.x, cube.z))
    {
        let ys: Vec<u8> = group.into_iter().map(|cube| cube.y).collect();
        count += count_consecutive(ys);
    }

    let mut cubes_sorted_for_z: Vec<Cube> = cubes.clone();
    cubes_sorted_for_z.sort_by_key(|cube| (cube.x, cube.y, cube.z));
    for (_, group) in &cubes_sorted_for_z
        .into_iter()
        .group_by(|cube| (cube.x, cube.y))
    {
        let zs: Vec<u8> = group.into_iter().map(|cube| cube.z).collect();
        count += count_consecutive(zs);
    }

    6 * cubes.len() - 2 * count
}

fn part_two(input: &str) -> usize {
    0
}

fn count_consecutive(vals: Vec<u8>) -> usize {
    let mut curr: Option<u8> = None;
    let mut count: usize = 0;

    for v in vals {
        if let Some(x) = curr {
            if v == x + 1 {
                count += 1;
            }
        }

        curr = Some(v)
    }

    count
}

#[derive(Clone)]
struct Cube {
    x: u8,
    y: u8,
    z: u8,
}

impl FromStr for Cube {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(3, ",").collect();

        let x: u8 = parts[0].parse()?;
        let y: u8 = parts[1].parse()?;
        let z: u8 = parts[2].parse()?;
        Ok(Cube { x, y, z })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(part_one(EXAMPLE), 64);
        assert_eq!(part_two(EXAMPLE), 0);
    }
}
