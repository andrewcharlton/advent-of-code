use itertools::Itertools;
use std::cmp;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

fn part_one(input: &str) -> usize {
    input.lines()
        .filter_map(|line| {
            let nums: Vec<i64> = line.split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect();
            if is_safe(&nums) { Some(()) } else { None }
        })
    .count()
}


fn part_two(input: &str) -> usize {
    input.lines()
        .filter_map(|line| {
            let nums: Vec<i64> = line.split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect();
            if is_safe(&nums) {
                return Some(());
            }

            for i in 0..nums.len() {
                let mut reduced_nums = nums.clone();
                reduced_nums.remove(i);
                if is_safe(&reduced_nums) {
                    return Some(());
                }
            }

            None
        })
    .count()
}

fn is_safe(nums: &Vec<i64>) -> bool {
    let mut up = 0;
    let mut down = 0;
    for (a, b) in nums.into_iter().tuple_windows() {
        let diff = a.abs_diff(*b);
        if diff == 0 || diff > 3 {
            return false;
        }

        if a > b {
            down += 1;
        } else if a < b {
            up += 1;
        }
    }

    return up == 0 || down == 0
}



#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(part_one(EXAMPLE), 2);
        assert_eq!(part_two(EXAMPLE), 4);
    }
}
