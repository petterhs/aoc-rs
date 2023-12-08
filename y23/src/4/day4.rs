fn part1(input: &str) -> u32 {
    input.lines().fold(0, |acc, line| {
        let mut words = line.split(&[':', '|']);

        let mut winning_numbers = Vec::new();
        let mut scratch_score = 0;

        for winning_num in words.nth(1).unwrap().split_whitespace() {
            // println!("{}", winning_num);
            winning_numbers.push(winning_num.parse::<u32>().unwrap());
        }

        for num in words.next().unwrap().split_whitespace() {
            if winning_numbers.contains(&num.parse::<u32>().unwrap()) {
                match scratch_score {
                    0 => scratch_score = 1,
                    _ => scratch_score *= 2,
                }
            }
        }

        acc + scratch_score
    })
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
        assert_eq!(part1(input), 13);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("test");
        assert_eq!(part2(input), 0);
    }
}
