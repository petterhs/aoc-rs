fn find_next_number(numbers: &[i32]) -> i32 {
    if numbers.iter().all(|&n| n == 0) {
        return 0;
    }

    let diff = numbers
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect::<Vec<i32>>();

    return numbers.iter().last().unwrap() + find_next_number(&diff);
}

fn find_prev_number(numbers: &[i32]) -> i32 {
    if numbers.iter().all(|&n| n == 0) {
        return 0;
    }

    let diff = numbers
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect::<Vec<i32>>();

    return numbers.iter().next().unwrap() - find_prev_number(&diff);
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            find_next_number(
                &line
                    .split(" ")
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
            )
        })
        .sum::<i32>()
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            find_prev_number(
                &line
                    .split(" ")
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
            )
        })
        .sum::<i32>()
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
        assert_eq!(part1(input), 114);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("test");
        assert_eq!(part2(input), 2);
    }
}
