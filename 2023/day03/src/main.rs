const INPUT: &str = include_str!("../input.txt");

fn main() {
    let (parts, symbols) = parse_input(INPUT);
    println!("Part one: {}", part_number_sum(&parts, &symbols));
    println!("Part two: {}", gear_sum(&parts, &symbols));
}

fn part_number_sum(parts: &Vec<Vec<Part>>, symbols: &Vec<Vec<Symbol>>) -> usize {
    parts
        .iter()
        .enumerate()
        .flat_map(|(y, line_parts)| {
            line_parts.iter().filter_map(move |p| {
                if y > 0 {
                    if symbols
                        .get(y - 1)
                        .unwrap()
                        .iter()
                        .any(|s| s.x + 1 >= p.x0 && s.x <= p.x1 + 1)
                    {
                        return Some(p.num);
                    }
                }

                if symbols
                    .get(y)
                    .unwrap()
                    .iter()
                    .any(|s| (p.x0 > 0 && s.x == p.x0 - 1) || s.x == p.x1 + 1)
                {
                    return Some(p.num);
                }

                if let Some(next_row_symbols) = symbols.get(y + 1) {
                    if next_row_symbols
                        .iter()
                        .any(|s| s.x + 1 >= p.x0 && s.x <= p.x1 + 1)
                    {
                        return Some(p.num);
                    }
                }

                None
            })
        })
        .sum()
}

fn gear_sum(parts: &Vec<Vec<Part>>, symbols: &Vec<Vec<Symbol>>) -> usize {
    symbols
        .iter()
        .enumerate()
        .flat_map(|(y, line_symbols)| {
            line_symbols
                .iter()
                .filter(|s| s.c == '*')
                .filter_map(move |s| {
                    let mut adjacent: Vec<usize> = Vec::new();
                    let x0 = if s.x > 0 { s.x - 1 } else { 0 };

                    // Row before
                    if y > 0 {
                        for p in parts.get(y - 1).unwrap() {
                            if p.x0 <= s.x + 1 && p.x1 >= x0 {
                                adjacent.push(p.num);
                            }
                        }
                    }

                    // Left and right
                    for p in parts.get(y).unwrap() {
                        if p.x1 == x0 || p.x0 == s.x + 1 {
                            adjacent.push(p.num);
                        }
                    }

                    // Row below
                    if let Some(next_row_parts) = parts.get(y + 1) {
                        for p in next_row_parts {
                            if p.x0 <= s.x + 1 && p.x1 >= x0 {
                                adjacent.push(p.num);
                            }
                        }
                    }

                    if adjacent.len() == 2 {
                        return Some(adjacent.iter().product::<usize>());
                    }
                    None
                })
        })
        .sum()
}

struct Part {
    num: usize,
    x0: usize,
    x1: usize,
}

struct Symbol {
    c: char,
    x: usize,
}

fn parse_input(input: &str) -> (Vec<Vec<Part>>, Vec<Vec<Symbol>>) {
    let mut all_parts: Vec<Vec<Part>> = Vec::new();
    let mut all_symbols: Vec<Vec<Symbol>> = Vec::new();

    for line in input.lines() {
        let mut parts: Vec<Part> = Vec::new();
        let mut symbols: Vec<Symbol> = Vec::new();
        let mut num = 0;
        let mut start = 0;

        for (x, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                if num == 0 {
                    start = x;
                }
                num = 10 * num + c.to_digit(10).unwrap();
                continue;
            }

            if num > 0 {
                parts.push(Part {
                    num: num as usize,
                    x0: start,
                    x1: x - 1,
                });
                num = 0;
            }

            if c == '.' {
                continue;
            }

            symbols.push(Symbol { x, c });
        }

        if num > 0 {
            parts.push(Part {
                num: num as usize,
                x0: start,
                x1: line.len() - 1,
            });
        }

        all_parts.push(parts);
        all_symbols.push(symbols);
    }

    (all_parts, all_symbols)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let (parts, symbols) = parse_input(EXAMPLE);
        assert_eq!(part_number_sum(&parts, &symbols), 4361);
        assert_eq!(gear_sum(&parts, &symbols), 467835);
    }
}
