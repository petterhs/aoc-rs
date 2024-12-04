use std::str::FromStr;

#[derive(Debug)]
struct MulFunc {
    a: u32,
    b: u32,
}

#[derive(Debug)]
enum Func {
    Mul(MulFunc),
    Do,
    Dont,
}

#[derive(Debug)]
struct ParseFuncError;

impl FromStr for MulFunc {
    type Err = ParseFuncError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let func = s
            .trim_end_matches(')')
            .split("mul(")
            .last()
            .unwrap_or_default();

        let mut iter = func.split(",");
        let a = iter.next();
        let b = iter.next();

        match (a, b) {
            (Some(a), Some(b)) => match (a.parse::<u32>(), b.parse::<u32>()) {
                (Ok(a), Ok(b)) => Ok(Self { a, b }),
                _ => Err(ParseFuncError),
            },
            _ => Err(ParseFuncError),
        }
    }
}

impl FromStr for Func {
    type Err = ParseFuncError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.parse::<MulFunc>().is_ok() {
            return s.parse::<MulFunc>().map(Func::Mul);
        }
        if s.contains("do()") {
            return Ok(Func::Do);
        }
        if s.contains("don't()") {
            return Ok(Func::Dont);
        }
        Err(ParseFuncError)
    }
}

fn part1(input: &str) -> u32 {
    input
        .split_inclusive(')')
        .map(|w| w.parse::<MulFunc>().map(|w| w.a * w.b).unwrap_or_default())
        .sum::<u32>()
}

fn part2(input: &str) -> u32 {
    let mut enabled = true;
    input
        .split_inclusive(')')
        .map(|w| w.parse::<Func>())
        .filter_map(Result::ok)
        .inspect(|w| println!("{:?}", w))
        .fold(0, |acc, w| match (enabled, w) {
            (true, Func::Mul(mul)) => acc + mul.a * mul.b,
            (_, Func::Do) => {
                enabled = true;
                acc
            }
            (_, Func::Dont) => {
                enabled = false;
                acc
            }
            _ => acc,
        })
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
        let input = include_str!("test");
        assert_eq!(part1(input), 161);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("test2");
        assert_eq!(part2(input), 48);
    }
}
