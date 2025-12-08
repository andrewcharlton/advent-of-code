use std::cmp;
use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("couldn't read input file");
    println!("Part one: {}", part_one(&input, 1000));
    println!("Part two: {}", part_two(&input));
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn parse(input: &str) -> Self {
        let mut parts = input.trim().splitn(3, ',');
        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();
        let z = parts.next().unwrap().parse().unwrap();
        Point { x, y, z }
    }

    fn distance(&self, other: &Point) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

fn part_one(input: &str, join_limit: usize) -> usize {
    let (mut circuits, distances) = parse_input(&input);

    for i in 0..join_limit {
        let (a, b, _) = distances.get(i).unwrap();

        let circuit_a = circuits.get(&a).unwrap().clone();
        let circuit_b = circuits.get(&b).unwrap().clone();
        if circuit_a == circuit_b {
            continue;
        }

        let min = cmp::min(circuit_a, circuit_b);
        let max = cmp::max(circuit_a, circuit_b);

        for val in circuits.values_mut() {
            if *val == max {
                *val = min;
            }
        }
    }

    let mut circuit_sizes: HashMap<usize, usize> = HashMap::new();
    for (_, c) in circuits.iter() {
        let current = circuit_sizes.get(&c).unwrap_or(&0);
        circuit_sizes.insert(*c, current + 1);
    }

    let mut circuit_sizes: Vec<usize> = circuit_sizes.values().map(|v| *v).collect();
    circuit_sizes.sort();

    let mut product = 1;
    for _ in 0..3 {
        product *= circuit_sizes.pop().unwrap();
    }
    product
}

fn part_two(input: &str) -> i64 {
    let (mut circuits, distances) = parse_input(&input);

    let mut circuit_count = circuits.len();

    let mut i = 0;
    loop {
        let (a, b, _) = distances.get(i).unwrap();
        i += 1;

        let circuit_a = circuits.get(&a).unwrap().clone();
        let circuit_b = circuits.get(&b).unwrap().clone();
        if circuit_a == circuit_b {
            continue;
        }

        let min = cmp::min(circuit_a, circuit_b);
        let max = cmp::max(circuit_a, circuit_b);

        for val in circuits.values_mut() {
            if *val == max {
                *val = min;
            }
        }

        circuit_count -= 1;
        if circuit_count == 1 {
            return a.x * b.x;
        }
    }
}

fn parse_input(input: &str) -> (HashMap<Point, usize>, Vec<(Point, Point, i64)>) {
    let points: Vec<Point> = input.lines().map(Point::parse).collect();

    let circuits: HashMap<Point, usize> = points.iter().enumerate().map(|(i, p)| (*p, i)).collect();

    let mut distances: Vec<(Point, Point, i64)> = Vec::new();

    for i in 0..points.len() - 1 {
        for j in i + 1..points.len() {
            let a = points.get(i).unwrap().clone();
            let b = points.get(j).unwrap().clone();
            distances.push((a, b, a.distance(&b)));
        }
    }
    distances.sort_by(|a, b| a.2.cmp(&b.2));

    (circuits, distances)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = fs::read_to_string("example.txt").expect("couldn't read example file");
        assert_eq!(part_one(&input, 10), 40);
        assert_eq!(part_two(&input), 25272);
    }
}
