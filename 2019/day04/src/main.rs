fn main() {
    let n = (245318..765747)
        .map(split)
        .filter(is_ascending)
        .filter(has_duplicates)
        .count();
    println!("Part one: {}", n);

    let n = (245318..765747)
        .map(split)
        .filter(is_ascending)
        .filter(has_duplicate_pair)
        .count();
    println!("Part two: {}", n);
}

fn split(mut n: usize) -> [usize; 6] {
    let mut s: [usize; 6] = [0, 0, 0, 0, 0, 0];

    for i in 0..6 {
        s[5 - i] = n % 10;
        n = n / 10;
    }

    s
}

fn is_ascending(n: &[usize; 6]) -> bool {
    for i in 0..5 {
        if n[i] > n[i + 1] {
            return false;
        }
    }

    true
}

fn has_duplicates(n: &[usize; 6]) -> bool {
    let mut current = n[0];
    for i in 1..6 {
        if n[i] == current {
            return true;
        }
        current = n[i]
    }

    false
}

fn has_duplicate_pair(n: &[usize; 6]) -> bool {
    let mut current = n[0];
    let mut size = 1;

    for i in 1..6 {
        if n[i] == current {
            size += 1;
            continue;
        }

        // New digit is different, check if previous
        // section had length 2
        if size == 2 {
            return true;
        }

        current = n[i];
        size = 1;
    }

    // check whether the final two are part of a pair
    size == 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_test() {
        assert_eq!(split(123456), [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn is_ascending_test() {
        assert_eq!(is_ascending(&[1, 2, 3, 4, 5, 6]), true, "123456");
        assert_eq!(is_ascending(&[1, 2, 3, 4, 3, 5]), false, "123435");
    }

    #[test]
    fn has_duplicates_test() {
        assert_eq!(has_duplicates(&[1, 2, 3, 4, 5, 6]), false, "123456");
        assert_eq!(has_duplicates(&[1, 2, 3, 3, 4, 5]), true, "123345");
    }

    #[test]
    fn has_just_duplicates_test() {
        assert_eq!(has_duplicate_pair(&[1, 1, 2, 2, 3, 3]), true, "112233");
        assert_eq!(has_duplicate_pair(&[1, 2, 3, 4, 4, 4]), false, "123444");
        assert_eq!(has_duplicate_pair(&[1, 1, 1, 1, 2, 2]), true, "111122");
    }
}
