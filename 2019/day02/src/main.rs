use std::fs;

fn main() {
    let ints: Vec<usize> = fs::read_to_string("input")
        .expect("couldn't read file")
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    println!("Part one: {}", run_simulation(ints.clone(), 12, 2));

    for i in 0..100 {
        for j in 0..100 {
            let ans = run_simulation(ints.clone(), i, j);
            if ans == 19690720 {
                println!("Part two: {}", 100 * i + j);
                return;
            }
        }
    }
}

fn run_simulation(mut ints: Vec<usize>, pos1: usize, pos2: usize) -> usize {
    ints[1] = pos1;
    ints[2] = pos2;
    process(ints)
}

fn process(mut ints: Vec<usize>) -> usize {
    let mut pos = 0;
    loop {
        match ints[pos] {
            1 => {
                let p = ints[pos + 3];
                ints[p] = ints[ints[pos + 1]] + ints[ints[pos + 2]];
            }
            2 => {
                let p = ints[pos + 3];
                ints[p] = ints[ints[pos + 1]] * ints[ints[pos + 2]];
            }
            99 => return ints[0],
            op => panic!("Unknown opcode: {}", op),
        }
        pos += 4;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_test() {
        assert_eq!(process(vec![1, 0, 0, 0, 99]), 2);
        assert_eq!(process(vec![2, 3, 0, 3, 99]), 2);
        assert_eq!(process(vec![2, 4, 4, 5, 99, 0]), 2);
        assert_eq!(process(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]), 30);
    }
}
