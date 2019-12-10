#[macro_use]
extern crate lazy_static;

extern crate regex;

use day16::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

lazy_static! {
    static ref re: Regex = Regex::new(r"(\d+),? (\d+),? (\d+),? (\d+)").unwrap();
}

fn main() {
    let input = fs::read_to_string("samples").expect("Couldn't read samples file");
    let samples = parse_samples(&input);

    let mut possible: HashSet<(usize, Op)> = HashSet::new();

    // Create a list of every possible combination
    for n in 0..16 {
        for op in all_ops() {
            possible.insert((n, op));
        }
    }

    let mut three_or_more = 0;

    for sample in &samples {
        let ops = test_sample(&sample);

        // Count which ones pass 3 or more (hence fail <= 13)
        if ops.len() <= 13 {
            three_or_more += 1;
        }

        // Find the ones that fail and remove them from the set
        for op in ops {
            possible.remove(&(sample.instruction[0], op));
        }
    }

    let mut op_map: HashMap<usize, Op> = HashMap::new();

    // Repeatedly find codes that map to a single instruction
    // and remove them (and any related ones) from the set
    for _ in 0..16 {
        let (instr, op) = find_single(&possible);
        println!("{}: {:?}", instr, op);

        op_map.insert(instr, op);

        possible.remove(&(instr, op));
        for i in 0..16 {
            possible.remove(&(i, op));
        }
    }

    // Run operations on the instructions

    let input = fs::read_to_string("input").expect("couldn't read input");
    let input: Vec<[usize; 4]> = input.lines().filter_map(parse_sample_line).collect();

    let mut r = [0, 0, 0, 0];
    for i in input {
        let op = op_map.get(&i[0]).unwrap();
        let ans = apply_op(op, &r, i[1], i[2]);
        r[i[3]] = ans;
        println!("{:?} {} {} {} {:?}", op, i[1], i[2], i[3], r);
    }

    println!("Part two: {:?}", r);
}

fn find_single(ops: &HashSet<(usize, Op)>) -> (usize, Op) {
    for i in 0..16 {
        let matches: Vec<(usize, Op)> = ops.iter().filter(|(n, _)| *n == i).cloned().collect();
        if matches.len() == 1 {
            return matches[0];
        }
    }

    panic!("no instructions with a single op");
}

fn test_sample(sample: &Sample) -> Vec<Op> {
    all_ops()
        .into_iter()
        .filter(|op| {
            apply_op(
                op,
                &sample.before,
                sample.instruction[1],
                sample.instruction[2],
            ) != sample.after[sample.instruction[3]]
        })
        .collect()
}

#[derive(Debug)]
struct Sample {
    before: [usize; 4],
    instruction: [usize; 4],
    after: [usize; 4],
}

impl Sample {
    fn new() -> Sample {
        Sample {
            before: [0, 0, 0, 0],
            instruction: [0, 0, 0, 0],
            after: [0, 0, 0, 0],
        }
    }
}

fn parse_samples(input: &str) -> Vec<Sample> {
    let mut samples: Vec<Sample> = Vec::new();
    let mut current = Sample::new();

    for line in input.lines() {
        if line.starts_with("Before") {
            current = Sample {
                before: parse_sample_line(line).unwrap(),
                ..current
            };
            continue;
        }

        if line.starts_with("After") {
            current = Sample {
                after: parse_sample_line(line).unwrap(),
                ..current
            };
            samples.push(current);
            current = Sample::new();
            continue;
        }

        let inst = parse_sample_line(line);
        if inst.is_some() {
            current = Sample {
                instruction: inst.unwrap(),
                ..current
            };
        }
    }

    samples
}

fn parse_sample_line(line: &str) -> Option<[usize; 4]> {
    let nums = re.captures(line);
    if nums.is_none() {
        return None;
    }

    let nums: Vec<usize> = nums
        .unwrap()
        .iter()
        .skip(1)
        .map(|d| d.unwrap().as_str().parse::<usize>().unwrap())
        .collect();

    Some([nums[0], nums[1], nums[2], nums[3]])
}
