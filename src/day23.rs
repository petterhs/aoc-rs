use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use std::collections::{HashMap, HashSet};
use std::{fmt::Display, str::FromStr};

type Pos = (i32, i32);

struct Elves(HashSet<Pos>);

impl Elves {
    fn num_empty_pos(&self) -> usize {
        let x_limits = self.0.iter().map(|(x, _)| x).minmax();
        let y_limits = self.0.iter().map(|(_, y)| y).minmax();

        if let (MinMax(x_min, x_max), MinMax(y_min, y_max)) = (x_limits, y_limits) {
            let area = (x_max - x_min + 1) * (y_max - y_min + 1);

            area as usize - self.0.len()
        } else {
            0
        }
    }
}

impl FromStr for Elves {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elves = HashSet::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    elves.insert((x as i32, y as i32));
                }
            }
        }
        Ok(Elves(elves))
    }
}

impl Display for Elves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x_limits = self.0.iter().map(|(x, _)| x).minmax();
        let y_limits = self.0.iter().map(|(_, y)| y).minmax();

        if let (MinMax(x_min, x_max), MinMax(y_min, y_max)) = (x_limits, y_limits) {
            for y in 0..=(y_max - y_min) {
                for x in 0..=(x_max - x_min) {
                    if self.0.contains(&(x + x_min, y + y_min)) {
                        write!(f, "#")?;
                    } else {
                        write!(f, ".")?;
                    }
                }

                writeln!(f)?;
            }
        } else {
            return Err(std::fmt::Error);
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
enum Direction {
    N,
    S,
    W,
    E,
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Direction::N => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::E,
            Direction::E => Direction::N,
        }
    }
}

fn part1() -> i32 {
    let input = include_str!("../input/23");
    let mut elves = Elves::from_str(input).unwrap();

    let mut direction = Direction::N;
    let mut round = 1;
    loop {
        let mut new_pos = HashMap::new();
        for elf in elves.0.iter() {
            if !(elves.0.contains(&(elf.0, elf.1 - 1))
                || elves.0.contains(&(elf.0 - 1, elf.1 - 1))
                || elves.0.contains(&(elf.0 + 1, elf.1 - 1))
                || elves.0.contains(&(elf.0, elf.1 + 1))
                || elves.0.contains(&(elf.0 - 1, elf.1 + 1))
                || elves.0.contains(&(elf.0 + 1, elf.1 + 1))
                || elves.0.contains(&(elf.0 - 1, elf.1))
                || elves.0.contains(&(elf.0 + 1, elf.1)))
            {
                continue;
            }

            let mut current_direction = direction.clone();
            for _ in 0..4 {
                match current_direction {
                    Direction::N => {
                        if !(elves.0.contains(&(elf.0, elf.1 - 1))
                            || elves.0.contains(&(elf.0 - 1, elf.1 - 1))
                            || elves.0.contains(&(elf.0 + 1, elf.1 - 1)))
                        {
                            new_pos.insert(*elf, (elf.0, elf.1 - 1));
                            break;
                        }
                    }
                    Direction::S => {
                        if !(elves.0.contains(&(elf.0, elf.1 + 1))
                            || elves.0.contains(&(elf.0 - 1, elf.1 + 1))
                            || elves.0.contains(&(elf.0 + 1, elf.1 + 1)))
                        {
                            new_pos.insert(*elf, (elf.0, elf.1 + 1));
                            break;
                        }
                    }
                    Direction::W => {
                        if !(elves.0.contains(&(elf.0 - 1, elf.1))
                            || elves.0.contains(&(elf.0 - 1, elf.1 - 1))
                            || elves.0.contains(&(elf.0 - 1, elf.1 + 1)))
                        {
                            new_pos.insert(*elf, (elf.0 - 1, elf.1));
                            break;
                        }
                    }
                    Direction::E => {
                        if !(elves.0.contains(&(elf.0 + 1, elf.1))
                            || elves.0.contains(&(elf.0 + 1, elf.1 - 1))
                            || elves.0.contains(&(elf.0 + 1, elf.1 + 1)))
                        {
                            new_pos.insert(*elf, (elf.0 + 1, elf.1));
                            break;
                        }
                    }
                }
                current_direction = current_direction.next();
            }
        }
        direction = direction.next();

        if round == 10 {
            println!("Part1: {}", elves.num_empty_pos());
        }

        if new_pos.is_empty() {
            return round;
        }
        round += 1;

        let mut non_dup = HashSet::new();

        new_pos
            .clone()
            .into_iter()
            .map(|(_, pos)| pos)
            .for_each(|pos| {
                if non_dup.contains(&pos) {
                    non_dup.remove(&pos);
                } else {
                    non_dup.insert(pos);
                }
            });

        for (elf, pos) in new_pos {
            if !non_dup.contains(&pos) {
                continue;
            }
            elves.0.remove(&elf);
            elves.0.insert(pos);
        }
    }
}

pub fn run() {
    println!("Part 1: {}", part1());
}
