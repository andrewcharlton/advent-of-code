use std::fs;

fn main() {
    println!("Part one: {}", minimum_fuel_median("input.txt"));
    println!("Part two: {}", minimum_fuel_mean("input.txt"));
}

fn minimum_fuel_median(filename: &str) -> u64 {
    let mut crabs: Vec<u64> = fs::read_to_string(filename)
        .expect("couldn't open file")
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    crabs.sort();
    let median = if crabs.len() % 2 == 0 {
        let a = *crabs.get((crabs.len()/2) - 1).unwrap();
        let b = *crabs.get(crabs.len()/2 ).unwrap();
        (a + b) / 2
    } else {
        *crabs.get((crabs.len() -1) / 2).unwrap()
    };

    println!("Median: {}", median);

    crabs.iter()
        .map(|x| if x > &median { x - median } else { median - x })
        .sum()
}

fn minimum_fuel_mean(filename: &str) -> usize {
    let crabs: Vec<usize> = fs::read_to_string(filename)
        .expect("couldn't open file")
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let mean = crabs.iter().sum::<usize>() + (crabs.len() / 2);
    let mean = mean / crabs.len();
    println!("Mean: {}", mean);

    crabs.iter()
        .map(|x| if x > &mean { x - mean } else { mean - x })
        .map(|x| x*(x+1)/2)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(minimum_fuel_median("example.txt"), 37, "Part one");
        assert_eq!(minimum_fuel_mean("example.txt"), 168, "Part one");
    }
}
