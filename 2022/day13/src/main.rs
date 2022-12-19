use serde_json::Value;
use std::cmp::Ordering;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

fn part_one(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();

    lines
        .chunks(3)
        .enumerate()
        .filter_map(|(i, chunk)| {
            let left = Packet::parse(chunk[0]);
            let right = Packet::parse(chunk[1]);
            let ord = left.cmp(&right);
            if ord == Ordering::Greater {
                return None;
            }
            Some(i + 1)
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    let mut packets: Vec<Packet> = input
        .lines()
        .filter_map(|line| {
            if line == "" {
                return None;
            }

            Some(Packet::parse(line))
        })
        .collect();

    let key0 = Packet::List(vec![Packet::List(vec![Packet::Num(2)])]);
    let key1 = Packet::List(vec![Packet::List(vec![Packet::Num(6)])]);

    packets.push(key0.clone());
    packets.push(key1.clone());
    packets.sort();

    packets.iter().enumerate().fold(1, |acc, (i, p)| {
        if *p == key0 || *p == key1 {
            println!("Found divider: {}", i + 1);
            acc * (i + 1)
        } else {
            acc
        }
    })
}

#[derive(Debug, Eq, Clone)]
enum Packet {
    Num(i64),
    List(Vec<Packet>),
}

impl Packet {
    fn parse(s: &str) -> Packet {
        let v: Value = serde_json::from_str(s).unwrap();
        Packet::from_value(&v)
    }

    fn from_value(v: &Value) -> Packet {
        match v {
            Value::Array(vals) => {
                let packets: Vec<Packet> = vals.iter().map(|v| Packet::from_value(v)).collect();
                Packet::List(packets)
            }
            Value::Number(n) => Packet::Num(n.as_i64().unwrap()),
            _ => panic!("Must be number of array"),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        use Packet::*;

        match (self, other) {
            (Num(x), Num(y)) => x.cmp(y),
            (Num(_), List(ys)) => {
                if ys.len() == 0 {
                    return Ordering::Greater;
                }

                let ord = self.cmp(ys.get(0).unwrap());
                if ord == Ordering::Equal {
                    // If they are equal, we need to work out whether the right will
                    // run out first
                    if ys.len() == 1 {
                        return Ordering::Equal;
                    }
                    return Ordering::Less;
                }

                ord
            }

            (List(xs), Num(_)) => {
                if xs.len() == 0 {
                    return Ordering::Less;
                }

                let ord = xs.get(0).unwrap().cmp(other);
                if ord == Ordering::Equal {
                    // If they are equal, the
                    // run out first
                    if xs.len() == 1 {
                        return Ordering::Equal;
                    }
                    return Ordering::Greater;
                }

                ord
            }

            (List(xs), List(ys)) => {
                for (i, x) in xs.iter().enumerate() {
                    let y = ys.get(i);
                    if y.is_none() {
                        // Right side ran out first
                        return Ordering::Greater;
                    }

                    let ord = x.cmp(y.unwrap());
                    if ord != Ordering::Equal {
                        return ord;
                    }
                }

                // Left side has run out of items, check whether right is out too.
                if xs.len() == ys.len() {
                    return Ordering::Equal;
                }

                // Otherwise left side ran out first.
                Ordering::Less
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn parsing() {
        let line = EXAMPLE.lines().next().unwrap();

        let p = Packet::parse(line);
        let exp: Packet = Packet::List(vec![
            Packet::Num(1),
            Packet::Num(1),
            Packet::Num(3),
            Packet::Num(1),
            Packet::Num(1),
        ]);
        assert_eq!(exp, p);
    }

    #[test]
    fn example() {
        assert_eq!(part_one(EXAMPLE), 13);
        assert_eq!(part_two(EXAMPLE), 140);
    }
}
