use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", solve(INPUT, false));
    println!("Part two: {}", solve(INPUT, true));
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Dir {
    N(u8),
    E(u8),
    S(u8),
    W(u8),
    START,
}

struct Grid {
    heat_values: Vec<Vec<usize>>,
    width: usize,
    height: usize,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Node {
    x: usize,
    y: usize,
    dir: Dir,
}

impl Grid {
    fn parse(s: &str) -> Self {
        let lines: Vec<&str> = s.lines().collect();
        let height = lines.len();
        let width = lines.get(0).unwrap().len();

        let heat_values = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();

        Grid {
            heat_values,
            height,
            width,
        }
    }

    fn get_heat_loss(&self, x: usize, y: usize) -> usize {
        let row = self.heat_values.get(y).unwrap();
        *row.get(x).unwrap()
    }

    fn crucible_moves(&self, node: &Node, heat_loss: &usize) -> Vec<(Node, usize)> {
        use Dir::*;

        let mut nodes: Vec<(usize, usize, Dir)> = Vec::new();

        let x = node.x;
        let y = node.y;

        match node.dir {
            START => {
                if node.y > 0 {
                    nodes.push((node.x, node.y - 1, N(1)));
                }
                if node.x < self.width - 1 {
                    nodes.push((x + 1, y, E(1)));
                }
                if node.y < self.height - 1 {
                    nodes.push((x, y + 1, S(1)));
                }
                if node.x > 0 {
                    nodes.push((x - 1, y, W(1)));
                }
            }

            N(n) => {
                if node.y > 0 && n < 3 {
                    nodes.push((x, y - 1, N(n + 1)));
                }
                if node.x < self.width - 1 {
                    nodes.push((x + 1, y, E(1)));
                }
                if node.x > 0 {
                    nodes.push((x - 1, y, W(1)));
                }
            }

            E(n) => {
                if node.y > 0 {
                    nodes.push((x, y - 1, N(1)));
                }
                if node.x < self.width - 1 && n < 3 {
                    nodes.push((x + 1, y, E(n + 1)));
                }
                if node.y < self.height - 1 {
                    nodes.push((x, y + 1, S(1)));
                }
            }

            S(n) => {
                if node.x < self.width - 1 {
                    nodes.push((x + 1, y, E(1)));
                }
                if node.y < self.height - 1 && n < 3 {
                    nodes.push((x, y + 1, S(n + 1)));
                }
                if node.x > 0 {
                    nodes.push((x - 1, y, W(1)));
                }
            }

            W(n) => {
                if node.y > 0 {
                    nodes.push((x, y - 1, N(1)));
                }
                if node.y < self.height - 1 {
                    nodes.push((x, y + 1, S(1)));
                }
                if node.x > 0 && n < 3 {
                    nodes.push((x - 1, y, W(n + 1)));
                }
            }
        }

        nodes
            .into_iter()
            .map(|(x, y, dir)| (Node { x, y, dir }, heat_loss + self.get_heat_loss(x, y)))
            .collect()
    }

    fn ultra_crucible_moves(&self, node: &Node, heat_loss: &usize) -> Vec<(Node, usize)> {
        use Dir::*;

        let mut nodes: Vec<(usize, usize, Dir)> = Vec::new();

        let start_x = node.x;
        let start_y = node.y;

        match node.dir {
            START => {
                if node.y > 3 {
                    nodes.push((node.x, node.y - 4, N(4)));
                }
                if node.x < self.width - 4 {
                    nodes.push((start_x + 4, start_y, E(4)));
                }
                if node.y < self.height - 4 {
                    nodes.push((start_x, start_y + 4, S(4)));
                }
                if node.x > 3 {
                    nodes.push((start_x - 4, start_y, W(4)));
                }
            }

            N(n) => {
                if node.y > 0 && n < 10 {
                    nodes.push((start_x, start_y - 1, N(n + 1)));
                }
                if node.x < self.width - 4 {
                    nodes.push((start_x + 4, start_y, E(4)));
                }
                if node.x > 3 {
                    nodes.push((start_x - 4, start_y, W(4)));
                }
            }

            E(n) => {
                if node.y > 3 {
                    nodes.push((start_x, start_y - 4, N(4)));
                }
                if node.x < self.width - 1 && n < 10 {
                    nodes.push((start_x + 1, start_y, E(n + 1)));
                }
                if node.y < self.height - 4 {
                    nodes.push((start_x, start_y + 4, S(4)));
                }
            }

            S(n) => {
                if node.x < self.width - 4 {
                    nodes.push((start_x + 4, start_y, E(4)));
                }
                if node.y < self.height - 1 && n < 10 {
                    nodes.push((start_x, start_y + 1, S(n + 1)));
                }
                if node.x > 3 {
                    nodes.push((start_x - 4, start_y, W(4)));
                }
            }

            W(n) => {
                if node.y > 3 {
                    nodes.push((start_x, start_y - 4, N(4)));
                }
                if node.y < self.height - 4 {
                    nodes.push((start_x, start_y + 4, S(4)));
                }
                if node.x > 0 && n < 10 {
                    nodes.push((start_x - 1, start_y, W(n + 1)));
                }
            }
        }

        nodes
            .into_iter()
            .map(|(new_x, new_y, dir)| {
                let mut node_heat_loss = *heat_loss;

                let (x0, x1) = if start_x > new_x {
                    (new_x, start_x)
                } else {
                    (start_x, new_x)
                };
                let (y0, y1) = if start_y > new_y {
                    (new_y, start_y)
                } else {
                    (start_y, new_y)
                };

                for x in x0..=x1 {
                    for y in y0..=y1 {
                        if x == start_x && y == start_y {
                            continue;
                        }

                        node_heat_loss += self.get_heat_loss(x, y);
                    }
                }

                (
                    Node {
                        x: new_x,
                        y: new_y,
                        dir,
                    },
                    node_heat_loss,
                )
            })
            .collect()
    }
}

fn solve(s: &str, ultra: bool) -> usize {
    use Dir::*;

    let grid = Grid::parse(s);

    let mut visited: HashSet<Node> = HashSet::new();
    let mut current: HashMap<Node, usize> = HashMap::new();
    current.insert(
        Node {
            x: 0,
            y: 0,
            dir: START,
        },
        0,
    );

    let mut best_heat_loss = usize::MAX;

    loop {
        let (best_node, heat_loss) = current
            .clone()
            .into_iter()
            .reduce(|(acc_node, acc_val), (node, val)| {
                if acc_val < val {
                    (acc_node, acc_val)
                } else {
                    (node, val)
                }
            })
            .unwrap();

        visited.insert(best_node.clone());

        if heat_loss > best_heat_loss {
            return best_heat_loss;
        }

        let next_nodes = if ultra {
            grid.ultra_crucible_moves(&best_node, &heat_loss)
        } else {
            grid.crucible_moves(&best_node, &heat_loss)
        };
        for (node, heat_loss) in next_nodes {
            if visited.contains(&node) {
                continue;
            }
            if current.contains_key(&node) {
                continue;
            }

            if node.x == grid.width - 1 && node.y == grid.height - 1 {
                if heat_loss < best_heat_loss {
                    best_heat_loss = heat_loss
                }
                continue;
            }

            current.insert(node, heat_loss);
        }

        current.remove(&best_node);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const EXAMPLE2: &str = include_str!("../example2.txt");

    #[test]
    fn example() {
        assert_eq!(solve(EXAMPLE, false), 102);
        assert_eq!(solve(EXAMPLE, true), 94);
        assert_eq!(solve(EXAMPLE2, true), 71);
    }
}
