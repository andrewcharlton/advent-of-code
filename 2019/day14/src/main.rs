use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let ans = part_one("input");
    println!("Part one: {}", ans);

    let ans = part_two("input", 1000000000000, ans);
    println!("Part two: {}", ans);
}

fn part_one(filename: &str) -> usize {
    let input = fs::read_to_string(filename).expect("couldn't read input");
    let recipes: HashMap<String, Recipe> = input.lines().map(parse_recipe).collect();
    let order = dependency_order(&recipes);
    ore_required(&recipes, &order, 1)
}

fn part_two(filename: &str, target: usize, ore_per_fuel: usize) -> usize {
    let mut n = target / ore_per_fuel;
    let input = fs::read_to_string(filename).expect("couldn't read input");
    let recipes: HashMap<String, Recipe> = input.lines().map(parse_recipe).collect();
    let order = dependency_order(&recipes);

    let mut ans = ore_required(&recipes, &order, n);
    while ans < target {
        n += max((target - ans) / ore_per_fuel, 1);
        ans = ore_required(&recipes, &order, n);
        println!("{} {}", n, ans);
    }

    while ans > target {
        n -= 1;
        ans = ore_required(&recipes, &order, n);
        println!("{} {}", n, ans);
    }

    n
}

fn ore_required(recipes: &HashMap<String, Recipe>, order: &Vec<String>, fuel: usize) -> usize {
    let mut amounts: HashMap<String, usize> = HashMap::new();
    amounts.insert("FUEL".to_string(), fuel);

    for s in order.iter().rev() {
        let recipe = recipes.get(s).unwrap();
        let amount = amounts.get(s).unwrap();
        for (k, n) in recipe.for_n(amount) {
            let current = *amounts.get(&k).unwrap_or(&0);
            amounts.insert(k, current + n);
        }
    }

    *amounts.get("ORE").unwrap()
}

fn dependency_order(recipes: &HashMap<String, Recipe>) -> Vec<String> {
    let mut satisfied: HashSet<String> = HashSet::new();
    satisfied.insert("ORE".to_string());
    let mut order = Vec::new();

    loop {
        let mut found = false;

        for (s, recipe) in recipes {
            if order.contains(s) {
                continue;
            }

            let deps_satisfied = recipe.requires.iter().all(|(s, _)| satisfied.contains(s));
            if deps_satisfied {
                satisfied.insert(s.clone());
                order.push(s.clone());
                found = true;
            }
        }

        if !found {
            break;
        }
    }

    order
}

#[derive(Debug)]
struct Recipe {
    produces: usize,
    requires: Vec<(String, usize)>,
}

impl Recipe {
    fn for_n(&self, n: &usize) -> Vec<(String, usize)> {
        let multiple = (n + self.produces - 1) / self.produces;

        self.requires
            .iter()
            .cloned()
            .map(|(s, n)| (s, n * multiple))
            .collect()
    }
}

fn parse_recipe(line: &str) -> (String, Recipe) {
    let mut parts = line.split("=>");

    let req = parts.next().unwrap().split(',');
    let req = req.map(parse_req).collect();

    let (output, n) = parse_req(parts.next().unwrap());

    (
        output,
        Recipe {
            produces: n,
            requires: req,
        },
    )
}

fn parse_req(input: &str) -> (String, usize) {
    let mut parts = input.trim().split(' ');
    let n: usize = parts.next().unwrap().parse().unwrap();
    let s = String::from(parts.next().unwrap());
    (s, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_examples() {
        assert_eq!(part_one("example0"), 165, "example 0");
        assert_eq!(part_one("example1"), 13312, "example 1");
        assert_eq!(part_one("example2"), 180697, "example 2");
        assert_eq!(part_one("example3"), 2210736, "example 3");
    }
}
