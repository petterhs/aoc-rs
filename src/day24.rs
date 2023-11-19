use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

type Pos = (usize, usize);

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Blizzard {
    pos: Pos,
    direction: Direction,
    max: usize,
}

impl Blizzard {
    fn new(pos: Pos, direction: Direction, max: usize) -> Self {
        Self {
            pos,
            direction,
            max,
        }
    }

    fn step(&mut self) {
        match self.direction {
            Direction::Up => {
                if self.pos.1 == 0 {
                    self.pos = (self.pos.0, self.max - 1);
                } else {
                    self.pos = (self.pos.0, self.pos.1 - 1);
                }
            }
            Direction::Down => {
                if self.pos.1 == self.max - 1 {
                    self.pos = (self.pos.0, 0);
                } else {
                    self.pos = (self.pos.0, self.pos.1 + 1);
                }
            }
            Direction::Left => {
                if self.pos.0 == 0 {
                    self.pos = (self.max - 1, self.pos.1);
                } else {
                    self.pos = (self.pos.0 - 1, self.pos.1);
                }
            }
            Direction::Right => {
                if self.pos.0 == self.max - 1 {
                    self.pos = (0, self.pos.1);
                } else {
                    self.pos = (self.pos.0 + 1, self.pos.1);
                }
            }
        }
    }
}

impl Display for Blizzard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.pos)
    }
}

#[derive(Debug)]
struct Valley {
    start_pos: Pos,
    end_pos: Pos,
    width: usize,
    height: usize,
}

impl Valley {
    fn new(start_pos: Pos, end_pos: Pos, width: usize, height: usize) -> Self {
        Self {
            start_pos,
            end_pos,
            width,
            height,
        }
    }
}

fn parse_input(str: &str) -> (Valley, Vec<Blizzard>) {
    let width = str.lines().next().unwrap().len() - 2;
    let height = str.lines().count() - 2;
    let start_pos = (str.lines().next().unwrap().find('.').unwrap() - 1, 0);
    let end_pos = (
        str.lines().last().unwrap().find('.').unwrap() - 1,
        height - 1,
    );
    let mut blizzards = Vec::new();

    for (y, line) in str.lines().skip(1).enumerate() {
        if width == 0 {}
        for (x, c) in line.chars().skip(1).enumerate() {
            match c {
                '^' => {
                    assert!(x != 0);
                    blizzards.push(Blizzard {
                        pos: (x, y),
                        direction: Direction::Up,
                        max: height,
                    });
                }
                'v' => {
                    assert!(x != 0);
                    blizzards.push(Blizzard {
                        pos: (x, y),
                        direction: Direction::Down,
                        max: height,
                    });
                }
                '<' => {
                    blizzards.push(Blizzard {
                        pos: (x, y),
                        direction: Direction::Left,
                        max: width,
                    });
                }
                '>' => {
                    blizzards.push(Blizzard {
                        pos: (x, y),
                        direction: Direction::Right,
                        max: width,
                    });
                }
                _ => (),
            }
        }
    }

    (Valley::new(start_pos, end_pos, width, height), blizzards)
}

fn shortest_path(valley: &Valley, blizzards: &[Blizzard]) -> usize {
    let steps = 1;
    let mut blizzards = blizzards.to_vec();

    let mut queue = VecDeque::new();
    let mut cache = HashSet::new();

    for b in blizzards.iter_mut() {
        b.step();
    }
    let path: Vec<Pos> = Vec::new();
    queue.push_back((valley.start_pos, blizzards, steps, path));

    while let Some((pos, blizzards, steps, path)) = queue.pop_front() {
        let mut blizzards = blizzards.clone();
        let pos = pos.clone();

        if cache.contains(&(pos, steps)) {
            continue;
        }

        // display_valley(&pos, &valley, &blizzards);
        for b in blizzards.iter_mut() {
            b.step();
        }

        let mut new_pos = vec![
            (pos.0 as i32, pos.1 as i32 - 1),
            (pos.0 as i32, pos.1 as i32 + 1),
            (pos.0 as i32 - 1, pos.1 as i32),
            (pos.0 as i32 + 1, pos.1 as i32),
            (pos.0 as i32, pos.1 as i32),
        ];

        let new_pos = new_pos
            .iter_mut()
            .filter(|p| {
                p.0 >= 0
                    && p.1 >= 0
                    && p.0 <= valley.width as i32 - 1
                    && p.1 <= valley.height as i32 - 1
                    && !blizzards
                        .iter()
                        .any(|b| b.pos == (p.0 as usize, p.1 as usize))
            })
            .map(|p| (p.0 as usize, p.1 as usize))
            .collect::<Vec<(usize, usize)>>();

        if new_pos.contains(&valley.end_pos) {
            println!("Found end");
            println!("Path: {:?}", path);
            return steps + 2;
        }

        for pos in new_pos {
            let mut path = path.clone();
            path.push(pos);
            queue.push_back((pos, blizzards.clone(), steps + 1, path));
        }

        cache.insert((pos, steps));
    }

    0
}

fn display_valley(pos: &Pos, valley: &Valley, blizzards: &[Blizzard]) {
    let mut valley_grid = vec![vec!['.'; valley.width]; valley.height];
    let mut blizzard_pos = vec![vec![0; valley.width]; valley.height];

    valley_grid[pos.1][pos.0] = 'E';

    for b in blizzards {
        blizzard_pos[b.pos.1][b.pos.0] += 1;
        valley_grid[b.pos.1][b.pos.0] = match b.direction {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        };
    }

    for (x, line) in blizzard_pos.iter().enumerate() {
        for (y, c) in line.iter().enumerate() {
            if *c > 1 {
                valley_grid[x][y] = blizzard_pos[x][y].to_string().chars().next().unwrap();
            }
        }
    }

    for line in valley_grid {
        println!("{}", line.iter().collect::<String>());
    }
}

fn part1() -> usize {
    let input = include_str!("../input/24");

    let (valley, blizzards) = parse_input(input);

    println!("{:?}", valley);
    println!("{:?}", blizzards);

    display_valley(&(0, 0), &valley, &blizzards);

    shortest_path(&valley, &blizzards)
}

pub fn run() {
    println!("Part 1: {}", part1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blizzard() {
        let input = include_str!("../input/test24_1");

        let (valley, mut blizzards) = parse_input(input);

        println!("{:?}", valley);
        display_valley(&(0, 0), &valley, &blizzards);
        println!("");

        for _ in 0..18 {
            for b in blizzards.iter_mut() {
                b.step();
            }
            display_valley(&(0, 0), &valley, &blizzards);
            println!("");
        }
        assert!(false);
    }
}
