use std::time::Instant;
use std::collections::VecDeque;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let now = Instant::now();

    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));

    let elapsed_time = now.elapsed();
    println!("Solved in {}µs", elapsed_time.as_micros());
}

fn part_one(input: &str) -> i64 {
    solve(input, false)
}


fn part_two(input: &str) -> i64 {
    solve(input, true)
}

fn solve(input: &str, concat: bool) -> i64 {
    input.lines()
        .map(|line| {
            let (target, nums) = line.split_once(":").unwrap();
            let target: i64 = target.parse().unwrap();
            let nums: Vec<i64> = nums.trim().split_whitespace().map(|num| num.parse().unwrap()).collect();
            (target, nums)
        })
        .filter_map(|(target, nums)| if can_make_target(target, nums.to_vec(), concat) > 0 { Some(target) } else { None })
            .sum()

}

fn can_make_target(target: i64, nums: Vec<i64>, concat: bool) -> usize {
    let mut nums = nums.clone();
    nums.reverse();

    let current = nums.pop().unwrap();
    can_make_target_rec(target, current, nums, concat)
}

fn can_make_target_rec(target: i64, current: i64, mut nums: Vec<i64>, concat: bool) -> usize {
    if current > target {
        return 0;
    }
    if nums.len() == 0 {
        if target == current {
            return 1;
        }
        return 0;
    }

    let n = nums.pop().unwrap();
    let mut result = can_make_target_rec(target, current+n, nums.clone(), concat) + can_make_target_rec(target, current*n, nums.clone(), concat);
    if concat {
        let pow = n.ilog10() + 1;
        let current = current * 10i64.pow(pow) + n;
        result += can_make_target_rec(target, current, nums.clone(), concat);
    }

    result
}


#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(part_one(EXAMPLE), 3749);
        assert_eq!(part_two(EXAMPLE), 11837);
    }
}
