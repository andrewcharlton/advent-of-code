use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("couldn't read input");
    let orbits = parse_input(&input);

    let total: usize = orbits.keys().map(|key| get_path_length(&orbits, key)).sum();
    println!("Part one: {}", total);

    let mut my_path = get_path_to_com(&orbits, "YOU");
    let mut santas_path = get_path_to_com(&orbits, "SAN");
    // Follow paths until they diverge
    loop {
        let m = my_path.remove(0);
        let s = santas_path.remove(0);
        if m != s {
            println!("Part two: {}", my_path.len() + santas_path.len());
            break;
        }
    }
}

fn parse_input(input: &str) -> HashMap<&str, &str> {
    input
        .lines()
        .map(|line| {
            let p: Vec<&str> = line.split(")").collect();
            (p[1], p[0])
        })
        .collect()
}

fn get_path_length(orbits: &HashMap<&str, &str>, object: &str) -> usize {
    if object == "COM" {
        return 0;
    }

    1 + get_path_length(orbits, orbits.get(object).expect("couldn't find path"))
}

fn get_path_to_com<'a>(orbits: &HashMap<&str, &'a str>, object: &'a str) -> Vec<&'a str> {
    if object == "COM" {
        return vec!["COM"];
    }

    let next = orbits.get(object).expect("couldn't find object");
    let mut path = get_path_to_com(orbits, next);
    path.push(object);
    path
}
