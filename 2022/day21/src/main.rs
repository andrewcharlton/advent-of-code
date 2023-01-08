use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

fn part_one(input: &str) -> usize {
    let mut monkeys = parse_input(input);

    let mut ids: Vec<String> = monkeys
        .iter()
        .filter(|(_, m)| if let Monkey::Num(_) = m { false } else { true })
        .map(|(id, _)| id.to_owned())
        .collect();

    loop {
        for (id, m) in &mut monkeys.iter_mut() {
            match m {
                Monkey::Num(_) => {}
                Monkey::Add(a, b) => {
                    let monkey_a = monkeys.get(&a.clone()).unwrap();
                    let monkey_b = monkeys.get(&b.clone()).unwrap();
                    match (monkey_a, monkey_b) {
                        (Monkey::Num(x), Monkey::Num(y)) => {
                            monkeys.insert(id.to_string(), Monkey::Num(x + y));
                        }
                        _ => {}
                    }
                }
                Monkey::Sub(a, b) => {
                    let monkey_a = monkeys.get(&a.clone()).unwrap();
                    let monkey_b = monkeys.get(&b.clone()).unwrap();
                    match (monkey_a, monkey_b) {
                        (Monkey::Num(x), Monkey::Num(y)) => {
                            monkeys.insert(id.to_string(), Monkey::Num(x - y));
                        }
                        _ => {}
                    }
                }
                Monkey::Mul(a, b) => {
                    let monkey_a = monkeys.get(&a.clone()).unwrap();
                    let monkey_b = monkeys.get(&b.clone()).unwrap();
                    match (monkey_a, monkey_b) {
                        (Monkey::Num(x), Monkey::Num(y)) => {
                            monkeys.insert(id.to_string(), Monkey::Num(x * y));
                        }
                        _ => {}
                    }
                }
                Monkey::Div(a, b) => {
                    let monkey_a = monkeys.get(&a.clone()).unwrap();
                    let monkey_b = monkeys.get(&b.clone()).unwrap();
                    match (monkey_a, monkey_b) {
                        (Monkey::Num(x), Monkey::Num(y)) => {
                            monkeys.insert(id.to_string(), Monkey::Num(x / y));
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn part_two(input: &str) -> usize {
    0
}

enum Monkey {
    Num(i64),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

fn parse_input(s: &str) -> HashMap<String, Monkey> {
    s.lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let id = parts[0].to_string();

            if let Ok(x) = parts[1].parse::<i64>() {
                return (id, Monkey::Num(x));
            }

            let parts: Vec<&str> = parts[1].split(" ").collect();
            match parts[1] {
                "+" => (id, Monkey::Add(parts[0].to_string(), parts[2].to_string())),
                "-" => (id, Monkey::Sub(parts[0].to_string(), parts[2].to_string())),
                "*" => (id, Monkey::Mul(parts[0].to_string(), parts[2].to_string())),
                "/" => (id, Monkey::Div(parts[0].to_string(), parts[2].to_string())),
                _ => panic!("unknown op: {}", parts[1]),
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(part_one(EXAMPLE), 0);
        assert_eq!(part_two(EXAMPLE), 0);
    }
}
