use std::convert::TryFrom;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("couldn't read input file");

    let codes = parse_codes(&input);
    let max_thruster = find_max(&codes, 0);
    println!("Part one: {}", max_thruster);

    let max_thruster = find_max(&codes, 5);
    println!("Part two: {}", max_thruster);
}

fn parse_codes(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect()
}

fn find_max(codes: &Vec<i32>, start: i32) -> i32 {
    combinations(start)
        .iter()
        .map(|c| test_combination(&codes, c))
        .max()
        .unwrap()
}

fn test_combination(codes: &Vec<i32>, phases: &[i32; 5]) -> i32 {
    let mut ans = 0;

    let mut amps = vec![
        Amplifier::new(codes, phases[0]),
        Amplifier::new(codes, phases[1]),
        Amplifier::new(codes, phases[2]),
        Amplifier::new(codes, phases[3]),
        Amplifier::new(codes, phases[4]),
    ];

    loop {
        for p in 0..5 {
            let x = amps[p].run(ans);
            if x.is_none() {
                return ans;
            }
            ans = x.unwrap();
        }
    }
}

fn permutations(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() == 1 {
        return vec![nums];
    }

    let mut all = Vec::new();
    for i in 0..nums.len() {
        let mut n = nums.clone();
        let x = n.remove(i);

        for mut p in permutations(n) {
            p.push(x);
            all.push(p);
        }
    }

    all
}

fn combinations(min: i32) -> Vec<[i32; 5]> {
    let mut combos = Vec::new();
    for a in min..(min + 5) {
        for b in min..(min + 5) {
            if a == b {
                continue;
            }

            for c in min..(min + 5) {
                if a == c || b == c {
                    continue;
                }

                for d in min..(min + 5) {
                    if a == d || b == d || c == d {
                        continue;
                    }

                    for e in min..(min + 5) {
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

struct Amplifier {
    codes: Vec<i32>,
    phase: Option<i32>,
    pos: usize,
}

impl Amplifier {
    fn new(codes: &Vec<i32>, phase: i32) -> Amplifier {
        Amplifier {
            codes: codes.clone(),
            phase: Some(phase),
            pos: 0,
        }
    }

    fn run(&mut self, input: i32) -> Option<i32> {
        loop {
            let code = self.codes[self.pos];

            // Work out whether the parameters are immediate or not
            let (code, op) = (code / 100, code % 100);
            let (code, a) = (code / 10, code % 10 == 1);
            let (code, b) = (code / 10, code % 10 == 1);
            let c = code % 10 == 1;

            // Calculate the positions to fetch the params from
            let a = self.get_pos(self.pos + 1, a);
            let b = self.get_pos(self.pos + 2, b);
            let c = self.get_pos(self.pos + 3, c);

            match op {
                1 => {
                    // Add
                    self.codes[c] = self.codes[a] + self.codes[b];
                    self.pos += 4;
                }
                2 => {
                    // Multiply
                    self.codes[c] = self.codes[a] * self.codes[b];
                    self.pos += 4;
                }
                3 => {
                    // Input - try phase first
                    if self.phase.is_none() {
                        self.codes[a] = input;
                    } else {
                        self.codes[a] = self.phase.unwrap();
                        self.phase = None;
                    }
                    self.pos += 2;
                }
                4 => {
                    // Output
                    self.pos += 2;
                    return Some(self.codes[a]);
                }
                5 => {
                    // Jump if true
                    if self.codes[a] != 0 {
                        self.pos = usize::try_from(self.codes[b]).unwrap();
                    } else {
                        self.pos += 3;
                    }
                }
                6 => {
                    // Jump if false
                    if self.codes[a] == 0 {
                        self.pos = usize::try_from(self.codes[b]).unwrap();
                    } else {
                        self.pos += 3;
                    }
                }
                7 => {
                    // Less than
                    if self.codes[a] < self.codes[b] {
                        self.codes[c] = 1;
                    } else {
                        self.codes[c] = 0;
                    }
                    self.pos += 4;
                }
                8 => {
                    // Equal
                    if self.codes[a] == self.codes[b] {
                        self.codes[c] = 1;
                    } else {
                        self.codes[c] = 0;
                    }
                    self.pos += 4;
                }
                99 => return None,
                unknown => panic!("unknown op code: {}", unknown),
            }
        }
    }

    fn get_pos(&self, pos: usize, immediate: bool) -> usize {
        if immediate {
            return pos;
        }

        self.codes
            .get(pos)
            .map_or(0, |p| usize::try_from(*p).unwrap())
    }
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
        assert_eq!(find_max(&codes, 0), 54321);
    }

    #[test]
    fn part1_example3() {
        let codes = parse_codes(
            &"3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"
        );
        assert_eq!(find_max(&codes, 0), 65210);
    }

    #[test]
    fn part2_example1() {
        let codes = parse_codes(
            &"3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
        );
        assert_eq!(test_combination(&codes, &[9, 8, 7, 6, 5]), 139629729);
    }

    #[test]
    fn part2_example2() {
        let codes = parse_codes(
            &"3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",
        );
        assert_eq!(find_max(&codes, 5), 18216);
    }
}
