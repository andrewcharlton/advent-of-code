use std::fs;

fn main() {
    let captcha = fs::read_to_string("captcha.txt").unwrap();
    let digits = convert_input(captcha);
    println!("Part one: {}", sum_matching(&digits, 1));
    println!("Part two: {}", sum_matching(&digits, digits.len() / 2));
}

fn sum_matching(digits: &Vec<u32>, offset: usize) -> u32 {
    let len = digits.len();

    digits.iter().enumerate().fold(0, |acc, x| {
        if digits[(x.0 + offset) % len] == *x.1 {
            acc + x.1
        } else {
            acc
        }
    })
}

fn convert_input(input: String) -> Vec<u32> {
    input
        .chars()
        .map(|c| c.to_digit(10))
        .filter(|d| d.is_some())
        .map(|d| d.unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_conversion_works() {
        assert_eq!(convert_input(String::from("1122\n\n")), vec![1, 1, 2, 2]);
    }

    #[test]
    fn part_one() {
        assert_eq!(sum_matching(&vec![1, 1, 2, 2], 1), 3, "1122 returns 3");
        assert_eq!(sum_matching(&vec![1, 1, 1, 1], 1), 4, "1111 returns 4");
        assert_eq!(sum_matching(&vec![1, 2, 3, 4], 1), 0, "1234 returns 0");
        assert_eq!(
            sum_matching(&vec![9, 1, 2, 1, 2, 1, 2, 9], 1),
            9,
            "91212129 returns 9"
        );
    }

    #[test]
    fn part_two() {
        assert_eq!(sum_matching(&vec![1, 2, 1, 2], 2), 6, "1212 returns 6");
        assert_eq!(sum_matching(&vec![1, 2, 2, 1], 2), 0, "1221 returns 0");
        assert_eq!(
            sum_matching(&vec![1, 2, 3, 4, 2, 5], 3),
            4,
            "123425 returns 4"
        );
        assert_eq!(
            sum_matching(&vec![1, 2, 3, 1, 2, 3], 3),
            12,
            "123123 returns 12"
        );
        assert_eq!(
            sum_matching(&vec![1, 2, 1, 3, 1, 4, 1, 5], 4),
            4,
            "12131415 returns 4"
        );
    }
}
