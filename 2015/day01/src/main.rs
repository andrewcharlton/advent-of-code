use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Couldn't read file.");

    println!("Part one: {}", find_floor(&input));
    println!("Part two: {}", into_basement(&input));
}

fn find_floor(input: &str) -> i64 {
    input.chars().fold(0, |acc, x| match x {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    })
}

fn into_basement(input: &str) -> usize {
    let mut floor = 0;
    let final_instruction = input.chars().enumerate().find(|(_, c)| {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };

        floor < 0
    });

    final_instruction.unwrap().0 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_floor_test() {
        assert_eq!(find_floor("()()"), 0);
        assert_eq!(find_floor("(()(()("), 3);
    }

    #[test]
    fn into_basement_test() {
        assert_eq!(into_basement(")"), 1);
        assert_eq!(into_basement("()())"), 5);
    }
}
