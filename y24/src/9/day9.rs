use itertools::Itertools;
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone)]
struct FileBlock {
    id: usize,
}

#[derive(Debug, Clone)]
enum Block {
    File(FileBlock),
    Free,
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::File(FileBlock { id }) => write!(f, "{:?}", id),
            Block::Free => write!(f, "."),
        }
    }
}

struct Filesystem {
    blocks: Vec<Block>,
}

impl Filesystem {
    fn checksum(&self) -> u64 {
        self.blocks
            .iter()
            .enumerate()
            .fold(0, |acc, (i, b)| match b {
                Block::File(FileBlock { id }) => acc + (i * id) as u64,
                Block::Free => acc,
            })
    }

    fn optimize(&mut self) {
        let mut last_free_index = 0;
        for idx in (0..self.blocks.len()).rev() {
            if let Block::File(FileBlock { id }) = self.blocks[idx] {
                let mut i = last_free_index;
                while i < idx {
                    if let Block::Free = self.blocks[i] {
                        self.blocks[i] = Block::File(FileBlock { id });
                        self.blocks[idx] = Block::Free;
                        last_free_index = i;
                        break;
                    }
                    i += 1;
                }
            }
        }
    }
}

impl FromStr for Filesystem {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blocks = vec![];

        for (i, block) in s
            .trim()
            .chars()
            .map(|c| c.to_digit(10).expect("parse error"))
            .collect_vec()
            .chunks(2)
            .into_iter()
            .enumerate()
        {
            if block[0] == 0 {
                panic!("parse error");
            }

            for _ in 0..block[0] {
                blocks.push(Block::File(FileBlock { id: i }));
            }
            if block.len() == 1 {
                break;
            }
            for _ in 0..block[1] {
                blocks.push(Block::Free);
            }
        }
        Ok(Filesystem { blocks })
    }
}

impl Display for Filesystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for block in self.blocks.iter() {
            write!(f, "{}", block)?;
        }
        Ok(())
    }
}

fn part1(input: &str) -> u64 {
    let mut filesystem = input.parse::<Filesystem>().expect("parse error");

    println!("1");
    filesystem.optimize();

    println!("2");
    filesystem.checksum() as u64
}

fn part2(input: &str) -> u32 {
    0
}

fn main() {
    println!("AoC 2024 - Day 9");
    let input = include_str!("input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_test_1() {
        let input = "12345";
        assert_eq!(part1(input), 60);
    }

    #[test]
    fn part1_test() {
        let input = include_str!("test");
        assert_eq!(part1(input), 1928);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("test");
        assert_eq!(part2(input), 0);
    }
}
