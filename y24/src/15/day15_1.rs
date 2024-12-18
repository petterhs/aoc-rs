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
    Box,
    Robot,
}

impl FromStr for Tile {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Tile::Empty),
            "#" => Ok(Tile::Wall),
            "O" => Ok(Tile::Box),
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
            Tile::Box => write!(f, "O"),
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
                    .map(|c| c.to_string().parse::<Tile>().unwrap())
                    .collect::<Vec<Tile>>()
            })
            .collect::<Vec<Vec<Tile>>>();
        let robot = s
            .lines()
            .enumerate()
            .find_map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .find_map(|(x, c)| if c == '@' { Some((x, y)) } else { None })
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
    fn try_move(&mut self, x: usize, y: usize, direction: Direction) -> bool {
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
            &self.tiles[y][x].clone(),
            &self.tiles[next_tile.1][next_tile.0],
        ) {
            (Tile::Empty, _) => {
                panic!("Invalid move");
            }
            (Tile::Wall, _) => return false,
            (_, Tile::Wall) => return false,
            (tile, Tile::Empty) => {
                self.tiles[y][x] = Tile::Empty;
                self.tiles[next_tile.1][next_tile.0] = tile.clone();

                if *tile == Tile::Robot {
                    self.robot = next_tile;
                }
            }
            (tile, Tile::Box) => {
                if !self.try_move(next_tile.0, next_tile.1, direction) {
                    return false;
                }
                self.tiles[y][x] = Tile::Empty;
                self.tiles[next_tile.1][next_tile.0] = tile.clone();
                if *tile == Tile::Robot {
                    self.robot = next_tile;
                }
            }
            _ => panic!("Invalid move"),
        }

        true
    }
}

fn part1(input: &str) -> u32 {
    let mut input = input.split("\n\n");

    let mut map = input
        .next()
        .expect("Input error")
        .parse::<Map>()
        .expect("Input error");

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
        let (x, y) = map.robot;
        if map.try_move(x, y, direction) {}
    }

    let mut sum = 0;
    for (y, row) in map.tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile == Tile::Box {
                sum += 100 * y + x;
            }
        }
    }

    sum as u32
}

fn main() {
    println!("AoC 2024 - Day 15 Part 1");
    let input = include_str!("input");
    println!("Part 1: {}", part1(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("test");
        assert_eq!(part1(input), 2028);
    }
}
