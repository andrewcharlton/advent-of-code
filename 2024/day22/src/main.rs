use std::time::Instant;
use std::collections::{HashMap,HashSet};
use std::ops::BitXor;
use std::collections::VecDeque;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let now = Instant::now();

    let (a, b) = solve(INPUT);


    println!("Part one: {}", a);
    println!("Part two: {:?}", b);

    let elapsed_time = now.elapsed();
    println!("Solved in {}µs", elapsed_time.as_micros());
}


fn solve(input: &str) -> (i64, i64) {
    let mut secrets: Vec<Secret> = input.lines().map(|line| Secret::new(line.parse().unwrap())).collect();
    let part_one: i64 = secrets.iter_mut().map(|s| s.get(2000)).sum();

    let all_sequences: HashSet<&[i64;4]> = secrets.iter().map(|s| s.sequences.keys()).flatten().collect();
    println!("{} unique sequences found", all_sequences.len());

    let mut best_score = 0;
    for sequence in all_sequences {
        let score: i64 = secrets.iter().filter_map(|s| s.sequences.get(sequence)).sum();
        if score > best_score {
            println!("Improvement found: {:?} scores {} bananas", sequence, score);
            best_score = score;
        }
    }

    (part_one, best_score)
}


struct Secret {
    n: i64,

    diffs: VecDeque<i64>,
    sequences: HashMap<[i64;4], i64>,
}

impl Secret {
    fn new(n: i64) -> Self {
        Secret{n, diffs: VecDeque::new(), sequences: HashMap::new()}
    }

    fn get(&mut self, term: usize) -> i64 {
        for _ in 0..term {
            self.advance();
        }

        self.n
    }

    fn advance(&mut self) {
        let n = self.n;
        let n = n.bitxor(n * 64) % 16777216;
        let n = n.bitxor(n / 32) % 16777216;
        let n = n.bitxor(n * 2048) % 16777216;

        self.diffs.push_back(n%10 - self.n%10);
        if self.diffs.len() >= 4 {
            if self.diffs.len() > 4 {
            self.diffs.pop_front();
            }
            let seq = [
                *self.diffs.get(0).unwrap(),
                *self.diffs.get(1).unwrap(),
                *self.diffs.get(2).unwrap(),
                *self.diffs.get(3).unwrap(),
            ];
            if !self.sequences.contains_key(&seq) {
                self.sequences.insert(seq, n % 10);
            }
        }
        
        self.n = n;
    }
}


#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE1: &str = include_str!("../example1.txt");
    const EXAMPLE2: &str = include_str!("../example2.txt");

    #[test]
    fn example() {
        let mut s = Secret::new(1);
        assert_eq!(s.get(2000), 8685429);

        let mut s = Secret::new(10);
        assert_eq!(s.get(2000), 4700978);

        let mut s = Secret::new(100);
        assert_eq!(s.get(2000), 15273692);

        let mut s = Secret::new(2024);
        assert_eq!(s.get(2000), 8667524);

        let (a,_) = solve(EXAMPLE1);
        assert_eq!(a, 37327623);

        let (_, b) = solve(EXAMPLE2);
        assert_eq!(b, 23);
    }
}
