use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let now = Instant::now();

    println!("Part one: {}", solve(INPUT, true));
    println!("Part two: {}", solve(INPUT, false));

    let elapsed_time = now.elapsed();
    println!("Solved in {}µs", elapsed_time.as_micros());
}

fn solve(input: &str, single_blocks: bool) -> usize {
    let mut disk = Disk::new(input, single_blocks);
    disk.solve()
}

struct Disk {
    blocks: Vec<Block>,
    gaps: Vec<Gap>,
}

impl Disk {
    fn new(input: &str, single_blocks: bool) -> Self {
        let mut id = 0;
        let mut pos = 0;
        let mut blocks: Vec<Block> = Vec::new();
        let mut gaps: Vec<Gap> = Vec::new();

        for (i, c) in input.trim().chars().enumerate() {
            let n: usize = c.to_digit(10).unwrap().try_into().unwrap();
            if n == 0 {
                continue;
            }
            if i % 2 == 0 {
                if single_blocks {
                    for p in pos..pos+n {
                        blocks.push(Block{pos: p, id, len: 1});
                    }
                } else {
                    blocks.push(Block{pos, id, len: n});
                }

                id += 1;
            } else {
                gaps.push(Gap{pos, len: n});
            }

            pos += n;
        }

        Disk{blocks, gaps}
    }

    fn solve(&mut self) -> usize {
        for i in (0..self.blocks.len()).rev() {
            let block = self.blocks.get_mut(i).unwrap();
            for gap in self.gaps.iter_mut() {
                if gap.pos > block.pos {
                    continue;
                }

                if gap.len >= block.len {
                    block.pos = gap.pos;
                    gap.pos += block.len;
                    gap.len -= block.len;

                    break;
                }
            }
        }

        self.blocks.iter().map(|block| block.checksum()).sum()
    }
}

#[derive(Debug,Clone)]
struct Block {
    pos: usize,
    id: usize,
    len: usize,
}

impl Block {
    fn checksum(&self) -> usize {
        (self.pos..self.pos+self.len).map(|i| i*self.id).sum()
    }
}

#[derive(Debug)]
struct Gap {
    pos: usize,
    len: usize,
}





#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        assert_eq!(solve(EXAMPLE, true), 1928);
        assert_eq!(solve(EXAMPLE, false), 2858);
    }
}
