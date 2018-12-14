fn main() {
    println!("Part one: {}", next_10(702831));
    println!("Part two: {}", how_many_until(&[7, 0, 2, 8, 3, 1]))
}

fn next_10(find_after: usize) -> usize {
    let mut scores = vec![3, 7];
    let mut elf1 = 0;
    let mut elf2 = 1;

    while scores.len() < find_after + 10 {
        let recipe1 = *scores.get(elf1).unwrap();
        let recipe2 = *scores.get(elf2).unwrap();

        let new_recipe = recipe1 + recipe2;
        if new_recipe >= 10 {
            scores.push(new_recipe / 10);
        }
        scores.push(new_recipe % 10);

        elf1 = (elf1 + 1 + recipe1) % scores.len();
        elf2 = (elf2 + 1 + recipe2) % scores.len();
    }

    scores[find_after..find_after + 10]
        .into_iter()
        .fold(0, |acc, x| 10 * acc + x)
}

fn how_many_until(looking_for: &[usize]) -> usize {
    let mut scores = vec![3, 7];
    let mut elf1 = 0;
    let mut elf2 = 1;

    loop {
        let recipe1 = *scores.get(elf1).unwrap();
        let recipe2 = *scores.get(elf2).unwrap();

        let new_recipe = recipe1 + recipe2;
        if new_recipe >= 10 {
            scores.push(new_recipe / 10);
            if scores.ends_with(looking_for) {
                return scores.len() - looking_for.len();
            }
        }

        scores.push(new_recipe % 10);
        if scores.ends_with(looking_for) {
            return scores.len() - looking_for.len();
        }

        elf1 = (elf1 + 1 + recipe1) % scores.len();
        elf2 = (elf2 + 1 + recipe2) % scores.len();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn next_10_test() {
        assert_eq!(5158916779, next_10(9));
        assert_eq!(0124515891, next_10(5));
        assert_eq!(9251071085, next_10(18));
        assert_eq!(5941429882, next_10(2018));
    }

    #[test]
    fn how_many_until_test() {
        assert_eq!(9, how_many_until(&[5, 1, 5, 8, 9]));
        assert_eq!(5, how_many_until(&[0, 1, 2, 4, 5]));
    }
}
