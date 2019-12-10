use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::f64::consts::PI;
use std::fs;
use std::iter::FromIterator;

fn main() {
    let input = fs::read_to_string("input").expect("couldn't read input");

    let locations = parse_locations(&input);
    let centre = locations
        .iter()
        .map(|(x, y)| (x, y, visible_asteroids(&locations, (*x, *y))))
        .max_by(|x, y| x.2.cmp(&y.2))
        .unwrap();
    println!(
        "Part one: ({}, {}) - {} asteroids visible",
        centre.0, centre.1, centre.2
    );

    let mut rel_vecs = relative_vectors(&locations, &(*centre.0, *centre.1));

    let mut keys = Vec::from_iter(rel_vecs.keys().cloned());
    keys.sort_by(|a, b| angle(a.0, a.1).partial_cmp(&angle(b.0, b.1)).unwrap());

    let mut i = 0;
    for key in keys.iter().cycle() {
        let list = rel_vecs.get_mut(key).unwrap();
        if list.len() == 0 {
            continue;
        }

        let loc = list.remove(0);
        i += 1;
        if i == 200 {
            println!("Part two: {}", (loc.0 + centre.0) * 100 + loc.1 + centre.1);
            break;
        }
    }
}

fn parse_locations(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| c == &'#')
                .map(move |(x, _)| (x, y))
        })
        .map(|(x, y)| (i32::try_from(x).unwrap(), i32::try_from(y).unwrap()))
        .collect()
}

fn visible_asteroids(locations: &Vec<(i32, i32)>, pos: (i32, i32)) -> usize {
    let visible: HashSet<(i32, i32)> = locations
        .iter()
        .filter(|&loc| loc != &pos)
        .map(|&(x, y)| minimal_vector(x - pos.0, y - pos.1))
        .collect();

    visible.len()
}

fn relative_vectors(
    locations: &Vec<(i32, i32)>,
    centre: &(i32, i32),
) -> HashMap<(i32, i32), Vec<(i32, i32)>> {
    let mut map: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();

    locations
        .iter()
        .filter(|&loc| loc != centre)
        .for_each(|loc| {
            let x = loc.0 - centre.0;
            let y = loc.1 - centre.1;

            let min_vec = minimal_vector(x, y);
            if map.contains_key(&min_vec) {
                map.get_mut(&min_vec).unwrap().push((x, y));
            } else {
                map.insert(min_vec, vec![(x, y)]);
            }
        });

    // Sort the vectors from closest to center to furthest.
    // Because they're parallel we can just use a single coordinate
    // here to compare
    for val in map.values_mut() {
        val.sort_by(|a, b| a.0.abs().cmp(&b.0.abs()));
    }

    map
}

fn minimal_vector(x: i32, y: i32) -> (i32, i32) {
    if x == 0 && y == 0 {
        return (0, 0);
    }

    if x == 0 {
        return (0, y / y.abs());
    }

    if y == 0 {
        return (x / x.abs(), 0);
    }

    let d = gcd(x.abs(), y.abs());
    (x / d, y / d)
}

fn gcd(a: i32, b: i32) -> i32 {
    if a % b == 0 {
        return b;
    }

    gcd(b, a % b)
}

fn angle(x: i32, y: i32) -> f64 {
    let x = f64::from(x);
    let y = f64::from(y);

    // Dot product used to calculate angle between positive y-axis
    // and vector
    let a = (y / (x.powi(2) + y.powi(2)).sqrt()).acos();
    if x < 0.0 {
        return a + PI;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_locations_test() {
        let input = ".#..#\n.....\n#####\n....#\n...##";
        let expected = vec![
            (1, 0),
            (4, 0),
            (0, 2),
            (1, 2),
            (2, 2),
            (3, 2),
            (4, 2),
            (4, 3),
            (3, 4),
            (4, 4),
        ];
        assert_eq!(expected, parse_locations(&input));
    }

    #[test]
    fn visible_asteroids_test() {
        let input = ".#..#\n.....\n#####\n....#\n...##";
        let locations = parse_locations(&input);
        assert_eq!(visible_asteroids(&locations, (3, 4)), 8);
    }
}
