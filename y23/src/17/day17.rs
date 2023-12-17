use std::collections::{BinaryHeap, HashMap};
use std::str::FromStr;

struct Grid(Vec<Vec<usize>>);

impl Grid {
    fn valid_pos(&self, pos: (i32, i32)) -> Option<(usize, usize)> {
        let (x, y) = pos;
        if x >= 0 && y >= 0 && x < self.0[0].len() as i32 && y < self.0.len() as i32 {
            Some((x as usize, y as usize))
        } else {
            None
        }
    }

    fn next_position(&self, pos: (usize, usize), direction: &Direction) -> Option<(usize, usize)> {
        let x = pos.0 as i32;
        let y = pos.1 as i32;
        match direction {
            Direction::Up => self.valid_pos((x, y - 1)),
            Direction::Down => self.valid_pos((x, y + 1)),
            Direction::Left => self.valid_pos((x - 1, y)),
            Direction::Right => self.valid_pos((x + 1, y)),
        }
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Vec::new();
        for line in s.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c.to_digit(10).unwrap() as usize);
            }
            grid.push(row);
        }
        Ok(Grid(grid))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn next_directions(direction: Direction, steps_in_direction: u32) -> Vec<Direction> {
    let mut directions = match direction {
        Direction::Up => vec![Direction::Left, Direction::Right],
        Direction::Down => vec![Direction::Left, Direction::Right],
        Direction::Left => vec![Direction::Up, Direction::Down],
        Direction::Right => vec![Direction::Up, Direction::Down],
    };

    if steps_in_direction < 3 {
        directions.push(direction);
    }

    directions
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct State {
    pos: (usize, usize),
    steps: usize,
    prev_direction: Direction,
    steps_in_direction: u32,
}

impl State {
    fn new(
        pos: (usize, usize),
        steps: usize,
        prev_direction: Direction,
        steps_in_direction: u32,
    ) -> Self {
        Self {
            pos,
            steps,
            prev_direction,
            steps_in_direction,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .steps
            .cmp(&self.steps)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

//TODO: Very slow, needs to be optimized
fn part1(input: &str) -> usize {
    let grid = Grid::from_str(input).unwrap();

    let start = State::new((0, 0), 0, Direction::Down, 0);
    let end = (grid.0.len() - 1, grid.0[0].len() - 1);

    //Find a path to use as a maximum to prune the search space
    let mut max = 0;
    let mut pos = (0, 0);
    for i in 0..grid.0.len() + grid.0[0].len() {
        if i % 2 == 0 {
            pos.1 += 1;
        } else {
            pos.0 += 1;
        }
        max += grid.0[pos.1][pos.0];

        if pos == end {
            break;
        }
    }
    println!("Max: {}", max);

    let mut heap = BinaryHeap::new();
    heap.push(start);

    let mut dist = HashMap::new();
    dist.insert(start, 0);

    while let Some(state) = heap.pop() {
        let State {
            pos,
            steps,
            prev_direction,
            steps_in_direction,
        } = state;
        if pos == end {
            return steps;
        }

        if let Some(d) = dist.get(&state) {
            if steps > *d && steps > max {
                continue;
            }
        }

        if steps % 10 == 0 {
            println!("{}: {:?}", steps, state);
        }

        let directions = next_directions(prev_direction, steps_in_direction);

        for next_direction in directions {
            if let Some(next_pos) = grid.next_position(pos, &next_direction) {
                let next_steps = steps + grid.0[next_pos.1][next_pos.0];

                let steps_in_direction = if prev_direction == next_direction {
                    steps_in_direction + 1
                } else {
                    1
                };

                let next_state =
                    State::new(next_pos, next_steps, next_direction, steps_in_direction);

                if next_steps < dist.get(&next_state).cloned().unwrap_or(usize::MAX)
                    && next_steps <= max
                {
                    heap.push(State::new(
                        next_pos,
                        next_steps,
                        next_direction,
                        steps_in_direction,
                    ));

                    dist.insert(next_state, next_steps);
                }
            }
        }
    }
    0
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
        assert_eq!(part1(input), 102);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("test");
        assert_eq!(part2(input), 0);
    }
}
