const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part one: {}", solve_pt1(INPUT));
    println!("Part two: {}", solve_pt2(INPUT));
}

fn solve_pt1(input: &str) -> i64 {
    let (seeds, maps) = parse_input(input);
    seeds.iter().map(|s| maps.transform(*s)).min().unwrap()
}

fn solve_pt2(input: &str) -> i64 {
    let (seed_ranges, maps) = parse_input(input);
    let seed_n = seed_ranges.len() / 2;

    let mut smallest = i64::MAX;

    for map in &maps.0 {
        if map.start + map.transform > smallest {
            // Even with the very smallest input, we can't beat the smallest value using this
            // window.
            continue;
        }

        // Find the smallest seed number that is > map.start
        for i in 0..seed_n {
            let start = seed_ranges.get(2 * i).unwrap();
            let size = seed_ranges.get(2 * i + 1).unwrap();

            if start > &map.end {
                // This doesn't overlap at all.
                continue;
            }

            if start + size < map.start {
                // No overlap at all
                continue;
            }

            if start < &map.start {
                // This is the smallest value we can find for this window, because it starts before
                // the window, and extends into it.
                smallest = map.start + map.transform;
                break;
            }

            // Otherwise the seed range is contained within the window, and the smallest value is
            // the start of that range.
            let best = start + map.transform;
            if smallest > best {
                smallest = best;
            }
        }
    }

    smallest
}

fn parse_input(s: &str) -> (Vec<i64>, Maps) {
    let mut chunks = s.split("\n\n");

    let seeds: Vec<i64> = chunks
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let maps: Maps = chunks
        .map(|chunk| Maps::new(chunk))
        .reduce(|acc, map| acc.merge(&map))
        .unwrap();

    (seeds, maps)
}

#[derive(Debug)]
struct Map {
    start: i64,
    end: i64,
    transform: i64,
}

impl Map {
    // merge creates a set of Maps completely covering the span of the initial map, but
    // with overlays from the next set of maps applied to it to (potentially) subdivide
    // it up.
    fn merge(&self, others: &Maps) -> Maps {
        let mut start = self.start + self.transform;
        let end = self.end + self.transform;
        let mut maps: Vec<Map> = Vec::new();
        for map in &others.0 {
            if map.end <= start {
                // Haven't reached the window yet, keep going.
                continue;
            }

            if map.start > end {
                // If the next overlap is beyond this window then just add up to the end and
                // we're done.
                maps.push(Map {
                    start: start - self.transform,
                    end: end - self.transform,
                    transform: self.transform,
                });
                start = end;
                break;
            }

            if map.start <= start && map.end >= end {
                // The remainder of the window is wholly consumed by an overlap
                maps.push(Map {
                    start: start - self.transform,
                    end: end - self.transform,
                    transform: self.transform + map.transform,
                });
                start = end;
                break;
            }

            if map.start > start {
                // We have a window until the overlap starts, so add the initial section
                maps.push(Map {
                    start: start - self.transform,
                    end: map.start - self.transform,
                    transform: self.transform,
                });
                start = map.start;
            }

            if map.end >= end {
                // If the overlay extends beyond to the end, we're done.
                maps.push(Map {
                    start: start - self.transform,
                    end: end - self.transform,
                    transform: self.transform + map.transform,
                });
                start = end;
                break;
            }

            // Otherwise, we add the overlay up until it's end, and then advance our start.
            maps.push(Map {
                start: start - self.transform,
                end: map.end - self.transform,
                transform: self.transform + map.transform,
            });
            start = map.end;
        }

        // If we ran out of overlaps, we add to the end
        if start != end {
            maps.push(Map {
                start: start - self.transform,
                end: end - self.transform,
                transform: self.transform,
            });
        }

        Maps(maps)
    }
}

#[derive(Debug)]
struct Maps(Vec<Map>);

impl Maps {
    fn new(s: &str) -> Maps {
        let maps: Vec<Map> = s
            .lines()
            .skip(1)
            .map(|line| {
                let mut parts = line.trim().split_whitespace();
                let dst: i64 = parts.next().unwrap().parse().unwrap();
                let src: i64 = parts.next().unwrap().parse().unwrap();
                let len: i64 = parts.next().unwrap().parse().unwrap();
                Map {
                    start: src,
                    end: src + len,
                    transform: dst - src,
                }
            })
            .collect();

        let mut maps = Maps(maps);
        maps.sort();

        // We try and populate the map with all missing windows.
        let mut start = 0;
        let mut additional_maps: Vec<Map> = Vec::new();
        for map in &maps.0 {
            if map.start > start {
                // There's a gap until the next existing window.
                additional_maps.push(Map {
                    start,
                    end: map.start,
                    transform: 0,
                });
                start = map.end;
                continue;
            }

            // If there's no gap, then we need to progress until the end of this window
            // and see if one opens up.
            start = map.end;
        }

        additional_maps.push(Map {
            start,
            end: i64::MAX,
            transform: 0,
        });

        maps.0.extend(additional_maps);
        maps.sort();
        maps
    }

    fn sort(&mut self) {
        self.0.sort_by(|a, b| a.start.cmp(&b.start));
    }

    fn transform(&self, n: i64) -> i64 {
        for map in &self.0 {
            if map.start <= n && map.end >= n {
                return n + map.transform;
            }
        }

        n
    }

    fn merge(&self, other: &Maps) -> Maps {
        let mut maps: Vec<Map> = Vec::new();
        for map in &self.0 {
            maps.extend(map.merge(other).0);
        }

        let mut maps = Maps(maps);
        maps.sort();
        maps
    }
}
