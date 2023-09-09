fn signal_strenght(input: &str) -> i32 {
    let mut signal = 0;
    let mut cycle = 1;
    let mut register = 1;
    for line in input.lines() {
        let mut words = line.split(" ");
        let first = words.next().unwrap();
        match first {
            "noop" => {
                if (cycle - 20) % 40 == 0 {
                    signal += cycle * register;
                }
                cycle += 1;
            }
            "addx" => {
                let value = words.next().unwrap().parse::<i32>().unwrap();
                for _ in 0..2 {
                    if (cycle - 20) % 40 == 0 {
                        signal += register * cycle;
                    }
                    cycle += 1;
                }
                register += value;
            }
            _ => panic!("Unknown instruction"),
        }
    }
    signal
}

fn part1() {
    let input = include_str!("../input/10");
    let signal = signal_strenght(input);
    println!("Part 1: {}", signal);
}

fn part2() {}

pub fn run() {
    println!("Day 10");

    part1();
    // part2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input/test10");
        let signal = signal_strenght(input);
        assert_eq!(signal, 13140);
    }
}
