use itertools::Itertools;

fn find_mirror(strings: &[String]) -> Option<u32> {
    let mut last_lines = Vec::new();

    for (i, line) in strings.iter().enumerate() {
        if last_lines.last() == Some(&line) {
            if last_lines.len() == 1 {
                return Some(i as u32);
            }
            let mut mirror = true;

            let mut last_lines_index = last_lines.len() - 2;
            for next_line in strings.iter().skip(i + 1) {
                if &next_line != &last_lines[last_lines_index] {
                    mirror = false;
                    break;
                }
                if last_lines_index == 0 {
                    break;
                }
                last_lines_index -= 1;
            }

            if mirror {
                return Some(i as u32);
            }
            last_lines.push(line);
        } else {
            last_lines.push(line);
        }
    }
    None
}

fn find_mirror_horizontal(input: &str) -> Option<u32> {
    let lines = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect_vec();

    find_mirror(&lines)
}

fn find_mirror_vertical(input: &str) -> Option<u32> {
    let mut columns = vec![String::new(); input.lines().next().unwrap().len()];
    for line in input.split("\n").filter(|line| !line.is_empty()) {
        for (i, c) in line.chars().enumerate() {
            columns[i].push(c);
        }
    }

    find_mirror(&columns)
}

fn part1(input: &str) -> u32 {
    //Group lines that are separated by a blank line
    let groups = input.split("\n\n").collect_vec();

    groups.iter().fold(0, |acc, group| {
        println!("Group: {:#?}", group.lines().collect_vec());
        acc + 100 * find_mirror_horizontal(group).unwrap_or(0)
            + find_mirror_vertical(group).unwrap_or(0)
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
        assert_eq!(part1(input), 405);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("test");
        assert_eq!(part2(input), 0);
    }

    #[test]
    fn test_part1_2() {
        let input = "..##.###..###.##.\n##..#..#..#..#..#\n###..#..##..#.###\n##..##########..#\n###.####..####.##\n..#...#.##.#...#.\n##..#.#....#.#..#\n..#.##.####.##.#.\n..##.########.##.";
        assert_eq!(part1(input), 1);
    }
}
