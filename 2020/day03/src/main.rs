use std::fs;

fn main() {
    let lines: Vec<Vec<bool>> = fs::read_to_string("input")
        .expect("unable to read file")
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    let mut r = trees_hit(&lines, 3, 1);
    println!("Part one: {}", r);

    r *= trees_hit(&lines, 1, 1);
    r *= trees_hit(&lines, 5, 1);
    r *= trees_hit(&lines, 7, 1);
    r *= trees_hit(&lines, 1, 2);
    println!("Part two: {}", r);
}

fn trees_hit(lines: &[Vec<bool>], x: usize, y: usize) -> usize {
    let width = lines[0].len();

    lines
        .iter()
        .enumerate()
        .filter(|(i, line)| i % y == 0 && *line.get((i / y * x) % width).unwrap())
        .count()
}
