fn part1(input: &str) -> u32 {
    input.lines().fold(0, |acc, line| {
        let digits = line
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<Vec<_>>();

        let line_num = digits.first().unwrap().to_string() + &digits.last().unwrap().to_string();

        acc + line_num.parse::<u32>().unwrap()
    })
}

fn part2(input: &str) -> u32 {
    let numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    input.lines().fold(0, |acc, line| {
        let mut line_output = String::new();
        //prepend and append spaces
        let chars = "    "
            .chars()
            .chain(line.chars())
            .chain("    ".chars())
            .collect::<Vec<_>>();

        //Find first digit
        'outer: for w in chars.windows(5) {
            if w.first().unwrap().is_ascii_digit() {
                line_output.push(*w.first().unwrap());
                break;
            }

            let w = w.iter().collect::<String>();

            for (i, number) in numbers.iter().enumerate() {
                if w.starts_with(number) {
                    line_output.push((i + 1).to_string().chars().next().unwrap());
                    break 'outer;
                }
            }
        }

        //Find last digit
        'outer: for w in chars.windows(5).rev() {
            if w.last().unwrap().is_ascii_digit() {
                line_output.push(*w.last().unwrap());
                break;
            }

            let w = w.iter().collect::<String>();

            for (i, number) in numbers.iter().enumerate() {
                if w.ends_with(number) {
                    line_output.push((i + 1).to_string().chars().next().unwrap());
                    break 'outer;
                }
            }
        }

        acc + line_output.parse::<u32>().unwrap()
    })
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
        assert_eq!(part1(input), 142);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("test2");
        assert_eq!(part2(input), 281);
    }
}
