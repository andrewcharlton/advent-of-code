use std::ops::BitXor;
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let now = Instant::now();

    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));

    let elapsed_time = now.elapsed();
    println!("Solved in {}µs", elapsed_time.as_micros());
}

fn part_one(input: &str) -> String {
    let mut c = Computer::new(input);
    let output = c.run();
    let output: Vec<String> = output.iter().map(|x| x.to_string()).collect();
    output.join(",")
}

fn part_two(input: &str) -> i64 {
    let original = Computer::new(input);

    let mut v = 0;

    let mut target: Vec<i64> = Vec::new();

    'outer: for i in 0..original.program.len() {
        v *= 8; // Both my input and the example divide a by 8 between repetitions of the loop, so
                // just hard code it here.
        target.insert(
            0,
            *original
                .program
                .get(original.program.len() - i - 1)
                .unwrap(),
        );

        let mut j = 0;
        loop {
            let mut c = original.clone();
            c.a = v + j;
            let output = c.run();
            if target == output {
                v += j;
                continue 'outer;
            }

            j += 1;
        }
    }

    v
}

#[derive(Clone)]
struct Computer {
    a: i64,
    b: i64,
    c: i64,

    program: Vec<i64>,
}

impl Computer {
    fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();

        let a: i64 = Self::get_register(lines.get(0));
        let b: i64 = Self::get_register(lines.get(0));
        let c: i64 = Self::get_register(lines.get(0));

        let program: Vec<i64> = lines
            .get(4)
            .unwrap()
            .split(":")
            .last()
            .unwrap()
            .trim()
            .split(",")
            .map(|d| d.parse().unwrap())
            .collect();

        Computer { a, b, c, program }
    }

    fn get_register(line: Option<&&str>) -> i64 {
        line.unwrap()
            .split(":")
            .last()
            .unwrap()
            .trim()
            .parse()
            .unwrap()
    }

    fn run(&mut self) -> Vec<i64> {
        let mut output: Vec<i64> = Vec::new();

        let mut idx = 0;
        loop {
            let (i, out) = self.perform_instruction(idx);
            if let Some(out) = out {
                output.push(out);
            }

            if let Some(i) = i {
                idx = i;
            } else {
                break;
            }
        }

        output
    }

    fn perform_instruction(&mut self, idx: usize) -> (Option<usize>, Option<i64>) {
        let op = self.program.get(idx);
        if op.is_none() {
            return (None, None);
        }
        let op = op.unwrap();

        let operand = *self.program.get(idx + 1).unwrap();

        match *op {
            0 => {
                let pow: u32 = self.combo_operand(operand).try_into().unwrap();
                self.a = self.a / 2i64.pow(pow);
            }
            1 => {
                self.b = self.b.bitxor(operand);
            }
            2 => {
                self.b = self.combo_operand(operand) % 8;
            }
            3 => {
                if self.a != 0 {
                    return (Some(operand.try_into().unwrap()), None);
                }
            }
            4 => {
                self.b = self.b.bitxor(self.c);
            }
            5 => {
                let output = self.combo_operand(operand) % 8;
                return (Some(idx + 2), Some(output));
            }
            6 => {
                let pow: u32 = self.combo_operand(operand).try_into().unwrap();
                self.b = self.a / 2i64.pow(pow);
            }
            7 => {
                let pow: u32 = self.combo_operand(operand).try_into().unwrap();
                self.c = self.a / 2i64.pow(pow);
            }
            _ => panic!("Unrecognised op code: {}", op),
        }

        (Some(idx + 2), None)
    }

    fn combo_operand(&self, operand: i64) -> i64 {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("invalid operand: {}", operand),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE1: &str = include_str!("../example1.txt");
    const EXAMPLE2: &str = include_str!("../example2.txt");

    #[test]
    fn example() {
        assert_eq!(part_one(EXAMPLE1), "4,6,3,5,6,3,5,2,1,0");
        assert_eq!(part_two(EXAMPLE2), 117440);
    }
}
