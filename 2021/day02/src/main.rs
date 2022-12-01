use std::fs;
use std::str::FromStr;

fn main() {
    let pos = part_one("input.txt");
    println!("Part one: {}x{}={}", pos.0, pos.1, pos.0 * pos.1);

    let pos = part_two("input.txt");
    println!("Part one: {}x{}={}", pos.0, pos.1, pos.0 * pos.1);
}

enum Instruction {
    Forward(i64),
    Up(i64),
    Down(i64),
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();

        let i = parts[1].parse::<i64>().unwrap();
        match parts[0] {
            "forward" => Ok(Instruction::Forward(i)),
            "up" => Ok(Instruction::Up(i)),
            "down" => Ok(Instruction::Down(i)),
            _ => Err("unknown instruction"),
        }
    }
}

fn parse_file(filename: &str) -> Vec<Instruction> {
    fs::read_to_string(filename)
        .expect("couldn't open file")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn part_one(filename: &str) -> (i64, i64) {
    parse_file(filename)
        .iter()
        .fold((0, 0), |(x, y), ins| match ins {
            Instruction::Forward(i) => (x + i, y),
            Instruction::Up(i) => (x, y - i),
            Instruction::Down(i) => (x, y + i),
        })
}

fn part_two(filename: &str) -> (i64, i64, i64) {
    parse_file(filename)
        .iter()
        .fold((0, 0, 0), |(x, y, aim), ins| match ins {
            Instruction::Forward(i) => (x + i, y + aim * i, aim),
            Instruction::Up(i) => (x, y, aim - i),
            Instruction::Down(i) => (x, y, aim + i),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part_one("example.txt"), (15, 10), "part one");
        assert_eq!(part_two("example.txt"), (15, 60, 10), "part two");
    }
}
