use std::collections::HashMap;
use std::fmt::{Display, Formatter};
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
    for expression in rule.expressions.iter() {
        let field = match expression.field {
            0 => rating.0,
            1 => rating.1,
            2 => rating.2,
            3 => rating.3,
            _ => panic!("Invalid field"),
        };

        if match &expression.condition {
            None => true,
            Some(Condition::LessThan(num)) if field < *num => true,
            Some(Condition::GreaterThan(num)) if field > *num => true,
            _ => false,
        } {
            if let RuleResult::Name(name) = &expression.result {
                let rule = rules.get(name).unwrap();
                return solve(rating, rule, rules);
            }
            return expression.result.clone();
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

#[derive(Debug, Clone, Copy)]
struct Range {
    min: u64,
    max: u64,
}

impl Range {
    fn len(&self) -> u64 {
        if self.min > self.max {
            panic!("Invalid range");
        }
        self.max - self.min + 1
    }
    fn is_valid(&self) -> bool {
        self.min <= self.max
    }

    fn decrease_max(&mut self, num: u64) {
        if self.max < num {
            panic!("Invalid range");
        }
        self.max = num;
    }

    fn increase_min(&mut self, num: u64) {
        if self.min > num {
            panic!("Invalid range");
        }
        self.min = num;
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.min, self.max)
    }
}

fn combinations(rule: &Rule, rules: &HashMap<String, Rule>, ranges: Vec<Range>) -> u64 {
    let mut sum = 0;
    let mut next_expression_ranges = ranges;
    for expression in rule.expressions.iter() {
        let mut next_rule_ranges = next_expression_ranges.clone();
        let range = next_rule_ranges[expression.field];

        if next_expression_ranges.iter().any(|range| !range.is_valid()) {
            println!("Invalid range");
            break;
        }

        //Modify the ranges
        match &expression.condition {
            Some(Condition::LessThan(num)) if range.min >= *num as u64 => {
                //Increase the min of the range for the next expression
                next_expression_ranges[expression.field].increase_min(*num as u64);
                continue; //Skip this expression
            }
            Some(Condition::GreaterThan(num)) if range.max <= *num as u64 => {
                //Decrease the max of the range for the next expression
                next_expression_ranges[expression.field].decrease_max(*num as u64);
                continue; //Skip this expression
            }
            Some(Condition::LessThan(num)) if range.max >= *num as u64 => {
                next_rule_ranges[expression.field].decrease_max(*num as u64 - 1);
                next_expression_ranges[expression.field].increase_min(*num as u64);
            }
            Some(Condition::GreaterThan(num)) if range.min <= *num as u64 => {
                next_rule_ranges[expression.field].increase_min(*num as u64 + 1);
                next_expression_ranges[expression.field].decrease_max(*num as u64);
            }
            None => {} //Use same range for next expression
            _ => continue,
        }
        match &expression.result {
            RuleResult::Name(name) => {
                let rule = rules.get(name).unwrap();
                sum += combinations(rule, rules, next_rule_ranges.clone());
            }
            RuleResult::Accepted => {
                sum += next_rule_ranges
                    .iter()
                    .fold(1, |acc, range| acc * range.len());
            }
            RuleResult::Rejected => {}
        }
    }

    sum
}

fn part2(input: &str) -> u64 {
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

    let ranges = vec![
        Range { min: 1, max: 4000 },
        Range { min: 1, max: 4000 },
        Range { min: 1, max: 4000 },
        Range { min: 1, max: 4000 },
    ];

    combinations(&rules.get("in").unwrap(), &rules, ranges)
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
        assert_eq!(part2(input), 167409079868000);
    }

    #[test]
    fn combinations_test() {
        let input = include_str!("test");
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

        let ranges = vec![
            Range { min: 1, max: 4000 },
            Range { min: 1, max: 4000 },
            Range { min: 1, max: 4000 },
            Range { min: 1, max: 4000 },
        ];

        let rule = rules.get("qkq").unwrap();
        let sum = combinations(rule, &rules, ranges);

        let expected_qkq = (1415 - 1 + 1) * 4000 * 4000 * 4000;
        println!("qkq_accept: {}", expected_qkq);
        let expected_crn = (4000 - 2663 + 1) * 4000 * 4000 * 4000;
        println!("crn: {}", expected_crn);
        println!("expected_sum: {}", expected_qkq + expected_crn);
        assert_eq!(sum, expected_qkq + expected_crn);
    }

    #[test]
    fn combinations_test_2() {
        let input = include_str!("test");
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

        let ranges = vec![
            Range { min: 1, max: 4000 },
            Range { min: 1, max: 4000 },
            Range { min: 1, max: 4000 },
            Range { min: 1, max: 4000 },
        ];

        let rule = rules.get("pv").unwrap();
        let sum = combinations(rule, &rules, ranges);

        let expected_pv = (1716) * 4000 * 4000 * 4000;
        println!("expectd_crn: {}", expected_pv);
        assert_eq!(sum, expected_pv);
    }
}
