#[derive(Debug)]
struct Monkey {
    id: i32,
    items: Vec<i32>,
    operation: Operation,
    test: Operation,
    if_true: Option<i32>,
    if_false: Option<i32>,
    num_inspected_items: i32,
}
impl Monkey {
    fn new(id: i32) -> Monkey {
        Monkey {
            id,
            items: Vec::new(),
            operation: Operation::Empty,
            test: Operation::Empty,
            if_true: None,
            if_false: None,
            num_inspected_items: 0,
        }
    }
    fn process_items(&mut self) -> Vec<(i32, i32)> {
        let mut result: Vec<(i32, i32)> = Vec::new();

        //Inspect items
        for item in self.items.iter_mut() {
            match self.operation {
                Operation::Add(rhs) => {
                    *item += rhs;
                }
                Operation::MultiplyBy(rhs) => {
                    *item *= rhs;
                }
                Operation::MultiplyBySelf => {
                    *item *= *item;
                }
                _ => {}
            }
            self.num_inspected_items += 1;
        }

        //Decrease items worry after inspection
        for item in self.items.iter_mut() {
            *item /= 3;
        }

        //Test items
        for item in self.items.iter() {
            match self.test {
                Operation::DivisibleBy(rhs) => {
                    if *item % rhs == 0 {
                        result.push((*item, self.if_true.unwrap()));
                    } else {
                        result.push((*item, self.if_false.unwrap()));
                    }
                }
                _ => {}
            }
        }

        //Clear items
        self.items.clear();

        result
    }
}

#[derive(Debug)]
enum Operation {
    Add(i32),
    MultiplyBy(i32),
    MultiplyBySelf,
    DivisibleBy(i32),
    Empty,
}

impl Operation {
    fn new(operation: &str, rhs: &str) -> Operation {
        match (operation, rhs) {
            ("+", rhs) => Operation::Add(rhs.parse::<i32>().unwrap()),
            ("*", "old") => Operation::MultiplyBySelf,

            ("*", rhs) => Operation::MultiplyBy(rhs.parse::<i32>().unwrap()),
            ("/", rhs) => Operation::DivisibleBy(rhs.parse::<i32>().unwrap()),
            _ => Operation::Empty,
        }
    }
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();

    input.lines().for_each(|line| {
        let mut words = line.split_whitespace();
        match words.next() {
            Some("Monkey") => {
                let id = words
                    .next()
                    .unwrap()
                    .split(':')
                    .nth(0)
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
                monkeys.push(Monkey::new(id));
            }
            Some("Starting") => {
                let _ = words.next().unwrap();
                while let Some(item) = words.next() {
                    let item = item.split(',').nth(0).unwrap().parse::<i32>().unwrap();
                    monkeys.last_mut().unwrap().items.push(item);
                }
            }
            Some("Operation:") => {
                let operation = words.clone().skip(3).next().unwrap();
                let rhs = words.skip(4).next().unwrap();
                let operation = Operation::new(operation, rhs);
                monkeys.last_mut().unwrap().operation = operation;
            }
            Some("Test:") => {
                let divider = words.skip(2).next().unwrap();
                let test = Operation::new("/", divider);
                monkeys.last_mut().unwrap().test = test;
            }
            Some("If") => match words.next() {
                Some("true:") => {
                    let if_true_id = words.last().unwrap().parse::<i32>().unwrap();
                    monkeys.last_mut().unwrap().if_true = Some(if_true_id);
                }
                Some("false:") => {
                    let if_false_id = words.last().unwrap().parse::<i32>().unwrap();
                    monkeys.last_mut().unwrap().if_false = Some(if_false_id);
                }
                _ => {}
            },
            _ => {}
        }
    });
    monkeys
}
fn part() {
    println!("Part 1");
    let input = include_str!("../input/11");
    let mut monkeys = parse_input(input);

    for _ in 0..20 {
        let mut throws: Vec<(i32, i32)> = Vec::new();
        for monkey in monkeys.iter_mut() {
            //Check if there are any throws to this monkey in this round
            for throw in throws.iter() {
                if throw.1 == monkey.id {
                    monkey.items.push(throw.0);
                }
            }
            throws.retain(|t| t.1 != monkey.id);

            let result = monkey.process_items();
            for throw in result {
                throws.push(throw);
            }
        }

        //Add throws to monkeys
        for throw in throws {
            let monkey = monkeys.iter_mut().find(|m| m.id == throw.1).unwrap();
            monkey.items.push(throw.0);
        }
    }

    let mut num_inspected_items = monkeys
        .iter()
        .map(|m| m.num_inspected_items)
        .collect::<Vec<i32>>();

    num_inspected_items.sort();

    let first = num_inspected_items.pop().unwrap();
    let second = num_inspected_items.pop().unwrap();
    println!("Max num inspected items: {}", first);
    println!("2nd Max num inspected items: {}", second);
    println!("Monkey business: {}", first * second);
}

pub fn run() {
    println!("Running day11");
    part();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = include_str!("../input/test11");
        let mut monkeys = parse_input(input);

        for round in 0..20 {
            let mut throws: Vec<(i32, i32)> = Vec::new();
            for monkey in monkeys.iter_mut() {
                //Check if there are any throws to this monkey in this round
                for throw in throws.iter() {
                    if throw.1 == monkey.id {
                        monkey.items.push(throw.0);
                    }
                }
                throws.retain(|t| t.1 != monkey.id);

                let result = monkey.process_items();
                for throw in result {
                    throws.push(throw);
                }
            }

            //Add throws to monkeys
            for throw in throws {
                let monkey = monkeys.iter_mut().find(|m| m.id == throw.1).unwrap();
                monkey.items.push(throw.0);
            }
        }

        let mut num_inspected_items = monkeys
            .iter()
            .map(|m| m.num_inspected_items)
            .collect::<Vec<i32>>();

        num_inspected_items.sort();

        let first = num_inspected_items.pop().unwrap();
        let second = num_inspected_items.pop().unwrap();
        println!("Max num inspected items: {}", first);
        println!("2nd Max num inspected items: {}", second);
        assert_eq!(first, 105);
        assert_eq!(second, 101);
    }
}
