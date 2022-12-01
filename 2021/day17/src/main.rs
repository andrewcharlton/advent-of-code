fn main() {
    println!("Hello, world!");
}

fn valid_x(min: i64, max: i64) -> Vec<i64> {
    let mut valid = Vec::new();
    for v in 1..=max {
        let mut x = 0;
        let mut v = v;
        loop {
            x += v;
            v -= 1;

            if x > max {
                break;
            }
            if x > min && x < max {
                valid.push(v);
                break;
            }
            if v == 0 {
                break;
            }
        }
    }

    valid
}
