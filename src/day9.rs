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

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::UpLeft => Direction::DownRight,
            Direction::Up => Direction::Down,
            Direction::UpRight => Direction::DownLeft,
            Direction::Left => Direction::Right,
            Direction::On => Direction::On,
            Direction::Right => Direction::Left,
            Direction::DownLeft => Direction::UpRight,
            Direction::Down => Direction::Up,
            Direction::DownRight => Direction::UpLeft,
        }
    }
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

struct RopeSegment {
    head_relative: Direction,
    tail: Pos,
    tail_visited: HashSet<Pos>,
}

impl RopeSegment {
    fn new() -> Self {
        let mut rope = RopeSegment {
            head_relative: Direction::On,
            tail: Pos::default(),
            tail_visited: HashSet::new(),
        };
        rope.tail_visited.insert(Pos::default());
        rope
    }
    fn move_segment(&mut self, m: &Move) -> Move {
        let mut tail_move = Move::new(Direction::On, 0);

        for _ in 0..m.steps {
            match &self.head_relative {
                Direction::On => {
                    self.head_relative = m.direction;
                }
                same_dir if *same_dir == m.direction => {
                    tail_move = Move::new(m.direction, 1);
                }
                opp0site if *opp0site == m.direction.opposite() => {
                    self.head_relative = Direction::On;
                }
                Direction::Up => match m.direction {
                    Direction::Left => {
                        self.head_relative = Direction::UpLeft;
                    }
                    Direction::Right => {
                        self.head_relative = Direction::UpRight;
                    }
                    Direction::UpRight => {
                        tail_move = Move::new(Direction::UpRight, 1);
                    }
                    Direction::UpLeft => {
                        tail_move = Move::new(Direction::UpLeft, 1);
                    }
                    Direction::DownRight => {
                        self.head_relative = Direction::Right;
                    }
                    Direction::DownLeft => {
                        self.head_relative = Direction::Left;
                    }
                    _ => {
                        panic!("Invalid move from Up, {:?}", m.direction);
                    }
                },
                Direction::Down => match m.direction {
                    Direction::Left => {
                        self.head_relative = Direction::DownLeft;
                    }
                    Direction::Right => {
                        self.head_relative = Direction::DownRight;
                    }
                    Direction::DownLeft => {
                        tail_move = Move::new(Direction::DownLeft, 1);
                    }
                    Direction::DownRight => {
                        tail_move = Move::new(Direction::DownRight, 1);
                    }
                    Direction::UpLeft => {
                        self.head_relative = Direction::Left;
                    }
                    Direction::UpRight => {
                        self.head_relative = Direction::Right;
                    }
                    _ => {
                        panic!("Invalid move from Down, {:?}", m.direction);
                    }
                },
                Direction::Left => match m.direction {
                    Direction::Up => {
                        self.head_relative = Direction::UpLeft;
                    }
                    Direction::Down => {
                        self.head_relative = Direction::DownLeft;
                    }
                    Direction::UpLeft => {
                        tail_move = Move::new(Direction::UpLeft, 1);
                    }
                    Direction::DownLeft => {
                        tail_move = Move::new(Direction::DownLeft, 1);
                    }
                    Direction::UpRight => {
                        self.head_relative = Direction::Up;
                    }
                    Direction::DownRight => {
                        self.head_relative = Direction::Down;
                    }
                    _ => {
                        panic!("Invalid move from Left, {:?}", m.direction);
                    }
                },
                Direction::Right => match m.direction {
                    Direction::Up => {
                        self.head_relative = Direction::UpRight;
                    }
                    Direction::Down => {
                        self.head_relative = Direction::DownRight;
                    }
                    Direction::UpRight => {
                        tail_move = Move::new(Direction::UpRight, 1);
                    }
                    Direction::DownRight => {
                        tail_move = Move::new(Direction::DownRight, 1);
                    }
                    Direction::UpLeft => {
                        self.head_relative = Direction::Up;
                    }
                    Direction::DownLeft => {
                        self.head_relative = Direction::Down;
                    }
                    _ => {
                        panic!("Invalid move from Right, {:?}", m.direction);
                    }
                },
                Direction::UpLeft => match m.direction {
                    Direction::Up => {
                        self.head_relative = Direction::Up;
                        tail_move = Move::new(Direction::UpLeft, 1);
                    }
                    Direction::Left => {
                        self.head_relative = Direction::Left;
                        tail_move = Move::new(Direction::UpLeft, 1);
                    }
                    Direction::Down => {
                        self.head_relative = Direction::Left;
                    }
                    Direction::Right => {
                        self.head_relative = Direction::Up;
                    }
                    Direction::UpRight => {
                        self.head_relative = Direction::Up;
                        tail_move = Move::new(Direction::Up, 1);
                    }
                    Direction::DownLeft => {
                        self.head_relative = Direction::Left;
                        tail_move = Move::new(Direction::Left, 1);
                    }
                    _ => {
                        println!("Invalid move");
                    }
                },
                Direction::UpRight => match m.direction {
                    Direction::Up => {
                        self.head_relative = Direction::Up;
                        tail_move = Move::new(Direction::UpRight, 1);
                    }
                    Direction::Right => {
                        self.head_relative = Direction::Right;
                        tail_move = Move::new(Direction::UpRight, 1);
                    }
                    Direction::Down => {
                        self.head_relative = Direction::Right;
                    }
                    Direction::Left => {
                        self.head_relative = Direction::Up;
                    }
                    Direction::UpLeft => {
                        self.head_relative = Direction::Up;
                        tail_move = Move::new(Direction::Up, 1);
                    }
                    Direction::DownRight => {
                        self.head_relative = Direction::Right;
                        tail_move = Move::new(Direction::Right, 1);
                    }
                    _ => {
                        println!("Invalid move from UpRight, {:?}", m.direction);
                    }
                },
                Direction::DownLeft => match m.direction {
                    Direction::Down => {
                        self.head_relative = Direction::Down;
                        tail_move = Move::new(Direction::DownLeft, 1);
                    }
                    Direction::Left => {
                        self.head_relative = Direction::Left;
                        tail_move = Move::new(Direction::DownLeft, 1);
                    }
                    Direction::Up => {
                        self.head_relative = Direction::Left;
                    }
                    Direction::Right => {
                        self.head_relative = Direction::Down;
                    }
                    Direction::DownRight => {
                        self.head_relative = Direction::Down;
                        tail_move = Move::new(Direction::Down, 1);
                    }
                    Direction::UpLeft => {
                        self.head_relative = Direction::Left;
                        tail_move = Move::new(Direction::Left, 1);
                    }
                    _ => {
                        println!("Invalid move from DownLeft, {:?}", m.direction);
                    }
                },
                Direction::DownRight => match m.direction {
                    Direction::Down => {
                        self.head_relative = Direction::Down;
                        tail_move = Move::new(Direction::DownRight, 1);
                    }
                    Direction::Right => {
                        self.head_relative = Direction::Right;
                        tail_move = Move::new(Direction::DownRight, 1);
                    }
                    Direction::Up => {
                        self.head_relative = Direction::Right;
                    }
                    Direction::Left => {
                        self.head_relative = Direction::Down;
                    }
                    Direction::DownLeft => {
                        self.head_relative = Direction::Down;
                        tail_move = Move::new(Direction::Down, 1);
                    }
                    Direction::UpRight => {
                        self.head_relative = Direction::Right;
                        tail_move = Move::new(Direction::Right, 1);
                    }
                    _ => {
                        println!("Invalid move from DownRight, {:?}", m.direction);
                    }
                },
            }
            self.tail.move_pos(&tail_move);
            self.tail_visited.insert(self.tail);
        }
        tail_move
    }
}

fn parse_input(input: &str) -> Vec<Move> {
    let mut result = Vec::new();

    for line in input.lines() {
        let mut iter = line.split(' ');
        let direction: Direction = iter.next().unwrap().into();
        let steps = iter.next().unwrap().parse::<i32>().unwrap();
        for _ in 0..steps {
            result.push(Move {
                direction,
                steps: 1,
            });
        }
    }
    result
}

pub fn part1() {
    println!("Part 1");

    let input = include_str!("../input/9");
    let moves = parse_input(input);
    let mut rope = RopeSegment::new();

    for m in moves {
        let _ = rope.move_segment(&m);
    }

    let num_visited_nodes = rope.tail_visited.iter().count();

    println!("Number of visited nodes: {}", num_visited_nodes);
}

fn part2() {
    let mut rope = [
        RopeSegment::new(),
        RopeSegment::new(),
        RopeSegment::new(),
        RopeSegment::new(),
        RopeSegment::new(),
        RopeSegment::new(),
        RopeSegment::new(),
        RopeSegment::new(),
        RopeSegment::new(),
    ];

    let input = include_str!("../input/9");
    let moves = parse_input(input);

    for m in moves {
        let mut next_segment_move = m;
        rope.iter_mut().for_each(|r| {
            if next_segment_move.direction != Direction::On {
                next_segment_move = r.move_segment(&next_segment_move);
            }
        });
    }
    let visited_tail_nodes = rope[8].tail_visited.iter().count();
    println!("Part 2");
    println!("Number of visited nodes: {}", visited_tail_nodes);
}

pub fn run() {
    println!("Day 8");
    part1();
    part2();
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
                steps: 1
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
    fn test_input_move_rope_segment() {
        let input = include_str!("../input/test9");
        let moves = parse_input(input);
        let mut rope = RopeSegment::new();

        for m in moves {
            rope.move_segment(&m);
        }

        assert_eq!(rope.tail, Pos((1, 2)));

        let num_visited_nodes = rope.tail_visited.iter().count();
        assert_eq!(num_visited_nodes, 13);
    }

    #[test]
    fn test_move_rope_segment() {
        let mut rope = RopeSegment {
            head_relative: Direction::On,
            tail: Pos::default(),
            tail_visited: HashSet::new(),
        };

        rope.move_segment(&Move::new(Direction::Right, 4));
        assert_eq!(rope.tail, Pos((3, 0)));
    }

    #[test]
    fn test_move_rope() {
        let mut rope = [
            RopeSegment::new(),
            RopeSegment::new(),
            RopeSegment::new(),
            RopeSegment::new(),
            RopeSegment::new(),
            RopeSegment::new(),
            RopeSegment::new(),
            RopeSegment::new(),
            RopeSegment::new(),
            RopeSegment::new(),
        ];

        let input = include_str!("../input/test9");
        let moves = parse_input(input);

        for m in moves {
            let mut next_segment_move = m;

            println!("\n\nHead Move: {:?}", &next_segment_move.direction);

            rope.iter_mut().enumerate().for_each(|(i, r)| {
                if next_segment_move.direction == Direction::On {
                } else {
                    println!(
                        "Segment: {} Previous tail moved: {:?}",
                        i, &next_segment_move.direction
                    );
                    next_segment_move = r.move_segment(&next_segment_move);
                    println!("Segment: {} New tail: {:?}", i, r.tail);
                }
            });
        }

        for r in rope.iter() {
            println!("Rope: {:?}", r.tail);
        }

        assert_eq!(rope[0].tail, Pos((1, 2)));
        assert_eq!(rope[1].tail, Pos((2, 2)));
        assert_eq!(rope[2].tail, Pos((3, 2)));
        assert_eq!(rope[3].tail, Pos((2, 2)));

        let visited_tail_nodes = rope[9].tail_visited.iter().count();
        assert_eq!(visited_tail_nodes, 1);
    }
    #[test]
    fn test_move_rope2() {
        let mut rope = [
            RopeSegment::new(),
            RopeSegment::new(),
            RopeSegment::new(),
            RopeSegment::new(),
            RopeSegment::new(),
            RopeSegment::new(),
            RopeSegment::new(),
            RopeSegment::new(),
            RopeSegment::new(),
        ];

        let input = include_str!("../input/test9_2");
        let moves = parse_input(input);

        for m in moves {
            // Create a 30x30 grid for visualizing positions from -15 to -15
            let mut grid = [[0; 30]; 30];
            let mut next_segment_move = m;

            println!("\n\nHead Move: {:?}", &next_segment_move.direction);

            rope.iter_mut().enumerate().for_each(|(i, r)| {
                if next_segment_move.direction == Direction::On {
                    grid[(r.tail.0 .0 + 15) as usize][(r.tail.0 .1 + 15) as usize] = i + 1;
                } else {
                    println!(
                        "Segment: {} Previous tail moved: {:?}",
                        i, &next_segment_move.direction
                    );
                    next_segment_move = r.move_segment(&next_segment_move);
                    println!("Segment: {} New tail: {:?}", i, r.tail);
                    println!("X: {}, Y: {}", r.tail.0 .0, r.tail.0 .1);
                    grid[(r.tail.0 .0 + 15) as usize][(r.tail.0 .1 + 15) as usize] = i + 1;
                }
            });

            grid[15][15] = 10;

            for i in (0..30).rev() {
                for j in 0..30 {
                    if grid[j][i] == 0 {
                        if i == 0 {
                            print!("-");
                            continue;
                        }
                        if j == 0 {
                            print!("|");
                            continue;
                        }
                        print!(".");
                    } else if grid[j][i] == 10 {
                        print!("s");
                    } else {
                        print!("{}", grid[j][i]);
                    }
                }
                println!();
            }
        }

        for r in rope.iter() {
            println!("Rope: {:?}", r.tail);
        }

        let mut grid = [[0; 30]; 30];

        rope[8].tail_visited.iter().for_each(|p| {
            grid[(p.0 .0 + 15) as usize][(p.0 .1 + 15) as usize] = 1;
        });

        for i in (0..30).rev() {
            for j in 0..30 {
                if grid[j][i] == 0 {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            println!();
        }

        let visited_tail_nodes = rope[8].tail_visited.iter().count();
        assert_eq!(visited_tail_nodes, 36);
    }

    #[test]
    fn test_move_rope3() {
        let mut rope = [
            RopeSegment::new(),
            RopeSegment::new(),
            RopeSegment::new(),
            RopeSegment::new(),
            RopeSegment::new(),
            RopeSegment::new(),
            RopeSegment::new(),
            RopeSegment::new(),
            RopeSegment::new(),
        ];

        let input = include_str!("../input/test9_3");
        let moves = parse_input(input);

        for m in moves {
            // Create a 30x30 grid for visualizing positions from -15 to -15
            let mut grid = [[0; 30]; 30];
            let mut next_segment_move = m;

            println!("\n\nHead Move: {:?}", &next_segment_move.direction);

            rope.iter_mut().enumerate().for_each(|(i, r)| {
                if next_segment_move.direction == Direction::On {
                    grid[(r.tail.0 .0 + 15) as usize][(r.tail.0 .1 + 15) as usize] = i + 1;
                } else {
                    println!(
                        "Segment: {} Previous tail moved: {:?}",
                        i, &next_segment_move.direction
                    );
                    next_segment_move = r.move_segment(&next_segment_move);
                    println!("Segment: {} New tail: {:?}", i, r.tail);
                    println!("X: {}, Y: {}", r.tail.0 .0, r.tail.0 .1);
                    grid[(r.tail.0 .0 + 15) as usize][(r.tail.0 .1 + 15) as usize] = i + 1;
                }
            });

            grid[15][15] = 10;

            for i in (0..30).rev() {
                for j in 0..30 {
                    if grid[j][i] == 0 {
                        if i == 0 {
                            print!("-");
                            continue;
                        }
                        if j == 0 {
                            print!("|");
                            continue;
                        }
                        print!(".");
                    } else if grid[j][i] == 10 {
                        print!("s");
                    } else {
                        print!("{}", grid[j][i]);
                    }
                }
                println!();
            }
        }

        for r in rope.iter() {
            println!("Rope: {:?}", r.tail);
        }

        let mut grid = [[0; 30]; 30];

        rope[8].tail_visited.iter().for_each(|p| {
            grid[(p.0 .0 + 15) as usize][(p.0 .1 + 15) as usize] = 1;
        });

        for i in (0..30).rev() {
            for j in 0..30 {
                if grid[j][i] == 0 {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            println!();
        }

        let visited_tail_nodes = rope[8].tail_visited.iter().count();
        assert_eq!(visited_tail_nodes, 8);
    }
}
