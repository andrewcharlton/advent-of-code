fn main() {
    let moons = vec![
        Moon::new(-8, -18, 6),
        Moon::new(-11, -14, 4),
        Moon::new(8, -3, -10),
        Moon::new(-2, -16, 1),
    ];

    let ans = energy_after(&moons, 1000);
    println!("Part one: {}", ans);
}

fn energy_after(moons: &Vec<Moon>, steps: usize) -> i64 {
    let mut moons = moons.clone();

    for _ in 0..steps {
        let positions = moons.clone();
        moons
            .iter_mut()
            .for_each(|moon| moon.update_velocity(&positions));
        moons.iter_mut().for_each(|moon| moon.update_position());
    }

    moons.iter().map(|moon| moon.energy()).sum()
}

#[derive(Clone, Debug)]
struct Moon {
    x: i64,
    y: i64,
    z: i64,
    v_x: i64,
    v_y: i64,
    v_z: i64,
}

impl Moon {
    fn new(x: i64, y: i64, z: i64) -> Moon {
        Moon {
            x,
            y,
            z,
            v_x: 0,
            v_y: 0,
            v_z: 0,
        }
    }

    fn update_velocity(&mut self, moons: &Vec<Moon>) {
        for moon in moons {
            if moon.x > self.x {
                self.v_x += 1;
            }
            if moon.x < self.x {
                self.v_x -= 1;
            }

            if moon.y > self.y {
                self.v_y += 1;
            }
            if moon.y < self.y {
                self.v_y -= 1;
            }

            if moon.z > self.z {
                self.v_z += 1;
            }
            if moon.z < self.z {
                self.v_z -= 1;
            }
        }
    }

    fn update_position(&mut self) {
        self.x += self.v_x;
        self.y += self.v_y;
        self.z += self.v_z;
    }

    fn energy(&self) -> i64 {
        (self.x.abs() + self.y.abs() + self.z.abs())
            * (self.v_x.abs() + self.v_y.abs() + self.v_z.abs())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn energy_after_test() {
        let mut moons = vec![
            Moon::new(-1, 0, 2),
            Moon::new(2, -10, -7),
            Moon::new(4, -8, 8),
            Moon::new(3, 5, -1),
        ];
        assert_eq!(energy_after(&mut moons, 10), 179);
    }
}
