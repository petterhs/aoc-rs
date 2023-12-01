use std::{collections::VecDeque, str::FromStr};

#[derive(Debug)]
struct Sensor {
    position: (i32, i32),
    closest_beacon: (i32, i32),
    range: i32,
}

//Example inpit: "Sensor at x=2832148, y=322979: closest beacon is at x=3015667, y=-141020"
impl FromStr for Sensor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s
            .split(|c| c == '=' || c == ',' || c == ':')
            .filter_map(|s| s.parse::<i32>().ok())
            .take(4)
            .collect::<Vec<i32>>();
        Ok(Sensor {
            position: (v[0], v[1]),
            closest_beacon: (v[2], v[3]),
            range: manhatten_distance((v[0], v[1]), (v[2], v[3])),
        })
    }
}

fn manhatten_distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn part1() {
    let input = include_str!("../input/15");
    let sensors = input
        .lines()
        .map(|s| Sensor::from_str(s).unwrap())
        .collect::<Vec<Sensor>>();

    let y = 2000000;

    let min_x = sensors
        .iter()
        .map(|s| s.position.0 - s.range)
        .min()
        .unwrap();

    let max_x = sensors
        .iter()
        .map(|s| s.position.0 + s.range)
        .max()
        .unwrap();

    let mut covered_positions = 0;
    for x in min_x..max_x {
        for s in &sensors {
            if (x, y) == s.position || (x, y) == s.closest_beacon {
                break;
            }
            let dist = manhatten_distance((x, y), s.position);
            if dist <= s.range {
                covered_positions += 1;
                break;
            }
        }
    }
    println!("Part 1: {}", covered_positions);
}

fn check_if_covered(sensors: &Vec<Sensor>, x: i32, y: i32) -> bool {
    for s in sensors {
        if (x, y) == s.position || (x, y) == s.closest_beacon {
            return true;
        }
        let dist = manhatten_distance((x, y), s.position);
        if dist <= s.range {
            return true;
        }
    }
    false
}

fn find_pos_not_covered(sensors: &Vec<Sensor>, size: i32) -> Option<(i32, i32)> {
    for s in sensors {
        println!("{:?}", s);

        //iterate over all positions 1 step away from sensor range
        //check if any sensor covers that position

        //Start at the top of the sensor range + 1 and go down each side
        let mut queue = VecDeque::new();
        queue.push_back((s.position.0, s.position.1 - s.range - 1));

        // println!("From top , going down left");
        while let Some((x, y)) = queue.pop_front() {
            if y > s.position.1 {
                break;
            }

            queue.push_back((x - 1, y + 1));
            if x < 0 || y < 0 || x > size || y > size {
                continue;
            }
            // println!("({}, {}) ", x, y);

            let covered = check_if_covered(&sensors, x, y);

            if !covered {
                return Some((x, y));
            }
        }

        queue.clear();
        queue.push_back((s.position.0, s.position.1 - s.range - 1));

        // println!("From top , going down right");
        while let Some((x, y)) = queue.pop_front() {
            if y > s.position.1 {
                break;
            }

            queue.push_back((x + 1, y + 1));

            if x < 0 || y < 0 || x > size || y > size {
                continue;
            }
            // println!("({}, {}) ", x, y);

            let covered = check_if_covered(&sensors, x, y);

            if !covered {
                return Some((x, y));
            }
        }

        queue.clear();
        queue.push_back((s.position.0, s.position.1 + s.range + 1));

        // println!("From bottom , going up left");
        while let Some((x, y)) = queue.pop_front() {
            if y < s.position.1 {
                break;
            }

            queue.push_back((x - 1, y - 1));
            if x < 0 || y < 0 || x > size || y > size {
                continue;
            }

            let covered = check_if_covered(&sensors, x, y);

            if !covered {
                return Some((x, y));
            }
        }

        queue.clear();
        queue.push_back((s.position.0, s.position.1 + s.range + 1));

        // println!("From bottom , going up right");
        while let Some((x, y)) = queue.pop_front() {
            if y < s.position.1 {
                break;
            }

            queue.push_back((x + 1, y - 1));

            if x < 0 || y < 0 || x > size || y > size {
                continue;
            }

            let covered = check_if_covered(&sensors, x, y);

            if !covered {
                return Some((x, y));
            }
        }
    }
    None
}

fn part2_bruteforce() {
    let input = include_str!("../input/15");
    let sensors = input
        .lines()
        .map(|s| Sensor::from_str(s).unwrap())
        .collect::<Vec<Sensor>>();

    let (x, y) = find_pos_not_covered(&sensors, 4000000).unwrap();

    println!("Part 2: {}", x as i64 * 4000000 + y as i64);
}

pub fn run() {
    println!("Day 15");
    part1();
    part2_bruteforce();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let s = "Sensor at x=2832148, y=322979: closest beacon is at x=3015667, y=-141020";
        let sensor = Sensor::from_str(s).unwrap();
        assert_eq!(sensor.position, (2832148, 322979));
        assert_eq!(sensor.closest_beacon, (3015667, -141020));
    }

    #[test]
    fn test_manhatten_dist() {
        assert_eq!(manhatten_distance((0, 0), (0, 0)), 0);
        assert_eq!(manhatten_distance((0, 0), (1, 1)), 2);
        assert_eq!(manhatten_distance((0, 0), (1, 0)), 1);
        assert_eq!(manhatten_distance((0, 0), (0, 1)), 1);
        assert_eq!(manhatten_distance((0, 0), (-1, 0)), 1);
        assert_eq!(manhatten_distance((0, 0), (0, -1)), 1);
    }

    #[test]
    fn part1() {
        println!("Part 1: {}", 0);

        let input = include_str!("../input/test15");
        let sensors = input
            .lines()
            .map(|s| Sensor::from_str(s).unwrap())
            .collect::<Vec<Sensor>>();

        let y = 10;

        let min_x = sensors
            .iter()
            .map(|s| s.position.0 - s.range)
            .min()
            .unwrap();

        let max_x = sensors
            .iter()
            .map(|s| s.position.0 + s.range)
            .max()
            .unwrap();

        let mut covered_positions = 0;
        for x in min_x..max_x {
            let mut covered = false;
            for s in &sensors {
                if (x, y) == s.position || (x, y) == s.closest_beacon {
                    break;
                }
                let dist = manhatten_distance((x, y), s.position);
                if dist <= s.range {
                    covered_positions += 1;
                    covered = true;
                    break;
                }
            }
            if covered {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("Covered positions: {}", covered_positions);
        assert_eq!(covered_positions, 26);
    }

    #[test]
    fn part2() {
        let input = include_str!("../input/test15");
        let sensors = input
            .lines()
            .map(|s| Sensor::from_str(s).unwrap())
            .collect::<Vec<Sensor>>();

        let (x, y) = find_pos_not_covered(&sensors, 21).unwrap();

        println!("Part 2: {}, {}", x, y);

        assert_eq!(x * 4000000 + y, 56000011);
    }
}
