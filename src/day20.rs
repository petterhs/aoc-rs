use std::{collections::LinkedList, fmt::Display, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Item {
    index: usize,
    value: i32,
}

impl Item {
    fn new(index: usize, value: i32) -> Item {
        Item { index, value }
    }
}

#[derive(Debug, Clone)]
struct ItemList {
    items: LinkedList<Item>,
}

impl ItemList {
    fn new() -> ItemList {
        ItemList {
            items: LinkedList::new(),
        }
    }

    fn add(&mut self, index: usize, value: i32) {
        self.items.push_back(Item::new(index, value));
    }

    fn insert_at(&mut self, index: usize, item: Item) {
        let mut split = self.items.split_off(index);
        self.items.push_back(item);

        self.items.append(&mut split);
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    fn at(&self, index: usize) -> Option<&Item> {
        let new_index = index % (self.len());
        self.items.iter().nth(new_index.try_into().unwrap())
    }

    fn find(&self, item: &Item) -> Option<usize> {
        self.items.iter().position(|i| i == item)
    }

    fn move_item(&mut self, item: &Item) {
        let index = self.find(item).unwrap();
        let mut new_index = index as i32 + item.value;

        new_index = new_index % (self.len() as i32 - 1);

        if new_index <= 0 {
            new_index += self.len() as i32 - 1;
        }

        println!("{} {} moves to index {}", item.value, index, new_index);

        let item = self.items.remove(index);

        self.insert_at((new_index).try_into().unwrap(), item);
    }
}

impl FromStr for ItemList {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut list = ItemList::new();
        let _ = s
            .lines()
            .enumerate()
            .for_each(|(i, l)| list.add(i, l.parse().unwrap()));
        Ok(list)
    }
}

impl Display for ItemList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for item in self.items.iter() {
            s.push_str(&format!("{} ", item.value));
        }

        write!(f, "{}", s)
    }
}

fn part1(input: &str) -> i32 {
    let list = input.parse::<ItemList>().unwrap();

    let mut modified_list = list.clone();

    for i in 0..list.len() {
        let item = list.at(i).unwrap();
        modified_list.move_item(item);
    }

    let index0 = modified_list
        .items
        .iter()
        .position(|i| i.value == 0)
        .unwrap();

    println!("index0: {}", index0);

    println!("1000: {}", modified_list.at(index0 + 1000).unwrap().value);
    println!("2000: {}", modified_list.at(index0 + 2000).unwrap().value);
    println!("3000: {}", modified_list.at(index0 + 3000).unwrap().value);

    modified_list.at(index0 + 1000).unwrap().value
        + modified_list.at(index0 + 2000).unwrap().value
        + modified_list.at(index0 + 3000).unwrap().value
}

fn part2(input: &str) -> usize {
    0
}

pub fn run() {
    // let input = include_str!("../input/20");
    let input = include_str!("../input/20");
    println!("Part 1 result: {}", part1(input));
    println!("Part 2 result: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at() {
        let input = include_str!("../input/test20");

        let list = input.parse::<ItemList>().unwrap();
        println!("{}", list);

        assert_eq!(list.at(0).unwrap().value, 1);
        assert_eq!(list.at(1).unwrap().value, 2);
        assert_eq!(list.at(2).unwrap().value, -3);
        assert_eq!(list.at(3).unwrap().value, 3);
        assert_eq!(list.at(7).unwrap().value, 1);
        assert_eq!(list.at(8).unwrap().value, 2);
        assert_eq!(list.at(14).unwrap().value, 1);

        let mut modified_list = list.clone();

        for i in 0..list.len() {
            let item = list.at(i).unwrap();
            modified_list.move_item(item);
            println!("{}", modified_list);
        }
        println!("{}", modified_list);

        let index0 = modified_list
            .items
            .iter()
            .position(|i| i.value == 0)
            .unwrap();

        assert_eq!(4, modified_list.at(index0 + 1000).unwrap().value);
        assert_eq!(-3, modified_list.at(index0 + 2000).unwrap().value);
        assert_eq!(2, modified_list.at(index0 + 3000).unwrap().value);
    }
}
