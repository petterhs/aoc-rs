use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Char {
    None,
    X,
    M,
    A,
    S,
}

enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
    UpRight,
    DownRight,
    UpLeft,
    DownLeft,
}

struct Grid {
    grid: Vec<Vec<Char>>,
}

impl From<char> for Char {
    fn from(c: char) -> Self {
        match c {
            'X' => Char::X,
            'M' => Char::M,
            'A' => Char::A,
            'S' => Char::S,
            _ => Char::None,
        }
    }
}

impl From<&str> for Grid {
    fn from(s: &str) -> Self {
        let grid = s
            .lines()
            .map(|line| line.chars().map(|c| Char::from(c)).collect())
            .collect();
        Grid { grid }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.grid {
            for tile in line {
                match tile {
                    Char::X => write!(f, "X")?,
                    Char::M => write!(f, "M")?,
                    Char::A => write!(f, "A")?,
                    Char::S => write!(f, "S")?,
                    Char::None => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn next_char(char: Char) -> Char {
    match char {
        Char::X => Char::M,
        Char::M => Char::A,
        Char::A => Char::S,
        Char::S => {
            panic!("No next char for S")
        }
        Char::None => Char::None,
    }
}

fn part1(input: &str) -> u32 {
    let grid = Grid::from(input);

    println!("{}", grid);

    let mut queue = vec![];

    grid.grid.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, tile)| match tile {
            Char::X => {
                queue.push((Char::X, Direction::None, (x, y)));
            }
            _ => {}
        });
    });

    let mut num_xmas = 0;

    while let Some((tile, direction, (x, y))) = queue.pop() {
        match (tile, direction) {
            (Char::S, _) => {
                num_xmas += 1;
            }
            (Char::X, _) => {
                if (x + 1) < grid.grid[0].len() && grid.grid[y][x + 1] == Char::M {
                    queue.push((Char::M, Direction::Right, (x + 1, y)));
                }

                if (x) > 0 && grid.grid[y][x - 1] == Char::M {
                    queue.push((Char::M, Direction::Left, (x - 1, y)));
                }

                if (y + 1) < grid.grid.len() && grid.grid[y + 1][x] == Char::M {
                    queue.push((Char::M, Direction::Down, (x, y + 1)));
                }

                if (y) > 0 && grid.grid[y - 1][x] == Char::M {
                    queue.push((Char::M, Direction::Up, (x, y - 1)));
                }

                if (x + 1) < grid.grid[0].len()
                    && (y + 1) < grid.grid.len()
                    && grid.grid[y + 1][x + 1] == Char::M
                {
                    queue.push((Char::M, Direction::DownRight, (x + 1, y + 1)));
                }

                if (x + 1) < grid.grid[0].len() && (y) > 0 && grid.grid[y - 1][x + 1] == Char::M {
                    queue.push((Char::M, Direction::UpRight, (x + 1, y - 1)));
                }

                if (x) > 0 && (y + 1) < grid.grid.len() && grid.grid[y + 1][x - 1] == Char::M {
                    queue.push((Char::M, Direction::DownLeft, (x - 1, y + 1)));
                }

                if (x) > 0 && (y) > 0 && grid.grid[y - 1][x - 1] == Char::M {
                    queue.push((Char::M, Direction::UpLeft, (x - 1, y - 1)));
                }
            }
            (character, Direction::Right) => {
                if (x + 1) < grid.grid[0].len() && grid.grid[y][x + 1] == next_char(character) {
                    queue.push((next_char(character), Direction::Right, (x + 1, y)));
                }
            }
            (character, Direction::Left) => {
                if (x) > 0 && grid.grid[y][x - 1] == next_char(character) {
                    queue.push((next_char(character), Direction::Left, (x - 1, y)));
                }
            }

            (character, Direction::Down) => {
                if (y + 1) < grid.grid.len() && grid.grid[y + 1][x] == next_char(character) {
                    queue.push((next_char(character), Direction::Down, (x, y + 1)));
                }
            }

            (character, Direction::Up) => {
                if (y) > 0 && grid.grid[y - 1][x] == next_char(character) {
                    queue.push((next_char(character), Direction::Up, (x, y - 1)));
                }
            }

            (character, Direction::DownRight) => {
                if (x + 1) < grid.grid[0].len()
                    && (y + 1) < grid.grid.len()
                    && grid.grid[y + 1][x + 1] == next_char(character)
                {
                    queue.push((next_char(character), Direction::DownRight, (x + 1, y + 1)));
                }
            }

            (character, Direction::UpRight) => {
                if (x + 1) < grid.grid[0].len()
                    && (y) > 0
                    && grid.grid[y - 1][x + 1] == next_char(character)
                {
                    queue.push((next_char(character), Direction::UpRight, (x + 1, y - 1)));
                }
            }

            (character, Direction::DownLeft) => {
                if (x) > 0
                    && (y + 1) < grid.grid.len()
                    && grid.grid[y + 1][x - 1] == next_char(character)
                {
                    queue.push((next_char(character), Direction::DownLeft, (x - 1, y + 1)));
                }
            }

            (character, Direction::UpLeft) => {
                if (x) > 0 && (y) > 0 && grid.grid[y - 1][x - 1] == next_char(character) {
                    queue.push((next_char(character), Direction::UpLeft, (x - 1, y - 1)));
                }
            }

            _ => {
                panic!("Unknown tile: {:?}", tile);
            }
        }
    }

    num_xmas
}

fn part2(input: &str) -> u32 {
    let grid = Grid::from(input);

    println!("{}", grid);

    let mut num_xmas = 0;

    let x_max = grid.grid[0].len();
    let y_max = grid.grid.len();

    grid.grid.iter().enumerate().for_each(|(y, line)| {
        line.iter()
            .enumerate()
            .filter(|(x, tile)| {
                if *x == 0 || *x == x_max - 1 || y == 0 || y == y_max - 1 {
                    return false;
                }
                match tile {
                    Char::A => true,
                    _ => false,
                }
            })
            .for_each(|(x, _tile)| {
                if grid.grid[y - 1][x - 1] == Char::S && grid.grid[y + 1][x + 1] == Char::M {
                    if grid.grid[y + 1][x - 1] == Char::S && grid.grid[y - 1][x + 1] == Char::M {
                        num_xmas += 1;
                    }

                    if grid.grid[y + 1][x - 1] == Char::M && grid.grid[y - 1][x + 1] == Char::S {
                        num_xmas += 1;
                    }
                }

                if grid.grid[y - 1][x - 1] == Char::M && grid.grid[y + 1][x + 1] == Char::S {
                    if grid.grid[y + 1][x - 1] == Char::S && grid.grid[y - 1][x + 1] == Char::M {
                        num_xmas += 1;
                    }

                    if grid.grid[y + 1][x - 1] == Char::M && grid.grid[y - 1][x + 1] == Char::S {
                        num_xmas += 1;
                    }
                }
            });
    });

    num_xmas
}

fn main() {
    println!("AoC 2024 - Day 1");
    let input = include_str!("input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("test1");
        assert_eq!(part1(input), 4);
    }
    #[test]
    fn part1_test2() {
        let input = include_str!("test");
        assert_eq!(part1(input), 18);
    }
    #[test]
    fn part2_test() {
        let input = include_str!("test");
        assert_eq!(part2(input), 9);
    }
}
