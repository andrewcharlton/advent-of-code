use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

fn part_one(input: &str) -> usize {
    let grid = Grid::parse(input);

    let mut all_visible: HashSet<(usize, usize)> = HashSet::new();

    for y in 0..grid.height {
        all_visible.extend(trees_visible_in_row(grid.row(y)).iter().map(|x| (*x, y)));
    }
    for x in 0..grid.width {
        all_visible.extend(trees_visible_in_row(grid.column(x)).iter().map(|y| (x, *y)));
    }

    all_visible.len()
}

fn trees_visible_in_row(trees: Vec<i8>) -> HashSet<usize> {
    // Set of all the positions of trees that are visible.
    let mut visible: HashSet<usize> = HashSet::new();

    // First, go forwards and check which ones are visible from the left/top
    let mut highest = -1;
    for (i, &height) in trees.iter().enumerate() {
        if height > highest {
            highest = height;
            visible.insert(i);
        }
    }

    // Now, go backwards and check which ones are visible from the right/bottom
    let mut highest = -1;
    for (i, &height) in trees.iter().enumerate().rev() {
        if height > highest {
            highest = height;
            visible.insert(i);
        }
    }

    visible
}

fn part_two(input: &str) -> usize {
    let grid = Grid::parse(input);

    let mut max: usize = 0;
    for x in 1..grid.width - 1 {
        for y in 1..grid.height - 1 {
            let row_score = trees_visible(grid.row(y), x);
            let column_score = trees_visible(grid.column(x), y);
            let score = row_score * column_score;
            if score > max {
                max = score;
            }
        }
    }

    max
}

fn trees_visible(trees: Vec<i8>, pos: usize) -> usize {
    let max_height = trees.get(pos).unwrap();

    let mut forward: usize = 0;
    for tree in trees[pos + 1..].iter() {
        forward += 1;
        if tree >= max_height {
            break;
        }
    }

    let mut backward: usize = 0;
    for tree in trees[..pos].iter().rev() {
        backward += 1;
        if tree >= max_height {
            break;
        }
    }

    forward * backward
}

struct Grid {
    rows: Vec<Vec<i8>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn parse(s: &str) -> Grid {
        let rows: Vec<Vec<i8>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as i8)
                    .collect::<Vec<i8>>()
            })
            .collect();

        let height = rows.len();
        let width = rows.first().unwrap().len();

        Grid {
            rows,
            height,
            width,
        }
    }

    fn row(&self, y: usize) -> Vec<i8> {
        self.rows.get(y).unwrap().to_vec()
    }

    fn column(&self, x: usize) -> Vec<i8> {
        self.rows.iter().map(|row| *row.get(x).unwrap()).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(part_one(EXAMPLE), 21);
        assert_eq!(part_two(EXAMPLE), 8);
    }
}
