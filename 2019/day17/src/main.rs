use std::collections::HashSet;
use std::fs;

mod intcode;
use intcode::{IntCodeComputer, Status};

fn main() {
    let input = fs::read_to_string("input").expect("couldn't read input");

    let (scaffold, pos, dir) = parse_scaffold(&input, true);

    let ans = intersection_sum(&scaffold);
    println!("Part one: {}", ans);

    let route = find_route(&scaffold, &pos, &dir);
    println!("Full route: {:?}", route);
}

fn parse_scaffold(input: &str, print: bool) -> (HashSet<(i64, i64)>, (i64, i64), (i64, i64)) {
    let mut computer = IntCodeComputer::new(input);

    let mut scaffold = HashSet::new();
    let mut robot_pos = (0, 0);
    let mut robot_dir = (0, 0);

    let mut x = 0;
    let mut y = 0;

    loop {
        let c = match computer.run(None) {
            Status::WaitingForInput => panic!("expecting input"),
            Status::Finished => return (scaffold, robot_pos, robot_dir),
            Status::Success(10) => {
                x = 0;
                y += 1;
                "\n"
            }
            Status::Success(35) => {
                scaffold.insert((x, y));
                x += 1;
                "#"
            }
            Status::Success(46) => {
                x += 1;
                " "
            }
            Status::Success(94) => {
                robot_pos = (x, y);
                robot_dir = (0, -1);
                scaffold.insert((x, y));
                x += 1;
                "^"
            }
            Status::Success(x) => panic!("unknown code: {}", x),
        };

        if print {
            print!("{}", c);
        }
    }
}

fn intersection_sum(scaffold: &HashSet<(i64, i64)>) -> i64 {
    scaffold
        .iter()
        .filter(|(x, y)| {
            if x == &0 || y == &0 {
                return false;
            }

            for pos in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
                if !scaffold.contains(&(x + pos.0, y + pos.1)) {
                    return false;
                }
            }

            true
        })
        .map(|(x, y)| x * y)
        .sum()
}

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
    Forward(i64),
}

fn find_route(
    scaffold: &HashSet<(i64, i64)>,
    pos: &(i64, i64),
    dir: &(i64, i64),
) -> Vec<Instruction> {
    let mut pos = pos.clone();
    let mut dir = dir.clone();

    let mut instructions = Vec::new();

    let mut current = 0;
    loop {
        // try to advance
        if scaffold.contains(&(pos.0 + dir.0, pos.1 + dir.1)) {
            current += 1;
            pos = (pos.0 + dir.0, pos.1 + dir.1);
            continue;
        }

        // if we can't, we're at the end of an instruction
        if current > 0 {
            instructions.push(Instruction::Forward(current));
            // println!("Moved forward: {}, now at ({}, {})", current, pos.0, pos.1);
        }

        // if we can't, try to turn left and then right
        let left = turn_left(dir);
        // println!(
        //     "Current dir: ({}, {}), Left: ({}, {})",
        //     dir.0, dir.1, left.0, left.1
        // );
        if scaffold.contains(&(pos.0 + left.0, pos.1 + left.1)) {
            instructions.push(Instruction::Left);
            current = 1;
            dir = left;
            pos = (pos.0 + left.0, pos.1 + left.1);
            continue;
        }

        let right = turn_right(dir);
        // println!(
        //     "Current dir: ({}, {}), Right: ({}, {})",
        //     dir.0, dir.1, right.0, right.1
        // );
        if scaffold.contains(&(pos.0 + right.0, pos.1 + right.1)) {
            instructions.push(Instruction::Right);
            current = 1;
            dir = right;
            pos = (pos.0 + right.0, pos.1 + right.1);
            continue;
        }

        // If we're at a dead-end, we must be finished
        return instructions;
    }
}

fn turn_left(dir: (i64, i64)) -> (i64, i64) {
    (dir.1, -dir.0)
}

fn turn_right(dir: (i64, i64)) -> (i64, i64) {
    (-dir.1, dir.0)
}
