use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Couldn't read file");
    let map = parse_map(&input);

    let mut carts = parse_carts(&input);

    while carts.len() > 1 {
        carts.sort();
        let mut to_remove = Vec::new();

        for i in 0..carts.len() {
            {
                let cart = carts.get_mut(i).unwrap();
                cart.turn(&map);
                cart.advance();
            }

            let collision = check_for_collision(&carts);
            if collision.is_some() {
                let collision = collision.unwrap();
                println!(
                    "Collision between {} and {} at ({}, {})",
                    collision.0, collision.1, collision.2, collision.3
                );
                to_remove.push(collision.0);
                to_remove.push(collision.1);
            }
        }

        if to_remove.len() > 0 {
            println!("Existing carts: {:?}", carts);
            to_remove.sort_by(|a, b| b.cmp(a));
            to_remove.dedup();
            println!("Removing {:?}", to_remove);
            for &i in to_remove.iter() {
                carts.remove(i);
            }
            println!("Remaining carts: {:?}", carts);
        }
    }

    println!("{:?}", carts);
}

#[derive(Debug)]
enum Corner {
    NWSE,
    SWNE,
    INT,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Cart {
    x: i32,
    y: i32,
    d_x: i32,
    d_y: i32,
    turn: usize,
}

impl Cart {
    fn advance(&mut self) {
        self.x = self.x + self.d_x;
        self.y = self.y + self.d_y;
    }

    fn turn(&mut self, map: &Map) {
        let corner = map.get(&(self.x, self.y));
        let (d_x, d_y) = match corner {
            Some(Corner::SWNE) => (-self.d_y, -self.d_x),
            Some(Corner::NWSE) => (self.d_y, self.d_x),
            Some(Corner::INT) => {
                let (d_x, d_y) = match self.turn {
                    0 => (self.d_y, -self.d_x),
                    2 => (-self.d_y, self.d_x),
                    _ => (self.d_x, self.d_y),
                };
                self.turn = (self.turn + 1) % 3;
                (d_x, d_y)
            }
            None => (self.d_x, self.d_y),
        };

        self.d_x = d_x;
        self.d_y = d_y;
    }
}

type Map = HashMap<(i32, i32), Corner>;

type Carts = Vec<Cart>;

fn check_for_collision(carts: &Carts) -> Option<(usize, usize, i32, i32)> {
    let mut seen = HashMap::new();
    for (i, cart) in carts.iter().enumerate() {
        let existing = seen.insert((cart.x, cart.y), i);
        if existing.is_some() {
            return Some((existing.unwrap(), i, cart.x, cart.y));
        }
    }

    None
}

fn parse_map(input: &str) -> Map {
    input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            let corners: Vec<((i32, i32), Corner)> = line
                .chars()
                .enumerate()
                .filter_map(|(x, c)| match c {
                    '/' => Some(((x, y), Corner::SWNE)),
                    '\\' => Some(((x, y), Corner::NWSE)),
                    '+' => Some(((x, y), Corner::INT)),
                    _ => None,
                })
                .map(|((x, y), c)| ((x as i32, y as i32), c))
                .collect();

            corners
        })
        .flatten()
        .collect()
}

fn parse_carts(input: &str) -> Carts {
    input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            let carts: Vec<Cart> = line
                .chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    let dir = match c {
                        '^' => Some((0, -1)),
                        '>' => Some((1, 0)),
                        'v' => Some((0, 1)),
                        '<' => Some((-1, 0)),
                        _ => None,
                    };

                    dir.map(|(d_x, d_y)| Cart {
                        x: x as i32,
                        y: y as i32,
                        d_x,
                        d_y,
                        turn: 0,
                    })
                })
                .collect();

            carts
        })
        .flatten()
        .collect()
}
