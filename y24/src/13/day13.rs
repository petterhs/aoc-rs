use std::str::FromStr;

#[derive(Debug)]
struct ClawMachine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

impl FromStr for ClawMachine {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let button_a = lines
            .next()
            .unwrap()
            .split(',')
            .map(|x| {
                let y = x.chars().filter(|c| c.is_digit(10)).collect::<String>();
                y.parse::<i64>().unwrap()
            })
            .collect::<Vec<i64>>();

        let button_a = (button_a[0], button_a[1]);

        let button_b = lines
            .next()
            .unwrap()
            .split(',')
            .map(|x| {
                let y = x.chars().filter(|c| c.is_digit(10)).collect::<String>();
                y.parse::<i64>().unwrap()
            })
            .collect::<Vec<i64>>();

        let button_b = (button_b[0], button_b[1]);

        let prize = lines
            .next()
            .unwrap()
            .split(',')
            .map(|x| {
                let y = x.chars().filter(|c| c.is_digit(10)).collect::<String>();
                y.parse::<i64>().unwrap()
            })
            .collect::<Vec<i64>>();

        let prize = (prize[0], prize[1]);

        Ok(ClawMachine {
            button_a,
            button_b,
            prize,
        })
    }
}

impl ClawMachine {
    fn solve(&self) -> i64 {
        let a = self.button_a;
        let b = self.button_b;
        let res = self.prize;

        if b.0 * a.1 == a.0 * b.1 {
            return 0;
        }

        let nb = (res.1 * a.0 - res.0 * a.1) / (b.1 * a.0 - b.0 * a.1);
        if b.0 * nb > res.0 {
            return 0;
        }

        let na = (res.0 - b.0 * nb) / a.0;

        if (na * a.1 + nb * b.1) != res.1 || (na * a.0 + nb * b.0) != res.0 {
            return 0;
        }

        na * 3 + nb
    }
}

fn part1(input: &str) -> i64 {
    let claw_machines = input
        .split("\n\n")
        .map(|x| x.parse::<ClawMachine>().unwrap())
        .collect::<Vec<ClawMachine>>();

    claw_machines.into_iter().map(|x| x.solve()).sum::<i64>()
}

fn part2(input: &str) -> i64 {
    let claw_machines = input
        .split("\n\n")
        .map(|x| x.parse::<ClawMachine>().unwrap())
        .map(|mut x| {
            x.prize = (x.prize.0 + 10000000000000, x.prize.1 + 10000000000000);
            x
        })
        .collect::<Vec<ClawMachine>>();

    claw_machines.into_iter().map(|x| x.solve()).sum::<i64>()
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
        assert_eq!(part1(input), 480);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("test");
        assert_eq!(part2(input), 0);
    }
}
