use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", solve_pt1(INPUT));
    println!("Part two: {}", solve_pt2(INPUT));
}

fn solve_pt1(input: &str) -> usize {
    input.trim().split(",").map(hash).sum()
}

fn solve_pt2(input: &str) -> usize {
    let mut boxes: Vec<Vec<(String, usize)>> = Vec::with_capacity(256);
    for _ in 0..256 {
        boxes.push(Vec::new());
    }

    let re: Regex = Regex::new(r"(.*)([-=])(\d?)").unwrap();
    for op in input.trim().split(",") {
        let caps = re.captures(op).unwrap();
        let label = caps.get(1).unwrap().as_str();
        let op = caps.get(2).unwrap().as_str();
        let box_num = hash(label);

        let b = boxes.get_mut(box_num).unwrap();
        if op == "=" {
            let num: usize = caps.get(3).unwrap().as_str().parse().unwrap();
            let mut found = false;
            for lens in b.iter_mut() {
                if lens.0 == label {
                    lens.1 = num;
                    found = true;
                    break;
                }
            }
            if !found {
                b.push((label.to_string(), num));
            }

            continue;
        }

        if let Some(i) = b
            .iter()
            .enumerate()
            .find(|(_, lens)| lens.0 == label)
            .map(|(i, _)| i)
        {
            b.remove(i);
        }
    }

    let mut total = 0;
    for (i, b) in boxes.iter().enumerate() {
        for (j, lens) in b.iter().enumerate() {
            total += (i + 1) * (j + 1) * lens.1;
        }
    }

    total
}

fn hash(s: &str) -> usize {
    s.chars().fold(0, |acc, x| ((acc + x as usize) * 17) % 256)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_example() {
        assert_eq!(solve_pt1(EXAMPLE), 1320);
        assert_eq!(solve_pt2(EXAMPLE), 145);
    }

    #[test]
    fn test_hash() {
        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("cm-"), 253);
        assert_eq!(hash("qp=3"), 97);
        assert_eq!(hash("cm=2"), 47);
        assert_eq!(hash("qp-"), 14);
        assert_eq!(hash("pc=4"), 180);
        assert_eq!(hash("ot=9"), 9);
        assert_eq!(hash("ab=5"), 197);
        assert_eq!(hash("pc-"), 48);
        assert_eq!(hash("pc=6"), 214);
        assert_eq!(hash("ot=7"), 231);
    }
}
