use std::{collections::HashMap, fmt::Display};

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
    Down,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid direction: {}", c),
        }
    }
}

struct JetStream {
    jet_stream: Vec<Direction>,
    position: usize,
}

impl JetStream {
    fn new(input: &str) -> Self {
        let mut jet_stream = Vec::new();

        for c in input.chars() {
            jet_stream.push(Direction::from(c));
        }
        JetStream {
            jet_stream,
            position: 0,
        }
    }
    fn next(&mut self) -> &Direction {
        let jet_stream_direction = self.jet_stream.get(self.position).unwrap();

        self.position += 1;
        if self.position >= self.jet_stream.len() {
            self.position = 0;
        }
        jet_stream_direction
    }
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
enum Rock {
    Horizontal,
    Plus,
    ReverseL,
    Vertical,
    Square,
}

impl Rock {
    fn height(&self) -> usize {
        match self {
            Rock::Horizontal => 1,
            Rock::Plus => 3,
            Rock::ReverseL => 3,
            Rock::Vertical => 4,
            Rock::Square => 2,
        }
    }

    fn width(&self) -> usize {
        match self {
            Rock::Horizontal => 4,
            Rock::Plus => 3,
            Rock::ReverseL => 3,
            Rock::Vertical => 1,
            Rock::Square => 2,
        }
    }

    fn get(&self) -> Vec<Vec<usize>> {
        match self {
            Rock::Horizontal => {
                vec![vec![1, 1, 1, 1]]
            }
            Rock::Plus => {
                vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]]
            }
            Rock::Vertical => {
                vec![vec![1], vec![1], vec![1], vec![1]]
            }
            Rock::ReverseL => {
                vec![vec![1, 1, 1], vec![0, 0, 1], vec![0, 0, 1]]
            }
            Rock::Square => {
                vec![vec![1, 1], vec![1, 1]]
            }
        }
    }
}

struct Rocks {
    rocks: Vec<Rock>,
    position: usize,
}

impl Rocks {
    fn new() -> Self {
        let rocks = vec![
            Rock::Horizontal,
            Rock::Plus,
            Rock::ReverseL,
            Rock::Vertical,
            Rock::Square,
        ];
        Rocks { rocks, position: 0 }
    }
    fn next(&mut self) -> &Rock {
        let rocks = self.rocks.get(self.position).unwrap();

        self.position += 1;
        if self.position >= self.rocks.len() {
            self.position = 0;
        }
        rocks
    }
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
enum ChamberPos {
    Air,
    Rock,
}

#[derive(Eq, PartialEq, Hash)]
struct Cache(usize, Rock, [Vec<ChamberPos>; 12]);

struct Chamber {
    height: usize,
    width: usize,
    chamber: Vec<Vec<ChamberPos>>,
    rocks_dropped: u64,
    repeating_sequence: Option<(usize, u64)>,
}

impl Chamber {
    fn new(width: usize, height: usize) -> Self {
        let mut chamber = Vec::new();

        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(ChamberPos::Air);
            }
            chamber.push(row);
        }

        Chamber {
            height,
            width,
            chamber,
            repeating_sequence: None,
            rocks_dropped: 0,
        }
    }

    fn increase_height(&mut self, min_height: usize) {
        let delta_height = min_height as i32 - self.height as i32;

        if delta_height <= 0 {
            return;
        }

        for _ in 0..delta_height {
            let mut row = Vec::new();
            for _ in 0..self.width {
                row.push(ChamberPos::Air);
            }
            self.chamber.push(row);
            self.height += 1;
        }
    }

    fn valid_move(&self, rock: &Rock, height: usize, pos: usize, direction: &Direction) -> bool {
        let r = rock.get();
        // println!("height: {:?}, pos: {:?}", height, pos);
        // println!("rock: {:?}", r);
        // println!("direction: {:?}", direction);

        let (dy, dx): (i32, i32) = match direction {
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Down => (-1, 0),
        };

        for (i, row) in r.iter().enumerate() {
            for (j, element) in row.iter().enumerate() {
                let y = usize::try_from((i + height) as i32 + dy).ok();
                let x = usize::try_from((j + pos) as i32 + dx).ok();

                // println!("y: {:?}, x: {:?}", y, x);

                match (x, y) {
                    (Some(x), Some(y)) => {
                        if x >= self.width || y == 0 {
                            // println!("OUTSIDE");
                            return false;
                        }

                        if y <= self.height
                            && *element == 1
                            && self.chamber[y - 1][x] == ChamberPos::Rock
                        {
                            // println!("BONK");
                            return false;
                        }
                    }
                    _ => return false,
                }
            }
        }
        // println!("Valid move\n");
        true
    }

    fn drop_rock(
        &mut self,
        rock: &Rock,
        pos: usize,
        jetstream: Option<&mut JetStream>,
        cache: &mut HashMap<Cache, (usize, u64)>,
    ) {
        let mut binding = JetStream::new("");
        let stream = jetstream.unwrap_or(&mut binding);

        let height = self.height;
        let mut pos = pos;

        for (h, _) in self.chamber.iter().rev().enumerate() {
            if stream.jet_stream.len() > 0 {
                let direction = stream.next();
                if self.valid_move(rock, height - h, pos, direction) {
                    match direction {
                        Direction::Left => pos -= 1,
                        Direction::Right => pos += 1,
                        _ => {}
                    }
                };
            }

            let valid_move = self.valid_move(rock, height - h, pos, &Direction::Down);
            if !valid_move {
                // println!("Hit bottom");
                self.insert_rock(&rock, pos, height - h - 1);
                let top_12_rows = self.chamber.last_chunk::<12>();

                match top_12_rows {
                    None => return,
                    Some(top_12_rows) => {
                        let mem = Cache(stream.position, rock.clone(), top_12_rows.clone());

                        if let Some((start_height, start_rocks_dropped)) = cache.get(&mem) {
                            let rocks = self.rocks_dropped - start_rocks_dropped;
                            let end_height = self.height;
                            self.repeating_sequence = Some((end_height - *start_height, rocks));
                        }

                        cache.insert(mem, (self.height, self.rocks_dropped));
                    }
                }

                return;
            }
        }
        //insert rock at bottom
        self.insert_rock(&rock, pos, 0);
    }

    fn insert_rock(&mut self, rock: &Rock, pos: usize, height: usize) {
        // println!("Inserting at height: {:?}, pos:  {:?}", height, pos);
        match rock {
            Rock::Horizontal => {
                self.chamber[height][pos] = ChamberPos::Rock;
                self.chamber[height][pos + 1] = ChamberPos::Rock;
                self.chamber[height][pos + 2] = ChamberPos::Rock;
                self.chamber[height][pos + 3] = ChamberPos::Rock;
            }
            Rock::Plus => {
                self.chamber[height][pos + 1] = ChamberPos::Rock;
                self.chamber[height + 1][pos] = ChamberPos::Rock;
                self.chamber[height + 2][pos + 1] = ChamberPos::Rock;
                self.chamber[height + 1][pos + 1] = ChamberPos::Rock;
                self.chamber[height + 1][pos + 2] = ChamberPos::Rock;
            }
            Rock::ReverseL => {
                self.chamber[height][pos] = ChamberPos::Rock;
                self.chamber[height][pos + 1] = ChamberPos::Rock;
                self.chamber[height][pos + 2] = ChamberPos::Rock;
                self.chamber[height + 1][pos + 2] = ChamberPos::Rock;
                self.chamber[height + 2][pos + 2] = ChamberPos::Rock;
            }
            Rock::Vertical => {
                self.chamber[height][pos] = ChamberPos::Rock;
                self.chamber[height + 1][pos] = ChamberPos::Rock;
                self.chamber[height + 2][pos] = ChamberPos::Rock;
                self.chamber[height + 3][pos] = ChamberPos::Rock;
            }
            Rock::Square => {
                self.chamber[height][pos] = ChamberPos::Rock;
                self.chamber[height][pos + 1] = ChamberPos::Rock;
                self.chamber[height + 1][pos] = ChamberPos::Rock;
                self.chamber[height + 1][pos + 1] = ChamberPos::Rock;
            }
        }

        self.increase_height(height + rock.height() + 4);
        self.rocks_dropped += 1;
        // println!("{}", self);
    }
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        for row in self.chamber.iter().rev() {
            for col in row {
                match col {
                    ChamberPos::Air => output.push('.'),
                    ChamberPos::Rock => output.push('#'),
                }
            }
            output.push('\n');
        }
        write!(f, "{}", output)
    }
}

fn part1(input: &str) -> usize {
    let mut jet_stream = JetStream::new(input);
    let mut rocks = Rocks::new();

    let mut chamber = Chamber::new(7, 4);

    let mut cache = HashMap::new();
    for _ in 0..2022 {
        chamber.drop_rock(rocks.next(), 2, Some(&mut jet_stream), &mut cache);
    }

    chamber.height - 4
}

fn part2(input: &str) -> u64 {
    let mut jet_stream = JetStream::new(input);
    let mut rocks = Rocks::new();

    let mut chamber = Chamber::new(7, 4);

    let mut cache = HashMap::new();

    let mut cycle = None;
    for _ in 0..3000 {
        chamber.drop_rock(rocks.next(), 2, Some(&mut jet_stream), &mut cache);

        if let Some(sequence) = chamber.repeating_sequence {
            println!("Cycle found.");
            cycle = Some(sequence);
            break;
        }
    }
    let rocks_dropped = chamber.rocks_dropped;
    let mut height = 0;

    if let Some((d_height, d_rocks)) = cycle {
        let num_cycles = (1000000000000 - rocks_dropped) / d_rocks;

        height += (d_height) as u64 * num_cycles;

        chamber.rocks_dropped += d_rocks * num_cycles;
        while chamber.rocks_dropped < 1000000000000 {
            chamber.drop_rock(rocks.next(), 2, Some(&mut jet_stream), &mut cache);
        }
    }

    chamber.height as u64 - 4 + height
}

pub fn run() {
    let mut input = include_str!("../input/17");
    input = input.strip_suffix("\n").unwrap();
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rocks() {
        let mut rocks = Rocks::new();

        assert_eq!(rocks.next(), &Rock::Horizontal);
        assert_eq!(rocks.next(), &Rock::Plus);
        assert_eq!(rocks.next(), &Rock::ReverseL);
        assert_eq!(rocks.next(), &Rock::Vertical);
        assert_eq!(rocks.next(), &Rock::Square);
        assert_eq!(rocks.next(), &Rock::Horizontal);
    }

    #[test]
    fn test_jetstream() {
        let mut jet_stream = JetStream::new("<<>>");

        assert_eq!(jet_stream.next(), &Direction::Left);
        assert_eq!(jet_stream.next(), &Direction::Left);
        assert_eq!(jet_stream.next(), &Direction::Right);
        assert_eq!(jet_stream.next(), &Direction::Right);
        assert_eq!(jet_stream.next(), &Direction::Left);
    }

    #[test]
    fn test_drop_rock() {
        let mut chamber = Chamber::new(7, 12);
        let mut cache = HashMap::new();
        println!("{}", chamber);

        println!("Drop rock: {:?}", Rock::Horizontal);
        chamber.drop_rock(&Rock::Horizontal, 2, None, &mut cache);
        println!("{}", chamber);

        println!("Drop rock: {:?}", Rock::Plus);
        chamber.drop_rock(&Rock::Plus, 2, None, &mut cache);
        println!("{}", chamber);

        assert_eq!(chamber.height, 12);

        println!("Drop rock: {:?}", Rock::ReverseL);
        chamber.drop_rock(&Rock::ReverseL, 2, None, &mut cache);
        println!("{}", chamber);
        assert_eq!(chamber.height, 12);

        println!("Drop rock: {:?}", Rock::Vertical);
        chamber.drop_rock(&Rock::Vertical, 2, None, &mut cache);
        println!("{}", chamber);
        assert_eq!(chamber.height, 13);

        println!("Drop rock: {:?}", Rock::Square);
        chamber.drop_rock(&Rock::Square, 2, None, &mut cache);
        println!("{}", chamber);
        assert_eq!(chamber.height, 15);
    }
    #[test]
    fn test_drop_rock_2() {
        let mut chamber = Chamber::new(7, 12);
        let mut cache = HashMap::new();
        println!("{}", chamber);

        println!("Drop rock: {:?}", Rock::Vertical);
        chamber.drop_rock(&Rock::Vertical, 2, None, &mut cache);
        println!("{}", chamber);

        println!("Drop rock: {:?}", Rock::Plus);
        chamber.drop_rock(&Rock::Plus, 0, None, &mut cache);
        println!("{}", chamber);

        assert_eq!(chamber.height, 12);
    }
    #[test]
    fn test_part1() {
        let mut input = include_str!("../input/test17");
        input = input.strip_suffix("\n").unwrap();
        assert_eq!(part1(input), 3068);
    }
    //
    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2("3"), 1222153);
    // }
}
