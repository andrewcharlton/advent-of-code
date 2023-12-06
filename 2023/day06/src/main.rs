const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", solve_pt1(INPUT));
    println!("Part two: {}", solve_pt2(INPUT));
}

fn solve_pt1(s: &str) -> u64 {
    let mut lines = s.lines();
    let times = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|d| d.parse().unwrap());

    let distances = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|d| d.parse().unwrap());

    times
        .zip(distances)
        .map(|(t, d): (u64, u64)| num_ways(t, d))
        .product()
}

fn solve_pt2(s: &str) -> u64 {
    let mut lines = s.lines();
    let time: u64 = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .replace(" ", "")
        .parse()
        .unwrap();

    let distance: u64 = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .replace(" ", "")
        .parse()
        .unwrap();

    num_ways(time, distance)
}

fn num_ways(t: u64, d: u64) -> u64 {
    // If we hold the boat for x seconds, then it's speed is x and
    // the time it travels is (t-x), so we're solving the equation
    // x(t-x) > d.
    // Rearranging, we get: n^2 - tx + d < 0
    // This is a standard quadratic so solution is:
    //   t ± √t^2 - 4d
    //   -------------
    //        2
    let det = t * t - 4 * d;
    let sqrt = (det as f64).sqrt();

    let low: u64 = unsafe { ((t as f64 - sqrt) / 2.0).ceil().to_int_unchecked() };
    let high: u64 = unsafe { ((t as f64 + sqrt) / 2.0).floor().to_int_unchecked() };

    // In the case of an exact solution, we only equal the target not beat it when x
    // is high or low, so we can't include those end two points.
    if low * (t - low) == d {
        return high - low - 1;
    }

    // Otherwise we subtract to find the difference - +1 to make it inclusive of low.
    high - low + 1
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(solve_pt1(EXAMPLE), 288);
        assert_eq!(solve_pt2(EXAMPLE), 71503);
    }
}
