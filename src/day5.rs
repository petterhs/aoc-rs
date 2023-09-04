use std::str::Lines;

fn do_move(stacks: &mut Vec<Vec<char>>, num: usize, from: usize, to: usize) {
    let mut temp: Vec<char> = Vec::new();

    for _ in 0..num {
        temp.push(stacks[from - 1].pop().unwrap());
    }
    println!("Temp: {:?}", temp);
    for _ in 0..num {
        stacks[to - 1].push(temp.pop().unwrap());
    }
}

fn parse_stack(lines: Lines) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();

    let first = lines.clone().next().unwrap();

    for _ in 0..(first.len() / 4 + 1) {
        stacks.push(Vec::new());
    }

    let _ = lines.filter(|l| l.starts_with('[')).for_each(|s| {
        //split line into vec of 3chars remove one whitespace between
        let mut chars = s.chars().collect::<Vec<char>>();

        //take out the second character then skip the next 3
        let mut pos = 0;
        while chars.len() > 3 {
            let c = chars[1];
            if c.is_alphabetic() {
                stacks[pos].push(c);
            }

            chars = chars.split_off(4);
            pos += 1;
        }
        let c = chars[1];
        if c.is_alphabetic() {
            stacks[pos].push(c);
        }
    });

    //revers each stack
    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    println!("Stacks: {:?}", stacks);
    stacks
}

fn part1() {
    let mut stacks = parse_stack(include_str!("../input/5").lines());
    let _sum = include_str!("../input/5")
        .lines()
        .filter(|s| s.starts_with("move"))
        .for_each(|s| {
            let mut words = s.split_whitespace();
            let _ = words.next();
            let num = words.next().unwrap().parse::<usize>().unwrap();
            let _ = words.next();
            let from = words.next().unwrap().parse::<usize>().unwrap();
            let _ = words.next();
            let to = words.next().unwrap().parse::<usize>().unwrap();

            println!("Move {} from {} to {}", num, from, to);
            for _ in 0..num {
                do_move(&mut stacks, 1, from, to);
            }
        });

    print!("Code: ");
    stacks.iter().for_each(|s| {
        print!("{}", s.last().unwrap());
    });
    println!();
}

fn part2() {
    let mut stacks = parse_stack(include_str!("../input/5").lines());
    let _sum = include_str!("../input/5")
        .lines()
        .filter(|s| s.starts_with("move"))
        .for_each(|s| {
            let mut words = s.split_whitespace();
            let _ = words.next();
            let num = words.next().unwrap().parse::<usize>().unwrap();
            let _ = words.next();
            let from = words.next().unwrap().parse::<usize>().unwrap();
            let _ = words.next();
            let to = words.next().unwrap().parse::<usize>().unwrap();

            println!("Move {} from {} to {}", num, from, to);
            do_move(&mut stacks, num as usize, from, to);
        });

    print!("Code: ");
    stacks.iter().for_each(|s| {
        print!("{}", s.last().unwrap());
    });
    println!();
}

pub fn run() {
    println!("Day 5");
    part1();
    part2();
}
