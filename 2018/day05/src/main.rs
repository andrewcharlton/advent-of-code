use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Couldnt read file");
    let chars: Vec<char> = input.chars().filter(|c| c.is_alphabetic()).collect();

    println!("Part one: {}", reduce_polymer(chars));
    println!("Part two: {}", problem_unit(&input));
}

fn reduce_polymer(chars: Vec<char>) -> usize {
    let mut stack = Vec::with_capacity(chars.len());
    let mut last: Option<&char> = chars.get(0);

    for c in chars.iter().skip(1) {
        if last.is_none() {
            last = Some(c);
            continue;
        }

        if reacts(c, last.unwrap()) {
            last = stack.pop();
            continue;
        }

        stack.push(&last.unwrap());
        last = Some(c);
    }

    if last.is_some() {
        stack.push(&last.unwrap());
    }

    stack.len()
}

fn problem_unit(input: &str) -> usize {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    alphabet
        .chars()
        .map(|c| {
            let polymer = remove_unit(input, &c);
            reduce_polymer(polymer)
        })
        .min()
        .unwrap()
}

fn reacts(a: &char, b: &char) -> bool {
    a != b && a.eq_ignore_ascii_case(b)
}

fn remove_unit(input: &str, unit: &char) -> Vec<char> {
    input
        .chars()
        .filter(|&c| c.is_alphabetic() && !c.eq_ignore_ascii_case(unit))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reacts_test() {
        assert_eq!(reacts(&'a', &'A'), true, "aA");
        assert_eq!(reacts(&'a', &'a'), false, "aa");
        assert_eq!(reacts(&'b', &'a'), false, "ba");
    }

    #[test]
    fn reduce_test() {
        let input = "dabAcCaCBAcCcaDA".chars().collect();
        assert_eq!(reduce_polymer(input), 10);
    }

    #[test]
    fn remove_unit_test() {
        assert_eq!(
            remove_unit("dabAcCaCBAcCcaDA", &'a')
                .iter()
                .collect::<String>(),
            String::from("dbcCCBcCcD")
        );
    }
}
