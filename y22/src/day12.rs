use std::collections::{HashSet, VecDeque};
#[derive(PartialEq, PartialOrd, Debug)]
struct Elevation(char);

impl Elevation {
    fn increment(&self) -> Elevation {
        Elevation((self.0 as u8 + 1).into())
    }
}

fn parse_input(input: &str) -> Vec<Vec<Elevation>> {
    let mut map = Vec::new();

    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(Elevation(c));
        }
        map.push(row);
    }
    map
}

fn valid_move(from: &Elevation, to: &Elevation) -> bool {
    if to.0 == 'E' {
        return valid_move(from, &Elevation('z'));
    }
    if from.0 == 'S' {
        return true;
    }
    if to <= from || from == to || &from.increment() == to {
        return true;
    }
    false
}

fn bfs(map: &Vec<Vec<Elevation>>, start: Elevation, goal: Elevation, is_reverse: bool) -> i32 {
    // Find S in the map
    let mut start_x = 0;
    let mut start_y = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, elevation) in row.iter().enumerate() {
            if elevation == &start {
                start_x = x;
                start_y = y;
            }
        }
    }

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((start_x, start_y, 0));
    while let Some((x, y, steps)) = queue.pop_front() {
        if visited.contains(&(x, y)) {
            continue;
        }
        let elevation = &map[y][x];
        if elevation == &goal {
            return steps;
        }
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
            let new_x = (x as i32 + dx) as usize;
            let new_y = (y as i32 + dy) as usize;

            if new_x < map[0].len() && new_y < map.len() {
                let new_elevation = &map[new_y][new_x];
                if is_reverse && valid_move(new_elevation, elevation) {
                    queue.push_back((new_x, new_y, steps + 1));
                } else if !is_reverse && valid_move(elevation, new_elevation) {
                    queue.push_back((new_x, new_y, steps + 1));
                }
            }
        }
        visited.insert((x, y));
    }
    return 0;
}

fn part1() {
    println!("Part 1");

    let input = include_str!("../input/12");
    let map = parse_input(input);

    let steps = bfs(&map, Elevation('S'), Elevation('E'), false);
    println!("Steps: {}", steps);
}

fn part2() {
    println!("Part 2");

    let input = include_str!("../input/12");
    let map = parse_input(input);

    let steps = bfs(&map, Elevation('E'), Elevation('a'), true);
    println!("Steps: {}", steps);
}

pub fn run() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment() {
        let a = Elevation('a');
        assert_eq!(a.0, 'a');
        assert_eq!(a.increment().0, 'b');
    }

    #[test]
    fn test_valid_move() {
        let a = Elevation('a');
        let b = Elevation('b');
        let c = Elevation('c');
        assert_eq!(valid_move(&a, &b), true);
        assert_eq!(valid_move(&a, &a), true);
        assert_eq!(valid_move(&a, &c), false);
    }

    #[test]
    fn test_parse_input() {
        let input = include_str!("../input/test12");
        let map = parse_input(input);

        assert_eq!(map[0][0], Elevation('S'));
        assert_eq!(map[0][3], Elevation('q'));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input/test12");
        let map = parse_input(input);

        assert_eq!(bfs(&map, Elevation('S'), Elevation('E'), false), 31);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input/test12");
        let map = parse_input(input);

        assert_eq!(bfs(&map, Elevation('E'), Elevation('a'), true), 29);
    }
}
