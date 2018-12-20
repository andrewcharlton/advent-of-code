use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Couldn't read file");
    let mut yard = parse_input(&input);

    for _ in 0..10 {
        yard = transform(yard);
    }
    println!("Part one: {}", total_resource_value(yard));

    // Try and find a repeat period for the scores
    let mut yard = parse_input(&input);
    let mut scores = Vec::new();
    let mut period = 0;
    loop {
        yard = transform(yard);
        let val = total_resource_value(yard);
        scores.push(val);

        let prev = scores
            .iter()
            .enumerate()
            .rev()
            .skip(1)
            .find(|&(_, x)| x == &val)
            .map(|(i, _)| i);

        if prev.is_none() {
            continue;
        }
        let prev = prev.unwrap();

        // Try and find 3 repeating values
        if prev > 5 {
            let mut matches = true;
            for i in 1..5 {
                if scores[prev - i] != scores[scores.len() - i - 1] {
                    matches = false;
                    break;
                }
            }

            if matches {
                period = scores.len() - prev - 1;
                break;
            }
        }
    }

    let target = 1_000_000_000;
    let offset = (target - scores.len()) % period;
    println!("Part two: {}", scores[scores.len() - 1 - period + offset]);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Acre {
    Open,
    Tree,
    Lumber,
}

type Yard = [[Acre; 50]; 50];

fn transform(yard: Yard) -> Yard {
    let mut next = [[Acre::Open; 50]; 50];

    for x in 0..50 {
        for y in 0..50 {
            let adjacent: Vec<Acre> = neighbours(x, y).iter().map(|&(x, y)| yard[x][y]).collect();
            next[x][y] = contents(yard[x][y], adjacent);
        }
    }

    next
}

fn neighbours(x: usize, y: usize) -> Vec<(usize, usize)> {
    let x_min = if x > 0 { x - 1 } else { 0 };
    let x_max = if x < 49 { x + 1 } else { 49 };

    let y_min = if y > 0 { y - 1 } else { 0 };
    let y_max = if y < 49 { y + 1 } else { 49 };

    let mut n = Vec::new();

    for i in x_min..x_max + 1 {
        for j in y_min..y_max + 1 {
            if i != x || j != y {
                n.push((i, j));
            }
        }
    }

    n
}

fn contents(current: Acre, adjacent: Vec<Acre>) -> Acre {
    match current {
        Acre::Open => {
            let trees = adjacent.iter().filter(|&a| *a == Acre::Tree).count();
            if trees >= 3 {
                Acre::Tree
            } else {
                Acre::Open
            }
        }
        Acre::Tree => {
            let lumberyards = adjacent.iter().filter(|&a| *a == Acre::Lumber).count();
            if lumberyards >= 3 {
                Acre::Lumber
            } else {
                Acre::Tree
            }
        }
        Acre::Lumber => {
            if adjacent.contains(&Acre::Lumber) && adjacent.contains(&Acre::Tree) {
                Acre::Lumber
            } else {
                Acre::Open
            }
        }
    }
}

fn total_resource_value(yard: Yard) -> usize {
    let mut trees = 0;
    let mut lumberyards = 0;

    for x in 0..50 {
        for y in 0..50 {
            match yard[x][y] {
                Acre::Open => (),
                Acre::Tree => trees += 1,
                Acre::Lumber => lumberyards += 1,
            }
        }
    }

    trees * lumberyards
}

fn parse_input(input: &str) -> Yard {
    let mut yard = [[Acre::Open; 50]; 50];

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let a = match c {
                '.' => Acre::Open,
                '|' => Acre::Tree,
                '#' => Acre::Lumber,
                _ => panic!("{} character not recognised", c),
            };
            yard[x][y] = a;
        }
    }

    yard
}
