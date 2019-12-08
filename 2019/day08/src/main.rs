use std::fs;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const AREA: usize = WIDTH * HEIGHT;

fn main() {
    let input = fs::read_to_string("input").expect("couldn't read input");
    let input = input.trim();

    let pixels = get_pixels(&input);

    let part1 = layer_counts(&pixels)
        .iter()
        .min_by(|x, y| x.0.cmp(&y.0))
        .map(|c| c.1 * c.2)
        .unwrap();
    println!("Part one: {}", part1);

    // Part two
    let image = build_image(&pixels, AREA);
    println!("{:?}", image);
    print_image(&image, WIDTH);
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Pixel {
    Black,
    White,
    Transparent,
}

fn get_pixels(input: &str) -> Vec<Pixel> {
    input
        .chars()
        .map(|d| match d {
            '0' => Pixel::Black,
            '1' => Pixel::White,
            '2' => Pixel::Transparent,
            d => panic!("Unrecognised digit: {}", d),
        })
        .collect()
}

fn layer_counts(pixels: &Vec<Pixel>) -> Vec<(usize, usize, usize)> {
    let mut counts = Vec::new();

    let (mut b, mut w, mut t) = (0, 0, 0);
    pixels.iter().enumerate().for_each(|(i, p)| {
        match p {
            Pixel::Black => b += 1,
            Pixel::White => w += 1,
            Pixel::Transparent => t += 1,
        }

        if i % AREA == AREA - 1 {
            counts.push((b, w, t));
            b = 0;
            w = 0;
            t = 0;
        }
    });

    counts
}

fn build_image(pixels: &Vec<Pixel>, size: usize) -> Vec<Pixel> {
    pixels
        .iter()
        .enumerate()
        .fold(vec![Pixel::Transparent; size], |mut acc, (i, p)| {
            let i = i % size;
            if acc[i] == Pixel::Transparent {
                acc[i] = *p;
            }

            acc
        })
}

fn print_image(pixels: &Vec<Pixel>, width: usize) {
    pixels.iter().enumerate().for_each(|(i, p)| {
        if i % width == 0 {
            print!("\n");
        }

        match p {
            Pixel::White => print!("■"),
            _ => print!(" "),
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_image_test() {
        let input = "0222112222120000";
        let pixels = get_pixels(&input);
        let image = build_image(&pixels, 4);
        assert_eq!(
            vec![Pixel::Black, Pixel::White, Pixel::White, Pixel::Black],
            image
        );
    }
}
