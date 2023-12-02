use std::str::FromStr;

#[derive(Debug)]
enum Colors {
    Blue,
    Red,
    Green,
}

impl FromStr for Colors {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blue" => Ok(Self::Blue),
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct CubeSet {
    blue: u32,
    red: u32,
    green: u32,
}

impl CubeSet {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { blue, red, green }
    }

    fn contains(&self, other: &Self) -> bool {
        self.blue >= other.blue && self.red >= other.red && self.green >= other.green
    }
}

impl FromStr for CubeSet {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = Self::new(0, 0, 0);

        let mut words = s.split(&[' ', ',']);

        let mut num = 0;

        while let Some(word) = words.next() {
            if let Ok(n) = word.parse::<u32>() {
                num = n;
                continue;
            }

            if let Ok(color) = word.parse::<Colors>() {
                match color {
                    Colors::Blue => set.blue = num,
                    Colors::Red => set.red = num,
                    Colors::Green => set.green = num,
                }
            }
        }
        Ok(set)
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    cubes: Vec<CubeSet>,
}

impl Game {
    fn max(&self) -> CubeSet {
        let mut max = CubeSet::new(0, 0, 0);
        self.cubes.iter().for_each(|cubeset| {
            if cubeset.blue > max.blue {
                max.blue = cubeset.blue;
            }

            if cubeset.red > max.red {
                max.red = cubeset.red;
            }

            if cubeset.green > max.green {
                max.green = cubeset.green;
            }
        });
        max
    }
}

impl FromStr for Game {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split(&[';', ':']);
        let id = words
            .next()
            .unwrap()
            .split(' ')
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let mut cubes = Vec::new();
        while let Some(line) = words.next() {
            cubes.push(line.parse::<CubeSet>().unwrap());
        }
        Ok(Self { id, cubes })
    }
}

fn part1(input: &str) -> u32 {
    let total = CubeSet::new(12, 13, 14);

    input
        .lines()
        .map(|line| line.parse::<Game>().unwrap())
        .filter(|game| game.cubes.iter().all(|cube| total.contains(cube)))
        .map(|game| game.id)
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.parse::<Game>().unwrap())
        .map(|game| game.max())
        .map(|cubeset| cubeset.blue * cubeset.red * cubeset.green)
        .sum()
}

fn main() {
    println!("AoC 2023 - Day 1");
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
        assert_eq!(part1(input), 8);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("test");
        assert_eq!(part2(input), 0);
    }
}
