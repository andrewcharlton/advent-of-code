use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("couldn't read file");
    let lines: Vec<&str> = input.lines().collect();

    let wire1 = parse_input(lines[0]);
    let wire2 = parse_input(lines[1]);

    let mut intersections: Vec<(Point, i64)> = Vec::new();
    let mut dist1 = 0;
    for link1 in &wire1 {
        let mut dist2 = 0;
        for link2 in &wire2 {
            match intersection(link1, link2) {
                Some((p, d)) => intersections.push((p, d + dist1 + dist2)),
                None => {}
            }
            dist2 += link2.length();
        }
        dist1 += link1.length();
    }

    let min_distance = intersections
        .iter()
        .map(|(p, _)| p.x.abs() + p.y.abs())
        .min()
        .unwrap();
    println!("Part one: {}", min_distance);

    let min_steps = intersections.iter().map(|(_, d)| d).min().unwrap();
    println!("Part two: {}", min_steps);
}

#[derive(Debug, PartialEq)]
enum Link {
    Horizontal { x0: i64, x1: i64, y: i64 },
    Vertical { x: i64, y0: i64, y1: i64 },
}

impl Link {
    fn length(&self) -> i64 {
        match self {
            Link::Horizontal { x0, x1, y: _ } => (x0 - x1).abs(),
            Link::Vertical { x: _, y0, y1 } => (y0 - y1).abs(),
        }
    }
}

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

fn intersection(l1: &Link, l2: &Link) -> Option<(Point, i64)> {
    match l1 {
        Link::Horizontal { x0, x1, y } => match l2 {
            Link::Vertical { x, y0, y1 } => {
                if bisects(x, x0, x1) && bisects(y, y0, y1) {
                    let dist = (x - x0).abs() + (y - y0).abs();
                    return Some((Point { x: *x, y: *y }, dist));
                }
                None
            }
            _ => None,
        },
        Link::Vertical { x, y0, y1 } => match l2 {
            Link::Horizontal { x0, x1, y } => {
                if bisects(x, x0, x1) && bisects(y, y0, y1) {
                    let dist = (x - x0).abs() + (y - y0).abs();
                    return Some((Point { x: *x, y: *y }, dist));
                }
                None
            }
            _ => None,
        },
    }
}

fn bisects(p: &i64, p0: &i64, p1: &i64) -> bool {
    (p0 <= p && p <= p1) || (p1 <= p && p <= p0)
}

fn parse_input(input: &str) -> Vec<Link> {
    let mut links = Vec::new();
    let mut loc = Point { x: 0, y: 0 };

    let cmds = input.split(",");
    for cmd in cmds {
        let (dir, n) = cmd.split_at(1);
        let n = n.parse::<i64>().unwrap();
        loc = match dir {
            "R" => {
                links.push(Link::Horizontal {
                    x0: loc.x,
                    x1: loc.x + n,
                    y: loc.y,
                });
                Point {
                    x: loc.x + n,
                    y: loc.y,
                }
            }

            "L" => {
                links.push(Link::Horizontal {
                    x0: loc.x,
                    x1: loc.x - n,
                    y: loc.y,
                });
                Point {
                    x: loc.x - n,
                    y: loc.y,
                }
            }

            "U" => {
                links.push(Link::Vertical {
                    x: loc.x,
                    y0: loc.y,
                    y1: loc.y + n,
                });
                Point {
                    x: loc.x,
                    y: loc.y + n,
                }
            }

            "D" => {
                links.push(Link::Vertical {
                    x: loc.x,
                    y0: loc.y,
                    y1: loc.y - n,
                });
                Point {
                    x: loc.x,
                    y: loc.y - n,
                }
            }

            dir => panic!("unrecognised op: {}", dir),
        }
    }

    links
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_parsing_test() {
        assert_eq!(
            parse_input("R5,D5,L5,U5"),
            vec!(
                Link::Horizontal { x0: 0, x1: 5, y: 0 },
                Link::Vertical {
                    x: 5,
                    y0: 0,
                    y1: -5
                },
                Link::Horizontal {
                    x0: 5,
                    x1: 0,
                    y: -5
                },
                Link::Vertical {
                    x: 0,
                    y0: -5,
                    y1: 0
                },
            )
        );
    }
}
