use itertools::Itertools;
use std::collections::HashSet;
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let now = Instant::now();

    let network = Network::new(INPUT);
    println!("Part one: {}", network.triad_count());
    println!("Part two: {}", network.password());

    let elapsed_time = now.elapsed();
    println!("Solved in {}µs", elapsed_time.as_micros());
}

type Computer = [char; 2];

struct Network {
    connections: HashSet<(Computer, Computer)>,
}

impl Network {
    fn new(input: &str) -> Self {
        let connections: HashSet<(Computer, Computer)> = input
            .lines()
            .map(|line| {
                let (a, b) = line.split_once("-").unwrap();

                let mut a = a.chars();
                let a: Computer = [a.next().unwrap(), a.next().unwrap()];

                let mut b = b.chars();
                let b: Computer = [b.next().unwrap(), b.next().unwrap()];

                if a < b {
                    (a, b)
                } else {
                    (b, a)
                }
            })
            .collect();

        Network { connections }
    }

    fn triad_count(&self) -> usize {
        let groups = self.initial_groups();
        let mut triads = self.group(&groups);
        triads.sort();

        triads
            .iter()
            .filter(|triad| triad.len() == 3)
            .filter_map(|triad| triad.iter().find(|computer| computer[0] == 't'))
            .count()
    }

    fn password(&self) -> String {
        let mut groups = self.initial_groups();
        let mut best = groups.get(0).unwrap().clone();

        loop {
            println!("Looping - Groups: {}, Best: {:?}", groups.len(), best);
            groups = self.group(&groups);
            if groups.len() == 0 {
                return best
                    .iter()
                    .map(|c| {
                        let mut s = String::new();
                        s.push(c[0]);
                        s.push(c[1]);
                        s
                    })
                    .join(",");
            }

            let max_len = groups.iter().map(|g| g.len()).max().unwrap();
            groups = groups.into_iter().filter(|g| g.len() == max_len).collect();

            for g in &groups {
                if g.len() > best.len() {
                    best = g.to_vec();
                }
            }
        }
    }

    fn group(&self, groups: &Vec<Vec<Computer>>) -> Vec<Vec<Computer>> {
        let mut results = Vec::new();

        for i in 0..groups.len() - 1 {
            let a = groups.get(i).unwrap();

            for j in i + 1..groups.len() {
                let b = groups.get(j).unwrap();
                if let Some(new_group) = self.union(a, b) {
                    results.push(new_group);
                }
            }
        }

        results.sort();
        results.dedup();
        results
    }

    fn union(&self, a: &Vec<Computer>, b: &Vec<Computer>) -> Option<Vec<Computer>> {
        let mut union = a.clone();
        union.extend(b);
        union.sort();
        union.dedup();

        for i in 0..union.len() - 1 {
            let x = *union.get(i).unwrap();
            for j in i + 1..union.len() {
                let y = *union.get(j).unwrap();
                if !self.connections.contains(&(x, y)) {
                    return None;
                }
            }
        }

        Some(union)
    }

    fn initial_groups(&self) -> Vec<Vec<Computer>> {
        self.connections
            .clone()
            .into_iter()
            .map(|(a, b)| vec![a, b])
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let network = Network::new(EXAMPLE);
        assert_eq!(network.triad_count(), 7);
        assert_eq!(network.password(), String::from("co,de,ka,ta"));
    }
}
