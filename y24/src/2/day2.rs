fn part1(input: &str) -> u32 {
    input.lines().fold(0, |acc, line| {
        let line_length = line.split_whitespace().count();
        if line_length == 0 {
            return acc;
        }
        if line_length == 1 {
            println!("Safe");
            return acc + 1;
        }

        let safe = line
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
            .windows(2)
            .fold(0, |acc, window| {
                if window[0].abs_diff(window[1]) > 3 {
                    return 0;
                } //Strictly descending
                if window[0] > window[1] {
                    acc + 1
                } else {
                    0
                }
            });

        if safe == line_length - 1 {
            println!("Safe");
            return acc + 1;
        }

        let safe = line
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
            .windows(2)
            .fold(0, |acc, window| {
                if window[0].abs_diff(window[1]) > 3 {
                    return 0;
                } //Strictly increasing
                if window[0] < window[1] {
                    acc + 1
                } else {
                    0
                }
            });

        if safe == line_length - 1 {
            println!("Safe");
            acc + 1
        } else {
            acc
        }
    })
}

fn check_descending(line: Vec<u32>) -> u32 {
    let mut pos = 0;
    let mut first_unsafe_pos = 0;
    let safe = line.windows(2).fold(0, |acc, window| {
        pos += 1;
        if window[0].abs_diff(window[1]) > 3 {
            if first_unsafe_pos == 0 {
                first_unsafe_pos = pos;
            }
            return 0;
        } //Strictly descending
        if window[0] > window[1] {
            acc + 1
        } else {
            if first_unsafe_pos == 0 {
                first_unsafe_pos = pos;
            }
            0
        }
    });
    if safe == line.len() - 1 {
        return 0;
    }
    return first_unsafe_pos;
}

fn check_increasing(line: Vec<u32>) -> u32 {
    let mut pos = 0;
    let mut first_unsafe_pos = 0;
    let safe = line.windows(2).fold(0, |acc, window| {
        pos += 1;
        if window[0].abs_diff(window[1]) > 3 {
            if first_unsafe_pos == 0 {
                first_unsafe_pos = pos;
            }
            return 0;
        } //Strictly descending
        if window[0] < window[1] {
            acc + 1
        } else {
            if first_unsafe_pos == 0 {
                first_unsafe_pos = pos;
            }
            0
        }
    });
    if safe == line.len() - 1 {
        return 0;
    }
    return first_unsafe_pos;
}

fn part2(input: &str) -> u32 {
    input.lines().fold(0, |acc, line| {
        let line_length = line.split_whitespace().count();
        if line_length == 0 {
            return acc;
        }
        if line_length <= 2 {
            println!("Safe");
            return acc + 1;
        }

        let levels = line
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let wrong_pos = check_descending(levels.clone());
        if wrong_pos == 0 {
            return acc + 1;
        }
        let wrong_pos_2 = check_increasing(levels.clone());
        if wrong_pos_2 == 0 {
            return acc + 1;
        }

        println!("{:?}", levels);
        println!("{:?}", wrong_pos);
        let mut modifiedlevels = levels.clone();
        modifiedlevels.remove(wrong_pos as usize - 1);

        let mut modifiedlevels2 = levels.clone();
        modifiedlevels2.remove(wrong_pos as usize);

        println!("mod: {:?}", modifiedlevels);

        if check_descending(modifiedlevels.clone()) == 0 {
            println!("Safe descending");
            return acc + 1;
        }

        if check_descending(modifiedlevels2.clone()) == 0 {
            println!("Safe descending");
            return acc + 1;
        }

        println!("{:?}", wrong_pos_2);
        let mut modifiedlevels = levels.clone();
        modifiedlevels.remove(wrong_pos_2 as usize - 1);

        let mut modifiedlevels2 = levels.clone();
        modifiedlevels2.remove(wrong_pos_2 as usize);

        println!("mod{:?}", modifiedlevels);

        if check_increasing(modifiedlevels) == 0 {
            println!("Safe increasing");
            return acc + 1;
        }

        let mut modifiedlevels2 = levels.clone();
        modifiedlevels2.remove(wrong_pos_2 as usize);
        println!("mod{:?}", modifiedlevels2);

        if check_increasing(modifiedlevels2) == 0 {
            println!("Safe increasing");
            return acc + 1;
        }

        return acc;
    })
}

fn main() {
    println!("AoC 2024 - Day 2");
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
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("test");
        assert_eq!(part2(input), 4);
    }
}
