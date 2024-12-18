use std::collections::HashSet;
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
    solve(input, 100, 101, 103)
}

fn part_two(input: &str) -> i64 {
    let mut scores: Vec<(i64, usize)> = Vec::new();
    for i in 1..10_000 {
        scores.push((i, solve(input, i, 101, 103)));
    }
    scores.sort_by_key(|k| k.1);

    let s = scores.get(0).unwrap();
    println!("Time: {}, Score: {}", s.0, s.1);

    visualise(input, s.0, 101, 103);

    s.0
}

fn solve(input: &str, time: i64, width: i64, height: i64) -> usize {
    let (mut q0, mut q1, mut q2, mut q3) = (0, 0, 0, 0);
    for line in input.lines() {
        match quadrant(line, time, width, height) {
            Some(0) => {
                q0 += 1;
            }
            Some(1) => {
                q1 += 1;
            }
            Some(2) => {
                q2 += 1;
            }
            Some(3) => {
                q3 += 1;
            }
            None => {}
            Some(x) => {
                println!("Unexpected quadrant {}: {}", line, x);
            }
        }
    }

    q0 * q1 * q2 * q3
}

fn quadrant(line: &str, time: i64, width: i64, height: i64) -> Option<i64> {
    let (x, y) = position(line, time, width, height);

    if x == width / 2 {
        return None;
    }
    if y == height / 2 {
        return None;
    }

    Some((2 * x / width) + 2 * (2 * y / height))
}

fn position(line: &str, time: i64, width: i64, height: i64) -> (i64, i64) {
    let (p_x, p_y, v_x, v_y) = parse_line(line);

    let x = (width + (p_x + v_x * time) % width) % width;
    let y = (height + (p_y + v_y * time) % height) % height;
    (x, y)
}

fn parse_line(line: &str) -> (i64, i64, i64, i64) {
    let mut parts = line.split_whitespace();

    let mut p = parts.next().unwrap().strip_prefix("p=").unwrap().split(",");
    let p_x = p.next().unwrap().parse().unwrap();
    let p_y = p.next().unwrap().parse().unwrap();

    let mut v = parts.next().unwrap().strip_prefix("v=").unwrap().split(",");
    let v_x = v.next().unwrap().parse().unwrap();
    let v_y = v.next().unwrap().parse().unwrap();

    (p_x, p_y, v_x, v_y)
}

fn visualise(input: &str, time: i64, width: i64, height: i64) {
    let mut positions: HashSet<(i64, i64)> = HashSet::new();

    for line in input.lines() {
        let pos = position(line, time, width, height);
        positions.insert(pos);
    }

    for y in 0..height {
        let line: String = (0..width)
            .map(|x| {
                if positions.contains(&(x, y)) {
                    '#'
                } else {
                    ' '
                }
            })
            .collect();
        println!("{}", line);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(solve(EXAMPLE, 100, 11, 7), 12);
        assert_eq!(part_two(EXAMPLE), 0);
    }
}
