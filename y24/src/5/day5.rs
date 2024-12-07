use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Rule {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct ParseRuleError;

impl FromStr for Rule {
    type Err = ParseRuleError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('|').collect();
        let x = parts[0].parse().map_err(|_| ParseRuleError);
        let y = parts[1].parse().map_err(|_| ParseRuleError);
        match (x, y) {
            (Ok(x), Ok(y)) => Ok(Rule { x, y }),
            _ => Err(ParseRuleError),
        }
    }
}

fn validate_instruction(rules: Vec<Rule>, mut instruction: VecDeque<u32>) -> bool {
    if instruction.len() < 2 {
        return true;
    }

    let first = instruction.pop_front().unwrap();

    if rules.iter().filter(|rule| rule.y == first).any(|rule| {
        //Assert that rule.x does not exist in the rest of the instruction
        instruction.iter().any(|i| i == &rule.x)
    }) {
        return false;
    }
    return validate_instruction(rules.clone(), instruction.clone());
}

fn part1(input: &str) -> u32 {
    let input = input.split("\n\n").collect::<Vec<&str>>();
    let rules = input[0]
        .lines()
        .map(|l| l.parse::<Rule>().unwrap())
        .collect::<Vec<Rule>>();
    let instructions = input[1].lines().map(|l| {
        l.split(',')
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<VecDeque<u32>>()
    });

    instructions
        .into_iter()
        .filter(|i| validate_instruction(rules.clone(), i.clone()))
        .map(|i| {
            let middle = i.len() / 2;
            *i.get(middle).unwrap() as u32
        })
        .sum::<u32>() as u32
}

fn part2(input: &str) -> u32 {
    let input = input.split("\n\n").collect::<Vec<&str>>();
    let rules = input[0]
        .lines()
        .map(|l| l.parse::<Rule>().unwrap())
        .collect::<Vec<Rule>>();
    let instructions = input[1].lines().map(|l| {
        l.split(',')
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<VecDeque<u32>>()
    });

    instructions
        .into_iter()
        .filter(|i| !validate_instruction(rules.clone(), i.clone()))
        .map(|i| {
            let mut i = i.into_iter().collect::<Vec<u32>>();
            i.sort_by(|a, b| {
                let rule = rules
                    .iter()
                    .filter(|rule| (rule.y == *a && rule.x == *b) || (rule.y == *b && rule.x == *a))
                    .next();

                match rule {
                    Some(rule) => {
                        if rule.y == *a {
                            return std::cmp::Ordering::Less;
                        } else {
                            return std::cmp::Ordering::Greater;
                        }
                    }
                    None => return std::cmp::Ordering::Equal,
                }
            });

            let middle = i.len() / 2;
            *i.get(middle).unwrap() as u32
        })
        .sum::<u32>() as u32
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
        assert_eq!(part1(input), 143);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("test");
        assert_eq!(part2(input), 123);
    }
}
