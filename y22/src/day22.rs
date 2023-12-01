use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_int(&self) -> usize {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
}

#[derive(Debug)]
enum Pos {
    None,
    Empty,
    Wall,
    Direction(Direction),
}

#[derive(Debug)]
enum Instruction {
    TurnLeft,
    TurnRight,
    MoveForward(usize),
}

type Position = (usize, usize);
struct Grid(Vec<Vec<Pos>>);

impl Grid {
    fn is_out_of_bounds(&self, pos: &Position) -> bool {
        pos.0 < 1 || pos.1 < 1 || pos.0 > self.0[0].len() || pos.1 > self.0.len()
    }

    fn is_wall(&self, pos: &Position) -> bool {
        match self.0[pos.1 - 1][pos.0 - 1] {
            Pos::Wall => true,
            _ => false,
        }
    }
    fn is_none(&self, pos: &Position) -> bool {
        let x = pos.0 - 1;
        let y = pos.1 - 1;

        match self.0[y][x] {
            Pos::None => true,
            _ => false,
        }
    }

    fn next_empty(&self, pos: Position, direction: &Direction) -> Position {
        let mut new_pos = pos;

        match direction {
            Direction::Up => new_pos = (new_pos.0, self.0.len()),
            Direction::Down => new_pos = (new_pos.0, 1),
            Direction::Right => new_pos = (1, new_pos.1),
            Direction::Left => new_pos = (self.0[0].len(), new_pos.1),
        }
        while self.is_none(&new_pos) {
            match direction {
                Direction::Up => new_pos.1 -= 1,
                Direction::Down => new_pos.1 += 1,
                Direction::Left => new_pos.0 -= 1,
                Direction::Right => new_pos.0 += 1,
            }
        }
        new_pos
    }
}

fn parse_input(input: &str) -> (Grid, Vec<Instruction>) {
    //Parse grid
    let mut grid = Vec::new();
    let max_row_len = input.lines().rev().skip(1).map(|x| x.len()).max().unwrap();
    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        let mut row = Vec::new();
        for c in line.chars() {
            match c {
                '#' => row.push(Pos::Wall),
                '.' => row.push(Pos::Empty),
                ' ' => row.push(Pos::None),
                _ => panic!("Unknown char: {}", c),
            }
        }
        while row.len() < max_row_len {
            row.push(Pos::None);
        }
        grid.push(row);
    }

    //Parse instructions
    let mut instructions = Vec::new();
    for line in input.lines().rev() {
        if line.is_empty() {
            break;
        }

        let mut number_buf = String::new();
        let mut i = 0;
        loop {
            match line.chars().nth(i) {
                Some(x) if x.is_alphabetic() => {
                    //Push the previous move instruction
                    let number = number_buf.parse::<usize>().unwrap();
                    instructions.push(Instruction::MoveForward(number));
                    number_buf.clear();

                    //Push the instruction
                    let instruction = match x {
                        'L' => Instruction::TurnLeft,
                        'R' => Instruction::TurnRight,
                        c => panic!("Unknown char: {}", c),
                    };
                    instructions.push(instruction);
                }
                Some(x) if x.is_numeric() => {
                    number_buf.push(x);
                }
                _ => break,
            }
            i += 1;
        }

        if !number_buf.is_empty() {
            let number = number_buf.parse::<usize>().unwrap();
            instructions.push(Instruction::MoveForward(number));
            number_buf.clear();
        }
    }

    (Grid(grid), instructions)
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.0.iter() {
            for pos in row {
                match pos {
                    Pos::None => write!(f, " "),
                    Pos::Empty => write!(f, "."),
                    Pos::Wall => write!(f, "#"),
                    Pos::Direction(Direction::Up) => write!(f, "^"),
                    Pos::Direction(Direction::Down) => write!(f, "v"),
                    Pos::Direction(Direction::Left) => write!(f, "<"),
                    Pos::Direction(Direction::Right) => write!(f, ">"),
                }?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part1() {
    let input = include_str!("../input/22");
    let (mut grid, instructions) = parse_input(input);
    println!("{}", grid);

    let mut pos = (0, 0);
    let mut direction = Direction::Right;

    for (x, x_pos) in grid.0[0].iter().enumerate() {
        if let Pos::Empty = x_pos {
            pos = (x + 1, 1);
            break;
        }
    }
    println!("Start pos: {:?}", pos);

    for instruction in instructions {
        match instruction {
            Instruction::TurnLeft => match direction {
                Direction::Up => direction = Direction::Left,
                Direction::Down => direction = Direction::Right,
                Direction::Left => direction = Direction::Down,
                Direction::Right => direction = Direction::Up,
            },
            Instruction::TurnRight => match direction {
                Direction::Up => direction = Direction::Right,
                Direction::Down => direction = Direction::Left,
                Direction::Left => direction = Direction::Up,
                Direction::Right => direction = Direction::Down,
            },
            Instruction::MoveForward(steps) => {
                for _ in 0..steps {
                    let mut new_pos = pos;
                    match direction {
                        Direction::Up => new_pos.1 -= 1,
                        Direction::Down => new_pos.1 += 1,
                        Direction::Left => new_pos.0 -= 1,
                        Direction::Right => new_pos.0 += 1,
                    }

                    if grid.is_out_of_bounds(&new_pos) {
                        new_pos = grid.next_empty(pos, &direction);
                    }

                    if grid.is_none(&new_pos) {
                        new_pos = grid.next_empty(pos, &direction);
                    }

                    if grid.is_wall(&new_pos) {
                        break;
                    }
                    pos = new_pos;
                    grid.0[pos.1 - 1][pos.0 - 1] = Pos::Direction(direction.clone());
                }
            }
        }
        // println!("{}", grid);
    }
    println!("{}", grid);

    println!("Final pos: {:?}", pos);
    let result = 1000 * pos.1 + 4 * pos.0 + direction.to_int();
    println!("Part1: {}", result);
}

pub fn run() {
    println!("Day 22");
    part1();
}
