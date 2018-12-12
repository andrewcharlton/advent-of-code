use std::collections::{HashMap, HashSet};
use std::fs;

type Point = (usize, usize);
type PointMap = HashMap<Point, usize>;
type Grid = Vec<Vec<usize>>;

fn main() {
    let input = fs::read_to_string("input").expect("Couldn't read file");

    let points = parse_input(&input);
    let grid = populate_grid(&points);

    let counts = get_counts(&grid);
    let max_area = counts.iter().map(|(_, v)| v).max().unwrap();

    println!("Part one: {}", max_area);

    let points: HashSet<Point> = parse_input(&input).keys().map(|&k| k).collect();
    println!("Part two: {}", safe_points(points));

    // let mut grid: [[(usize, bool); 320]; 320] = [[(0, false); MAX_X]; MAX_Y];
    // let point = parse_input(&input);

    // println!("Used to calculate grid size");
    // println!("{} Starting points", input.len());
    // println!("Min x: {:?}", input.iter().min_by_key(|x| x.1).unwrap().1);
    // println!("Max x: {:?}", input.iter().max_by_key(|x| x.1).unwrap().1);
    // println!("Min y: {:?}", input.iter().min_by_key(|x| x.2).unwrap().2);
    // println!("Max y: {:?}", input.iter().max_by_key(|x| x.2).unwrap().2);
}

fn distance(p: Point, q: Point) -> usize {
    let mut d = 0;
    d += if p.0 > q.0 { p.0 - q.0 } else { q.0 - p.0 };
    d += if p.1 > q.1 { p.1 - q.1 } else { q.1 - p.1 };
    d
}

fn populate_grid(points: &PointMap) -> Grid {
    let max_x = points.keys().map(|p| p.0).max().unwrap();
    let max_y = points.keys().map(|p| p.1).max().unwrap();
    let mut grid: Grid = vec![vec![0; max_y + 2]; max_x + 2];

    for x in 0..max_x + 2 {
        for y in 0..max_y + 2 {
            let mut set = false;
            let mut min_d = 0;
            let mut closest = 0;
            for (&p, &k) in points.iter() {
                let d = distance(p, (x, y));
                if !set || d < min_d {
                    set = true;
                    min_d = d;
                    closest = k;
                    continue;
                }

                if d == min_d {
                    closest = 0;
                }
            }

            grid[x][y] = closest
        }
    }

    grid
}

fn safe_points(points: HashSet<Point>) -> usize {
    let max_x = points.iter().map(|p| p.0).max().unwrap();
    let max_y = points.iter().map(|p| p.1).max().unwrap();

    let mut count = 0;

    for x in 0..max_x {
        for y in 0..max_y {
            let mut sum = 0;
            for p in points.iter() {
                sum += if p.0 > x { p.0 - x } else { x - p.0 };
                sum += if p.1 > y { p.1 - y } else { y - p.1 };
            }
            if sum < 10000 {
                count += 1;
            }
        }
    }

    count
}

fn get_counts(grid: &Grid) -> HashMap<usize, usize> {
    let edges = get_edge_pieces(grid);

    let mut counts: HashMap<usize, usize> = HashMap::new();
    for row in grid.iter() {
        for &v in row.iter() {
            if !edges.contains(&v) {
                counts.insert(v, counts.get(&v).unwrap_or(&0) + 1);
            }
        }
    }
    counts
}

fn get_edge_pieces(grid: &Grid) -> HashSet<usize> {
    let mut edges: HashSet<usize> = HashSet::new();

    for &k in grid[0].iter() {
        edges.insert(k);
    }

    for row in grid.iter() {
        let mut row = row.iter();
        edges.insert(*row.next().unwrap());
        edges.insert(*row.last().unwrap());
    }

    for &k in grid[grid.len() - 1].iter() {
        edges.insert(k);
    }

    edges
}

fn parse_input(input: &str) -> PointMap {
    let points: Vec<Point> = input
        .lines()
        .map(|s| {
            let coords: Vec<usize> = s.split(", ").map(|c| c.parse().unwrap()).collect();
            (coords[0], coords[1])
        })
        .collect();

    let min_x = points.iter().map(|p| p.0).min().unwrap();
    let min_y = points.iter().map(|p| p.1).min().unwrap();

    points
        .into_iter()
        .enumerate()
        .map(|(i, p)| ((p.0 - min_x, p.1 - min_y), i + 1))
        .collect()
}
