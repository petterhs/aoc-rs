use std::collections::{HashMap, HashSet};

fn get_antinodes(antenna: (i32, i32), other_antennas: &[(i32, i32)]) -> Vec<(i32, i32)> {
    if other_antennas.len() == 0 {
        return vec![];
    }
    let mut canditates = vec![];
    for other_antenna in other_antennas {
        canditates.push((
            2 * antenna.0 - other_antenna.0,
            2 * antenna.1 - other_antenna.1,
        ));
        canditates.push((
            2 * other_antenna.0 - antenna.0,
            2 * other_antenna.1 - antenna.1,
        ));
    }

    canditates.extend(get_antinodes(other_antennas[0], &other_antennas[1..]));
    canditates
}

fn get_antinodes2(
    antenna: (i32, i32),
    other_antennas: &[(i32, i32)],
    x_max: i32,
    y_max: i32,
) -> Vec<(i32, i32)> {
    if other_antennas.len() == 0 {
        return vec![];
    }
    let mut canditates = vec![];
    for other_antenna in other_antennas {
        let diff_x = antenna.0 - other_antenna.0;
        let diff_y = antenna.1 - other_antenna.1;

        let mut x = antenna.0;
        let mut y = antenna.1;
        while x >= 0 && x < x_max && y >= 0 && y < y_max {
            canditates.push((x, y));
            x += diff_x;
            y += diff_y;
        }
        x = other_antenna.0;
        y = other_antenna.1;
        while x >= 0 && x < x_max && y >= 0 && y < y_max {
            canditates.push((x, y));
            x -= diff_x;
            y -= diff_y;
        }
    }

    canditates.extend(get_antinodes2(
        other_antennas[0],
        &other_antennas[1..],
        x_max,
        y_max,
    ));

    canditates
}

fn part1(input: &str) -> u32 {
    let mut frequency_antennas = HashMap::<char, Vec<(i32, i32)>>::new();

    let y_max = input.lines().count() as i32;
    let x_max = input.lines().next().unwrap().chars().count() as i32;

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c != '.' {
                let frequency = c;
                let antenna = (x as i32, y as i32);
                let antennas = frequency_antennas.entry(frequency).or_insert(vec![]);
                antennas.push(antenna);
            }
        });
    });

    let mut antinodes = HashSet::<(i32, i32)>::new();
    let mut candidates = vec![];

    for ele in frequency_antennas.iter() {
        let (_frequency, antennas) = ele;
        candidates.extend(
            get_antinodes(antennas[0], &antennas[1..])
                .into_iter()
                .filter(|(x, y)| *x >= 0 && *x < x_max && *y >= 0 && *y < y_max),
        );
    }

    for candidate in candidates {
        antinodes.insert(candidate);
    }

    antinodes.len() as u32
}

fn part2(input: &str) -> u32 {
    let mut frequency_antennas = HashMap::<char, Vec<(i32, i32)>>::new();

    let y_max = input.lines().count() as i32;
    let x_max = input.lines().next().unwrap().chars().count() as i32;

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c != '.' {
                let antenna = (x as i32, y as i32);
                let antennas = frequency_antennas.entry(c).or_insert(vec![]);
                antennas.push(antenna);
            }
        });
    });

    let mut antinodes = HashSet::<(i32, i32)>::new();
    let mut candidates = vec![];

    for ele in frequency_antennas.iter() {
        let (_frequency, antennas) = ele;
        candidates.extend(
            get_antinodes2(antennas[0], &antennas[1..], x_max, y_max)
                .into_iter()
                .filter(|(x, y)| *x >= 0 && *x < x_max && *y >= 0 && *y < y_max),
        );
    }

    for candidate in candidates {
        antinodes.insert(candidate);
    }

    antinodes.len() as u32
}

fn main() {
    println!("AoC 2024 - Day 8");
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
        assert_eq!(part1(input), 14);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("test");
        assert_eq!(part2(input), 34);
    }
}
