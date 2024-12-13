use std::collections::HashMap;

fn num_stones(stone: u64, blinks_left: u32, cache: &mut HashMap<(u64, u32), usize>) -> usize {
    if blinks_left == 0 {
        return 1;
    }

    if let Some(sum) = cache.get(&(stone, blinks_left)) {
        return *sum;
    }

    let sum = match stone {
        0 => num_stones(1, blinks_left - 1, cache),
        _ => {
            let string = stone.to_string();
            if string.len() % 2 == 0 {
                let stone1 = string[..string.len() / 2].parse().unwrap();
                let stone2 = string[string.len() / 2..].parse().unwrap_or_default();
                num_stones(stone1, blinks_left - 1, cache)
                    + num_stones(stone2, blinks_left - 1, cache)
            } else {
                if stone > u64::MAX / 2024 {
                    panic!("Overflow");
                }

                num_stones(stone * 2024, blinks_left - 1, cache)
            }
        }
    };
    cache.insert((stone, blinks_left), sum);
    sum
}

fn part1(input: &str) -> usize {
    let mut cache = HashMap::new();
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .map(|s| num_stones(s, 25, &mut cache))
        .sum()
}

fn part2(input: &str) -> usize {
    let mut cache = HashMap::new();
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .map(|s| num_stones(s, 75, &mut cache))
        .sum()
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
        assert_eq!(part1(input), 55312);
    }
}
