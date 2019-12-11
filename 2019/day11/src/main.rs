use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("couldn't read input file");

    let part_one = run(&input, 0).len();
    println!("Part one: {}", part_one);

    println!("Part two:\n");
    let grid = run(&input, 1);
    print_grid(&grid);
}

fn run(input: &str, start: i64) -> HashMap<(i32, i32), i64> {
    let mut computer = IntCodeComputer::new(&input);
    let mut grid: HashMap<(i32, i32), i64> = HashMap::new();
    let mut bot = Bot::new();

    grid.insert((0, 0), start);

    loop {
        let pos = bot.pos();
        let current = grid.get(&pos).unwrap_or(&0);
        let color = computer.run(Some(*current));
        let dir = computer.run(None);

        if color.is_none() || dir.is_none() {
            return grid;
        }

        grid.insert(pos, color.unwrap());
        match dir.unwrap() {
            0 => bot.turn_left(),
            1 => bot.turn_right(),
            x => panic!("unknown turn: {}", x),
        }
    }
}

fn print_grid(grid: &HashMap<(i32, i32), i64>) {
    let min_x = *grid.keys().map(|(x, _)| x).min().unwrap();
    let max_x = *grid.keys().map(|(x, _)| x).max().unwrap();
    let min_y = *grid.keys().map(|(_, y)| y).min().unwrap();
    let max_y = *grid.keys().map(|(_, y)| y).max().unwrap();

    for y in (min_y..max_y + 1).rev() {
        for x in min_x..max_x + 1 {
            if grid.get(&(x, y)).unwrap_or(&0) == &1 {
                print!("O");
            } else {
                print!(" ");
            }
        }

        print!("\n");
    }
}

struct Bot {
    x: i32,
    y: i32,
    d_x: i32,
    d_y: i32,
}

impl Bot {
    fn new() -> Bot {
        Bot {
            x: 0,
            y: 0,
            d_x: 0,
            d_y: 1,
        }
    }

    fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn turn_left(&mut self) {
        self.update(-self.d_y, self.d_x);
    }

    fn turn_right(&mut self) {
        self.update(self.d_y, -self.d_x);
    }

    fn update(&mut self, d_x: i32, d_y: i32) {
        self.x += d_x;
        self.y += d_y;
        self.d_x = d_x;
        self.d_y = d_y;
    }
}

struct IntCodeComputer {
    codes: Vec<i64>,
    rel_base: i64,
    pos: usize,
}

impl IntCodeComputer {
    fn new(input: &str) -> IntCodeComputer {
        let codes = input
            .trim()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();

        IntCodeComputer {
            codes: codes,
            rel_base: 0,
            pos: 0,
        }
    }

    fn run(&mut self, input: Option<i64>) -> Option<i64> {
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
                    self.set_value(a, input.expect("needed some input"));
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
