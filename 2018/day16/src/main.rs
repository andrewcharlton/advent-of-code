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
    println!("{:?}", samples.len());

    let mut count = 0;
    let mut op_map: HashMap<usize, HashSet<Op>> = HashMap::new();
    for i in 0..16 {
        op_map.insert(i, all_ops().into_iter().clone().collect());
    }

    for sample in samples.iter() {
        let ops = test_sample(&sample);
        if ops.len() >= 3 {
            count += 1;
        }

        let op_code = sample.instruction[0];

        let possible = op_map
            .get(&op_code)
            .unwrap()
            .into_iter()
            .filter(|op| ops.contains(op))
            .map(|op| op.clone())
            .collect();

        op_map.insert(op_code, possible);
    }

    println!("Part one: {}", count);

    let assigned_codes = assign_opcodes(op_map);
    println!("{:?}", assigned_codes);
}

fn assign_opcodes(mut map: HashMap<usize, HashSet<Op>>) -> HashMap<usize, Op> {
    let mut assigned = HashMap::new();
    let mut num_assigned = 0;

    while num_assigned < 16 {
        let mut latest: Option<Op> = None;
        for (i, s) in map.iter() {
            if s.len() == 1 {
                latest = s.drain().next();
                assigned.insert(*i, latest.unwrap());
                break;
            }
        }

        if latest.is_none() {
            panic!("Shit got real");
        }

        for (i, mut s) in map.iter_mut() {
            s.remove(&latest.unwrap());
            map.insert(*i, s);
        }
    }

    assigned
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
            ) == sample.after[sample.instruction[3]]
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
