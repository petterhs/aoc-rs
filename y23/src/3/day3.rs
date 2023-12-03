use std::collections::{HashMap, HashSet};

#[derive(Debug, Hash, Eq, PartialEq)]
enum Type {
    Symbol(char),
    Number(char),
}

impl From<char> for Type {
    fn from(s: char) -> Self {
        if let Some(_) = s.to_digit(10) {
            return Self::Number(s);
        }

        Self::Symbol(s)
    }
}

fn solution(input: &str) -> (u32, u32) {
    let mut cache = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            cache.insert((x, y), Type::from(c));
        }
    }

    let rel_pos = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let mut counted_numbers = HashMap::new();

    for (k, v) in cache.iter() {
        if let Type::Symbol(s) = v {
            let mut numbers = HashSet::new();
            for (x, y) in rel_pos.iter() {
                let x = k.0 as i32 + x;
                let y = k.1 as i32 + y;

                if x < 0 || y < 0 {
                    continue;
                }

                if let Some(Type::Number(n)) =
                    cache.get(&(x.try_into().unwrap(), y.try_into().unwrap()))
                {
                    let mut num = String::from(*n);

                    let mut x_pluss = x + 1;
                    while let Some(Type::Number(n)) =
                        cache.get(&(x_pluss.try_into().unwrap(), y.try_into().unwrap()))
                    {
                        num.push(*n);
                        x_pluss += 1;
                    }

                    let mut x_minus = x - 1;
                    while let Ok(x) = x_minus.try_into() {
                        if let Some(Type::Number(n)) = cache.get(&(x, y.try_into().unwrap())) {
                            let mut n = String::from(*n);
                            n.push_str(&num);
                            num = n;
                            x_minus -= 1;
                        } else {
                            break;
                        }
                    }

                    numbers.insert(num.parse::<u32>().unwrap());
                }
            }
            counted_numbers.insert((k, s), numbers);
        }
    }

    let part1 = counted_numbers
        .iter()
        .map(|(_, numbers)| numbers.iter().sum::<u32>())
        .sum();

    let part2 = counted_numbers
        .iter()
        .filter(|((_, symbol), _)| **symbol == '*')
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v.iter().fold(1, |acc, n| acc * n))
        .sum();

    (part1, part2)
}

fn main() {
    println!("AoC 2023 - Day 1");
    let input = include_str!("input");
    let (part1, part2) = solution(input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_test() {
        let input = include_str!("test");
        let (part1, part2) = solution(input);
        assert_eq!(part1, 4361);
        assert_eq!(part2, 467835);
    }
}
