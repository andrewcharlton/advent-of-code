use std::fs;

fn main() {
    let mut passes: Vec<usize> = fs::read_to_string("input")
        .expect("couldn't read file")
        .lines()
        .map(seat_id)
        .collect();

    passes.sort();

    println!("Part one: {}", passes.iter().max().unwrap());

    let mut current = passes.get(0).unwrap() - 1;
    for id in passes {
        if id - current == 2 {
            println!("Part two: {}", current + 1);
            return;
        }
        current = id;
    }
}

fn seat_id(spec: &str) -> usize {
    spec.chars().fold(0, |mut acc, c| {
        acc *= 2;
        if c == 'B' || c == 'R' {
            acc += 1;
        }
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seat_id_test() {
        assert_eq!(seat_id(&"BFFFBBFRRR"), 567);
        assert_eq!(seat_id(&"FFFBBBFRRR"), 119);
        assert_eq!(seat_id(&"BBFFBBFRLL"), 820);
    }
}
