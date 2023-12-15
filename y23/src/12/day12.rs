use std::collections::HashMap;
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, PartialEq)]
enum SpringCondition {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for SpringCondition {
    fn from(c: char) -> Self {
        match c {
            '.' => SpringCondition::Operational,
            '#' => SpringCondition::Damaged,
            '?' => SpringCondition::Unknown,
            _ => panic!("Invalid spring condition"),
        }
    }
}

impl Display for SpringCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpringCondition::Operational => write!(f, "."),
            SpringCondition::Damaged => write!(f, "#"),
            SpringCondition::Unknown => write!(f, "?"),
        }
    }
}

#[derive(Debug, Clone)]
struct SpringGroup {
    num: usize,
    min_following_positions: usize,
}

#[derive(Debug, Clone)]
struct Springs {
    conditions: Vec<SpringCondition>,
    groups: Vec<SpringGroup>,
}

impl Springs {
    fn recalculate_groups(&mut self) {
        let mut min_following_positions = 0;

        for group in self.groups.iter_mut().rev() {
            group.min_following_positions = min_following_positions;
            min_following_positions += group.num + 1;
        }
    }
}

impl FromStr for Springs {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut conditions = Vec::new();
        let mut words = s.split_whitespace();

        let springs = words.next().unwrap();
        let groups = words.next().unwrap();

        for c in springs.chars() {
            conditions.push(c.into());
        }

        let mut min_following_positions = 0;
        let groups: Vec<SpringGroup> = groups
            .split(',')
            .map(|g| g.parse::<usize>().unwrap())
            .rev()
            .map(|g| {
                let sg = SpringGroup {
                    num: g as usize,
                    min_following_positions,
                };
                min_following_positions += g + 1;
                sg
            })
            .collect();

        Ok(Springs {
            conditions,
            groups: groups.into_iter().rev().collect(),
        })
    }
}

impl Display for Springs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in &self.conditions {
            write!(f, "{}", c)?;
        }

        for g in &self.groups {
            write!(f, " {}", g.num)?;
        }
        Ok(())
    }
}

fn possible_permutations(
    conditions: &[SpringCondition],
    groups: &[SpringGroup],
    cache: &mut HashMap<(usize, usize, usize), usize>,
) -> Option<usize> {
    if let Some(group) = groups.first() {
        let min_following_positions = group.min_following_positions;
        //remove the n last conditions
        if conditions.len() < min_following_positions as usize {
            return None;
        }

        if conditions.len() < group.num {
            return None;
        }

        if let Some(permutations) =
            cache.get(&(conditions.len(), groups.len(), min_following_positions))
        {
            return Some(*permutations);
        }

        let mut permutations = 0;

        let max_steps = conditions.len() - min_following_positions as usize - group.num;

        //Take one spring condition at a time
        let mut candidates = conditions.windows(group.num).enumerate().peekable();
        while let Some((steps, window)) = candidates.next() {
            if steps > max_steps {
                break;
            }

            if window.first().unwrap() == &SpringCondition::Operational {
                continue;
            } else if window
                .iter()
                .all(|c| *c == SpringCondition::Damaged || *c == SpringCondition::Unknown)
            {
                //If the next position is a damaged spring, we cannot place a group there
                if let Some((_, next)) = candidates.peek() {
                    if next.last().unwrap() == &SpringCondition::Damaged {
                        if window.first().unwrap() == &SpringCondition::Damaged {
                            break;
                        }
                        continue;
                    }
                }
                if groups.len() == 1 {
                    //check that there are no more damaged springs after the group
                    let mut valid_permutation = true;
                    conditions[(steps + group.num)..].iter().for_each(|c| {
                        if c == &SpringCondition::Damaged {
                            valid_permutation = false;
                        }
                    });
                    if valid_permutation {
                        permutations += 1;
                    }
                } else {
                    if steps + group.num + 1 > conditions.len() {
                        continue;
                    }

                    let new_conditions = &conditions[(group.num + 1 + steps)..];

                    if let Some(perms) = possible_permutations(new_conditions, &groups[1..], cache)
                    {
                        permutations += perms;
                    }
                }
            }

            //Cannot place group after the first Operational spring
            if window.first().unwrap() == &SpringCondition::Damaged {
                break;
            }
        }

        cache.insert(
            (conditions.len(), groups.len(), min_following_positions),
            permutations,
        );
        Some(permutations)
    } else {
        return Some(1);
    }
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let springs = Springs::from_str(line).unwrap();
            let mut cache = HashMap::new();
            let permutations =
                possible_permutations(&springs.conditions, &springs.groups, &mut cache);
            permutations.expect("No possible permutations for line")
        })
        .fold(0, |acc, line| acc + line)
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .enumerate()
        .map(|(_, line)| {
            let springs = Springs::from_str(line).unwrap();

            let mut extended_springs = springs.clone();

            for _ in 0..4 {
                extended_springs.conditions.push(SpringCondition::Unknown);
                extended_springs
                    .conditions
                    .extend(springs.conditions.clone());

                extended_springs.groups.extend(springs.groups.clone());
            }

            extended_springs.recalculate_groups();

            let cache = &mut HashMap::new();

            let permutations = possible_permutations(
                &extended_springs.conditions,
                &extended_springs.groups,
                cache,
            );

            permutations.expect("No possible permutations for line")
        })
        .fold(0, |acc, line| acc + line)
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
    fn test_possible_permutations() {
        let input = "?? 1";
        let springs = Springs::from_str(input).unwrap();
        let mut cache = HashMap::new();

        let permutations = possible_permutations(&springs.conditions, &springs.groups, &mut cache);
        assert_eq!(permutations, Some(2));
    }
    #[test]
    fn test_possible_permutations_2() {
        let input = "#.# 1,1";
        let springs = Springs::from_str(input).unwrap();

        let permutations =
            possible_permutations(&springs.conditions, &springs.groups, &mut HashMap::new());
        assert_eq!(permutations, Some(1));
    }

    #[test]
    fn test_possible_permutations_3() {
        let input = "??..??...?##. 1,1,3";
        let springs = Springs::from_str(input).unwrap();
        let mut cache = HashMap::new();

        let permutations = possible_permutations(&springs.conditions, &springs.groups, &mut cache);
        assert_eq!(permutations, Some(4));
    }
    #[test]
    fn test_possible_permutations_4() {
        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        let springs = Springs::from_str(input).unwrap();
        let mut cache = HashMap::new();

        let permutations = possible_permutations(&springs.conditions, &springs.groups, &mut cache);
        assert_eq!(permutations, Some(1));
    }

    #[test]
    fn test_possible_permutations_5() {
        let input = "????.######..#####. 1,6,5";
        let springs = Springs::from_str(input).unwrap();
        let mut cache = HashMap::new();

        let permutations = possible_permutations(&springs.conditions, &springs.groups, &mut cache);
        assert_eq!(permutations, Some(4));
    }
    #[test]
    fn test_possible_permutations_6() {
        let input = "?###???????? 3,2,1";
        let springs = Springs::from_str(input).unwrap();

        let mut cache = HashMap::new();
        let permutations = possible_permutations(&springs.conditions, &springs.groups, &mut cache);
        assert_eq!(permutations, Some(10));
    }

    #[test]
    fn test_possible_permutations_7() {
        let input = "??.?# 1,1";
        let springs = Springs::from_str(input).unwrap();

        let mut cache = HashMap::new();
        let permutations = possible_permutations(&springs.conditions, &springs.groups, &mut cache);
        assert_eq!(permutations, Some(2));
    }

    #[test]
    fn test_possible_permutations_8() {
        let input = "???????# 1";
        let springs = Springs::from_str(input).unwrap();

        let mut cache = HashMap::new();
        let permutations = possible_permutations(&springs.conditions, &springs.groups, &mut cache);
        assert_eq!(permutations, Some(1));
    }

    #[test]
    fn test_possible_permutations_9() {
        let input = "#### 2";
        let springs = Springs::from_str(input).unwrap();

        let mut cache = HashMap::new();
        let permutations = possible_permutations(&springs.conditions, &springs.groups, &mut cache);
        assert_eq!(permutations, Some(0));
    }

    #[test]
    fn part1_test() {
        let input = include_str!("test");
        assert_eq!(part1(input), 21);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("test");
        assert_eq!(part2(input), 525152);
    }
}
