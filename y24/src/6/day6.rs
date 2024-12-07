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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    fn step(&self) -> Option<Self> {
        let (x, y) = self.position;
        let direction = self.direction;

        if direction == Direction::Up && y == 0 {
            return None;
        }

        if direction == Direction::Down && y == usize::MAX {
            return None;
        }

        let (x, y) = match direction {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };
        Some(Guard {
            position: (x, y),
            direction,
        })
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

#[derive(Debug, Clone)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    visited: HashSet<(usize, usize)>,
    path: HashSet<Guard>,
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
                            guard = Some(Guard::new(x, y, dir));
                        }
                        tile
                    })
                    .collect()
            })
            .collect();
        let visited = HashSet::new();
        let path = HashSet::new();
        Ok(Map {
            tiles,
            visited,
            path,
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
        self.path.insert(guard.clone());
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
                    self.guard = guard.step();
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
                    self.guard = guard.step();
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
                    self.guard = guard.step();
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
                    self.guard = guard.step();
                }
            }
        };
        return true;
    }
}

fn part1(input: &str) -> u32 {
    let mut map = input.parse::<Map>().unwrap();

    while map.tick() {}

    map.visited.len() as u32
}

fn part2(input: &str) -> u32 {
    let mut map = input.parse::<Map>().unwrap();

    let y_max = map.tiles.len();
    let x_max = map.tiles[0].len();

    let start_guard = map.guard.unwrap().position;

    let mut candidates = HashSet::new();

    while map.tick() {
        let mut new_map = map.clone();

        let guard_position = new_map.guard.clone().unwrap();

        let new_obsticle_pos = guard_position.step();

        if new_obsticle_pos.is_none() {
            break;
        }

        let new_obsticle_pos = new_obsticle_pos.unwrap().position;

        if new_obsticle_pos.0 >= x_max || new_obsticle_pos.1 >= y_max {
            break;
        }
        if map.tiles[new_obsticle_pos.1][new_obsticle_pos.0] == Tile::Obstacle {
            continue;
        }
        if new_obsticle_pos == start_guard {
            continue;
        }
        if map.visited.contains(&new_obsticle_pos) {
            continue;
        }

        //Try to place obstacle in front of guard and turn the guard Right
        //Check if the guard then reaches a path (both position and direction) it has already been on.
        //If so the guard is in a loop.
        new_map.tiles[new_obsticle_pos.1][new_obsticle_pos.0] = Tile::Obstacle;
        let guard = new_map.guard.unwrap().turn_right();
        new_map.guard = Some(guard);

        while new_map.tick() {
            if new_map.path.contains(&new_map.guard.clone().unwrap()) {
                candidates.insert(new_obsticle_pos);
                break;
            }
        }
    }

    candidates.len() as u32
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
        assert_eq!(part2(input), 6);
    }
}
