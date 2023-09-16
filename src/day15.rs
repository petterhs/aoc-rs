use std::str::FromStr;

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

fn part2() {
    println!("Part 2: {}", 0);
}

pub fn run() {
    println!("Day 15");
    part1();
    part2();
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
}
