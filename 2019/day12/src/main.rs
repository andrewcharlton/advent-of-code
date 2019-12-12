use std::cmp::max;
use std::collections::HashMap;

fn main() {
    let x = Dimension::new(-8, -11, 8, -2);
    let y = Dimension::new(-18, -14, -3, -16);
    let z = Dimension::new(6, 4, -10, 1);

    let ans = energy_after(&x, &y, &z, 1000);
    println!("Part one: {}", ans);

    let ans = first_repeat(&x, &y, &z);
    println!("Part two: {}", ans);
}

fn energy_after(x: &Dimension, y: &Dimension, z: &Dimension, steps: usize) -> i64 {
    let mut x = x.clone();
    let mut y = y.clone();
    let mut z = z.clone();

    for _ in 0..steps {
        x = x.next();
        y = y.next();
        z = z.next();
    }

    total_energy(&x, &y, &z)
}

fn first_repeat(x: &Dimension, y: &Dimension, z: &Dimension) -> usize {
    let (a_x, d_x) = find_loop(x);
    let (a_y, d_y) = find_loop(y);
    let (a_z, d_z) = find_loop(z);

    max(max(a_x, a_y), a_z) + lcm(lcm(d_x, d_y), d_z)
}

fn find_loop(d: &Dimension) -> (usize, usize) {
    let mut n = 0;
    let mut previous: HashMap<Dimension, usize> = HashMap::new();

    let mut d = d.clone();

    loop {
        if previous.contains_key(&d) {
            let p = previous.get(&d).unwrap();
            return (*p, n - p);
        }

        let next = d.next();
        previous.insert(d, n);

        d = next;
        n += 1;
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Dimension {
    pos: [i64; 4],
    v: [i64; 4],
}

impl Dimension {
    fn new(a: i64, b: i64, c: i64, d: i64) -> Dimension {
        Dimension {
            pos: [a, b, c, d],
            v: [0, 0, 0, 0],
        }
    }

    fn next(&self) -> Dimension {
        let mut v = self.v.clone();
        for i in 0..4 {
            for j in 0..4 {
                if i == j {
                    continue;
                }
                if self.pos[i] < self.pos[j] {
                    v[i] += 1;
                }
                if self.pos[i] > self.pos[j] {
                    v[i] -= 1;
                }
            }
        }

        let pos = [
            self.pos[0] + v[0],
            self.pos[1] + v[1],
            self.pos[2] + v[2],
            self.pos[3] + v[3],
        ];
        Dimension { pos, v }
    }
}

fn total_energy(x: &Dimension, y: &Dimension, z: &Dimension) -> i64 {
    let pe: Vec<i64> = (0..4)
        .map(|m| x.pos[m].abs() + y.pos[m].abs() + z.pos[m].abs())
        .collect();

    let ke: Vec<i64> = (0..4)
        .map(|m| x.v[m].abs() + y.v[m].abs() + z.v[m].abs())
        .collect();

    pe.iter().zip(ke.iter()).map(|(ke, pe)| ke * pe).sum()
}

fn hcf(a: usize, b: usize) -> usize {
    if a % b == 0 {
        return b;
    }

    hcf(b, a % b)
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / hcf(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn energy_after_test() {
        let x = Dimension::new(-1, 2, 4, 3);
        let y = Dimension::new(0, -10, -8, 5);
        let z = Dimension::new(2, -7, 8, -1);
        assert_eq!(energy_after(&x, &y, &z, 10), 179);
    }

    #[test]
    fn first_repeat_example1() {
        let x = Dimension::new(-1, 2, 4, 3);
        let y = Dimension::new(0, -10, -8, 5);
        let z = Dimension::new(2, -7, 8, -1);
        assert_eq!(first_repeat(&x, &y, &z), 2772);
    }

    #[test]
    fn first_repeat_example2() {
        let x = Dimension::new(-8, 5, 2, 9);
        let y = Dimension::new(-10, 5, -7, -8);
        let z = Dimension::new(0, 10, 3, -3);
        assert_eq!(first_repeat(&x, &y, &z), 4686774924);
    }
}
