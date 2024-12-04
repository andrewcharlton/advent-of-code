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
    let rows: usize = input.lines()
        .map(|line| count_occurences(line))
        .sum();

    let grid: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();

    let width = grid.get(0).unwrap().len();
    let columns: usize = (0..width)
        .map(|i| {
            let column: String = grid
                .iter()
                .map(|row| row.get(i).unwrap())
                .collect();
            column
        })
        .map(|column| count_occurences(&column))
        .sum();

    let width: i64 = width.try_into().unwrap();
    let mut diagonals: usize = 0;
    for dir in [-1, 1] {
        for start in -width..2*width+1 {
            let diagonal: String = grid
                .iter()
                .enumerate()
                .filter_map(|(y, row)| {
                    let x: i64 = start + (TryInto::<i64>::try_into(y).unwrap() * dir);
                    if x < 0 {
                        return None;
                    }

                    let x: usize = x.try_into().unwrap();
                    row.get(x)
                })
                .collect();

            diagonals += count_occurences(&diagonal);
        }
    }

    rows + columns + diagonals
}


fn part_two(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();

    let height = grid.len();
    let width = grid.get(0).unwrap().len();

    let mut count = 0;
    for y in 1..height-1 {
        for x in 1..width-1 {
            if *grid.get(y).unwrap().get(x).unwrap() != 'A' {
                continue;
            }

            let top_left = *grid.get(y-1).unwrap().get(x-1).unwrap();
            let bottom_right = *grid.get(y+1).unwrap().get(x+1).unwrap();
            if !(top_left == 'M' && bottom_right == 'S') && !(top_left == 'S' && bottom_right == 'M') {
                continue;
            }

            let top_right = *grid.get(y-1).unwrap().get(x+1).unwrap();
            let bottom_left = *grid.get(y+1).unwrap().get(x-1).unwrap();
            if !(top_right == 'M' && bottom_left == 'S') && !(top_right == 'S' && bottom_left == 'M') {
                continue;
            }

            count += 1;
        }
    }

    count
}

fn count_occurences(input: &str) -> usize {
    let forward = input.matches("XMAS").count();
    let backwards = input.matches("SAMX").count();
    forward + backwards
}


#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(part_one(EXAMPLE), 18);
        assert_eq!(part_two(EXAMPLE), 9);
    }
}
