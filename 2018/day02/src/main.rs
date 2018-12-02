use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Couldn't read file");
    let input = input.lines().collect();

    let answer = doubles_and_triples(&input);
    println!("Part one: {}", answer.0 * answer.1);

    let answer = common_characters(&input).unwrap();
    println!("Part two: {}", answer);
}

fn checksum(input: &str) -> (bool, bool) {
    let mut counts: HashMap<char, u8> = HashMap::default();

    for c in input.chars() {
        counts.insert(c, counts.get(&c).unwrap_or(&0) + 1);
    }

    let two = counts.values().find(|&v| *v == 2).is_some();
    let three = counts.values().find(|&v| *v == 3).is_some();
    (two, three)
}

fn doubles_and_triples(input: &Vec<&str>) -> (u64, u64) {
    input.iter().map(|x| checksum(x)).fold((0, 0), |acc, x| {
        (
            if x.0 { acc.0 + 1 } else { acc.0 },
            if x.1 { acc.1 + 1 } else { acc.1 },
        )
    })
}

fn match_strings(a: &str, b: &str) -> Option<String> {
    let matching_chars: Vec<char> = a
        .chars()
        .zip(b.chars())
        .filter_map(|(x, y)| if x == y { Some(x) } else { None })
        .collect();

    if matching_chars.len() == a.len() - 1 {
        Some(matching_chars.iter().clone().collect())
    } else {
        None
    }
}

fn common_characters(input: &Vec<&str>) -> Option<String> {
    for (i, s) in input.iter().enumerate() {
        for t in input.iter().skip(i) {
            let m = match_strings(s, t);
            if m.is_some() {
                return m;
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksum_test() {
        assert_eq!(checksum("abcdef"), (false, false), "abcdef");
        assert_eq!(checksum("bababc"), (true, true), "bababc");
        assert_eq!(checksum("abbcde"), (true, false), "abbcde");
        assert_eq!(checksum("aabcdd"), (true, false), "aabcd");
        assert_eq!(checksum("abcccd"), (false, true), "abccd");
        assert_eq!(checksum("abcdee"), (true, false), "abcdee");
        assert_eq!(checksum("ababab"), (false, true), "ababab");
    }

    #[test]
    fn doubles_and_triples_test() {
        assert_eq!(
            doubles_and_triples(&vec![
                "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab"
            ]),
            (4, 3)
        );
    }

    #[test]
    fn match_strings_test() {
        assert_eq!(match_strings("abcde", "abfde"), Some(String::from("abde")));
    }

    #[test]
    fn common_characters_test() {
        assert_eq!(
            common_characters(&vec![
                "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz"
            ]),
            Some(String::from("fgij"))
        );
    }

}
