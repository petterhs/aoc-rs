fn next_pipe(pos: (usize, usize), prev_pos: (usize, usize), pipe: char) -> Option<(usize, usize)> {
    println!("{:?} {:?} {}", pos, prev_pos, pipe);
    let (x, y) = pos;

    let delta_pos = (
        pos.0 as i32 - prev_pos.0 as i32,
        pos.1 as i32 - prev_pos.1 as i32,
    );

    match (pipe, delta_pos) {
        ('|', (0, 1)) => Some((x, y + 1)),
        ('|', (0, -1)) => Some((x, y - 1)),
        ('-', (1, 0)) => Some((x + 1, y)),
        ('-', (-1, 0)) => Some((x - 1, y)),
        ('L', (-1, 0)) => Some((x, y - 1)),
        ('L', (0, 1)) => Some((x + 1, y)),
        ('J', (1, 0)) => Some((x, y - 1)),
        ('J', (0, 1)) => Some((x - 1, y)),
        ('7', (1, 0)) => Some((x, y + 1)),
        ('7', (0, -1)) => Some((x - 1, y)),
        ('F', (-1, 0)) => Some((x, y + 1)),
        ('F', (0, -1)) => Some((x + 1, y)),
        ('S', _) => Some((x, y)),
        (' ', _) => None,
        _ => {
            println!(
                "No valid pipe found: {} {} {}",
                pipe, delta_pos.0, delta_pos.1,
            );
            None
        }
    }
}

fn part1(input: &str) -> u32 {
    let mut map = Vec::new();
    let mut start = (0, 0);
    input.lines().enumerate().for_each(|(y, line)| {
        let mut row = Vec::new();
        line.chars().enumerate().for_each(|(x, c)| {
            if c == 'S' {
                start = (x, y);
            }
            row.push(c);
        });
        map.push(row);
    });

    let delta_pos = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    println!("Start: {} {}", start.0, start.1);

    for d_pos in delta_pos {
        let mut prev_pos = start;
        let mut pos = (
            (start.0 as i32 + d_pos.0) as usize,
            (start.1 as i32 + d_pos.1) as usize,
        );

        let mut pipe = map[pos.1 as usize][pos.0 as usize];

        if next_pipe(pos, prev_pos, pipe).is_none() {
            continue;
        }

        let mut dist = 1;
        while let Some(next_pos) = next_pipe(pos, prev_pos, pipe) {
            dist += 1;

            if map[next_pos.1 as usize][next_pos.0 as usize] == 'S' {
                println!("Found start again: {} {}", dist, pipe);
                return dist / 2;
            }

            let next_pipe = map[next_pos.1 as usize][next_pos.0 as usize];
            prev_pos = (pos.0 as usize, pos.1 as usize);
            pos = (next_pos.0, next_pos.1);
            pipe = next_pipe;
        }
        println!("{} {}", dist, pipe);
        break;
    }
    return 0;
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
        assert_eq!(part1(input), 4);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("test");
        assert_eq!(part2(input), 0);
    }
}
