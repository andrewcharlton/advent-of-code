use rand::random;
use std::collections::HashMap;
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let now = Instant::now();

    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));

    let elapsed_time = now.elapsed();
    println!("Solved in {}µs", elapsed_time.as_micros());
}

fn part_one(input: &str) -> usize {
    let mut grid = Grid::new(input);
    grid.connect();
    grid.value('z')
}

fn part_two(input: &str) -> String {
    let mut grid = Grid::new(input);
    grid.wires = HashMap::new();

    // Populated manually by searching for a set of digits that didn't work.
    let swaps: Vec<(Wire,Wire)> = vec![
        (['z', '0', '6'], ['v', 'w', 'r']),
        (['t', 'q', 'm'], ['z', '1', '1']),
        (['h', 'c', 'm'], ['g', 'f', 'v']),
        (['z', '1', '6'], ['k', 'f', 's']),
    ];
    for gate in grid.gates.iter_mut() {
        for swap in &swaps {
            if gate.out == swap.0 {
                gate.out = swap.1;
                continue;
            }
            if gate.out == swap.1 {
                gate.out = swap.0;
            }
        }
    }

    let mut all_gates = grid.gates.clone();
    let mut pow = 2;
    for i in 0..44 {
        let x = [
            'x',
            char::from_digit(i / 10, 10).unwrap(),
            char::from_digit(i % 10, 10).unwrap(),
        ];
        grid.wires.insert(x, random());

        let y = [
            'y',
            char::from_digit(i / 10, 10).unwrap(),
            char::from_digit(i % 10, 10).unwrap(),
        ];
        grid.wires.insert(y, random());

        grid.connect();
        let x = grid.value('x');
        let y = grid.value('y');
        let z = grid.value('z');

        if (x + y) % pow != z % pow {
            println!("{}: {} + {} != {}", i, x, y, z);
            println!("Gates used:");
            for g in all_gates {
                if grid.gates.contains(&g) {
                    continue;
                }

                println!(
                    "{}{}{} {:?} {}{}{} -> {}{}{}",
                    g.a[0],
                    g.a[1],
                    g.a[2],
                    g.op,
                    g.b[0],
                    g.b[1],
                    g.b[2],
                    g.out[0],
                    g.out[1],
                    g.out[2]
                );
            }

            return String::from("");
        }

        pow *= 2;
        all_gates = grid.gates.clone();
    }

    let mut swaps: Vec<String> = swaps.into_iter().map(|(a, b)| {
        let a: String = a.iter().collect();
        let b: String = b.iter().collect();
        vec![a, b]
    }).flatten().collect();
    swaps.sort();
    swaps.join(",")
}

type Wire = [char; 3];

#[derive(Debug, Clone, PartialEq)]
enum Op {
    AND,
    OR,
    XOR,
}

#[derive(Debug, Clone, PartialEq)]
struct Gate {
    a: Wire,
    b: Wire,
    op: Op,
    out: Wire,
}

#[derive(Debug)]
struct Grid {
    wires: HashMap<Wire, bool>,
    gates: Vec<Gate>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let (wires, gates) = input.split_once("\n\n").unwrap();

        let wires: HashMap<Wire, bool> = wires
            .lines()
            .map(|line| {
                let (wire, value) = line.split_once(": ").unwrap();

                let mut wire = wire.chars();
                let wire = [
                    wire.next().unwrap(),
                    wire.next().unwrap(),
                    wire.next().unwrap(),
                ];

                let value = value == "1";

                (wire, value)
            })
            .collect();

        let gates: Vec<Gate> = gates
            .lines()
            .map(|line| {
                let mut parts = line.split_whitespace();

                let mut a = parts.next().unwrap().chars();
                let a = [a.next().unwrap(), a.next().unwrap(), a.next().unwrap()];

                let op = parts.next().unwrap();
                let op = match op {
                    "AND" => Op::AND,
                    "OR" => Op::OR,
                    "XOR" => Op::XOR,
                    _ => panic!("unrecognised op {}", op),
                };

                let mut b = parts.next().unwrap().chars();
                let b = [b.next().unwrap(), b.next().unwrap(), b.next().unwrap()];

                parts.next();

                let mut out = parts.next().unwrap().chars();
                let out = [
                    out.next().unwrap(),
                    out.next().unwrap(),
                    out.next().unwrap(),
                ];

                Gate { a, b, op, out }
            })
            .collect();

        Grid { wires, gates }
    }

    fn connect(&mut self) {
        while self.gates.len() > 0 {
            let mut found = false;
            for i in 0..self.gates.len() {
                let gate = self.gates.get(i).unwrap();
                let a = self.wires.get(&gate.a);
                if a.is_none() {
                    continue;
                }

                let b = self.wires.get(&gate.b);
                if b.is_none() {
                    continue;
                }

                found = true;
                let a = *a.unwrap();
                let b = *b.unwrap();
                let out = match gate.op {
                    Op::AND => a && b,
                    Op::OR => a || b,
                    Op::XOR => (a && !b) || (!a && b),
                };

                self.wires.insert(gate.out, out);
                self.gates.remove(i);
                break;
            }

            if !found {
                return;
            }
        }
    }

    fn value(&self, prefix: char) -> usize {
        let mut output_bits: Vec<(Wire, usize)> = self
            .wires
            .clone()
            .into_iter()
            .filter(|(w, _)| w[0] == prefix)
            .map(|(w, b)| (w, if b { 1 } else { 0 }))
            .collect();
        output_bits.sort();
        output_bits.reverse();
        output_bits
            .into_iter()
            .map(|(_, n)| n)
            .reduce(|acc, x| 2 * acc + x)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE1: &str = include_str!("../example1.txt");
    const EXAMPLE2: &str = include_str!("../example2.txt");

    #[test]
    fn example() {
        assert_eq!(part_one(EXAMPLE1), 4);
        assert_eq!(part_one(EXAMPLE2), 2024);
    }
}
