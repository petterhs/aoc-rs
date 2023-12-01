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

    fn iterate_2d_matrix(
        &self,
        start_row: usize,
        start_col: usize,
    ) -> (
        impl Iterator<Item = &VisableTree>,
        impl Iterator<Item = &VisableTree>,
        impl Iterator<Item = &VisableTree>,
        impl Iterator<Item = &VisableTree>,
    ) {
        let num_rows = self.0.len();
        let num_cols = if num_rows > 0 { self.0[0].len() } else { 0 };

        let up_iter = (0..start_row).rev().map(move |r| &self.0[r][start_col]);
        let down_iter = (start_row + 1..num_rows).map(move |r| &self.0[r][start_col]);
        let left_iter = (0..start_col).rev().map(move |c| &self.0[start_row][c]);
        let right_iter = (start_col + 1..num_cols).map(move |c| &self.0[start_row][c]);

        (up_iter, down_iter, left_iter, right_iter)
    }

    fn calculate_max_scenic_score(&mut self) -> i32 {
        self.0
            .iter()
            .enumerate()
            .map(|(row_index, row)| {
                row.iter()
                    .enumerate()
                    .map(|(col_index, this_tree)| {
                        let (up_iter, down_iter, left_iter, right_iter) =
                            self.iterate_2d_matrix(row_index, col_index);

                        let mut up_trees = 0;
                        for tree in up_iter {
                            up_trees += 1;
                            if tree.height >= this_tree.height {
                                break;
                            }
                        }

                        let mut down_trees = 0;
                        for tree in down_iter {
                            down_trees += 1;
                            if tree.height >= this_tree.height {
                                break;
                            }
                        }

                        let mut left_trees = 0;
                        for tree in left_iter {
                            left_trees += 1;
                            if tree.height >= this_tree.height {
                                break;
                            }
                        }

                        let mut right_trees = 0;
                        for tree in right_iter {
                            right_trees += 1;
                            if tree.height >= this_tree.height {
                                break;
                            }
                        }

                        let score = up_trees * down_trees * left_trees * right_trees;

                        score
                    })
                    .max()
                    .unwrap()
                    .clone()
            })
            .max()
            .unwrap()
            .clone()
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
    println!("Count: {}", count);
}

fn part2() {
    let input = include_str!("../input/8");
    let mut trees = parse_input(input);
    let max = trees.calculate_max_scenic_score();

    println!("Max scenic score: {}", max);
}

pub fn run() {
    part1();
    part2();
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

    #[test]
    fn test_iterator() {
        let input = include_str!("../input/test8");

        let trees = parse_input(input);

        let (up_iter, down_iter, left_iter, right_iter) = trees.iterate_2d_matrix(3, 3);

        let up_iter = up_iter.collect::<Vec<&VisableTree>>();

        assert!(up_iter.len() == 3);

        let down_iter = down_iter.collect::<Vec<&VisableTree>>();

        assert!(down_iter.len() == 1);

        let left_iter = left_iter.collect::<Vec<&VisableTree>>();

        assert!(left_iter.len() == 3);

        let right_iter = right_iter.collect::<Vec<&VisableTree>>();

        assert!(right_iter.len() == 1);

        for tree in up_iter.iter() {
            println!("up: {:?}", tree);
        }

        assert!(up_iter[0].height == 3);
        assert!(up_iter[1].height == 1);
        assert!(up_iter[2].height == 7);

        assert!(down_iter[0].height == 9);

        assert!(left_iter[0].height == 5);
        assert!(left_iter[1].height == 3);
        assert!(left_iter[2].height == 3);

        assert!(right_iter[0].height == 9);
    }

    #[test]
    fn test_scenic_score() {
        let input = include_str!("../input/test8");

        let mut trees = parse_input(input);
        let max = trees.calculate_max_scenic_score();

        assert_eq!(max, 8);
    }
}
