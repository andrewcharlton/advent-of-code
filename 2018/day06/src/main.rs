use std::collections::HashMap;
use std::fs;

const MAX_X: usize = 320;
const MAX_Y: usize = 320;

fn main() {
    let input = fs::read_to_string("input").expect("Couldn't read file");

    let mut grid: [[(usize, bool); 320]; 320] = [[(0, false); MAX_X]; MAX_Y];
    let point = parse_input(&input);

    // println!("Used to calculate grid size");
    // println!("{} Starting points", input.len());
    // println!("Min x: {:?}", input.iter().min_by_key(|x| x.1).unwrap().1);
    // println!("Max x: {:?}", input.iter().max_by_key(|x| x.1).unwrap().1);
    // println!("Min y: {:?}", input.iter().min_by_key(|x| x.2).unwrap().2);
    // println!("Max y: {:?}", input.iter().max_by_key(|x| x.2).unwrap().2);
}

type PointMap = HashMap<(usize, usize), usize>;

fn parse_input(input: &str) -> HashMap<(usize, usize), usize> {
    let points: Vec<(usize, usize)> = input
        .lines()
        .map(|s| {
            let coords: Vec<usize> = s.split(", ").map(|c| c.parse().unwrap()).collect();
            (coords[0], coords[1])
        })
        .collect();

    let min_x = points.iter().min_by_key(|x| x.0).unwrap().0;
    let min_y = points.iter().min_by_key(|x| x.1).unwrap().1;

    let mut map = HashMap::new();
    for (i, (x, y)) in points.iter().enumerate() {
        map.insert((x - min_x + 1, y - min_y + 1), i + 1);
    }

    map
}

fn reduce_grid(start_points: PointMap) -> usize {
    let (max_x, max_y) = grid_size(&start_points);

    let mut grid = [[(usize, bool); max_x]; max_y];
    let mut next = start_points;
    let mut changed = true;

    while changed == true {}
}

fn grid_size(points: &PointMap) -> (usize, usize) {
    let max_x = points.keys().max_by_key(|p| p.0).unwrap().0;
    let max_y = points.keys().max_by_key(|p| p.1).unwrap().1;
    (max_x, max_y)
}

fn neighbours(x: usize, y: usize) -> Vec<(usize, usize)> {
    vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
}
