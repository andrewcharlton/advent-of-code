const SERIAL_NUMBER: i64 = 7989;

fn main() {
    let image = create_sum_table(SERIAL_NUMBER);

    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_power = 0;
    for x in 1..299 {
        for y in 1..299 {
            let power = calc_square_power(&image, x, y, 3);
            if power > max_power {
                max_x = x;
                max_y = y;
                max_power = power;
            }
        }
    }

    println!("Part one: {}, {}", max_x, max_y);

    let mut max_size = 0;
    for size in 4..25 {
        for x in 1..302 - size {
            for y in 1..302 - size {
                let power = calc_square_power(&image, x, y, size);
                if power > max_power {
                    max_x = x;
                    max_y = y;
                    max_power = power;
                    max_size = size;
                }
            }
        }
    }
    println!("Part two: {}, {}, {}", max_x, max_y, max_size);
}

fn create_sum_table(serial_number: i64) -> [[i64; 301]; 301] {
    let mut image: [[i64; 301]; 301] = [[0; 301]; 301];

    for x in 1..301 {
        for y in 1..301 {
            image[x][y] = image[x][y - 1] + image[x - 1][y] + calc_power(x, y, serial_number)
                - image[x - 1][y - 1];
        }
    }

    image
}

fn calc_power(x: usize, y: usize, serial_number: i64) -> i64 {
    let rack_id = x as i64 + 10;
    let mut power = rack_id * (y as i64) + serial_number as i64;
    power *= rack_id;
    power /= 100;
    power %= 10;
    power -= 5;

    power
}

fn calc_square_power(image: &[[i64; 301]; 301], x: usize, y: usize, size: usize) -> i64 {
    let (min_x, max_x) = (x + size - 1, x - 1);
    let (min_y, max_y) = (y + size - 1, y - 1);

    image[max_x][max_y] - image[max_x][min_y] - image[min_x][max_y] + image[min_x][min_y]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calc_power_test() {
        assert_eq!(calc_power(3, 5, 8), 4);
        assert_eq!(calc_power(122, 79, 57), -5);
        assert_eq!(calc_power(217, 196, 39), 0);
        assert_eq!(calc_power(101, 153, 71), 4);
    }

    #[test]
    fn calc_square_power_test() {
        let image = create_sum_table(18);
        assert_eq!(calc_square_power(&image, 33, 45, 3), 29);
    }
}
