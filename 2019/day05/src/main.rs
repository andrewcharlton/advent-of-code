use std::convert::TryFrom;
use std::fs;

fn main() {
    let codes: Vec<i32> = fs::read_to_string("input")
        .expect("couldn't read file")
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let ans = run(&codes, 1);
    println!("Part one: {}\n", ans);

    let ans = run(&codes, 5);
    println!("Part two: {}", ans);
}

fn run(codes: &Vec<i32>, input: i32) -> i32 {
    let mut codes = codes.clone();

    let mut pos = 0;
    let mut ans = 0;
    loop {
        let (op, output) = operate(&mut codes, pos, input);
        match output {
            Some(x) => ans = x,
            None => {}
        }

        match op {
            Some(x) => pos = x,
            None => break,
        }
    }

    ans
}

#[derive(Debug, PartialEq)]
enum Op {
    Add(bool, bool, bool),
    Mult(bool, bool, bool),
    Input(bool),
    Output(bool),
    JumpIfTrue(bool, bool),
    JumpIfFalse(bool, bool),
    LessThan(bool, bool, bool),
    Equals(bool, bool, bool),
    Finish,
}

fn decode_op(op: i32) -> Op {
    let (op, instr) = (op / 100, op % 100);
    let (op, first) = (op / 10, op % 10 == 1);
    let (op, second) = (op / 10, op % 10 == 1);
    let third = op % 10 == 1;

    match instr {
        1 => Op::Add(first, second, third),
        2 => Op::Mult(first, second, third),
        3 => Op::Input(first),
        4 => Op::Output(first),
        5 => Op::JumpIfTrue(first, second),
        6 => Op::JumpIfFalse(first, second),
        7 => Op::LessThan(first, second, third),
        8 => Op::Equals(first, second, third),
        99 => Op::Finish,
        unknown => panic!("Unknown op code: {}", unknown),
    }
}

fn operate(codes: &mut Vec<i32>, pos: usize, input: i32) -> (Option<usize>, Option<i32>) {
    let op = decode_op(codes[pos]);

    match op {
        Op::Add(f, s, t) => {
            let a = codes[get_pos(codes, pos + 1, f)];
            let b = codes[get_pos(codes, pos + 2, s)];
            let p = get_pos(codes, pos + 3, t);
            codes[p] = a + b;
            return (Some(pos + 4), None);
        }

        Op::Mult(f, s, t) => {
            let a = codes[get_pos(codes, pos + 1, f)];
            let b = codes[get_pos(codes, pos + 2, s)];
            let p = get_pos(codes, pos + 3, t);
            codes[p] = a * b;
            return (Some(pos + 4), None);
        }

        Op::Input(f) => {
            let p = get_pos(codes, pos + 1, f);
            codes[p] = input;
            return (Some(pos + 2), None);
        }

        Op::Output(f) => {
            let output = codes[get_pos(codes, pos + 1, f)];
            println!("{}", output);
            return (Some(pos + 2), Some(output));
        }

        Op::JumpIfTrue(f, s) => {
            let jump = codes[get_pos(codes, pos + 1, f)];
            if jump != 0 {
                let p = usize::try_from(codes[get_pos(codes, pos + 2, s)])
                    .expect("jump to be positive");
                return (Some(p), None);
            }
            return (Some(pos + 3), None);
        }

        Op::JumpIfFalse(f, s) => {
            let jump = codes[get_pos(codes, pos + 1, f)];
            if jump == 0 {
                let p = usize::try_from(codes[get_pos(codes, pos + 2, s)])
                    .expect("jump to be positive");
                return (Some(p), None);
            }
            return (Some(pos + 3), None);
        }

        Op::LessThan(f, s, t) => {
            let a = codes[get_pos(codes, pos + 1, f)];
            let b = codes[get_pos(codes, pos + 2, s)];
            let p = get_pos(codes, pos + 3, t);
            if a < b {
                codes[p] = 1;
            } else {
                codes[p] = 0;
            }
            return (Some(pos + 4), None);
        }

        Op::Equals(f, s, t) => {
            let a = codes[get_pos(codes, pos + 1, f)];
            let b = codes[get_pos(codes, pos + 2, s)];
            let p = get_pos(codes, pos + 3, t);
            if a == b {
                codes[p] = 1;
            } else {
                codes[p] = 0;
            }
            return (Some(pos + 4), None);
        }

        Op::Finish => {
            return (None, None);
        }
    }
}

fn get_pos(input: &Vec<i32>, pos: usize, immediate: bool) -> usize {
    if immediate {
        return pos;
    }

    usize::try_from(input[pos]).expect("pos should be positive")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_op_test() {
        assert_eq!(decode_op(1002), Op::Mult(false, true, false));
    }

    #[test]
    fn position_mode_test() {
        let codes = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        assert_eq!(run(&codes, 5), 1);
        assert_eq!(run(&codes, 0), 0);
    }

    #[test]
    fn immediate_mode_test() {
        let codes = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        assert_eq!(run(&codes, 5), 1);
        assert_eq!(run(&codes, 0), 0);
    }

    #[test]
    fn larger_test() {
        let codes = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        assert_eq!(run(&codes, 5), 999);
        assert_eq!(run(&codes, 8), 1000);
        assert_eq!(run(&codes, 57), 1001);
    }
}
