use std::convert::TryFrom;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("couldn't read input file");

    let codes = parse_codes(&input);
    let max_thruster = find_max(&codes);
    println!("Part one: {}", max_thruster);
}

fn parse_codes(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect()
}

fn find_max(codes: &Vec<i32>) -> i32 {
    combinations()
        .iter()
        .map(|c| test_combination(&codes, c))
        .max()
        .unwrap()
}

fn combinations() -> Vec<[i32; 5]> {
    let mut combos = Vec::new();
    for a in 0..5 {
        for b in 0..5 {
            if a == b {
                continue;
            }

            for c in 0..5 {
                if a == c || b == c {
                    continue;
                }

                for d in 0..5 {
                    if a == d || b == d || c == d {
                        continue;
                    }

                    for e in 0..5 {
                        if a == e || b == e || c == e || d == e {
                            continue;
                        }

                        combos.push([a, b, c, d, e]);
                    }
                }
            }
        }
    }

    combos
}

fn test_combination(codes: &Vec<i32>, phases: &[i32; 5]) -> i32 {
    let mut ans = 0;

    for amp in 0..5 {
        let mut codes = codes.clone();
        let mut pos = 0;
        let mut input = vec![phases[amp], ans];
        loop {
            let (next, output) = operate(&mut codes, pos, &mut input);
            match output {
                Some(x) => ans = x,
                None => {}
            }

            match next {
                Some(p) => pos = p,
                None => break,
            }
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

fn operate(codes: &mut Vec<i32>, pos: usize, input: &mut Vec<i32>) -> (Option<usize>, Option<i32>) {
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
            let i = input.remove(0);
            codes[p] = i;
            return (Some(pos + 2), None);
        }

        Op::Output(f) => {
            let output = codes[get_pos(codes, pos + 1, f)];
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
    fn part1_example1() {
        let codes = parse_codes(&"3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        assert_eq!(test_combination(&codes, &[4, 3, 2, 1, 0]), 43210);
    }

    #[test]
    fn part1_example2() {
        let codes = parse_codes(
            &"3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
        );
        assert_eq!(find_max(&codes), 54321);
    }
}
