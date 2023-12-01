use std::collections::{HashMap, HashSet};

fn get_outer_limits(volume: &HashSet<(i32, i32, i32)>) -> ((i32, i32, i32), (i32, i32, i32)) {
    let mut max = volume.iter().next().unwrap().clone();
    let mut min = max;

    for pos in volume.iter() {
        if pos.0 > max.0 {
            max.0 = pos.0
        }
        if pos.1 > max.1 {
            max.1 = pos.1
        }
        if pos.2 > max.2 {
            max.2 = pos.2
        }

        if pos.0 < min.0 {
            min.0 = pos.0
        }
        if pos.1 < min.1 {
            min.1 = pos.1
        }
        if pos.2 < min.2 {
            min.2 = pos.2
        }
    }

    //Increase limits by 1
    min = (min.0 - 1, min.1 - 1, min.2 - 1);
    max = (max.0 + 1, max.1 + 1, max.2 + 1);

    (min, max)
}

fn is_in_bounds(pos: &(i32, i32, i32), limits: ((i32, i32, i32), (i32, i32, i32))) -> bool {
    if pos.0 < limits.0 .0
        || pos.0 > limits.1 .0
        || pos.1 < limits.0 .1
        || pos.1 > limits.1 .1
        || pos.2 < limits.0 .2
        || pos.2 > limits.1 .2
    {
        return false;
    }
    return true;
}

fn get_surface_area(neighbors: &HashMap<(i32, i32, i32), Vec<usize>>) -> i32 {
    neighbors
        .values()
        .map(|v| {
            let s: usize = v.iter().sum::<usize>();
            s
        })
        .sum::<usize>() as i32
}

fn get_surface(volume: &HashSet<(i32, i32, i32)>) -> HashMap<(i32, i32, i32), Vec<usize>> {
    let mut store = HashMap::new();
    let delta = vec![
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];
    for pos in volume {
        let x = pos.0;
        let y = pos.1;
        let z = pos.2;

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
}

fn part1(input: &str) -> i32 {
    let mut volume = HashSet::new();

    for line in input.lines() {
        let pos: Vec<i32> = line.split(',').map(|t| t.parse().unwrap()).collect();

        let x = pos[0];
        let y = pos[1];
        let z = pos[2];

        volume.insert((x, y, z));
    }

    get_surface_area(&get_surface(&volume))
}

fn part2(input: &str) -> i32 {
    let mut lava = HashSet::new();

    for line in input.lines() {
        let pos: Vec<i32> = line.split(',').map(|t| t.parse().unwrap()).collect();

        let x = pos[0];
        let y = pos[1];
        let z = pos[2];

        lava.insert((x, y, z));
    }

    let limits = get_outer_limits(&lava);
    let lava_surface = get_surface(&lava);

    //Breadth first search to find all reachable air position from outside the lava
    //droplet
    //
    let delta = vec![
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];

    let mut air_outside = HashSet::new();
    let mut stack = Vec::new();

    // stack.push((limits.0 .0 - 1, limits.0 .1 - 1, limits.0 .2 - 1));
    stack.push(limits.0);

    while let Some(pos) = stack.pop() {
        if lava.get(&pos).is_none() {
            air_outside.insert(pos);
        } else {
            continue;
        }

        for d in delta.clone() {
            let next_pos = (pos.0 + d.0, pos.1 + d.1, pos.2 + d.2);

            if is_in_bounds(&next_pos, limits) && air_outside.get(&next_pos).is_none() {
                stack.push(next_pos);
            }
        }
    }

    //iterate over all lava position's air neighbors and check if the air pos is
    //in the HashSet of reacable air from the outside
    let mut outide_lava_surface = 0;
    for (lava_pos, neighbors) in lava_surface.iter() {
        for (i, neighbor) in neighbors.iter().enumerate() {
            if *neighbor == 1 {
                let neighbor_pos = (
                    lava_pos.0 + delta[i].0,
                    lava_pos.1 + delta[i].1,
                    lava_pos.2 + delta[i].2,
                );
                if air_outside.get(&neighbor_pos).is_some() {
                    outide_lava_surface += 1;
                }
            }
        }
    }

    outide_lava_surface
}

pub fn run() {
    let input = include_str!("../input/18");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = include_str!("../input/test18");
        assert_eq!(part1(input), 64);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input/test18");
        assert_eq!(part2(input), 58);
    }
}
