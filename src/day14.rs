use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
enum CavePointType {
    Air,
    Rock,
    Sand,
    SandEntry,
}

impl Display for CavePointType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CavePointType::Air => write!(f, "."),
            CavePointType::Rock => write!(f, "#"),
            CavePointType::Sand => write!(f, "O"),
            CavePointType::SandEntry => write!(f, "+"),
        }
    }
}

#[derive(Debug)]
struct RockPath(Vec<(i32, i32)>);

impl RockPath {
    fn new() -> RockPath {
        RockPath(Vec::new())
    }
    fn add_point(&mut self, point: (i32, i32)) {
        //Add points from last point to the new point
        let (x, y) = point;
        if self.0.is_empty() {
            self.0.push((x, y));
            return;
        }

        let (last_x, last_y) = self.0.last().unwrap().clone();
        println!("last: ({}, {})", last_x, last_y);
        println!("new: ({}, {})", x, y);

        if self.0.is_empty() {
            self.0.push((x, y));
        } else {
            //Add points from last point to the new point
            if x == last_x {
                let y_diff = y - last_y;
                println!("y_diff: {:?}", y_diff);
                if y_diff > 0 {
                    for y in 0..y_diff {
                        self.0.push((x, last_y + y + 1));
                    }
                } else {
                    for y in 0..-y_diff {
                        self.0.push((x, last_y - y - 1));
                    }
                }
            } else if y == last_y {
                let x_diff = x - last_x;
                println!("x_diff: {:?}", x_diff);
                if x_diff > 0 {
                    for x in 0..x_diff {
                        self.0.push((last_x + x + 1, y));
                    }
                } else {
                    for x in 0..-x_diff {
                        self.0.push((last_x - x - 1, y));
                    }
                }
            } else {
                panic!("Invalid path");
            }
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
struct ParsePathError;

impl FromStr for RockPath {
    type Err = ParsePathError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rock_path = RockPath::new();
        s.split(" -> ")
            .map(|s| {
                let mut coords = s.split(",");
                let x = coords
                    .next()
                    .unwrap()
                    .parse::<i32>()
                    .or(Err(ParsePathError))?;
                let y = coords
                    .next()
                    .unwrap()
                    .parse::<i32>()
                    .or(Err(ParsePathError))?;

                Ok((x, y))
            })
            .collect::<Result<Vec<(i32, i32)>, Self::Err>>()?
            .iter()
            .for_each(|p| rock_path.add_point(p.clone()));
        Ok(rock_path)
    }
}

impl Iterator for RockPath {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

struct CaveMap {
    map: Vec<Vec<CavePointType>>,
    x_limits: (i32, i32),
    y_limit: i32,
    sand_entry: (i32, i32),
}

impl CaveMap {
    fn new() -> CaveMap {
        CaveMap {
            map: vec![vec![CavePointType::Air; 1000]; 1000],
            x_limits: (500, 500),
            y_limit: (0),
            sand_entry: (500, 0),
        }
    }
    fn add_rocks(&mut self, rocks: &mut RockPath) {
        for (x, y) in rocks.by_ref() {
            if y > self.y_limit {
                self.y_limit = y;
            }
            if x < self.x_limits.0 {
                self.x_limits.0 = x;
            }
            if x > self.x_limits.1 {
                self.x_limits.1 = x;
            }
            self.map[y as usize][x as usize] = CavePointType::Rock;
        }
    }

    fn drop_one_sand(&mut self, x: &i32, y: &i32) -> Result<(), ()> {
        let mut y = y.clone();
        let x = x.clone();

        if self.map[y as usize][x as usize] == CavePointType::Sand {
            return Err(());
        }

        loop {
            y += 1;
            if y > self.y_limit {
                return Err(()); //Reached bottom
            }
            if self.map[y as usize][x as usize] != CavePointType::Air {
                if self.map[y as usize][x as usize - 1] == CavePointType::Air {
                    return self.drop_one_sand(&(x - 1), &y);
                } else if self.map[y as usize][x as usize + 1] == CavePointType::Air {
                    return self.drop_one_sand(&(x + 1), &y);
                } else {
                    self.map[y as usize - 1][x as usize] = CavePointType::Sand;
                    return Ok(());
                }
            }
        }
    }
}

impl Display for CaveMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.y_limit + 1 {
            for x in self.x_limits.0 - 1..self.x_limits.1 + 2 {
                if x == self.sand_entry.0 && y == self.sand_entry.1 {
                    write!(f, "{}", CavePointType::SandEntry)?;
                } else {
                    write!(f, "{}", self.map[y as usize][x as usize])?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn part1() -> i32 {
    let input = include_str!("../input/14");
    let mut cave_map = CaveMap::new();
    let mut rocks = input
        .lines()
        .map(|l| l.parse::<RockPath>().unwrap())
        .collect::<Vec<RockPath>>();
    rocks.iter_mut().for_each(|r| cave_map.add_rocks(r));
    println!("{}", cave_map);

    let mut dropped_sand = 0;
    while let Ok(()) = cave_map.drop_one_sand(&500, &0) {
        dropped_sand += 1;
        println!("{}", cave_map);
    }
    println!("Dropped sand: {}", dropped_sand);
    dropped_sand
}

fn part2() -> i32 {
    0
}

pub fn run() {
    println!("Day 14");
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rock_path_iterator() {
        let mut rock_path = RockPath::new();
        rock_path.add_point((0, 0));
        rock_path.add_point((0, 2));
        rock_path.add_point((-1, 2));
        rock_path.add_point((-1, -5));
        rock_path.add_point((1, -5));

        println!("{:?}", rock_path);

        assert_eq!(rock_path.next(), Some((1, -5)));
        assert_eq!(rock_path.next(), Some((0, -5)));
        assert_eq!(rock_path.next(), Some((-1, -5)));
        assert_eq!(rock_path.next(), Some((-1, -4)));
        assert_eq!(rock_path.next(), Some((-1, -3)));
        assert_eq!(rock_path.next(), Some((-1, -2)));
        assert_eq!(rock_path.next(), Some((-1, -1)));
        assert_eq!(rock_path.next(), Some((-1, 0)));
        assert_eq!(rock_path.next(), Some((-1, 1)));
        assert_eq!(rock_path.next(), Some((-1, 2)));
        assert_eq!(rock_path.next(), Some((0, 2)));
        assert_eq!(rock_path.next(), Some((0, 1)));
        assert_eq!(rock_path.next(), Some((0, 0)));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input/test14");
        let mut cave_map = CaveMap::new();
        let mut rocks = input
            .lines()
            .map(|l| l.parse::<RockPath>().unwrap())
            .collect::<Vec<RockPath>>();
        rocks.iter_mut().for_each(|r| cave_map.add_rocks(r));
        println!("{}", cave_map);

        let mut dropped_sand = 0;
        while let Ok(()) = cave_map.drop_one_sand(&500, &0) {
            dropped_sand += 1;
            println!("{}", cave_map);
        }
        println!("Dropped sand: {}", dropped_sand);
        assert_eq!(dropped_sand, 24);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 0);
    }
}
