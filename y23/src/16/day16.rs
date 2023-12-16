use std::{collections::HashSet, fmt::Display};

struct Grid {
    grid: Vec<Vec<Tile>>,
}

impl Grid {
    fn valid_position(&self, position: (i32, i32)) -> bool {
        let (x, y) = position;
        x >= 0 && y >= 0 && x < self.grid[0].len() as i32 && y < self.grid.len() as i32
    }
}

#[derive(Debug, Clone)]
enum Tile {
    Empty,
    MirrorRight,
    MirrorLeft,
    SplitterHorizontal,
    SplitterVertical,
    Energized,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '/' => Tile::MirrorRight,
            '\\' => Tile::MirrorLeft,
            '-' => Tile::SplitterHorizontal,
            '|' => Tile::SplitterVertical,
            _ => panic!("Unknown tile: {}", c),
        }
    }
}

impl From<&str> for Grid {
    fn from(s: &str) -> Self {
        let grid = s
            .lines()
            .map(|line| line.chars().map(|c| Tile::from(c)).collect())
            .collect();
        Grid { grid }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.grid {
            for tile in line {
                match tile {
                    Tile::Empty => write!(f, ".")?,
                    Tile::MirrorRight => write!(f, "/")?,
                    Tile::MirrorLeft => write!(f, "\\")?,
                    Tile::SplitterHorizontal => write!(f, "-")?,
                    Tile::SplitterVertical => write!(f, "|")?,
                    Tile::Energized => write!(f, "#")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Beam {
    position: (usize, usize),
    direction: Direction,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Beam {
    fn next_position(&self, grid: &Grid) -> Option<(usize, usize)> {
        let x = self.position.0 as i32;
        let y = self.position.1 as i32;
        let (x, y) = match self.direction {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };
        if grid.valid_position((x, y)) {
            Some((x as usize, y as usize))
        } else {
            None
        }
    }
}

fn part1(input: &str) -> u32 {
    let grid = Grid::from(input);

    println!("{}", grid);

    let mut beams = vec![Beam {
        position: (0, 0),
        direction: match grid.grid[0][0] {
            Tile::MirrorLeft | Tile::SplitterVertical => Direction::Down,
            _ => Direction::Right,
        },
    }];

    let mut visited_beams = HashSet::new();

    while let Some(beam) = beams.pop() {
        if visited_beams.contains(&beam) {
            continue;
        }
        visited_beams.insert(beam.clone());
        match beam.next_position(&grid) {
            Some(pos) => match grid.grid[pos.1][pos.0] {
                Tile::Empty => {
                    beams.push(Beam {
                        position: pos,
                        direction: beam.direction,
                    });
                }
                Tile::MirrorRight => {
                    beams.push(Beam {
                        position: pos,
                        direction: match beam.direction {
                            Direction::Up => Direction::Right,
                            Direction::Down => Direction::Left,
                            Direction::Left => Direction::Down,
                            Direction::Right => Direction::Up,
                        },
                    });
                }
                Tile::MirrorLeft => {
                    beams.push(Beam {
                        position: pos,
                        direction: match beam.direction {
                            Direction::Up => Direction::Left,
                            Direction::Down => Direction::Right,
                            Direction::Left => Direction::Up,
                            Direction::Right => Direction::Down,
                        },
                    });
                }
                Tile::SplitterHorizontal => match beam.direction {
                    Direction::Up | Direction::Down => {
                        beams.push(Beam {
                            position: pos,
                            direction: Direction::Left,
                        });
                        beams.push(Beam {
                            position: pos,
                            direction: Direction::Right,
                        });
                    }
                    Direction::Left | Direction::Right => {
                        beams.push(Beam {
                            position: pos,
                            direction: beam.direction,
                        });
                    }
                },
                Tile::SplitterVertical => match beam.direction {
                    Direction::Left | Direction::Right => {
                        beams.push(Beam {
                            position: pos,
                            direction: Direction::Up,
                        });
                        beams.push(Beam {
                            position: pos,
                            direction: Direction::Down,
                        });
                    }
                    Direction::Up | Direction::Down => {
                        beams.push(Beam {
                            position: pos,
                            direction: beam.direction,
                        });
                    }
                },
                _ => {}
            },
            None => {}
        }
    }

    //unique positions
    let unique = visited_beams
        .iter()
        .map(|b| b.position)
        .collect::<HashSet<_>>()
        .len() as u32;

    //display grid with visited_beams
    let output_grid = vec![vec![Tile::Empty; grid.grid[0].len()]; grid.grid.len()];
    let mut grid = Grid { grid: output_grid };
    for beam in visited_beams {
        grid.grid[beam.position.1][beam.position.0] = Tile::Energized;
    }
    println!("{}", grid);
    unique
}

fn part2(input: &str) -> u32 {
    0
}

fn main() {
    println!("AoC 2023 - Day 1");
    let input = include_str!("input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("test");
        assert_eq!(part1(input), 46);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("test");
        assert_eq!(part2(input), 0);
    }
}
