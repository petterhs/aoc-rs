use std::collections::HashMap;

fn part1(input: &str) -> i32 {
    let mut store = HashMap::new();
    let delta = vec![
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];

    for line in input.lines() {
        let pos: Vec<i32> = line.split(',').map(|t| t.parse().unwrap()).collect();

        let x = pos[0];
        let y = pos[1];
        let z = pos[2];

        let mut neigbors = vec![1, 1, 1, 1, 1, 1];

        for (i, d) in delta.iter().enumerate() {
            let mut opposite_index = i + 1;

            if i == 1 || i == 3 || i == 5 {
                opposite_index = i - 1;
            }

            let neighbor = (x + d.0, y + d.1, z + d.2);

            if store.contains_key(&neighbor) {
                let v: &mut Vec<usize> = store.get_mut(&neighbor).unwrap();
                v[opposite_index] = 0;

                neigbors[i] = 0;
            }
        }
        store.insert((x, y, z), neigbors);
    }

    store
        .values()
        .map(|v| {
            let s: usize = v.iter().sum::<usize>();
            s
        })
        .sum::<usize>() as i32
}

pub fn run() {
    let input = include_str!("../input/18");
    println!("Part1: {}", part1(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = include_str!("../input/test18");
        assert_eq!(part1(input), 64);
    }
}
