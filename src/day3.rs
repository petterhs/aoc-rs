#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Priority(u32);

impl Priority {
    fn new(c: char) -> Self {
        let priority = match c {
            'a'..='z' => c as u32 - 'a' as u32 + 1,
            'A'..='Z' => c as u32 - 'A' as u32 + 1 + 26,
            _ => panic!("Invalid character: {}", c),
        };
        Priority(priority)
    }
}

impl Into<u32> for Priority {
    fn into(self) -> u32 {
        self.0
    }
}

fn get_missplaced_item(s: &str) -> Option<char> {
    let mut array: [bool; 53] = [false; 53];

    //iterate over the first half of the string and count the number of each letter
    for c in s.chars().take(s.len() / 2) {
        let priority = Priority::new(c);
        array[priority.0 as usize] = true;
    }
    for c in s.chars().skip(s.len() / 2) {
        let priority = Priority::new(c);
        if array[priority.0 as usize] {
            return Some(c);
        }
    }
    None
}

pub fn run() {
    println!("Day 3");
    let file = std::fs::read_to_string("input/3").unwrap();
    let lines: Vec<&str> = file.split("\n").collect();

    let mut sum: u32 = 0;

    for line in lines {
        println!("{}", line);
        if let Some(c) = get_missplaced_item(line) {
            sum += Priority::new(c).0;
            println!("{}: {}", c, Priority::new(c).0);
        }
    }
    println!("Sum: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(1u32, Priority::new('a').into());
        assert_eq!(16u32, Priority::new('p').into());
    }
    #[test]
    fn test_capital() {
        assert_eq!(38u32, Priority::new('L').into());
    }

    #[test]
    fn test_missplaced() {
        let mut s = "vJrwpWtwJgWrhcsFMMfFFhFp";
        assert_eq!(Some('p'), get_missplaced_item(s));

        s = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        assert_eq!(Some('L'), get_missplaced_item(s));

        s = "PmmdzqPrVvPwwTWBwg";
        assert_eq!(Some('P'), get_missplaced_item(s));

        s = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn";
        assert_eq!(Some('v'), get_missplaced_item(s));

        s = "ttgJtRGJQctTZtZT";
        assert_eq!(Some('t'), get_missplaced_item(s));

        s = "CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(Some('s'), get_missplaced_item(s));
    }
}
