use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("unable to read file");
    let lines = content.lines();

    let (mut a, mut b, mut c, mut latest) = (0, 0, 0, 0);
    for line in lines {
        if line == "" {
            if latest > c {
                c = latest;
            }
            if latest > b {
                c = b;
                b = latest;
            }
            if latest > a {
                b = a;
                a = latest;
            }

            latest = 0;
            continue;
        }

        latest += line.parse::<u64>().unwrap();
    }

    println!("First: {}", a);
    println!("Second: {}", b);
    println!("Third: {}", c);
    println!("Sum: {}", a + b + c);
}
