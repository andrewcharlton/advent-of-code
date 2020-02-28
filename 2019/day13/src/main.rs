use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs;
use std::io;

fn main() {
    let input = fs::read_to_string("input").expect("couldn't read input");

    part_one(&input);
    part_two(&input, true);
}

fn part_one(input: &str) {
    let mut computer = IntCodeComputer::new(&input, 1);
    let mut tiles: HashMap<(i64, i64), i64> = HashMap::new();
    loop {
        let x = computer.run(None);
        match x {
            Status::Finished => break,
            _ => {}
        }
        let x = x.unwrap();

        let y = computer.run(None).unwrap();
        let tile = computer.run(None).unwrap();
        tiles.insert((x, y), tile);
    }

    let ans = tiles.iter().filter(|&(_, v)| *v == 2).count();
    println!("Part one: {}", ans);
}

fn part_two(input: &str, autoplay: bool) {
    println!("Part two!\n\n");

    let mut computer = IntCodeComputer::new(&input, 2);
    let mut tiles: HashMap<(i64, i64), i64> = HashMap::new();

    // Keep track of where the ball is so we can move the paddle
    // there
    let mut ball_pos = 0;
    let mut paddle_pos = 0;

    loop {
        let mut x = computer.run(None);
        match x {
            Status::Finished => break,
            Status::Success(_) => {}
            Status::WaitingForInput => {
                if !autoplay {
                    print_grid(&tiles);
                }

                let input = if autoplay {
                    if ball_pos > paddle_pos {
                        1
                    } else if ball_pos < paddle_pos {
                        -1
                    } else {
                        0
                    }
                } else {
                    get_input()
                };

                x = computer.run(Some(input));
            }
        }
        let x = x.unwrap();

        let y = computer.run(None).unwrap();
        let tile = computer.run(None).unwrap();

        // Update tile and paddle positions
        match tile {
            3 => paddle_pos = x,
            4 => ball_pos = x,
            _ => {}
        }

        if x == -1 && y == 0 {
            println!("Score: {}", tile);
        } else {
            tiles.insert((x, y), tile);
        }
    }
}

fn get_input() -> i64 {
    println!("Move:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("nothing was entered");
    let input = input.trim();
    match input {
        "\\" => return -1,
        "" => return 0,
        "/" => return 1,
        _ => {
            println!("Unknown input");
            return get_input();
        }
    }
}

fn print_grid(tiles: &HashMap<(i64, i64), i64>) {
    let min_x = *tiles.keys().map(|(x, _)| x).min().unwrap();
    let max_x = *tiles.keys().map(|(x, _)| x).max().unwrap();
    let min_y = *tiles.keys().map(|(_, y)| y).min().unwrap();
    let max_y = *tiles.keys().map(|(_, y)| y).max().unwrap();

    for y in min_y..max_y + 1 {
        for x in min_x..max_x + 1 {
            match tiles.get(&(x, y)) {
                None | Some(0) => print!(" "),
                Some(1) => print!("|"),
                Some(2) => print!("■"),
                Some(3) => print!("-"),
                Some(4) => print!("●"),
                Some(x) => panic!("unknown tile: {}", x),
            }
        }

        print!("\n");
    }
}

enum Status {
    Success(i64),
    WaitingForInput,
    Finished,
}

impl Status {
    fn unwrap(&self) -> i64 {
        match self {
            Status::Success(x) => *x,
            _ => panic!("no value to unwrap"),
        }
    }
}

struct IntCodeComputer {
    codes: Vec<i64>,
    rel_base: i64,
    pos: usize,
}

impl IntCodeComputer {
    fn new(input: &str, quarters: i64) -> IntCodeComputer {
        let mut codes: Vec<i64> = input
            .trim()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();

        codes[0] = quarters;

        IntCodeComputer {
            codes: codes,
            rel_base: 0,
            pos: 0,
        }
    }

    fn run(&mut self, input: Option<i64>) -> Status {
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
                    match input {
                        Some(x) => {
                            self.set_value(a, x);
                            self.pos += 2;
                        }
                        None => return Status::WaitingForInput,
                    }
                }
                4 => {
                    // Output
                    self.pos += 2;
                    return Status::Success(self.get_value(a));
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
                99 => return Status::Finished,
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
