use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    // println!("Part one: {}", solve(INPUT, 1));
    println!("Part two: {}", solve(INPUT, 5));
}

fn solve(input: &str, multiplier: usize) -> usize {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let ans = Parser::new(line, multiplier).solve();
            println!("Line {}: {}", i + 1, ans);
            ans
        })
        .sum()
}

struct Parser {
    sections: Vec<String>,
    groupings: String,
    memoized: HashMap<(String, String), Vec<(String, usize)>>,
}

impl Parser {
    fn new(line: &str, multiplier: usize) -> Self {
        let (sections, groupings) = line.split_once(" ").unwrap();

        let sections = cycle_string(sections, '?', multiplier);
        let sections: Vec<String> = sections
            .split(".")
            .into_iter()
            .filter(|section| section.len() > 0)
            .map(|s| s.to_string())
            .collect();

        let groupings = format!(",{},", cycle_string(groupings, ',', multiplier));

        Parser {
            sections,
            groupings,
            memoized: HashMap::new(),
        }
    }

    fn solve(&mut self) -> usize {
        self.solve_recursively(
            self.sections.clone(),
            self.groupings.clone().trim_matches(',').to_string(),
        )
    }

    fn solve_recursively(&mut self, sections: Vec<String>, groupings: String) -> usize {
        if sections.len() == 0 && groupings == "" {
            // If we have no sections left, and no more groups to cover then we're good.
            return 1;
        }
        if sections.len() == 0 {
            // Otherwise if we've got no more sections left, and groupings remain, we
            // can't solve it.
            return 0;
        }

        let mut total = 0;
        for (group, n) in self.parse_section(sections.get(0).unwrap(), "") {
            if groupings.starts_with(&group) {
                let sections = &sections[1..];
                let groupings = groupings
                    .strip_prefix(&group)
                    .unwrap()
                    .trim_start_matches(",")
                    .to_string();
                total += n * self.solve_recursively(sections.to_vec(), groupings);
            }
        }
        total
    }

    fn parse_section(&mut self, section: &str, prefix: &str) -> Vec<(String, usize)> {
        if let Some(v) = self
            .memoized
            .get(&(section.to_string(), prefix.to_string()))
        {
            // If we already have this memoized, we just return the memoized version.
            return v.to_vec();
        }

        // If the prefix can't be found, nothing else we add to it will be available either.
        if !self.contains_grouping(&prefix) {
            return vec![];
        }

        // Otherwise, we find the first unknown character
        if let Some(i) = section.find('?') {
            let mut map: HashMap<String, usize> = HashMap::new();

            // We can replace the unknown with a . and cut the segment in two.
            let (a, b) = section.split_at(i);
            let b = b.strip_prefix("?").unwrap();
            if a.len() == 0 && b.len() == 0 {
                // If we have nothing either side of the "?" then this gives us nothing
                map.insert(prefix.to_string(), 1);
            } else if b.len() == 0 {
                // There's nothing following this, so we just add the len
                let subsection = if prefix.len() > 0 {
                    format!("{},{}", prefix, a.len())
                } else {
                    a.len().to_string()
                };
                map.insert(subsection, 1);
            } else {
                let next_prefix = if a.len() > 0 && prefix.len() > 0 {
                    format!("{},{}", prefix, a.len())
                } else if a.len() > 0 {
                    a.len().to_string()
                } else {
                    prefix.to_string()
                };

                for (p, n) in self.parse_section(b, &next_prefix) {
                    map.insert(p, n);
                }
            }

            // We can also replace the unknown with a # and check the sections for those.
            let a = section.replacen("?", "#", 1);
            for (p, n) in self.parse_section(&a, &prefix) {
                if let Some(v) = map.get_mut(&p) {
                    *v += n
                } else {
                    map.insert(p.to_string(), n);
                }
            }

            let mut vec: Vec<(String, usize)> = map
                .into_iter()
                .filter(|(subsection, _)| self.contains_grouping(subsection))
                .collect();
            vec.sort();

            self.memoized
                .insert((section.to_string(), prefix.to_string()), vec.clone());
            return vec.to_vec();
        }

        // If there were no unknowns then we just have a single section full of #'s, which can only
        // be achieved one way.
        let next_section = if prefix == "" {
            section.len().to_string()
        } else {
            format!("{},{}", prefix, section.len())
        };

        let vec = if self.contains_grouping(&next_section) {
            vec![(next_section, 1)]
        } else {
            vec![]
        };

        self.memoized
            .insert((section.to_string(), prefix.to_string()), vec.clone());
        vec
    }

    fn contains_grouping(&self, group: &str) -> bool {
        if group == "" {
            return true;
        }

        let group = format!(",{},", group);
        self.groupings.contains(&group)
    }
}

fn cycle_string(s: &str, join_char: char, multiplier: usize) -> String {
    let mut s = s.to_string();
    s.push(join_char);
    s.chars().cycle().take(multiplier * s.len() - 1).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_pt1() {
        assert_eq!(Parser::new("???.### 1,1,3", 1).solve(), 1);
        assert_eq!(Parser::new(".??..??...?##. 1,1,3", 1).solve(), 4);
        assert_eq!(Parser::new("?#?#?#?#?#?#?#? 1,3,1,6", 1).solve(), 1);
        assert_eq!(Parser::new("????.#...#... 4,1,1", 1).solve(), 1);
        assert_eq!(Parser::new("????.######..#####. 1,6,5", 1).solve(), 4);
        assert_eq!(Parser::new("?###???????? 3,2,1", 1).solve(), 10);
    }

    #[test]
    fn test_solve_pt2() {
        assert_eq!(Parser::new("???.### 1,1,3", 5).solve(), 1);
        assert_eq!(Parser::new(".??..??...?##. 1,1,3", 5).solve(), 16384);
        assert_eq!(Parser::new("?#?#?#?#?#?#?#? 1,3,1,6", 5).solve(), 1);
        assert_eq!(Parser::new("????.#...#... 4,1,1", 5).solve(), 16);
        assert_eq!(Parser::new("????.######..#####. 1,6,5", 5).solve(), 2500);
        assert_eq!(Parser::new("?###???????? 3,2,1", 5).solve(), 506250);
    }

    #[test]
    fn test_get_combinations() {
        // let mut parser = Parser::new("???.### 1,1,3", 1);
        //
        // assert_eq!(parser.parse_section("#", ""), vec![make_section("1", 1)]);
        // assert_eq!(parser.parse_section("##", ""), vec![]);
        // assert_eq!(parser.parse_section("###", ""), vec![make_section("3", 1)]);
        // assert_eq!(
        //     parser.parse_section("###", "1"),
        //     vec![make_section("1,3", 1)]
        // );
        // assert_eq!(
        //     parser.parse_section("###", "1,1"),
        //     vec![make_section("1,1,3", 1)]
        // );
        // assert_eq!(parser.parse_section("###", "1,1,3"), vec![]);
        //
        // assert_eq!(
        //     parser.parse_section("?", ""),
        //     vec![make_section("", 1), make_section("1", 1)]
        // );
        // assert_eq!(
        //     parser.parse_section("?", "1"),
        //     vec![make_section("1", 1), make_section("1,1", 1)]
        // );
        //
        // assert_eq!(
        //     parser.parse_section("??", ""),
        //     vec![make_section("", 1), make_section("1", 2)]
        // );
        // assert_eq!(
        //     parser.parse_section("??", "1"),
        //     vec![make_section("1", 1), make_section("1,1", 2)]
        // );
        //
        // assert_eq!(
        //     parser.parse_section("?#?", "1"),
        //     vec![make_section("1,1", 1), make_section("1,3", 1)]
        // );
        // assert_eq!(
        //     parser.parse_section("?#?", "1"),
        //     vec![make_section("1,1", 1), make_section("1,3", 1)]
        // );

        let mut parser = Parser::new(".?.?????????????#?.? 1,15", 5);

        assert_eq!(
            parser.parse_section("?????????????#?", ""),
            vec![make_section("1", 1), make_section("15", 1)],
        );

        assert_eq!(parser.solve(), 81);
    }

    fn make_section(s: &str, n: usize) -> (String, usize) {
        (s.to_string(), n)
    }
}
