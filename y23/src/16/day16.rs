use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

struct Grid {
    grid: Vec<Vec<Tile>>,
}

impl Grid {
    fn valid_position(&self, position: (i32, i32)) -> bool {
        let (x, y) = position;
        x >= 0 && y >= 0 && x < self.grid[0].len() as i32 && y < self.grid.len() as i32
    }

    fn next_position(
        &self,
        position: (usize, usize),
        direction: &Direction,
    ) -> Option<(usize, usize)> {
        let x = position.0 as i32;
        let y = position.1 as i32;
        let (x, y) = match direction {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };
        if self.valid_position((x, y)) {
            Some((x as usize, y as usize))
        } else {
            None
        }
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
    fn next_beams(&self, grid: &Grid) -> Option<Vec<Beam>> {
        let mut beams = Vec::new();

        match grid.grid[self.position.1][self.position.0] {
            Tile::MirrorRight => {
                let direction = match self.direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                };
                if let Some(pos) = grid.next_position(self.position, &direction) {
                    beams.push(Beam {
                        position: pos,
                        direction,
                    });
                }
            }
            Tile::MirrorLeft => {
                let direction = match self.direction {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                };
                if let Some(pos) = grid.next_position(self.position, &direction) {
                    beams.push(Beam {
                        position: pos,
                        direction,
                    });
                }
            }
            Tile::SplitterHorizontal => match self.direction {
                Direction::Up | Direction::Down => {
                    if let Some(pos) = grid.next_position(self.position, &Direction::Left) {
                        beams.push(Beam {
                            position: pos,
                            direction: Direction::Left,
                        });
                    }
                    if let Some(pos) = grid.next_position(self.position, &Direction::Right) {
                        beams.push(Beam {
                            position: pos,
                            direction: Direction::Right,
                        });
                    }
                }
                Direction::Left | Direction::Right => {
                    if let Some(pos) = grid.next_position(self.position, &self.direction) {
                        beams.push(Beam {
                            position: pos,
                            direction: self.direction.clone(),
                        });
                    }
                }
            },
            Tile::SplitterVertical => match self.direction {
                Direction::Left | Direction::Right => {
                    if let Some(pos) = grid.next_position(self.position, &Direction::Up) {
                        beams.push(Beam {
                            position: pos,
                            direction: Direction::Up,
                        });
                    }
                    if let Some(pos) = grid.next_position(self.position, &Direction::Down) {
                        beams.push(Beam {
                            position: pos,
                            direction: Direction::Down,
                        });
                    }
                }
                Direction::Up | Direction::Down => {
                    if let Some(pos) = grid.next_position(self.position, &self.direction) {
                        beams.push(Beam {
                            position: pos,
                            direction: self.direction.clone(),
                        });
                    }
                }
            },
            Tile::Empty => {
                if let Some(pos) = grid.next_position(self.position, &self.direction) {
                    beams.push(Beam {
                        position: pos,
                        direction: self.direction.clone(),
                    });
                }
            }
            _ => {}
        }
        Some(beams)
    }
}

fn energized_positions(grid: &Grid, beam: &Beam) -> HashSet<Beam> {
    let mut visited_beams = HashSet::new();

    let mut beams = vec![beam.clone()];

    while let Some(beam) = beams.pop() {
        if visited_beams.contains(&beam) {
            continue;
        }
        visited_beams.insert(beam.clone());

        if let Some(next_beams) = beam.next_beams(grid) {
            beams.extend(next_beams);
        }
    }

    visited_beams
}

fn part1(input: &str) -> u32 {
    let grid = Grid::from(input);

    // println!("{}", grid);

    let beam = Beam {
        position: (0, 0),
        direction: Direction::Right,
    };

    let visited_beams = energized_positions(&grid, &beam);

    //unique positions
    let unique = visited_beams
        .iter()
        .map(|b| b.position)
        .collect::<HashSet<_>>()
        .len() as u32;

    //display grid with visited_beams
    // let output_grid = vec![vec![Tile::Empty; grid.grid[0].len()]; grid.grid.len()];
    // let mut grid = Grid { grid: output_grid };
    // for beam in visited_beams {
    //     grid.grid[beam.position.1][beam.position.0] = Tile::Energized;
    // }
    // println!("{}", grid);
    unique
}

fn part2(input: &str) -> u32 {
    let grid = Grid::from(input);

    let mut starting_beams = Vec::new();

    for y in 0..grid.grid.len() {
        starting_beams.push(Beam {
            position: (0, y),
            direction: Direction::Right,
        });
        starting_beams.push(Beam {
            position: (grid.grid[0].len() - 1, y),
            direction: Direction::Left,
        });
    }

    for x in 0..grid.grid[0].len() {
        starting_beams.push(Beam {
            position: (x, 0),
            direction: Direction::Down,
        });
        starting_beams.push(Beam {
            position: (x, grid.grid.len() - 1),
            direction: Direction::Up,
        });
    }

    let mut visited_beams = HashMap::new();

    for beam in starting_beams {
        visited_beams.insert(beam.clone(), energized_positions(&grid, &beam));
    }
    //unique positions
    let max_unique = visited_beams
        .iter()
        .map(|(_, v)| v.iter().map(|b| b.position).collect::<HashSet<_>>())
        .map(|s| s.len())
        .max();

    max_unique.unwrap() as u32
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
        assert_eq!(part2(input), 51);
    }
}
