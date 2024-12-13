use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::str::FromStr;

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<u32>>,
    x_size: u32,
    y_size: u32,
}

impl FromStr for Grid {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<u32>> = s
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        let x_size = grid[0].len() as u32;
        let y_size = grid.len() as u32;
        Ok(Self {
            grid,
            x_size,
            y_size,
        })
    }
}

fn path_reacable_tops(
    grid: &Grid,
    pos: (u32, u32),
    reachable_tops: &mut HashSet<(u32, u32)>,
    cache: &mut HashMap<(u32, u32), u32>,
) -> u32 {
    let (x, y) = pos;
    if let Some(&num_paths) = cache.get(&pos) {
        return num_paths;
    }

    let height = grid.grid[y as usize][x as usize];
    if height == 9 {
        reachable_tops.insert(pos);
        return 1;
    }

    let mut num_paths = 0;
    if x > 0 && grid.grid[y as usize][(x - 1) as usize] == height + 1 {
        num_paths += path_reacable_tops(grid, (x - 1, y), reachable_tops, cache);
    }

    if x < grid.x_size - 1 && grid.grid[y as usize][(x + 1) as usize] == height + 1 {
        num_paths += path_reacable_tops(grid, (x + 1, y), reachable_tops, cache);
    }

    if y > 0 && grid.grid[(y - 1) as usize][x as usize] == height + 1 {
        num_paths += path_reacable_tops(grid, (x, y - 1), reachable_tops, cache);
    }

    if y < grid.y_size - 1 && grid.grid[(y + 1) as usize][x as usize] == height + 1 {
        num_paths += path_reacable_tops(grid, (x, y + 1), reachable_tops, cache);
    }
    cache.insert(pos, num_paths);
    num_paths
}

fn part1(input: &str) -> u32 {
    let grid = Grid::from_str(input).unwrap();
    let mut cache = HashMap::new();
    let mut sum = 0;
    for x in 0..grid.x_size {
        for y in 0..grid.y_size {
            if grid.grid[y as usize][x as usize] == 0 {
                let mut reachable_tops = HashSet::new();
                path_reacable_tops(&grid, (x, y), &mut reachable_tops, &mut cache);
                sum += reachable_tops.len() as u32;
            }
        }
    }
    sum
}

fn part2(input: &str) -> u32 {
    let grid = Grid::from_str(input).unwrap();
    let mut cache = HashMap::new();
    let mut sum = 0;
    for x in 0..grid.x_size {
        for y in 0..grid.y_size {
            if grid.grid[y as usize][x as usize] == 0 {
                let mut reachable_tops = HashSet::new();
                sum += path_reacable_tops(&grid, (x, y), &mut reachable_tops, &mut cache);
            }
        }
    }
    sum
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
        assert_eq!(part1(input), 36);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("test");
        assert_eq!(part2(input), 81);
    }
}
