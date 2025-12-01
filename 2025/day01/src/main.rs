use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("couldn't read example file");
    let (part_one, part_two) = run(&input);
    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}

fn run(input: &str) -> (isize, isize) {
    let mut dial = Dial::new(50);

    input
        .lines()
        .map(|line| Instruction::parse(line))
        .for_each(|i| dial.rotate(i));

    (dial.landed_on_zero, dial.passed_zero)
}

enum Instruction {
    Left(isize),
    Right(isize),
}

impl Instruction {
    fn parse(s: &str) -> Instruction {
        use Instruction::*;

        let (dir, n) = s.split_at(1);
        let n: isize = n.trim().parse().unwrap();

        match dir {
            "L" => Left(n),
            "R" => Right(n),
            _ => panic!("Failed to parse instruction: {}", s),
        }
    }
}

struct Dial {
    pos: isize,
    landed_on_zero: isize,
    passed_zero: isize,
}

impl Dial {
    fn new(start_pos: isize) -> Dial {
        Dial {
            pos: start_pos,
            landed_on_zero: 0,
            passed_zero: 0,
        }
    }

    fn rotate(&mut self, i: Instruction) {
        use Instruction::*;

        match i {
            Left(x) => {
                self.passed_zero += x / 100;
                let new_pos = self.pos - (x % 100);

                if self.pos > 0 && new_pos < 0 {
                    self.passed_zero += 1;
                }
                self.pos = new_pos % 100;
            }
            Right(x) => {
                self.passed_zero += x / 100;
                self.pos = self.pos + (x % 100);

                if self.pos > 100 {
                    self.passed_zero += 1;
                }
            }
        }

        self.pos = (self.pos + 100) % 100;
        if self.pos == 0 {
            self.passed_zero += 1;
            self.landed_on_zero += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = fs::read_to_string("example").expect("couldn't read example file");
        let (part_one, part_two) = run(&input);
        assert_eq!(part_one, 3);
        assert_eq!(part_two, 6);
    }
}
