extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref re: Regex = Regex::new(r":(\d{2})] (.+)$").unwrap();
    static ref guard_re: Regex = Regex::new(r"Guard #(\d+) begins shift").unwrap();
}

fn main() {
    let input = fs::read_to_string("sorted-input").expect("Couldn't find file");
    let records = input
        .lines()
        .filter_map(|line| Record::parse(line))
        .collect();

    let guard = find_guard_with_most_sleep(&records);
    let time = most_common_sleeping_time(&records, guard);
    println!(
        "Part one - Guard: {}, Time: {}, Answer: {}",
        guard,
        time,
        guard * time
    );

    let answer = most_asleep(&records);
    println!(
        "Part two - Guard: {}, Time: {}, Answer: {}",
        answer.0,
        answer.1,
        answer.0 * answer.1,
    );
}

enum Record {
    NewGuard(usize),
    WokeUp(usize),
    FellAsleep(usize),
}

impl Record {
    fn parse(input: &str) -> Option<Record> {
        let caps = guard_re.captures(input);
        if caps.is_some() {
            return Some(Record::NewGuard(
                caps.unwrap().get(1).unwrap().as_str().parse().unwrap(),
            ));
        }

        let caps = re.captures(input);
        if caps.is_some() {
            let caps = caps.unwrap();
            let time: usize = caps[1].parse().unwrap();
            return match &caps[2] {
                "falls asleep" => Some(Record::FellAsleep(time)),
                "wakes up" => Some(Record::WokeUp(time)),
                _ => None,
            };
        }

        None
    }
}

fn find_guard_with_most_sleep(records: &Vec<Record>) -> usize {
    let mut current_guard = 0;
    let mut fell_asleep: usize = 0;

    let mut sleep = HashMap::new();

    for record in records.iter() {
        match record {
            Record::NewGuard(guard) => current_guard = *guard,
            Record::FellAsleep(time) => fell_asleep = *time,
            Record::WokeUp(time) => {
                sleep.insert(
                    current_guard,
                    sleep.get(&current_guard).unwrap_or(&0) + time - fell_asleep,
                );
            }
        }
    }

    *sleep
        .iter()
        .max_by_key(|(_, &time)| time)
        .map(|x| x.0)
        .unwrap()
}

fn most_common_sleeping_time(records: &Vec<Record>, guard: usize) -> usize {
    let mut sleep = [0; 60];

    let mut correct_guard = false;
    let mut fell_asleep: usize = 0;

    for record in records {
        match record {
            Record::NewGuard(g) => correct_guard = guard == *g,
            Record::FellAsleep(time) => fell_asleep = *time,
            Record::WokeUp(time) => {
                if correct_guard {
                    for i in fell_asleep..*time {
                        sleep[i] += 1;
                    }
                }
            }
        }
    }

    sleep
        .iter()
        .enumerate()
        .max_by_key(|(_, &v)| v)
        .map(|x| x.0)
        .unwrap()
}

fn most_asleep(records: &Vec<Record>) -> (usize, usize) {
    let mut sleep = HashMap::new();
    let mut current_guard = 0;
    let mut fell_asleep = 0;

    for record in records {
        match record {
            Record::NewGuard(guard) => current_guard = *guard,
            Record::FellAsleep(time) => fell_asleep = *time,
            Record::WokeUp(time) => {
                for t in fell_asleep..*time {
                    let key = (current_guard, t);
                    sleep.insert(key, sleep.get(&key).unwrap_or(&0) + 1);
                }
            }
        }
    }

    *sleep
        .iter()
        .max_by_key(|(_, &v)| v)
        .map(|(k, _)| k)
        .unwrap()
}
