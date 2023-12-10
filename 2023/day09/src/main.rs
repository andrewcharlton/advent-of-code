const INPUT: &str = include_str!("../input.txt");

fn main() {
    let (prev, next) = solve(INPUT);
    println!("Part one: {}", next);
    println!("Part two: {}", prev);
}

fn solve(input: &str) -> (i64, i64) {
    input
        .lines()
        .map(|line| line.split(" ").map(|n| n.parse::<i64>().unwrap()).collect())
        .map(|nums| diff(nums))
        .reduce(|acc, x| (acc.0 + x.0, acc.1 + x.1))
        .unwrap()
}

fn diff(input: Vec<i64>) -> (i64, i64) {
    if input.iter().all(|n| n == &0) {
        return (0, 0);
    }

    let mut deltas: Vec<i64> = Vec::new();
    for i in 0..input.len() - 1 {
        deltas.push(input.get(i + 1).unwrap() - input.get(i).unwrap());
    }

    let (prev, next) = diff(deltas);
    (input.first().unwrap() - prev, input.last().unwrap() + next)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let (prev, next) = solve(EXAMPLE);
        assert_eq!(next, 114);
        assert_eq!(prev, 2);
    }
}
