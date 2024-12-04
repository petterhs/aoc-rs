use std::collections::HashMap;
fn part1(input: &str) -> u32 {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    input.lines().for_each(|line| {
        let mut numbers = line.split("   ");
        left.push(numbers.next().unwrap().parse::<u32>().unwrap());
        right.push(numbers.next().unwrap().parse::<u32>().unwrap());
    });

    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .fold(0, |acc, (l, r)| acc + l.abs_diff(*r))
}

fn part2(input: &str) -> u32 {
    let mut occurences_right: HashMap<u32, u32> = HashMap::new();
    let mut left_numbers: Vec<u32> = Vec::new();

    input.lines().for_each(|line| {
        let mut numbers = line.split("   ");
        let left = numbers.next().unwrap().parse::<u32>().unwrap();
        let right = numbers.next().unwrap().parse::<u32>().unwrap();

        left_numbers.push(left);
        occurences_right.insert(right, occurences_right.get(&right).unwrap_or(&0) + 1);
    });

    left_numbers.iter().fold(0, |acc, l| {
        let r = occurences_right.get(l);

        match r {
            Some(occurences) => {
                if occurences > &0 {
                    acc + l * r.unwrap()
                } else {
                    acc
                }
            }
            None => acc,
        }
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
        assert_eq!(part1(input), 11);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("test");
        assert_eq!(part2(input), 31);
    }
}
