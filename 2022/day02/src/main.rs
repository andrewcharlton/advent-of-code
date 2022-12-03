use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("unable to read file");

    println!("Part one: {}", part_one(&content));
    println!("Part two: {}", part_two(&content));
}

fn part_one(input: &str) -> u64 {
    input.lines().into_iter().fold(0, |acc, line| {
        acc + match line {
            "A X" => 3 + 1, // Rock - Rock (Draw)
            "A Y" => 6 + 2, // Rock - Paper (Win)
            "A Z" => 0 + 3, // Rock - Scissors (Loss)
            "B X" => 0 + 1, // Paper - Rock (Loss)
            "B Y" => 3 + 2, // Paper - Paper (Draw)
            "B Z" => 6 + 3, // Paper - Scissors (Win)
            "C X" => 6 + 1, // Scissors - Rock (Win)
            "C Y" => 0 + 2, // Scissors - Paper (Loss)
            "C Z" => 3 + 3, // Scissors - Scissors (Draw)
            _ => panic!("unrecognised input"),
        }
    })
}

fn part_two(input: &str) -> u64 {
    input.lines().into_iter().fold(0, |acc, line| {
        acc + match line {
            "A X" => 0 + 3, // Rock - Scissors (Loss)
            "A Y" => 3 + 1, // Rock - Rock (Draw)
            "A Z" => 6 + 2, // Rock - Paper (Win)
            "B X" => 0 + 1, // Paper - Rock (Loss)
            "B Y" => 3 + 2, // Paper - Paper (Draw)
            "B Z" => 6 + 3, // Paper - Scissors (Win)
            "C X" => 0 + 2, // Scissors - Paper (Loss)
            "C Y" => 3 + 3, // Scissors - Scissors (Draw)
            "C Z" => 6 + 1, // Scissors - Rock (Win)
            _ => panic!("unrecognised input"),
        }
    })
}
