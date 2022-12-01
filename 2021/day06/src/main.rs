use std::fs;

fn main() {
    let mut counts: [u64;9] = fs::read_to_string("input.txt")
        .expect("couldn't read file")
        .trim()
        .split(",")
        .map(|x| {
            x.parse::<usize>().unwrap()
        })
        .fold([0,0,0,0,0,0,0,0,0], |mut acc, x| {
            acc[x] += 1;
            acc
        });

    for _ in 0..80 {
        counts = iterate(counts);
    }
    let sum: u64 = counts.iter().sum();
    println!("Part one: {}", sum);

    for _ in 80..256 {
        counts = iterate(counts);
    }
    let sum: u64 = counts.iter().sum();
    println!("Part two: {}", sum);
}

fn iterate(fish: [u64; 9]) -> [u64;9] {
    [
        fish[1],
        fish[2],
        fish[3],
        fish[4],
        fish[5],
        fish[6],
        fish[0] + fish[7],
        fish[8],
        fish[0],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterate_test() {
        let state = [0,1,1,2,1,0,0,0,0,];
        let state = iterate(state);
        assert_eq!(state, [1,1,2,1,0,0,0,0,0], "First pass");

        let state = iterate(state);
        assert_eq!(state, [1, 2, 1, 0,0,0,1,0,1], "Second pass");
    }
}
