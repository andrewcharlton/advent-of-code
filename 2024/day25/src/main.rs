use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

const HEIGHT: u8 = 7;

fn main() {
    let now = Instant::now();

    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));

    let elapsed_time = now.elapsed();
    println!("Solved in {}µs", elapsed_time.as_micros());
}

fn part_one(input: &str) -> usize {
    let schematics: Vec<Schematic> = input.split("\n\n").map(|s| Schematic::parse(s)).collect();

    let mut matches = 0;
    for i in 0..schematics.len() - 1 {
        let a = schematics.get(i).unwrap();
        for j in i + 1..schematics.len() {
            let b = schematics.get(j).unwrap();
            if a.can_unlock(b) {
                println!("Match: a: {:?}, b: {:?}", a, b);
                matches += 1;
            }
        }
    }

    matches
}

fn part_two(input: &str) -> i64 {
    0
}

#[derive(Clone, Debug)]
enum Schematic {
    Key(u8, u8, u8, u8, u8),
    Lock(u8, u8, u8, u8, u8),
}

impl Schematic {
    fn parse(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let is_lock = *lines.get(0).unwrap() == "#####";

        let mut columns: Vec<Option<u8>> = vec![None, None, None, None, None];
        for (i, line) in lines.iter().enumerate() {
            for (j, char) in line.chars().enumerate() {
                if char == '.' {
                    continue;
                }

                if !is_lock && columns.get(j).unwrap().is_some() {
                    continue;
                }

                if let Some(v) = columns.get_mut(j) {
                    let value: u8 = i.try_into().unwrap();
                    let value = if is_lock { value } else { HEIGHT - value - 1 };
                    *v = Some(value);
                }
            }
        }

        if is_lock {
            return Schematic::Lock(
                columns.get(0).unwrap().unwrap(),
                columns.get(1).unwrap().unwrap(),
                columns.get(2).unwrap().unwrap(),
                columns.get(3).unwrap().unwrap(),
                columns.get(4).unwrap().unwrap(),
            );
        }

        Schematic::Key(
            columns.get(0).unwrap().unwrap(),
            columns.get(1).unwrap().unwrap(),
            columns.get(2).unwrap().unwrap(),
            columns.get(3).unwrap().unwrap(),
            columns.get(4).unwrap().unwrap(),
        )
    }

    fn can_unlock(&self, other: &Schematic) -> bool {
        use Schematic::{Key, Lock};

        match (self, other) {
            (Key(_, _, _, _, _), Key(_, _, _, _, _)) => false,
            (Lock(_, _, _, _, _), Lock(_, _, _, _, _)) => false,
            (Key(a, b, c, d, e), Lock(v, w, x, y, z))
            | (Lock(a, b, c, d, e), Key(v, w, x, y, z)) => {
                a + v < HEIGHT - 1
                    && b + w < HEIGHT - 1
                    && c + x < HEIGHT - 1
                    && d + y < HEIGHT - 1
                    && e + z < HEIGHT - 1
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        for schematic in EXAMPLE.split("\n\n") {
            let schematic = Schematic::parse(schematic);
            println!("{:?}", schematic);
        }

        assert_eq!(part_one(EXAMPLE), 3);
        assert_eq!(part_two(EXAMPLE), 0);
    }
}
