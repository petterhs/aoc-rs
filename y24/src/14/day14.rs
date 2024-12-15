use std::fmt::{self, Display};
use std::str::FromStr;

#[derive(PartialEq)]
struct Xy(i32, i32);

struct Robot {
    pos: Xy,
    velocity: Xy,
}

impl FromStr for Robot {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec = s
            .split_whitespace()
            .map(|x| {
                let xy = x
                    .split(',')
                    .map(|s| {
                        let y = s
                            .chars()
                            .filter(|c| c.is_digit(10) || *c == '-')
                            .collect::<String>();
                        y.parse::<i32>().unwrap()
                    })
                    .collect::<Vec<i32>>();

                (xy[0], xy[1])
            })
            .collect::<Vec<(i32, i32)>>();

        Ok(Robot {
            pos: Xy(vec[0].0, vec[0].1),
            velocity: Xy(vec[1].0, vec[1].1),
        })
    }
}

struct Map {
    robots: Vec<Robot>,
    max: Xy,
}

impl FromStr for Map {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let robots = s
            .lines()
            .map(|x| x.parse::<Robot>().unwrap())
            .collect::<Vec<Robot>>();
        Ok(Map {
            robots,
            max: Xy(101, 103),
        })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.max.1 {
            for x in 0..self.max.0 {
                let mut found = 0;
                for robot in &self.robots {
                    if robot.pos.0 == x && robot.pos.1 == y {
                        found += 1;
                    }
                }
                if found > 0 {
                    write!(f, "{}", found)?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
impl Map {
    fn step(&mut self) {
        for robot in &mut self.robots {
            let mut new_pos_x = robot.pos.0 as i32 + robot.velocity.0;
            let mut new_pos_y = robot.pos.1 as i32 + robot.velocity.1;

            // println!("Pos: ({}, {})", robot.pos.0, robot.pos.1);
            // println!("Vel: ({}, {})", robot.velocity.0, robot.velocity.1);
            // println!("New Pos: ({}, {})", new_pos_x, new_pos_y);

            if new_pos_x < 0 {
                new_pos_x = self.max.0 + new_pos_x;
            }
            if new_pos_y < 0 {
                new_pos_y = self.max.1 + new_pos_y;
            }
            if new_pos_x >= self.max.0 {
                new_pos_x = new_pos_x - self.max.0;
            }
            if new_pos_y >= self.max.1 {
                new_pos_y = new_pos_y - self.max.1;
            }
            robot.pos.0 = new_pos_x;
            robot.pos.1 = new_pos_y;
        }
    }
}

fn part1(input: &str, max: Xy) -> u32 {
    let mut map = input.parse::<Map>().unwrap();

    map.max = max;

    for _ in 0..100 {
        map.step();
    }

    let mut quadrants = (0, 0, 0, 0);
    for robot in &map.robots {
        if robot.pos.0 < map.max.0 / 2 && robot.pos.1 < map.max.1 / 2 {
            quadrants.0 += 1;
        } else if robot.pos.0 >= map.max.0 / 2 + 1 && robot.pos.1 < map.max.1 / 2 {
            quadrants.1 += 1;
        } else if robot.pos.0 < map.max.0 / 2 && robot.pos.1 >= map.max.1 / 2 + 1 {
            quadrants.2 += 1;
        } else if robot.pos.0 >= map.max.0 / 2 + 1 && robot.pos.1 >= map.max.1 / 2 + 1 {
            quadrants.3 += 1;
        }
    }

    quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3
}

fn part2(input: &str, max: Xy) -> u32 {
    let mut map = input.parse::<Map>().unwrap();

    map.max = max;

    for i in 0..10000 {
        map.step();
        let mut center = 0;
        for robot in &map.robots {
            if robot.pos.0 < map.max.0 / 2 + 1 + 20
                && robot.pos.0 > map.max.0 / 2 - 1 - 20
                && robot.pos.1 < map.max.1 / 2 + 1 + 20
                && robot.pos.1 > map.max.1 / 2 - 1 - 20
            {
                center += 1;
            }
        }
        if center > 250 {
            println!("Step: {}", i + 1);
            println!("Center: {:?}", center);
            println!("{}", map);
            println!();
        }
    }
    0
}

fn main() {
    println!("AoC 2024 - Day 1");
    let input = include_str!("input");
    println!("Part 1: {}", part1(input, Xy(101, 103)));
    println!("Part 2: {}", part2(input, Xy(101, 103)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("test");
        assert_eq!(part1(input, Xy(11, 7)), 12);
    }
}
