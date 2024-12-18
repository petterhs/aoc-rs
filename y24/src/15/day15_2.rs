use std::collections::VecDeque;
use std::{fmt::Display, str::FromStr};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "^"),
            Direction::Down => write!(f, "v"),
            Direction::Left => write!(f, "<"),
            Direction::Right => write!(f, ">"),
        }
    }
}

#[derive(Clone, PartialEq)]
enum Tile {
    Empty,
    Wall,
    BoxLeft,
    BoxRight,
    Robot,
}

impl FromStr for Tile {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Tile::Empty),
            "#" => Ok(Tile::Wall),
            "O" => Ok(Tile::BoxLeft),
            "@" => Ok(Tile::Robot),
            _ => Err(()),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Wall => write!(f, "#"),
            Tile::BoxLeft => write!(f, "["),
            Tile::BoxRight => write!(f, "]"),
            Tile::Robot => write!(f, "@"),
        }
    }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    robot: (usize, usize),
}

impl FromStr for Map {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| {
                        let tile = c.to_string().parse::<Tile>().unwrap();

                        let tiles = match tile {
                            Tile::Robot => vec![Tile::Robot, Tile::Empty],
                            Tile::BoxLeft => vec![Tile::BoxLeft, Tile::BoxRight],
                            _ => vec![tile.clone(), tile.clone()],
                        };
                        tiles
                    })
                    .flatten()
                    .collect::<Vec<Tile>>()
            })
            .collect::<Vec<Vec<Tile>>>();
        let robot = tiles
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(|(x, tile)| {
                    if *tile == Tile::Robot {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .expect("Robot not found");
        Ok(Map { tiles, robot })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if (x, y) == self.robot {
                    write!(f, "@")?;
                } else {
                    write!(f, "{}", tile)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    fn can_move(&mut self, x: usize, y: usize, direction: &Direction) -> bool {
        let next_tile = match direction {
            Direction::Up => {
                if y == 1 {
                    return false;
                }
                (x, y - 1)
            }
            Direction::Down => {
                if y == self.tiles.len() - 1 {
                    return false;
                }
                (x, y + 1)
            }
            Direction::Left => {
                if x == 0 {
                    return false;
                }
                (x - 1, y)
            }
            Direction::Right => {
                if x == self.tiles[y].len() - 1 {
                    return false;
                }
                (x + 1, y)
            }
        };

        match (
            direction,
            &self.tiles[y][x].clone(),
            &self.tiles[next_tile.1][next_tile.0],
        ) {
            (_, Tile::Empty, _) => {
                panic!("Checking move of empty tile");
            }
            (_, Tile::Wall, _) => return false,
            (_, _, Tile::Wall) => return false,

            (_, _, Tile::Empty) => {}
            (Direction::Up | Direction::Down, _, Tile::BoxLeft) => {
                if !self.can_move(next_tile.0, next_tile.1, direction) {
                    return false;
                }
                if !self.can_move(next_tile.0 + 1, next_tile.1, direction) {
                    return false;
                }
            }

            (Direction::Up | Direction::Down, _, Tile::BoxRight) => {
                if !self.can_move(next_tile.0, next_tile.1, direction) {
                    return false;
                }
                if !self.can_move(next_tile.0 - 1, next_tile.1, direction) {
                    return false;
                }
            }

            (_, _, Tile::BoxLeft | Tile::BoxRight) => {
                if !self.can_move(next_tile.0, next_tile.1, direction) {
                    return false;
                }
            }
            _ => {
                println!(
                    "{} {} {}",
                    direction, self.tiles[y][x], self.tiles[next_tile.1][next_tile.0],
                );
                panic!("Invalid move checked");
            }
        }

        true
    }
    fn try_move(&mut self, x: usize, y: usize, direction: &Direction) -> bool {
        let next_tile = match direction {
            Direction::Up => {
                if y == 0 {
                    return false;
                }
                (x, y - 1)
            }
            Direction::Down => {
                if y == self.tiles.len() - 1 {
                    return false;
                }
                (x, y + 1)
            }
            Direction::Left => {
                if x == 0 {
                    return false;
                }
                (x - 1, y)
            }
            Direction::Right => {
                if x == self.tiles[y].len() - 1 {
                    return false;
                }
                (x + 1, y)
            }
        };

        match (
            direction,
            &self.tiles[y][x].clone(),
            &self.tiles[next_tile.1][next_tile.0],
        ) {
            (_, Tile::Empty, _) => {
                panic!("Trying to move empty tile");
            }
            (_, Tile::Wall, _) => return false,
            (_, _, Tile::Wall) => return false,

            (_, tile, Tile::Empty) => {
                self.tiles[y][x] = Tile::Empty;
                self.tiles[next_tile.1][next_tile.0] = tile.clone();

                if *tile == Tile::Robot {
                    self.robot = next_tile;
                }
            }
            (Direction::Up | Direction::Down, tile, Tile::BoxLeft) => {
                if !self.can_move(next_tile.0, next_tile.1, &direction) {
                    return false;
                }
                if !self.can_move(next_tile.0 + 1, next_tile.1, &direction) {
                    return false;
                }

                self.try_move(next_tile.0, next_tile.1, direction);
                self.try_move(next_tile.0 + 1, next_tile.1, direction);

                self.tiles[y][x] = Tile::Empty;
                self.tiles[next_tile.1][next_tile.0] = tile.clone();
                if *tile == Tile::Robot {
                    self.robot = next_tile;
                }
            }

            (Direction::Up | Direction::Down, tile, Tile::BoxRight) => {
                if !self.can_move(next_tile.0, next_tile.1, &direction) {
                    return false;
                }
                if !self.can_move(next_tile.0 - 1, next_tile.1, &direction) {
                    return false;
                }

                self.try_move(next_tile.0, next_tile.1, direction);
                self.try_move(next_tile.0 - 1, next_tile.1, direction);

                self.tiles[y][x] = Tile::Empty;
                self.tiles[next_tile.1][next_tile.0] = tile.clone();
                if *tile == Tile::Robot {
                    self.robot = next_tile;
                }
            }

            (Direction::Left | Direction::Right, tile, Tile::BoxLeft)
            | (Direction::Left | Direction::Right, tile, Tile::BoxRight) => {
                if !self.try_move(next_tile.0, next_tile.1, direction) {
                    return false;
                }
                self.tiles[y][x] = Tile::Empty;
                self.tiles[next_tile.1][next_tile.0] = tile.clone();
                if *tile == Tile::Robot {
                    self.robot = next_tile;
                }
            }
            _ => {
                println!(
                    "{} {} {}",
                    direction, self.tiles[y][x], self.tiles[next_tile.1][next_tile.0],
                );
                panic!("Invalid move")
            }
        }

        true
    }
}

fn part2(input: &str) -> u32 {
    let mut input = input.split("\n\n");

    let mut map = input
        .next()
        .expect("Input error")
        .parse::<Map>()
        .expect("Input error");

    // println!("{}", map);

    let directions = input
        .next()
        .expect("input error")
        .chars()
        .filter(|c| c != &'\n')
        .map(|c| match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid direction"),
        })
        .collect::<VecDeque<Direction>>();

    for direction in directions {
        // println!("{}", direction);
        let (x, y) = map.robot;
        if map.try_move(x, y, &direction) {
            // println!("{}", map);
        }
    }

    let mut sum = 0;
    for (y, row) in map.tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile == Tile::BoxLeft {
                sum += 100 * y + x;
            }
        }
    }

    sum as u32
}

fn main() {
    println!("AoC 2024 - Day 15 Part 2");
    let input = include_str!("input");
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_test() {
        let input = include_str!("test1");
        assert_eq!(part2(input), 9021);
    }
}
