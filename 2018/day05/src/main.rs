use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Couldnt read file");
    let chars: Vec<char> = input.chars().filter(|c| c.is_alphabetic()).collect();

    println!("Part one: {}", reduce_polymer(chars));
    println!("Part two: {}", problem_unit(&input));
}

fn reduce_polymer(mut chars: Vec<char>) -> usize {
    let mut i = 0;
    loop {
        let x = chars.get(i).unwrap();
        let y = chars.get(i + 1);
        if y.is_none() {
            // End of the vector, escape
            break;
        }

        let y = y.unwrap();
        if reacts(*x, *y) {
            chars.remove(i + 1);
            chars.remove(i);
            if i > 0 {
                i = i - 1;
            }
        } else {
            i += 1;
        }
    }

    chars.len()
}

fn problem_unit(input: &str) -> usize {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    alphabet
        .chars()
        .map(|c| {
            let polymer = remove_unit(input, c);
            reduce_polymer(polymer)
        })
        .min()
        .unwrap()
}

fn reacts(a: char, b: char) -> bool {
    if a == b {
        return false;
    }

    if a.to_lowercase().next().unwrap() == b.to_lowercase().next().unwrap() {
        return true;
    }

    return false;
}

fn remove_unit(input: &str, unit: char) -> Vec<char> {
    let lower = unit.to_lowercase().next().unwrap();
    let upper = unit.to_uppercase().next().unwrap();

    input
        .chars()
        .filter(|&c| c.is_alphabetic() && c != lower && c != upper)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reacts_test() {
        assert_eq!(reacts('a', 'A'), true, "aA");
        assert_eq!(reacts('a', 'a'), false, "aa");
        assert_eq!(reacts('b', 'a'), false, "ba");
    }

    #[test]
    fn reduce_test() {
        let input = "dabAcCaCBAcCcaDA".chars().collect();
        assert_eq!(reduce_polymer(input), 10);
    }

    #[test]
    fn remove_unit_test() {
        assert_eq!(
            remove_unit("dabAcCaCBAcCcaDA", 'a')
                .iter()
                .collect::<String>(),
            String::from("dbcCCBcCcD")
        );
    }
}
