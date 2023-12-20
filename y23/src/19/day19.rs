use std::collections::HashMap;
use std::str::FromStr;

enum Field {
    X = 0,
    M = 1,
    A = 2,
    S = 3,
}
impl From<char> for Field {
    fn from(c: char) -> Self {
        match c {
            'x' => Field::X,
            'm' => Field::M,
            'a' => Field::A,
            's' => Field::S,
            _ => panic!("Invalid field"),
        }
    }
}

#[derive(Debug)]
enum Condition {
    LessThan(usize),
    GreaterThan(usize),
}

impl FromStr for Condition {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num = s.split_at(2).1;
        let num = num.parse().unwrap();
        match s.chars().nth(1).unwrap() {
            '<' => Ok(Condition::LessThan(num)),
            '>' => Ok(Condition::GreaterThan(num)),
            _ => panic!("Invalid condition"),
        }
    }
}

#[derive(Debug)]
struct Expression {
    field: usize,
    condition: Option<Condition>,
    result: RuleResult,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum RuleResult {
    Name(String),
    Accepted,
    Rejected,
}

impl FromStr for RuleResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(RuleResult::Accepted),
            "R" => Ok(RuleResult::Rejected),
            _ => Ok(RuleResult::Name(s.to_string())),
        }
    }
}

impl FromStr for Expression {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':');

        if parts.clone().count() == 1 {
            return Ok(Expression {
                field: 0,
                condition: None,
                result: parts.next().unwrap().to_string().parse().unwrap(),
            });
        }

        let condition = parts.next().unwrap();
        let field = Field::from(condition.chars().nth(0).unwrap()) as usize;
        let condition = Some(condition.parse().unwrap());
        let result = parts.next().unwrap().to_string().parse().unwrap();

        Ok(Expression {
            field,
            condition,
            result,
        })
    }
}

#[derive(Debug)]
struct Rule {
    name: String,
    expressions: Vec<Expression>,
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let expressions = s.split("{").collect::<Vec<_>>();

        let name = expressions[0].to_string();
        let expressions = expressions[1]
            .trim_end_matches("}")
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>();

        Ok(Rule { name, expressions })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PartRating(usize, usize, usize, usize);
impl PartRating {
    fn sum(&self) -> usize {
        self.0 + self.1 + self.2 + self.3
    }
}
impl FromStr for PartRating {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields = s.split(',').collect::<Vec<_>>();
        let x = fields[0]
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse()
            .unwrap();
        let m = fields[1]
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse()
            .unwrap();

        let a = fields[2]
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse()
            .unwrap();

        let s = fields[3]
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse()
            .unwrap();

        Ok(PartRating(x, m, a, s))
    }
}

fn solve(rating: &PartRating, rule: &Rule, rules: &HashMap<String, Rule>) -> RuleResult {
    println!("Solving {:?} : rule: {}", rating, rule.name);
    for expression in rule.expressions.iter() {
        println!("   {:?}", expression);
        let field = match expression.field {
            0 => rating.0,
            1 => rating.1,
            2 => rating.2,
            3 => rating.3,
            _ => panic!("Invalid field"),
        };

        let result = expression.result.clone();
        println!("   {:?} : {:?}", field, result);

        if expression.condition.is_none() {
            if let RuleResult::Name(name) = &expression.result {
                let rule = rules.get(name).unwrap();
                return solve(rating, rule, rules);
            }
            return result;
        }

        if let Some(condition) = &expression.condition {
            match condition {
                Condition::LessThan(num) if field < *num => match &expression.result {
                    RuleResult::Name(name) => {
                        let rule = rules.get(name).unwrap();
                        return solve(rating, rule, rules);
                    }
                    _ => return result,
                },
                Condition::GreaterThan(num) if field > *num => match &expression.result {
                    RuleResult::Name(name) => {
                        let rule = rules.get(name).unwrap();
                        return solve(rating, rule, rules);
                    }
                    _ => return result,
                },
                _ => {
                    println!(" failed  {:?} : {:?}", field, result);
                }
            }
        }
    }
    RuleResult::Rejected
}

fn part1(input: &str) -> u32 {
    let mut input = input.split("\n\n");

    let rules = input
        .next()
        .expect("No rules")
        .lines()
        .map(|line| {
            let line = line.parse::<Rule>().unwrap();
            (line.name.clone(), line)
        })
        .collect::<HashMap<_, _>>();

    let ratings = input
        .next()
        .expect("No ratings")
        .lines()
        .map(|line| line.parse::<PartRating>().unwrap())
        .collect::<Vec<_>>();

    let mut acc = 0;
    ratings.iter().for_each(|rating| {
        let rule = rules.get("in").unwrap();
        if RuleResult::Accepted == solve(rating, rule, &rules) {
            acc += rating.sum() as u32;
        }
    });
    acc
}

fn part2(input: &str) -> u32 {
    0
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
        assert_eq!(part1(input), 19114);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("test");
        assert_eq!(part2(input), 0);
    }
}
