use std::collections::HashMap;
#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(c: char) -> Self {
        match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("Invalid instruction"),
        }
    }
}

#[derive(Debug)]
struct Instructions {
    instructions: Vec<Instruction>,
    index: usize,
}

impl Instructions {
    fn new(input: &str) -> Self {
        let instructions = input
            .chars()
            .map(|c| Instruction::from(c))
            .collect::<Vec<Instruction>>();

        Self {
            instructions,
            index: 0,
        }
    }

    fn next(&mut self) -> Option<&Instruction> {
        let instruction = &self.instructions[self.index];
        self.index += 1;
        if self.index == self.instructions.len() {
            self.index = 0;
        }
        Some(instruction)
    }
}

type Node = String;

#[derive(Debug)]
struct Path(Node, Node);

impl From<&str> for Path {
    fn from(s: &str) -> Self {
        let mut nodes = s
            .strip_prefix("(")
            .unwrap()
            .strip_suffix(")")
            .unwrap()
            .split(",");
        let node1 = nodes.next().unwrap().trim();
        let node2 = nodes.next().unwrap().trim();
        Self(node1.to_string(), node2.to_string())
    }
}

fn is_start_node(node: &str) -> bool {
    node.ends_with("A")
}

fn is_end_node(node: &str) -> bool {
    node.ends_with("Z")
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn least_common_multiple(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn lcm_array(a: &[u64]) -> u64 {
    let mut lcm = a[0];
    for i in 1..a.len() {
        lcm = least_common_multiple(lcm, a[i]);
    }
    lcm
}

fn part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let mut instructions = Instructions::new(lines.next().unwrap());

    let mut map = HashMap::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }
        let mut words = line.split("=");
        let node = words.next().unwrap().trim();

        let path = Path::from(words.next().unwrap().trim());

        map.insert(node, path);
    }

    let mut current_node = "AAA";
    let mut steps = 0;

    while let Some(instruction) = instructions.next() {
        if current_node == "ZZZ" {
            break;
        }
        let path = map.get(&current_node).unwrap();
        match instruction {
            Instruction::Left => {
                current_node = &path.0;
            }
            Instruction::Right => {
                current_node = &path.1;
            }
        }
        steps += 1;
    }
    steps
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut instructions = Instructions::new(lines.next().unwrap());

    let mut map = HashMap::new();

    let mut current_nodes = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }
        let mut words = line.split("=");
        let node = words.next().unwrap().trim();

        let path = Path::from(words.next().unwrap().trim());

        map.insert(node, path);

        if is_start_node(&node) {
            current_nodes.push(node);
        }
    }

    let mut steps = 0;
    let mut cycles = vec![(0, 0); current_nodes.len()];

    //Find cycles
    while let Some(instruction) = instructions.next() {
        if current_nodes.iter().all(|n| is_end_node(n)) {
            return steps;
        }

        if steps == 40000 {
            break;
        }

        //TODO: Solve the generalized case
        for (i, node) in current_nodes.iter_mut().enumerate() {
            if is_end_node(node) {
                if cycles[i].1 == 0 {
                    cycles[i].1 = steps;
                } else if cycles[i].0 == 0 {
                    println!(
                        "{}: End node reached: {}, cycle {}, start {}",
                        i,
                        node,
                        steps - cycles[i].1,
                        cycles[i].1
                    );
                    cycles[i].0 = steps - cycles[i].1;
                }
            }
            let path = map.get(node).unwrap();
            match instruction {
                Instruction::Left => {
                    *node = &path.0;
                }
                Instruction::Right => {
                    *node = &path.1;
                }
            }
        }
        steps += 1;
    }

    // All cycles start at 0 so we don't need to find the offset
    // Only one end node can be reached from a start node
    // And the time all paths get to a end node is the least common
    // multiple of all cycles
    lcm_array(&cycles.iter().map(|c| c.0).collect::<Vec<u64>>())
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
        assert_eq!(part1(input), 2);

        let input = include_str!("test2");
        assert_eq!(part1(input), 6);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("test3");
        assert_eq!(part2(input), 6);
    }
}
