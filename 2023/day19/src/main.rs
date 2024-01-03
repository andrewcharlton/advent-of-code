use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", solve(INPUT));
    println!("Part two: {}", solve_slices(INPUT));
}

fn solve(input: &str) -> usize {
    let (workflows, parts) = parse_input(input);

    parts
        .iter()
        .filter(|part| part.is_accepted(&workflows))
        .map(|part| part.sum())
        .sum()
}

fn solve_slices(input: &str) -> usize {
    use Destination::*;

    let (workflows, _) = parse_input(input);
    let slice = PartSlice {
        x_min: 1,
        x_max: 4000,
        m_min: 1,
        m_max: 4000,
        a_min: 1,
        a_max: 4000,
        s_min: 1,
        s_max: 4000,
    };

    let mut total = 0;

    let mut current: Vec<(Destination, PartSlice)> =
        vec![(Destination::Workflow(String::from("in")), slice)];

    loop {
        if let Some((dest, slice)) = current.pop() {
            match dest {
                Accepted => total += slice.combinations(),
                Rejected => {}
                Workflow(w) => {
                    let workflow = workflows.get(&w).unwrap();
                    current.extend_from_slice(&workflow.apply_slice(&slice));
                }
            }
        } else {
            return total;
        }
    }
}

fn parse_input(input: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let workflows = workflows
        .lines()
        .map(|line| {
            let workflow = Workflow::parse(line);
            (workflow.name.clone(), workflow)
        })
        .collect();

    let parts = parts.lines().map(Part::parse).collect();

    (workflows, parts)
}

#[derive(Clone, Debug)]
enum Destination {
    Accepted,
    Rejected,
    Workflow(String),
}

impl Destination {
    fn parse(s: &str) -> Self {
        use Destination::*;

        match s {
            "A" => Accepted,
            "R" => Rejected,
            _ => Workflow(s.to_string()),
        }
    }
}

enum Category {
    X,
    M,
    A,
    S,
}

impl Category {
    fn parse(s: &str) -> Self {
        use Category::*;

        match s {
            "x" => X,
            "m" => M,
            "a" => A,
            "s" => S,
            _ => panic!("unrecognised category: {}", s),
        }
    }
}

#[derive(Clone)]
enum Operator {
    LessThan,
    GreaterThan,
}

impl Operator {
    fn parse(s: &str) -> Self {
        use Operator::*;

        match s {
            "<" => LessThan,
            ">" => GreaterThan,
            _ => panic!("unrecognised operator: {}", s),
        }
    }
}

struct Comparison {
    s: String,
    category: Category,
    op: Operator,
    n: usize,
    dest: Destination,
}

impl Comparison {
    fn parse(s: &str) -> Self {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\w+)([<>])(\d+):(\w+)").unwrap());
        let caps = RE.captures(s).unwrap();

        Comparison {
            s: caps.get(0).unwrap().as_str().to_string(),
            category: Category::parse(caps.get(1).unwrap().as_str()),
            op: Operator::parse(caps.get(2).unwrap().as_str()),
            n: caps.get(3).unwrap().as_str().parse().unwrap(),
            dest: Destination::parse(caps.get(4).unwrap().as_str()),
        }
    }

    fn apply(&self, part: &Part) -> Option<Destination> {
        use Category::*;
        use Operator::*;

        let val = match self.category {
            X => part.x,
            M => part.m,
            A => part.a,
            S => part.s,
        };

        match self.op {
            LessThan => {
                if val < self.n {
                    Some(self.dest.clone())
                } else {
                    None
                }
            }
            GreaterThan => {
                if val > self.n {
                    Some(self.dest.clone())
                } else {
                    None
                }
            }
        }
    }

    fn split_slice(
        &self,
        slice: &PartSlice,
    ) -> (Option<PartSlice>, Option<(Destination, PartSlice)>) {
        use Category::*;
        use Operator::*;

        dbg!("Applying comparison: {} to slice {:?}", &self.s, slice);

        let mut a = slice.clone();
        let mut any_remaining = true;

        let mut b = slice.clone();
        let mut split_succeeds = true;

        match (&self.category, &self.op) {
            (X, LessThan) => {
                if slice.x_max < self.n {
                    // Everything goes through
                    any_remaining = false;
                } else if slice.x_min >= self.n {
                    // Nothing goes through
                    split_succeeds = false;
                } else {
                    // Otherwise, we get a split.
                    a.x_min = self.n;
                    b.x_max = self.n - 1;
                }
            }
            (X, GreaterThan) => {
                if slice.x_min > self.n {
                    // Everything goes through
                    any_remaining = false;
                } else if slice.x_max <= self.n {
                    // Nothing goes through
                    split_succeeds = false;
                } else {
                    // Otherwise, we get a split.
                    a.x_max = self.n;
                    b.x_min = self.n + 1;
                }
            }
            (M, LessThan) => {
                if slice.m_max < self.n {
                    // Everything goes through
                    any_remaining = false;
                } else if slice.m_min >= self.n {
                    // Nothing goes through
                    split_succeeds = false;
                } else {
                    // Otherwise, we get a split.
                    a.m_min = self.n;
                    b.m_max = self.n - 1;
                }
            }
            (M, GreaterThan) => {
                if slice.m_min > self.n {
                    // Everything goes through
                    any_remaining = false;
                } else if slice.m_max <= self.n {
                    // Nothing goes through
                    split_succeeds = false;
                } else {
                    // Otherwise, we get a split.
                    a.m_max = self.n;
                    b.m_min = self.n + 1;
                }
            }
            (A, LessThan) => {
                if slice.a_max < self.n {
                    // Everything goes through
                    any_remaining = false;
                } else if slice.a_min >= self.n {
                    // Nothing goes through
                    split_succeeds = false;
                } else {
                    // Otherwise, we get a split.
                    a.a_min = self.n;
                    b.a_max = self.n - 1;
                }
            }
            (A, GreaterThan) => {
                if slice.a_min > self.n {
                    // Everything goes through
                    any_remaining = false;
                } else if slice.a_max <= self.n {
                    // Nothing goes through
                    split_succeeds = false;
                } else {
                    // Otherwise, we get a split.
                    a.a_max = self.n;
                    b.a_min = self.n + 1;
                }
            }
            (S, LessThan) => {
                if slice.s_max < self.n {
                    // Everything goes through
                    any_remaining = false;
                } else if slice.s_min >= self.n {
                    // Nothing goes through
                    split_succeeds = false;
                } else {
                    // Otherwise, we get a split.
                    a.s_min = self.n;
                    b.s_max = self.n - 1;
                }
            }
            (S, GreaterThan) => {
                if slice.s_min > self.n {
                    // Everything goes through
                    any_remaining = false;
                } else if slice.s_max <= self.n {
                    // Nothing goes through
                    split_succeeds = false;
                } else {
                    // Otherwise, we get a split.
                    a.s_max = self.n;
                    b.s_min = self.n + 1;
                }
            }
        };

        let a = if any_remaining { Some(a) } else { None };
        let b = if split_succeeds {
            Some((self.dest.clone(), b))
        } else {
            None
        };
        dbg!("    -> {:?}", &a);
        dbg!("    -> {:?}", &b);
        (a, b)
    }
}

struct Workflow {
    name: String,
    comparisons: Vec<Comparison>,
    dest: Destination,
}

impl Workflow {
    fn parse(s: &str) -> Self {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\w+)\{(.*),(\w+)\}$").unwrap());
        let caps = RE.captures(s).unwrap();
        let comparisons = caps
            .get(2)
            .unwrap()
            .as_str()
            .split(",")
            .map(Comparison::parse)
            .collect();

        Workflow {
            name: caps.get(1).unwrap().as_str().to_string(),
            comparisons,
            dest: Destination::parse(caps.get(3).unwrap().as_str()),
        }
    }

    fn apply(&self, part: &Part) -> Destination {
        for comparison in &self.comparisons {
            if let Some(dest) = comparison.apply(part) {
                return dest;
            }
        }
        self.dest.clone()
    }

    fn apply_slice(&self, slice: &PartSlice) -> Vec<(Destination, PartSlice)> {
        let mut slices = Vec::new();

        let mut remaining = slice.clone();
        for comparison in &self.comparisons {
            let (r, split) = comparison.split_slice(&remaining);

            if split.is_some() {
                slices.push(split.unwrap());
            }

            if r.is_some() {
                remaining = r.unwrap();
            } else {
                return slices;
            }
        }

        // If we still have some left over, that will go to the final destination.
        slices.push((self.dest.clone(), remaining));

        slices
    }
}

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn parse(s: &str) -> Self {
        static RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}$").unwrap());
        let caps = RE.captures(s).unwrap();

        Part {
            x: caps.get(1).unwrap().as_str().parse().unwrap(),
            m: caps.get(2).unwrap().as_str().parse().unwrap(),
            a: caps.get(3).unwrap().as_str().parse().unwrap(),
            s: caps.get(4).unwrap().as_str().parse().unwrap(),
        }
    }

    fn is_accepted(&self, workflows: &HashMap<String, Workflow>) -> bool {
        use Destination::*;

        let mut current = String::from("in");

        loop {
            let workflow = workflows.get(&current).unwrap();
            let next = workflow.apply(self);

            match next {
                Accepted => return true,
                Rejected => return false,
                Workflow(x) => current = x,
            }
        }
    }

    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Clone, Debug)]
struct PartSlice {
    x_min: usize,
    x_max: usize,
    m_min: usize,
    m_max: usize,
    a_min: usize,
    a_max: usize,
    s_min: usize,
    s_max: usize,
}

impl PartSlice {
    fn combinations(&self) -> usize {
        (self.x_max - self.x_min + 1)
            * (self.m_max - self.m_min + 1)
            * (self.a_max - self.a_min + 1)
            * (self.s_max - self.s_min + 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(solve(EXAMPLE), 19114, "Part one");
        assert_eq!(solve_slices(EXAMPLE), 167409079868000, "Part one");
    }
}
