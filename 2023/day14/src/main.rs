use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", solve_pt1(INPUT));
    println!("Part two: {}", solve_pt2(INPUT));
}

fn solve_pt1(input: &str) -> usize {
    let rows: Vec<String> = input.lines().map(|line| line.to_string()).collect();
    let rows = tilt_vertical(&rows, false);
    load(&rows)
}

fn solve_pt2(input: &str) -> usize {
    let mut seen: HashMap<String, usize> = HashMap::new();
    let mut rows: Vec<String> = input.lines().map(|line| line.to_string()).collect();
    let mut cycle_num = 0;

    loop {
        let combined = rows.join("\n");
        if let Some(prev_cycle_num) = seen.get(&combined) {
            println!("Found cycle: {} -> {}", prev_cycle_num, cycle_num);
            let cycle_len = cycle_num - prev_cycle_num;
            let n = prev_cycle_num + (1000000000 - prev_cycle_num) % cycle_len;
            let pattern: Vec<String> = seen
                .iter()
                .find(|(_, v)| v == &&n)
                .map(|(k, _)| k)
                .unwrap()
                .split("\n")
                .map(|line| line.to_string())
                .collect();

            return load(&pattern);

            // We've seen this before, so we're now just stuck in a loop
        } else {
            seen.insert(combined, cycle_num);
        }

        rows = cycle(&rows);
        cycle_num += 1;
    }
}

fn load(rows: &Vec<String>) -> usize {
    let height = rows.len();
    rows.iter()
        .enumerate()
        .map(|(i, row)| (height - i) * row.chars().filter(|c| c == &'O').count())
        .sum()
}

fn cycle(rows: &Vec<String>) -> Vec<String> {
    let rows = tilt_vertical(&rows, false);
    let rows = tilt_horizontal(&rows, false);
    let rows = tilt_vertical(&rows, true);
    let rows = tilt_horizontal(&rows, true);
    rows
}

fn tilt_vertical(rows: &Vec<String>, reverse: bool) -> Vec<String> {
    let rows = transpose(rows);
    let rows = tilt_horizontal(&rows, reverse);
    transpose(&rows)
}

fn tilt_horizontal(rows: &Vec<String>, reverse: bool) -> Vec<String> {
    rows.iter().map(|row| tilt(row, reverse)).collect()
}

fn transpose(rows: &Vec<String>) -> Vec<String> {
    let mut transposed: Vec<String> = Vec::new();

    for line in rows {
        for (i, c) in line.chars().enumerate() {
            if let Some(s) = transposed.get_mut(i) {
                s.push(c);
            } else {
                transposed.push(c.to_string());
            }
        }
    }

    transposed
}

fn tilt(row: &str, reverse: bool) -> String {
    row.split("#")
        .map(|chunk| {
            let mut chars: Vec<char> = chunk.chars().collect();
            chars.sort_by(|a, b| if reverse { a.cmp(b) } else { b.cmp(a) });
            chars.into_iter().collect()
        })
        .collect::<Vec<String>>()
        .join(&"#")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tilt() {
        assert_eq!(tilt(".OO.O#.O.#..O", false), "OOO..#O..#O..".to_string());
        assert_eq!(tilt(".OO.O#.O.#..O", true), "..OOO#..O#..O".to_string());
    }
}
