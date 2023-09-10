#[derive(Debug)]
struct Monkey {
    id: i64,
    items: Vec<i64>,
    operation: Operation,
    divisor: i64,
    if_true: Option<i64>,
    if_false: Option<i64>,
    num_inspected_items: i64,
}
impl Monkey {
    fn new(id: i64) -> Monkey {
        Monkey {
            id,
            items: Vec::new(),
            operation: Operation::Empty,
            divisor: 0,
            if_true: None,
            if_false: None,
            num_inspected_items: 0,
        }
    }
    fn process_items(&mut self, part1: bool, modulus: i64) -> Vec<(i64, i64)> {
        let mut result: Vec<(i64, i64)> = Vec::new();

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
            //Decrease items worry after inspection
            if part1 {
                *item /= 3;
            } else {
                *item %= modulus;
            }

            if *item % self.divisor == 0 {
                result.push((*item, self.if_true.unwrap()));
            } else {
                result.push((*item, self.if_false.unwrap()));
            }
        }

        //Clear items
        self.items.clear();

        result
    }
}

#[derive(Debug)]
enum Operation {
    Add(i64),
    MultiplyBy(i64),
    MultiplyBySelf,
    Empty,
}

impl Operation {
    fn new(operation: &str, rhs: &str) -> Operation {
        match (operation, rhs) {
            ("+", rhs) => Operation::Add(rhs.parse::<i64>().unwrap()),
            ("*", "old") => Operation::MultiplyBySelf,

            ("*", rhs) => Operation::MultiplyBy(rhs.parse::<i64>().unwrap()),
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
                    .parse::<i64>()
                    .unwrap();
                monkeys.push(Monkey::new(id));
            }
            Some("Starting") => {
                let _ = words.next().unwrap();
                while let Some(item) = words.next() {
                    let item = item.split(',').nth(0).unwrap().parse::<i64>().unwrap();
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
                monkeys.last_mut().unwrap().divisor =
                    words.skip(2).next().unwrap().parse::<i64>().unwrap();
            }
            Some("If") => match words.next() {
                Some("true:") => {
                    let if_true_id = words.last().unwrap().parse::<i64>().unwrap();
                    monkeys.last_mut().unwrap().if_true = Some(if_true_id);
                }
                Some("false:") => {
                    let if_false_id = words.last().unwrap().parse::<i64>().unwrap();
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
        let mut throws: Vec<(i64, i64)> = Vec::new();
        for monkey in monkeys.iter_mut() {
            //Check if there are any throws to this monkey in this round
            for throw in throws.iter() {
                if throw.1 == monkey.id {
                    monkey.items.push(throw.0);
                }
            }
            throws.retain(|t| t.1 != monkey.id);

            let result = monkey.process_items(true, 0);
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
        .collect::<Vec<i64>>();

    num_inspected_items.sort();

    let first = num_inspected_items.pop().unwrap();
    let second = num_inspected_items.pop().unwrap();
    println!("Max num inspected items: {}", first);
    println!("2nd Max num inspected items: {}", second);
    println!("Monkey business: {}", first * second);
}

fn part2() {
    println!("Part 2");
    let input = include_str!("../input/11");
    let mut monkeys = parse_input(input);

    let mut modulo = 1;
    for monkey in &monkeys {
        modulo *= monkey.divisor;
    }
    println!("Modulo: {}", modulo);

    for _ in 0..10000 {
        let mut throws: Vec<(i64, i64)> = Vec::new();
        for monkey in monkeys.iter_mut() {
            //Check if there are any throws to this monkey in this round
            for throw in throws.iter() {
                if throw.1 == monkey.id {
                    monkey.items.push(throw.0);
                }
            }
            throws.retain(|t| t.1 != monkey.id);

            let result = monkey.process_items(false, modulo);
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
        .collect::<Vec<i64>>();

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
    part2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = include_str!("../input/test11");
        let mut monkeys = parse_input(input);

        for _ in 0..20 {
            let mut throws: Vec<(i64, i64)> = Vec::new();
            for monkey in monkeys.iter_mut() {
                //Check if there are any throws to this monkey in this round
                for throw in throws.iter() {
                    if throw.1 == monkey.id {
                        monkey.items.push(throw.0);
                    }
                }
                throws.retain(|t| t.1 != monkey.id);

                let result = monkey.process_items(true, 0);
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
            .collect::<Vec<i64>>();

        num_inspected_items.sort();

        let first = num_inspected_items.pop().unwrap();
        let second = num_inspected_items.pop().unwrap();
        println!("Max num inspected items: {}", first);
        println!("2nd Max num inspected items: {}", second);
        assert_eq!(first, 105);
        assert_eq!(second, 101);
    }
}
