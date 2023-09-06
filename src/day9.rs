use std::collections::HashSet;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    UpLeft,
    Up,
    UpRight,
    Left,
    On,
    Right,
    DownLeft,
    Down,
    DownRight,
}

impl From<&str> for Direction {
    fn from(c: &str) -> Self {
        match c {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction c: {}", c),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Move {
    direction: Direction,
    steps: i32,
}

impl Move {
    fn new(direction: Direction, steps: i32) -> Self {
        Move { direction, steps }
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Default, Copy, Clone)]
struct Pos((i32, i32));

impl Pos {
    fn move_pos(&mut self, m: &Move) {
        match m.direction {
            Direction::Up => self.0 .1 += m.steps,
            Direction::Down => self.0 .1 -= m.steps,
            Direction::Left => self.0 .0 -= m.steps,
            Direction::Right => self.0 .0 += m.steps,
            Direction::UpRight => {
                self.0 .0 += m.steps;
                self.0 .1 += m.steps;
            }
            Direction::UpLeft => {
                self.0 .0 -= m.steps;
                self.0 .1 += m.steps;
            }
            Direction::DownLeft => {
                self.0 .0 -= m.steps;
                self.0 .1 -= m.steps;
            }
            Direction::DownRight => {
                self.0 .0 += m.steps;
                self.0 .1 -= m.steps;
            }
            Direction::On => {}
        }
    }
}

struct RopeBridge {
    head_relative: Direction,
    tail: Pos,
    tail_visited: HashSet<Pos>,
}

impl RopeBridge {
    fn new() -> Self {
        let mut rope = RopeBridge {
            head_relative: Direction::On,
            tail: Pos::default(),
            tail_visited: HashSet::new(),
        };
        rope.tail_visited.insert(Pos::default());
        rope
    }
    fn move_rope(&mut self, m: &Move) {
        for _ in 0..m.steps {
            println!("Tail pos: {:?}", self.tail);
            match &self.head_relative {
                Direction::On => {
                    println!("Move from on");
                    self.head_relative = m.direction;
                }
                same_dir if *same_dir == m.direction => {
                    println!("Move in same direction");
                    self.tail.move_pos(&Move::new(m.direction, 1));
                    self.tail_visited.insert(self.tail);
                }
                Direction::Up => match m.direction {
                    Direction::Down => {
                        self.head_relative = Direction::On;
                    }
                    Direction::Left => {
                        self.head_relative = Direction::UpLeft;
                    }
                    Direction::Right => {
                        self.head_relative = Direction::UpRight;
                    }
                    _ => {
                        println!("Invalid move");
                    }
                },
                Direction::Down => match m.direction {
                    Direction::Up => {
                        self.head_relative = Direction::On;
                    }
                    Direction::Left => {
                        self.head_relative = Direction::DownLeft;
                    }
                    Direction::Right => {
                        self.head_relative = Direction::DownRight;
                    }
                    _ => {
                        println!("Invalid move");
                    }
                },
                Direction::Left => match m.direction {
                    Direction::Right => {
                        self.head_relative = Direction::On;
                    }
                    Direction::Up => {
                        self.head_relative = Direction::UpLeft;
                    }
                    Direction::Down => {
                        self.head_relative = Direction::DownLeft;
                    }
                    _ => {
                        println!("Invalid move");
                    }
                },
                Direction::Right => match m.direction {
                    Direction::Left => {
                        self.head_relative = Direction::On;
                    }
                    Direction::Up => {
                        self.head_relative = Direction::UpRight;
                    }
                    Direction::Down => {
                        self.head_relative = Direction::DownRight;
                    }
                    _ => {
                        println!("Invalid move");
                    }
                },
                Direction::UpLeft => match m.direction {
                    Direction::Up => {
                        self.head_relative = Direction::Up;
                        self.tail.move_pos(&Move::new(Direction::UpLeft, 1));
                        self.tail_visited.insert(self.tail);
                    }
                    Direction::Left => {
                        self.head_relative = Direction::Left;
                        self.tail.move_pos(&Move::new(Direction::UpLeft, 1));
                        self.tail_visited.insert(self.tail);
                    }
                    Direction::Down => {
                        self.head_relative = Direction::Left;
                    }
                    Direction::Right => {
                        self.head_relative = Direction::Up;
                    }
                    _ => {
                        println!("Invalid move");
                    }
                },
                Direction::UpRight => match m.direction {
                    Direction::Up => {
                        self.head_relative = Direction::Up;
                        self.tail.move_pos(&Move::new(Direction::UpRight, 1));
                        self.tail_visited.insert(self.tail);
                    }
                    Direction::Right => {
                        self.head_relative = Direction::Right;
                        self.tail.move_pos(&Move::new(Direction::UpRight, 1));
                        self.tail_visited.insert(self.tail);
                    }
                    Direction::Down => {
                        self.head_relative = Direction::Right;
                    }
                    Direction::Left => {
                        self.head_relative = Direction::Up;
                    }
                    _ => {
                        println!("Invalid move");
                    }
                },
                Direction::DownLeft => match m.direction {
                    Direction::Down => {
                        self.head_relative = Direction::Down;
                        self.tail.move_pos(&Move::new(Direction::DownLeft, 1));
                        self.tail_visited.insert(self.tail);
                    }
                    Direction::Left => {
                        self.head_relative = Direction::Left;
                        self.tail.move_pos(&Move::new(Direction::DownLeft, 1));
                        self.tail_visited.insert(self.tail);
                    }
                    Direction::Up => {
                        self.head_relative = Direction::Left;
                    }
                    Direction::Right => {
                        self.head_relative = Direction::Down;
                    }
                    _ => {
                        println!("Invalid move");
                    }
                },
                Direction::DownRight => match m.direction {
                    Direction::Down => {
                        self.head_relative = Direction::Down;
                        self.tail.move_pos(&Move::new(Direction::DownRight, 1));
                        self.tail_visited.insert(self.tail);
                    }
                    Direction::Right => {
                        self.head_relative = Direction::Right;
                        self.tail.move_pos(&Move::new(Direction::DownRight, 1));
                        self.tail_visited.insert(self.tail);
                    }
                    Direction::Up => {
                        self.head_relative = Direction::Right;
                    }
                    Direction::Left => {
                        self.head_relative = Direction::Down;
                    }
                    _ => {
                        println!("Invalid move");
                    }
                },
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<Move> {
    let mut result = Vec::new();

    for line in input.lines() {
        println!("Line: {}", line);
        let mut iter = line.split(' ');
        let direction: Direction = iter.next().unwrap().into();
        let steps = iter.next().unwrap().parse::<i32>().unwrap();
        result.push(Move { direction, steps });
    }
    result
}

pub fn run() {
    println!("Day 8");

    let input = include_str!("../input/9");
    let moves = parse_input(input);
    let mut rope = RopeBridge::new();

    for m in moves {
        rope.move_rope(&m);
    }

    let num_visited_nodes = rope.tail_visited.iter().count();

    println!("Number of visited nodes: {}", num_visited_nodes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = include_str!("../input/test9");
        let result = parse_input(input);

        assert_eq!(
            result[0],
            Move {
                direction: Direction::Right,
                steps: 4
            }
        );
    }

    #[test]
    fn test_parse_input_real() {
        let input = include_str!("../input/9");
        let result = parse_input(input);

        assert_eq!(
            result[0],
            Move {
                direction: Direction::Down,
                steps: 1
            }
        );
    }

    #[test]
    fn test_input_move_rope() {
        let input = include_str!("../input/test9");
        let moves = parse_input(input);
        let mut rope = RopeBridge::new();

        for m in moves {
            rope.move_rope(&m);
        }

        assert_eq!(rope.tail, Pos((1, 2)));

        let num_visited_nodes = rope.tail_visited.iter().count();
        assert_eq!(num_visited_nodes, 13);
    }

    #[test]
    fn test_move_rope() {
        let mut rope = RopeBridge {
            head_relative: Direction::On,
            tail: Pos::default(),
            tail_visited: HashSet::new(),
        };

        rope.move_rope(&Move::new(Direction::Right, 4));
        assert_eq!(rope.tail, Pos((3, 0)));
    }
}
