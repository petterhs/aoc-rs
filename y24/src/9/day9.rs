use itertools::Itertools;
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone)]
struct FileBlock {
    id: usize,
    size: u32,
}

#[derive(Debug, Clone)]
enum Block {
    File(FileBlock),
    Free(u32),
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::File(FileBlock { id, size }) => {
                let mut string = String::new();
                for _ in 0..*size {
                    string.push_str(&id.to_string());
                }
                write!(f, "{}", string)
            }
            Block::Free(size) => {
                let mut string = String::new();
                for _ in 0..*size {
                    string.push_str(".");
                }
                write!(f, "{}", string)
            }
        }
    }
}

struct Filesystem {
    blocks: Vec<Block>,
}

impl Filesystem {
    fn checksum(&self) -> u64 {
        let mut checksum = 0;

        let mut idx: u64 = 0;

        for block in self.blocks.iter() {
            match block {
                Block::File(FileBlock { id, size }) => {
                    for _ in 0..*size {
                        checksum += (idx * *id as u64) as u64;
                        idx += 1;
                    }
                }
                Block::Free(size) => idx += *size as u64,
            }
        }
        checksum
    }

    fn optimize(&mut self) {
        let mut last_free_index = 0;
        for idx in (0..self.blocks.len()).rev() {
            if let Block::File(FileBlock { id, size }) = self.blocks[idx] {
                let mut i = last_free_index;
                let mut file_size = size;

                while file_size > 0 && i < idx {
                    if let Block::Free(free_size) = self.blocks[i] {
                        if free_size < file_size {
                            self.blocks[i] = Block::File(FileBlock {
                                id,
                                size: free_size,
                            });
                            self.blocks[idx] = Block::File(FileBlock {
                                id,
                                size: file_size - free_size,
                            });
                            file_size -= free_size;
                        } else {
                            self.blocks[i] = Block::File(FileBlock {
                                id,
                                size: file_size,
                            });
                            self.blocks[idx] = Block::Free(file_size);
                            self.blocks
                                .insert(i + 1, Block::Free(free_size - file_size));

                            break;
                        }
                        last_free_index = i;
                    }
                    i += 1;
                }
            }
        }
    }

    fn optimize2(&mut self) {
        for idx in (0..self.blocks.len()).rev() {
            if let Block::File(FileBlock { id, size }) = self.blocks[idx] {
                let mut i = 0;
                while i < idx {
                    if let Block::Free(free_size) = self.blocks[i] {
                        if free_size >= size {
                            self.blocks[i] = Block::File(FileBlock { id, size });

                            self.blocks[idx] = Block::Free(size);
                            self.blocks.insert(i + 1, Block::Free(free_size - size));
                            break;
                        }
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
            blocks.push(Block::File(FileBlock {
                id: i,
                size: block[0],
            }));
            if block.len() == 1 {
                break;
            }
            blocks.push(Block::Free(block[1]));
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

    filesystem.optimize();

    filesystem.checksum() as u64
}

fn part2(input: &str) -> u64 {
    let mut filesystem = input.parse::<Filesystem>().expect("parse error");

    filesystem.optimize2();

    filesystem.checksum() as u64
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
        assert_eq!(part2(input), 2858);
    }
}
