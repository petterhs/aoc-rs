enum Operation {
    Add,
    Multiply,
    Concatenate,
}

fn test_operation(goal: u64, lhs: u64, rest: &[u64]) -> bool {
    if goal == lhs {
        return true;
    }

    if rest.len() == 0 {
        return false;
    }

    if test_operation(goal, lhs + rest[0], &rest[1..]) {
        return true;
    }

    if test_operation(goal, lhs * rest[0], &rest[1..]) {
        return true;
    }

    return false;
}

fn test_operation_2(goal: u64, lhs: u64, rest: &[u64]) -> bool {
    if lhs > goal {
        return false;
    }

    if rest.len() == 0 {
        if lhs == goal {
            return true;
        }
        return false;
    }

    if test_operation_2(goal, lhs + rest[0], &rest[1..]) {
        return true;
    }

    if test_operation_2(goal, lhs * rest[0], &rest[1..]) {
        return true;
    }

    if test_operation_2(
        goal,
        (lhs.to_string() + &rest[0].to_string())
            .parse::<u64>()
            .unwrap(),
        &rest[1..],
    ) {
        return true;
    }

    return false;
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let split = line.split(":").collect::<Vec<&str>>();

            let test_value = split[0].trim().parse::<u64>().unwrap();
            let numbers = split[1]
                .trim()
                .split(" ")
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            if test_operation(test_value, numbers[0], &numbers[1..]) {
                return test_value;
            }

            return 0;
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let split = line.split(":").collect::<Vec<&str>>();

            let test_value = split[0].trim().parse::<u64>().unwrap();
            let numbers = split[1]
                .trim()
                .split(" ")
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            if test_operation_2(test_value, numbers[0], &numbers[1..]) {
                return test_value;
            }

            return 0;
        })
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
        assert_eq!(part1(input), 3749);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("test");
        assert_eq!(part2(input), 11387);
    }
}
