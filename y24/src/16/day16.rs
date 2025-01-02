use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Start,
    End,
}

impl FromStr for Tile {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Tile::Empty),
            "#" => Ok(Tile::Wall),
            "S" => Ok(Tile::Start),
            "E" => Ok(Tile::End),
            _ => Err(()),
        }
    }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl FromStr for Map {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles: Vec<Vec<Tile>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse().unwrap())
                    .collect()
            })
            .collect();

        let (start, end) = tiles
            .iter()
            .enumerate()
            .fold((None, None), |(start, end), (y, row)| {
                if let Some(x) = row.iter().position(|t| *t == Tile::Start) {
                    (Some((x, y)), end)
                } else if let Some(x) = row.iter().position(|t| *t == Tile::End) {
                    (start, Some((x, y)))
                } else {
                    (start, end)
                }
            });

        Ok(Map {
            tiles,
            start: start.unwrap(),
            end: end.unwrap(),
        })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.tiles.iter() {
            for tile in row.iter() {
                let c = match tile {
                    Tile::Empty => '.',
                    Tile::Wall => '#',
                    Tile::Start => 'S',
                    Tile::End => 'E',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

fn turn_cost(direction: &Direction, new_direction: &Direction) -> usize {
    use Direction::*;
    match (direction, new_direction) {
        (direction, new_direction) if *direction == *new_direction => 0,
        (North, South) | (South, North) | (West, East) | (East, West) => 2000,
        (North, East) | (East, North) | (South, West) | (West, South) => 1000,
        (North, West) | (West, North) | (South, East) | (East, South) => 1000,
        _ => {
            println!("{:?} -> {:?}", direction, new_direction);

            panic!("Invalid direction cost");
        }
    }
}

impl Map {
    fn shortest_path_cost(&self) -> u32 {
        let mut shortest_path_cost = 0;

        let mut queue = Vec::new();

        queue.push((self.start, 0, Direction::East));
        let mut visited = vec![vec![false; self.tiles[0].len()]; self.tiles.len()];

        while let Some(((x, y), cost, direction)) = queue.pop() {
            if (x, y) == self.end {
                shortest_path_cost = cost;
                break;
            }
            if self.tiles[y][x] == Tile::Wall || visited[y][x] {
                continue;
            }
            visited[y][x] = true;
            if x > 0 {
                let new_cost = cost + turn_cost(&direction, &Direction::West);
                queue.push(((x - 1, y), new_cost + 1, Direction::West));
            }
            if y > 0 {
                let new_cost = cost + turn_cost(&direction, &Direction::North);
                queue.push(((x, y - 1), new_cost + 1, Direction::North));
            }
            if x < self.tiles[0].len() - 1 {
                let new_cost = cost + turn_cost(&direction, &Direction::East);
                queue.push(((x + 1, y), new_cost + 1, Direction::East));
            }
            if y < self.tiles.len() - 1 {
                let new_cost = cost + turn_cost(&direction, &Direction::South);
                queue.push(((x, y + 1), new_cost + 1, Direction::South));
            }
            queue.sort_by(|a, b| b.1.cmp(&a.1));
        }
        shortest_path_cost as u32
    }

    fn shortest_path(&self) -> u32 {
        let mut shortest_path_cost = usize::MAX;
        let mut on_any_shortest_path: HashSet<(usize, usize)> = HashSet::new();

        let mut queue = Vec::new();
        let visited = Vec::new();
        let mut cache: HashMap<((usize, usize), Direction), (usize, Vec<(usize, usize)>)> =
            HashMap::new();

        queue.push((self.start, 0, Direction::East, visited));
        while let Some(((x, y), cost, direction, mut visited)) = queue.pop() {
            if visited.contains(&(x, y)) {
                continue;
            }

            if cost > shortest_path_cost {
                break;
            }

            let mut found_alternate_path = false;
            if let Some((cached_cost, cached_visited)) = cache.get_mut(&((x, y), direction.clone()))
            {
                if cost > *cached_cost {
                    continue;
                }

                found_alternate_path = true;
                visited.extend(cached_visited.iter());
            }
            cache.insert(((x, y), direction.clone()), (cost, visited.clone()));

            visited.push((x, y));
            if (x, y) == self.end {
                shortest_path_cost = cost;
                visited.iter().for_each(|(x, y)| {
                    on_any_shortest_path.insert((*x, *y));
                });
                continue;
            }

            if x > 0 && Direction::East != direction && self.tiles[y][x - 1] != Tile::Wall {
                let new_cost = cost + turn_cost(&direction, &Direction::West);
                queue.push(((x - 1, y), new_cost + 1, Direction::West, visited.clone()));
            }
            if y > 0 && Direction::South != direction && self.tiles[y - 1][x] != Tile::Wall {
                let new_cost = cost + turn_cost(&direction, &Direction::North);
                queue.push(((x, y - 1), new_cost + 1, Direction::North, visited.clone()));
            }
            if x < self.tiles[0].len() - 1
                && Direction::West != direction
                && self.tiles[y][x + 1] != Tile::Wall
            {
                let new_cost = cost + turn_cost(&direction, &Direction::East);
                queue.push(((x + 1, y), new_cost + 1, Direction::East, visited.clone()));
            }
            if y < self.tiles.len() - 1
                && Direction::North != direction
                && self.tiles[y + 1][x] != Tile::Wall
            {
                let new_cost = cost + turn_cost(&direction, &Direction::South);
                queue.push(((x, y + 1), new_cost + 1, Direction::South, visited.clone()));
            }

            if found_alternate_path {
                //remove duplicates in the queue where pos, cost and direction are equal.
                //keep the one with most visited
                let filtered_queue = queue
                    .clone()
                    .into_iter()
                    .filter(|(pos, cost, dir, visited)| {
                        let visited_len = visited.len();
                        !queue.iter().any(|(p, c, d, v)| {
                            p == pos && c == cost && d == dir && visited_len < v.len()
                        })
                    })
                    .collect::<Vec<_>>();
                queue = filtered_queue;
            }

            queue.sort_by(|a, b| b.1.cmp(&a.1));
        }
        on_any_shortest_path.len() as u32
    }
}

fn part1(input: &str) -> u32 {
    let map = input.parse::<Map>().unwrap();

    map.shortest_path_cost()
}

fn part2(input: &str) -> u32 {
    let map = input.parse::<Map>().unwrap();
    map.shortest_path()
}

fn main() {
    println!("AoC 2024 - Day 16");
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
        assert_eq!(part1(input), 7036);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("test");
        assert_eq!(part2(input), 45);
    }
}
