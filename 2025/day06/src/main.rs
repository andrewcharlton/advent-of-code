use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("couldn't read input file");
    println!("Part one: {}", solve(&input, false));
    println!("Part two: {}", solve(&input, true));
}

fn solve(input: &str, cephalopod: bool) -> u64 {
    let columns = parse_input(input);
    let mut total = 0;

    for mut col in columns {
        let op = col.pop().unwrap().trim();
        let nums = if cephalopod {
            parse_cephalopod(col)
        } else {
            parse_nums(col)
        };

        match op {
            "*" => total += nums.iter().product::<u64>(),
            "+" => total += nums.iter().sum::<u64>(),
            _ => panic!("unknown op: {}", op),
        }
    }

    total
}

fn parse_nums(col: Vec<&str>) -> Vec<u64> {
    col.iter().map(|num| num.trim().parse().unwrap()).collect()
}

fn parse_cephalopod(col: Vec<&str>) -> Vec<u64> {
    // Iterate through each digit separately.
    (0..col.get(0).unwrap().len())
        .map(|i| {
            col.iter().fold(0, |acc, row| {
                let c = row.chars().nth(i).unwrap();
                match c {
                    ' ' => acc,
                    _ => acc * 10 + (c.to_digit(10).unwrap() as u64),
                }
            })
        })
        .filter(|num| *num > 0)
        .collect()
}

fn parse_input(input: &str) -> Vec<Vec<&str>> {
    let mut lines: Vec<&str> = input.lines().collect();

    let mut op_positions: Vec<usize> = lines
        .last()
        .unwrap()
        .chars()
        .enumerate()
        .filter_map(|(x, c)| if c == ' ' { None } else { Some(x) })
        .collect();

    op_positions.reverse();

    op_positions
        .iter()
        .map(|i| {
            (0..lines.len())
                .map(|j| {
                    if *i == 0 {
                        return *lines.get(j).unwrap();
                    }

                    // Split the line at position i.
                    let line = lines.get_mut(j).unwrap();
                    let (l, r) = line.split_at(*i);
                    *line = l;
                    r
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = fs::read_to_string("example.txt").expect("couldn't read example file");
        assert_eq!(solve(&input, false), 4277556);
        assert_eq!(solve(&input, true), 3263827);
    }
}
