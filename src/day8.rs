use std::{
    fmt::{Display, Formatter, Result},
    iter::Rev,
    ops::Range,
};

#[derive(Debug)]
struct VisableTree {
    height: i32,
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
}

impl VisableTree {
    fn new(height: i32) -> Self {
        VisableTree {
            height,
            top: false,
            bottom: false,
            left: false,
            right: false,
        }
    }
}

#[derive(Debug)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

impl Direction {}

struct Trees(Vec<Vec<VisableTree>>);

#[derive(Debug, Clone)]
enum RangeDirection {
    Forward(Range<usize>),
    Reverse(Rev<Range<usize>>),
}

impl Iterator for RangeDirection {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            RangeDirection::Forward(range) => range.next(),
            RangeDirection::Reverse(range) => range.next(),
        }
    }
}

impl Trees {
    fn ranges(&self, dir: &Direction) -> (RangeDirection, RangeDirection) {
        let rows = self.0.len();
        let cols = self.0[0].len();
        match dir {
            Direction::Top => {
                let first = RangeDirection::Forward(0..cols);
                let second = RangeDirection::Forward(0..rows);
                (first, second)
            }
            Direction::Bottom => {
                let first = RangeDirection::Forward(0..cols);
                let second = RangeDirection::Reverse((0..rows).rev());
                (first, second)
            }
            Direction::Left => {
                let first = RangeDirection::Forward(0..rows);
                let second = RangeDirection::Forward(0..cols);
                (first, second)
            }
            Direction::Right => {
                let first = RangeDirection::Forward(0..rows);
                let second = RangeDirection::Reverse((0..cols).rev());
                (first, second)
            }
        }
    }

    fn check_visibility(&mut self) {
        let dirs = vec![
            Direction::Top,
            Direction::Bottom,
            Direction::Left,
            Direction::Right,
        ];

        for dir in dirs {
            let (first, second) = self.ranges(&dir);

            for a in first {
                let mut tallest_tree: i32 = -1;
                for b in second.clone() {
                    let height = match dir {
                        Direction::Top => self.0[b][a].height,
                        Direction::Bottom => self.0[b][a].height,
                        Direction::Left => self.0[a][b].height,
                        Direction::Right => self.0[a][b].height,
                    };
                    if height > tallest_tree {
                        match dir {
                            Direction::Top => self.0[b][a].top = true,
                            Direction::Bottom => self.0[b][a].bottom = true,
                            Direction::Left => self.0[a][b].left = true,
                            Direction::Right => self.0[a][b].right = true,
                        }
                        tallest_tree = height;
                    }
                }
            }
        }
    }

    fn visible_from_dir(&self, dir: &Direction) {
        println!("{:?}", dir);
        for row in self.0.iter() {
            for tree in row.iter() {
                match dir {
                    Direction::Top => {
                        if tree.top {
                            print!("X");
                        } else {
                            print!(" ");
                        }
                    }
                    Direction::Bottom => {
                        if tree.bottom {
                            print!("X");
                        } else {
                            print!(" ");
                        }
                    }
                    Direction::Left => {
                        if tree.left {
                            print!("X");
                        } else {
                            print!(" ");
                        }
                    }
                    Direction::Right => {
                        if tree.right {
                            print!("X");
                        } else {
                            print!(" ");
                        }
                    }
                }
            }
            println!();
        }
        println!("");
    }
}

impl Display for Trees {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for row in self.0.iter() {
            for tree in row.iter() {
                if tree.top || tree.bottom || tree.left || tree.right {
                    write!(f, "X")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_input(input: &str) -> Trees {
    Trees(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|num| VisableTree::new(num.to_digit(10).unwrap() as i32))
                    .collect::<Vec<VisableTree>>()
            })
            .collect::<Vec<Vec<VisableTree>>>(),
    )
}

fn part1() {
    let input = include_str!("../input/8");
    let mut trees = parse_input(input);
    trees.check_visibility();

    let count = trees
        .0
        .iter()
        .map(|row| {
            row.iter()
                .filter(|tree| tree.top || tree.bottom || tree.left || tree.right)
                .count()
        })
        .sum::<usize>();

    trees.visible_from_dir(&Direction::Top);
    trees.visible_from_dir(&Direction::Bottom);
    trees.visible_from_dir(&Direction::Left);
    trees.visible_from_dir(&Direction::Right);

    println!("{}", trees);
    println!("count: {}", count);
}

pub fn run() {
    part1();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let input = include_str!("../input/test8");
        let mut trees = parse_input(input);
        trees.check_visibility();

        let count = trees
            .0
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|tree| tree.top || tree.bottom || tree.left || tree.right)
                    .count()
            })
            .sum::<usize>();

        trees.visible_from_dir(&Direction::Top);
        trees.visible_from_dir(&Direction::Bottom);
        trees.visible_from_dir(&Direction::Left);
        trees.visible_from_dir(&Direction::Right);

        println!("{}", trees);
        println!("count: {}", count);
        assert!(count == 21);
    }

    #[test]
    fn test_other_input() {
        let input = include_str!("../input/test8_2");
        let mut trees = parse_input(input);
        trees.check_visibility();

        let count = trees
            .0
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|tree| tree.top || tree.bottom || tree.left || tree.right)
                    .count()
            })
            .sum::<usize>();

        trees.visible_from_dir(&Direction::Top);
        trees.visible_from_dir(&Direction::Bottom);
        trees.visible_from_dir(&Direction::Left);
        trees.visible_from_dir(&Direction::Right);

        assert!(count == 65);
    }
}
