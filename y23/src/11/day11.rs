use std::collections::HashSet;

#[derive(Debug, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

fn distance(a: &Position, b: &Position) -> usize {
    (a.x as isize - b.x as isize).abs() as usize + (a.y as isize - b.y as isize).abs() as usize
}

fn parse_input(input: &str) -> (Vec<Position>, HashSet<usize>, HashSet<usize>) {
    let mut galaxies = Vec::new();
    let mut rows = HashSet::new();
    let mut cols = HashSet::new();
    (0..input.lines().count()).for_each(|i| {
        rows.insert(i);
    });

    (0..input.lines().next().unwrap().chars().count()).for_each(|i| {
        cols.insert(i);
    });

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| match c {
            '#' => {
                galaxies.push(Position::new(x, y));
                rows.remove(&y);
                cols.remove(&x);
            }
            _ => {}
        });
    });

    (galaxies, rows, cols)
}

fn expand_galaxies(
    galaxies: &mut Vec<Position>,
    rows: &HashSet<usize>,
    cols: &HashSet<usize>,
    expansion: usize,
) {
    for galaxy in galaxies.iter_mut() {
        let galaxy_x = galaxy.x;
        let galaxy_y = galaxy.y;

        //Expand to the right
        for x in cols.iter() {
            if x < &galaxy_x {
                galaxy.x += expansion;
            }
        }

        //Expand down
        for y in rows.iter() {
            if y < &galaxy_y {
                galaxy.y += expansion;
            }
        }
    }
}

fn part1(input: &str) -> usize {
    let (mut galaxies, rows, cols) = parse_input(input);

    expand_galaxies(&mut galaxies, &rows, &cols, 1);

    //Distance between all galaxies
    let mut distances = Vec::new();
    for (i, galaxy) in galaxies.iter().enumerate() {
        for (j, other) in galaxies.iter().enumerate() {
            if i != j {
                distances.push(distance(galaxy, other));
            }
        }
    }

    distances.iter().sum::<usize>() / 2
}

fn part2(input: &str, expansion: usize) -> usize {
    let (mut galaxies, rows, cols) = parse_input(input);

    expand_galaxies(&mut galaxies, &rows, &cols, expansion);

    //Distance between all galaxies
    let mut distances = Vec::new();
    for (i, galaxy) in galaxies.iter().enumerate() {
        for (j, other) in galaxies.iter().enumerate() {
            if i != j {
                distances.push(distance(galaxy, other));
            }
        }
    }

    distances.iter().sum::<usize>() / 2
}

fn main() {
    println!("AoC 2023 - Day 1");
    let input = include_str!("input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input, 1000000 - 1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("test");
        assert_eq!(part1(input), 374);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("test");
        assert_eq!(part2(input, 9), 1030);
        assert_eq!(part2(input, 99), 8410);
    }
}
