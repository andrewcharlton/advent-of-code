use std::fs;

fn main() {
    let mut entries: Vec<i64> = fs::read_to_string("input")
        .expect("couldn't open file")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    entries.sort_by(|a, b| b.cmp(a));

    println!("Part one: {}", expense_sum(&entries, 2020, 2).unwrap_or(0));
    println!("Part two: {}", expense_sum(&entries, 2020, 3).unwrap_or(0));
}

fn expense_sum(entries: &[i64], total: i64, depth: i8) -> Option<i64> {
    if depth == 1 {
        return entries.iter().find(|&&x| x == total).map(|&x| x);
    }

    entries.iter().enumerate().find_map(|(i, &x)| {
        if x < total {
            expense_sum(&entries[i + 1..entries.len()], total - x, depth - 1).map(|y| x * y)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expense_sum_test() {
        let mut entries = vec![1721, 979, 366, 299, 675, 1456];
        entries.sort_by(|a, b| b.cmp(a));

        assert_eq!(expense_sum(&entries, 2020, 2), Some(514579));
        assert_eq!(expense_sum(&entries, 2020, 3), Some(241861950));
    }
}
