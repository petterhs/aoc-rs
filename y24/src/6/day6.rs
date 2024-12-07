use std::{
    collections::HashSet,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Guard(Direction),
    Obstacle,
}

impl FromStr for Tile {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Tile::Empty),
            "#" => Ok(Tile::Obstacle),
            "^" => Ok(Tile::Guard(Direction::Up)),
            "v" => Ok(Tile::Guard(Direction::Down)),
            "<" => Ok(Tile::Guard(Direction::Left)),
            ">" => Ok(Tile::Guard(Direction::Right)),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Guard {
    position: (usize, usize),
    direction: Direction,
}

impl Guard {
    fn new(x: usize, y: usize, direction: Direction) -> Self {
        Guard {
            position: (x, y),
            direction,
        }
    }

    fn step(&self) -> Self {
        let (x, y) = self.position;
        let direction = self.direction;
        let (x, y) = match direction {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };
        Guard {
            position: (x, y),
            direction,
        }
    }

    fn turn_right(&self) -> Self {
        let direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        };
        Guard {
            position: self.position,
            direction,
        }
    }
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    visited: HashSet<(usize, usize)>,
    guard: Option<Guard>,
}

impl FromStr for Map {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut guard = None;
        let tiles = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let tile = c.to_string().parse().unwrap();
                        if let Tile::Guard(dir) = tile {
                            guard = Some(Guard {
                                position: (x, y),
                                direction: dir,
                            });
                        }
                        tile
                    })
                    .collect()
            })
            .collect();
        let visited = HashSet::new();
        Ok(Map {
            tiles,
            visited,
            guard,
        })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for row in &self.tiles {
            for tile in row {
                let c = match tile {
                    Tile::Empty => '.',
                    Tile::Obstacle => '#',
                    Tile::Guard(Direction::Up) => '^',
                    Tile::Guard(Direction::Down) => 'v',
                    Tile::Guard(Direction::Left) => '<',
                    Tile::Guard(Direction::Right) => '>',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    fn tick(&mut self) -> bool {
        let y_max = self.tiles.len();
        let x_max = self.tiles[0].len();

        if let None = self.guard {
            return false;
        }
        let guard = self.guard.clone().unwrap();

        let (x, y) = guard.position;
        let direction = guard.direction;

        self.visited.insert(guard.position.clone());
        match direction {
            Direction::Up => {
                if y == 0 {
                    self.tiles[y][x] = Tile::Empty;
                    self.guard = None;
                    return false;
                }
                if self.tiles[y - 1][x] == Tile::Obstacle {
                    self.tiles[y][x] = Tile::Guard(Direction::Right);
                    self.guard = Some(guard.turn_right());
                } else {
                    self.tiles[y - 1][x] = Tile::Guard(Direction::Up);
                    self.tiles[y][x] = Tile::Empty;
                    self.guard = Some(guard.step());
                }
            }
            Direction::Down => {
                if y == y_max - 1 {
                    self.tiles[y][x] = Tile::Empty;
                    self.guard = None;
                    return false;
                }
                if self.tiles[y + 1][x] == Tile::Obstacle {
                    self.tiles[y][x] = Tile::Guard(Direction::Left);
                    self.guard = Some(guard.turn_right());
                } else {
                    self.tiles[y + 1][x] = Tile::Guard(Direction::Down);
                    self.tiles[y][x] = Tile::Empty;
                    self.guard = Some(guard.step());
                }
            }
            Direction::Left => {
                if x == 0 {
                    self.tiles[y][x] = Tile::Empty;
                    self.guard = None;
                    return false;
                }
                if self.tiles[y][x - 1] == Tile::Obstacle {
                    self.tiles[y][x] = Tile::Guard(Direction::Up);

                    self.guard = Some(guard.turn_right());
                } else {
                    self.tiles[y][x - 1] = Tile::Guard(Direction::Left);
                    self.tiles[y][x] = Tile::Empty;
                    self.guard = Some(guard.step());
                }
            }
            Direction::Right => {
                if x == x_max - 1 {
                    self.tiles[y][x] = Tile::Empty;
                    self.guard = None;
                    return false;
                }
                if self.tiles[y][x + 1] == Tile::Obstacle {
                    self.tiles[y][x] = Tile::Guard(Direction::Down);
                    self.guard = Some(guard.turn_right());
                } else {
                    self.tiles[y][x + 1] = Tile::Guard(Direction::Right);
                    self.tiles[y][x] = Tile::Empty;
                    self.guard = Some(guard.step());
                }
            }
        };
        return true;
    }
}

fn part1(input: &str) -> u32 {
    let mut map = input.parse::<Map>().unwrap();
    println!("{}", map);

    while map.tick() {}

    map.visited.len() as u32
}

fn part2(input: &str) -> u32 {
    0
}

fn main() {
    println!("AoC 2024 - Day 6");
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
        assert_eq!(part1(input), 41);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("test");
        assert_eq!(part2(input), 0);
    }
}
