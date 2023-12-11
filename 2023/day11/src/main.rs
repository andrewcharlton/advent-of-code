use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", min_path_length(INPUT, 2));
    println!("Part two: {}", min_path_length(INPUT, 1000000));
}

fn min_path_length(input: &str, expansion: u64) -> u64 {
    let galaxies = parse_input(input);
    let populated_columns: HashSet<usize> = galaxies.iter().map(|(x, _)| *x).collect();
    let populated_rows: HashSet<usize> = galaxies.iter().map(|(_, y)| *y).collect();

    let mut total_dist = 0;

    for i in 0..galaxies.len() - 1 {
        for j in i + 1..galaxies.len() {
            let a = galaxies.get(i).unwrap();
            let b = galaxies.get(j).unwrap();

            let columns_traversed = if a.0 > b.0 { (b.0, a.0) } else { (a.0, b.0) };
            for c in columns_traversed.0..columns_traversed.1 {
                if populated_columns.contains(&c) {
                    total_dist += 1;
                } else {
                    total_dist += expansion;
                }
            }

            let rows_traversed = if a.1 > b.1 { (b.1, a.1) } else { (a.1, b.1) };
            for r in rows_traversed.0..rows_traversed.1 {
                if populated_rows.contains(&r) {
                    total_dist += 1;
                } else {
                    total_dist += expansion;
                }
            }
        }
    }

    total_dist
}

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| if c == '#' { Some((x, y)) } else { None })
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(min_path_length(EXAMPLE, 2), 374);
        assert_eq!(min_path_length(EXAMPLE, 10), 1030);
        assert_eq!(min_path_length(EXAMPLE, 100), 8410);
    }
}
