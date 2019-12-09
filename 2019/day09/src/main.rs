use std::convert::TryFrom;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("couldn't read input file");

    let codes = parse_codes(&input);

    run("Part one", &codes, 1);
    run("Part two", &codes, 2);
}

fn run(name: &str, codes: &Vec<i64>, input: i64) {
    println!("{}:", name);
    let mut computer = IntCodeComputer::new(&codes);
    loop {
        let output = computer.run(input);
        match output {
            Some(x) => println!("{}", x),
            None => break,
        }
    }

    println!("\n");
}

fn parse_codes(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect()
}

struct IntCodeComputer {
    codes: Vec<i64>,
    rel_base: i64,
    pos: usize,
}

impl IntCodeComputer {
    fn new(codes: &Vec<i64>) -> IntCodeComputer {
        IntCodeComputer {
            codes: codes.clone(),
            rel_base: 0,
            pos: 0,
        }
    }

    fn run(&mut self, input: i64) -> Option<i64> {
        loop {
            let code = self.codes[self.pos];

            // Work out whether the parameters are immediate or not
            let (code, op) = (code / 100, code % 100);
            let (code, a) = (code / 10, code % 10);
            let (code, b) = (code / 10, code % 10);
            let c = code % 10;

            // Calculate the positions to fetch the params from
            let a = self.get_pos(self.pos + 1, a);
            let b = self.get_pos(self.pos + 2, b);
            let c = self.get_pos(self.pos + 3, c);

            match op {
                1 => {
                    // Add
                    self.set_value(c, self.get_value(a) + self.get_value(b));
                    self.pos += 4;
                }
                2 => {
                    // Multiply
                    self.set_value(c, self.get_value(a) * self.get_value(b));
                    self.pos += 4;
                }
                3 => {
                    // Input - try phase first
                    self.set_value(a, input);
                    self.pos += 2;
                }
                4 => {
                    // Output
                    self.pos += 2;
                    return Some(self.get_value(a));
                }
                5 => {
                    // Jump if true
                    if self.get_value(a) != 0 {
                        self.pos = usize::try_from(self.get_value(b)).unwrap();
                    } else {
                        self.pos += 3;
                    }
                }
                6 => {
                    // Jump if false
                    if self.get_value(a) == 0 {
                        self.pos = usize::try_from(self.get_value(b)).unwrap();
                    } else {
                        self.pos += 3;
                    }
                }
                7 => {
                    // Less than
                    if self.get_value(a) < self.get_value(b) {
                        self.set_value(c, 1);
                    } else {
                        self.set_value(c, 0);
                    }
                    self.pos += 4;
                }
                8 => {
                    // Equal
                    if self.get_value(a) == self.get_value(b) {
                        self.set_value(c, 1);
                    } else {
                        self.set_value(c, 0);
                    }
                    self.pos += 4;
                }
                9 => {
                    self.rel_base += self.get_value(a);
                    self.pos += 2;
                }
                99 => return None,
                unknown => panic!("unknown op code: {}", unknown),
            }
        }
    }

    fn get_pos(&self, pos: usize, mode: i64) -> Option<usize> {
        match mode {
            0 => self.codes.get(pos).and_then(|p| usize::try_from(*p).ok()),
            1 => Some(pos),
            2 => self
                .codes
                .get(pos)
                .and_then(|p| usize::try_from(self.rel_base + *p).ok()),
            x => panic!("unknown mode: {}", x),
        }
    }

    fn get_value(&self, pos: Option<usize>) -> i64 {
        match pos {
            Some(p) => *self.codes.get(p).unwrap_or(&0),
            None => panic!("illegal position"),
        }
    }

    fn set_value(&mut self, pos: Option<usize>, val: i64) {
        if pos.is_none() {
            panic!("illegal position");
        }
        let pos = pos.unwrap();

        // Expand the codes vector if needed
        if self.codes.len() <= pos {
            self.codes.resize(pos + 1, 0);
        }

        self.codes[pos] = val;
    }
}
