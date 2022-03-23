use std::collections::HashSet;
use std::convert::TryFrom;
use std::fs;
use std::str::FromStr;

enum Op {
    Nop,
    Acc,
    Jmp,
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Op, ()> {
        match s {
            "nop" => Ok(Op::Nop),
            "jmp" => Ok(Op::Jmp),
            "acc" => Ok(Op::Acc),
            _ => Err(()),
        }
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("couldn't read input");

    let ops = parse_input(&input);
    println!("Part one: {}", last_accumulator(&ops));

    let visited = HashSet::new();
    println!(
        "Part two: {}",
        find_terminator(&ops, &visited, 0, 0, true).unwrap()
    );
}

fn parse_input(input: &str) -> Vec<(Op, isize)> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(" ").collect();
            (
                parts.get(0).unwrap().parse().unwrap(),
                parts.get(1).unwrap().parse().unwrap(),
            )
        })
        .collect()
}

fn last_accumulator(ops: &Vec<(Op, isize)>) -> isize {
    let mut acc: isize = 0;
    let mut pos: usize = 0;
    let mut visited: HashSet<usize> = HashSet::new();

    loop {
        let is_new = visited.insert(pos);
        if !is_new {
            return acc;
        }

        let (op, n) = ops.get(pos).unwrap();
        match op {
            Op::Nop => pos += 1,
            Op::Acc => {
                acc += n;
                pos += 1;
            }
            Op::Jmp => pos = usize::try_from(isize::try_from(pos).unwrap() + n).unwrap(),
        }
    }
}

fn find_terminator(
    ops: &Vec<(Op, isize)>,
    visited: &HashSet<usize>,
    acc: isize,
    pos: usize,
    can_switch: bool,
) -> Option<isize> {
    // If we get to the end of the ops, we've terminated!
    if pos == ops.len() {
        return Some(acc);
    }

    if pos > ops.len() {
        return None;
    }

    let mut visited = visited.clone();
    let is_new = visited.insert(pos);
    if !is_new {
        return None;
    }

    let (op, n) = ops.get(pos).unwrap();
    match op {
        Op::Acc => {
            return find_terminator(ops, &visited, acc + n, pos + 1, can_switch);
        }
        Op::Nop => {
            let nop = find_terminator(ops, &visited, acc, pos + 1, can_switch);
            if can_switch {
                return nop.or({
                    let jmp_pos = usize::try_from(isize::try_from(pos).unwrap() + n);
                    if jmp_pos.is_err() {
                        return None;
                    }
                    find_terminator(ops, &visited, acc, jmp_pos.unwrap(), false)
                });
            }
            return nop;
        }
        Op::Jmp => {
            let jmp_pos = usize::try_from(isize::try_from(pos).unwrap() + n);
            if jmp_pos.is_err() {
                return None;
            }
            let jmp = find_terminator(ops, &visited, acc, jmp_pos.unwrap(), can_switch);

            if can_switch {
                return jmp.or(find_terminator(ops, &visited, acc, pos + 1, false));
            }
            return jmp;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn last_accumulator_test() {
        let ops = parse_input(&INPUT);
        assert_eq!(last_accumulator(&ops), 5);
    }

    #[test]
    fn find_terminator_test() {
        let ops = parse_input(&INPUT);
        let visited = HashSet::new();
        assert_eq!(find_terminator(&ops, &visited, 0, 0, true), Some(8));
    }
}
