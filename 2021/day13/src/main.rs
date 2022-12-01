use std::collections::HashSet;
use std::fs;

fn main() {
    folds("input.txt");
}

fn folds(filename: &str) {
    let file = fs::read_to_string(filename).expect("couldn't open file");
    let (points, folds) = file.split_once("\n\n").unwrap();

    let mut points: HashSet<(i64, i64)> = points
        .lines()
        .map(|line| {
            let (x, y) = line.trim().split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    for fold in folds.lines() {
        let (dir, n) = fold
            .trim_start_matches("fold along ")
            .split_once("=")
            .unwrap();
        let n: i64 = n.parse().unwrap();

        let mut folded_points: HashSet<(i64, i64)> = HashSet::new();
        for (x, y) in &points {
            folded_points.insert(transform_point(*x, *y, dir, n));
        }
        println!("Points remaining: {}", folded_points.len());
        points = folded_points;
    }

    // Print the points
    let max_x = *points.iter().map(|(x, _)| x).max().unwrap();
    let max_y = *points.iter().map(|(_, y)| y).max().unwrap();

    for y in 0..=max_y {
        let mut line: String = String::new();
        for x in 0..=max_x {
            if points.contains(&(x, y)) {
                line.push('#');
            } else {
                line.push(' ');
            }
        }
        println!("{}", line);
    }
}

fn transform_point(x: i64, y: i64, dir: &str, n: i64) -> (i64, i64) {
    match dir {
        "x" => {
            if x < n {
                (x, y)
            } else {
                (2 * n - x, y)
            }
        }
        "y" => {
            if y < n {
                (x, y)
            } else {
                (x, 2 * n - y)
            }
        }
        _ => panic!("Unrecognised direction: '{}'", dir),
    }
}
