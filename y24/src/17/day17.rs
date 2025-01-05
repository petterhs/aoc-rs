use std::str::FromStr;

struct Computer {
    a: i64,
    b: i64,
    c: i64,
}

impl FromStr for Computer {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut computer = Computer { a: 0, b: 0, c: 0 };
        let lines: Vec<i64> = s
            .lines()
            .map(|x| {
                x.chars()
                    .filter(|c| c.is_digit(10))
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap()
            })
            .collect();

        computer.a = lines[0];
        computer.b = lines[1];
        computer.c = lines[2];
        Ok(computer)
    }
}

impl Computer {
    fn run(&mut self, instructions: Vec<i64>) -> Vec<i64> {
        let mut instruction_pointer = 0;
        let mut result = Vec::new();
        loop {
            let opcode = instructions.get(instruction_pointer);
            let operand = instructions.get(instruction_pointer + 1);
            if opcode.is_none() {
                break;
            }

            let combo_operand = match operand.unwrap() {
                0..=3 => Some(*operand.unwrap() as u32),
                4 => Some(self.a as u32),
                5 => Some(self.b as u32),
                6 => Some(self.c as u32),
                _ => None,
            };

            match opcode.unwrap() {
                0 => {
                    //adv
                    self.a = self.a >> combo_operand.unwrap();
                }
                1 => {
                    //bxl
                    self.b = self.b ^ operand.unwrap();
                }
                2 => {
                    //bst
                    self.b = combo_operand.unwrap() as i64 % 8;
                }
                3 => {
                    //jnz
                    if self.a != 0 {
                        instruction_pointer = usize::try_from(*operand.unwrap())
                            .expect("operand out of bounds for usize");
                        continue;
                    }
                }
                4 => {
                    //bxc
                    self.b = self.b ^ self.c;
                }
                5 => {
                    //out
                    let value = combo_operand.unwrap() as i64 % 8;
                    result.push(value);
                }
                6 => {
                    //bdv
                    self.b = self.a >> combo_operand.unwrap();
                }
                7 => {
                    //cdv
                    self.c = self.a >> combo_operand.unwrap();
                }

                _ => panic!("Unknown instruction"),
            }
            instruction_pointer += 2;
        }

        result
    }

    fn run_str(&mut self, instructions: Vec<i64>) -> String {
        self.run(instructions)
            .iter()
            .fold("".to_owned(), |acc, x| acc + x.to_string().as_str() + ",")
            .trim_end_matches(',')
            .to_string()
    }
}

fn part1(input: &str) -> String {
    let split = input.split("\n\n").collect::<Vec<&str>>();

    let mut computer = Computer::from_str(split[0]).unwrap();

    let program = split[1]
        .chars()
        .filter(|x| x.is_digit(10))
        .map(|x| x.to_string().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    computer.run_str(program)
}

fn part2(input: &str) -> i64 {
    let split = input.split("\n\n").collect::<Vec<&str>>();

    let mut computer = Computer::from_str(split[0]).unwrap();

    let program = split[1]
        .chars()
        .filter(|x| x.is_digit(10))
        .map(|x| x.to_string().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut a = 1;

    loop {
        computer.a = a;
        let result = computer.run(program.clone());

        if result == program {
            return a;
        }

        if program.ends_with(&result) {
            a *= 8;
            continue;
        }

        a += 1;
    }
}

fn main() {
    println!("AoC 2024 - Day 17");
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
        assert_eq!(part1(input), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part1_test1() {
        let mut computer = Computer { a: 0, b: 0, c: 9 };
        let program = vec![2, 6];
        assert_eq!(computer.run_str(program), "");
        assert_eq!(computer.b, 1);
    }

    #[test]
    fn part1_test2() {
        let mut computer = Computer { a: 10, b: 0, c: 0 };
        let program = vec![5, 0, 5, 1, 5, 4];
        assert_eq!(computer.run_str(program), "0,1,2");
    }

    #[test]
    fn part1_test3() {
        let mut computer = Computer {
            a: 2024,
            b: 0,
            c: 0,
        };
        let program = vec![0, 1, 5, 4, 3, 0];
        assert_eq!(1012 % 8, 4);
        assert_eq!(computer.run_str(program), "4,2,5,6,7,7,7,7,3,1,0");
        assert_eq!(computer.a, 0);
    }

    #[test]
    fn part1_test4() {
        let mut computer = Computer { a: 0, b: 29, c: 0 };
        let program = vec![1, 7];
        assert_eq!(computer.run_str(program), "");
        assert_eq!(computer.b, 26);
    }
    #[test]
    fn part1_test5() {
        let mut computer = Computer {
            a: 0,
            b: 2024,
            c: 43690,
        };
        let program = vec![4, 0];
        assert_eq!(computer.run_str(program), "");
        assert_eq!(computer.b, 44354);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("test2");
        assert_eq!(part2(input), 117440);
    }
}
