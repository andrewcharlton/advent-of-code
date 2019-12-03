use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("couldn't read file");
    let (min_distance, min_steps) = solve(&input);
    println!("Part one: {}", min_distance);
    println!("Part two: {}", min_steps);
}

fn solve(input: &str) -> (i64, i64) {
    let lines: Vec<&str> = input.lines().collect();

    let wire1 = parse_input(lines[0]);
    let wire2 = parse_input(lines[1]);

    println!("{:?}\n\n{:?}\n\n", wire1, wire2);

    let mut intersections: Vec<(Point, i64)> = Vec::new();
    let mut dist1 = 0;
    for s1 in &wire1 {
        let mut dist2 = 0;
        for s2 in &wire2 {
            match s1.bisects(&s2) {
                Some((p, d)) => intersections.push((p, d + dist1 + dist2)),
                None => {}
            }
            dist2 += s2.length();
        }
        dist1 += s1.length();
    }

    println!("{:?}", intersections);

    let min_distance = intersections
        .iter()
        .map(|(p, _)| p.x.abs() + p.y.abs())
        .min()
        .unwrap();

    let min_steps = intersections.iter().map(|(_, d)| *d).min().unwrap();

    (min_distance, min_steps)
}

fn parse_input(input: &str) -> Vec<Section> {
    let mut links = Vec::new();
    let mut p = Point { x: 0, y: 0 };

    for cmd in input.split(",") {
        let (next_p, dir) = next_point(&p, cmd);
        links.push(Section {
            start: p.clone(),
            end: next_p.clone(),
            dir,
        });
        p = next_p;
    }

    links
}

fn next_point(p: &Point, cmd: &str) -> (Point, Direction) {
    let (dir, n) = cmd.split_at(1);
    let n = n.parse::<i64>().unwrap();

    match dir {
        "R" => (Point { x: p.x + n, y: p.y }, Direction::Horizontal),
        "L" => (Point { x: p.x - n, y: p.y }, Direction::Horizontal),
        "U" => (Point { x: p.x, y: p.y + n }, Direction::Vertical),
        "D" => (Point { x: p.x, y: p.y - n }, Direction::Vertical),
        dir => panic!("unrecognised op: {}", dir),
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq)]
enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Debug, PartialEq)]
struct Section {
    dir: Direction,
    start: Point,
    end: Point,
}

impl Section {
    fn length(&self) -> i64 {
        (self.end.x - self.start.x).abs() + (self.end.y - self.start.y).abs()
    }

    fn bisects(&self, other: &Section) -> Option<(Point, i64)> {
        if self.dir == other.dir {
            return None;
        }

        let (h, v) = if self.dir == Direction::Horizontal {
            (self, other)
        } else {
            (other, self)
        };

        let (x, y) = (v.start.x, h.start.y);

        if (h.start.x - x) * (h.end.x - x) >= 0 || (v.start.y - y) * (v.end.y - y) >= 0 {
            return None;
        }

        let distance = (h.start.x - x).abs() + (v.start.y - y).abs();
        Some((Point { x, y }, distance))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "R8,U5,L5,D3\nU7,R6,D4,L4";
        let (min_dist, min_steps) = solve(&input);
        assert_eq!(6, min_dist, "min distance");
        assert_eq!(30, min_steps, "min_steps");
    }

    #[test]
    fn example2() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        let (min_dist, min_steps) = solve(&input);
        assert_eq!(159, min_dist, "min distance");
        assert_eq!(610, min_steps, "min_steps");
    }

    #[test]
    fn example3() {
        let input =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let (min_dist, min_steps) = solve(&input);
        assert_eq!(135, min_dist, "min distance");
        assert_eq!(410, min_steps, "min_steps");
    }
}
