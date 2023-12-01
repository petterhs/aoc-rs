fn fully_overlapping_ranges(a: (u32, u32), b: (u32, u32)) -> bool {
    let (a1, a2) = a;
    let (b1, b2) = b;

    if a1 <= b1 && a2 >= b2 || b1 <= a1 && b2 >= a2 {
        return true;
    }
    false
}

fn overlapping_ranges(a: (u32, u32), b: (u32, u32)) -> bool {
    let (a1, a2) = a;
    let (b1, b2) = b;

    if a1 <= b1 && a2 >= b1 || b1 <= a1 && b2 >= a1 {
        return true;
    }

    false
}

pub fn run() {
    println!("Day 4");

    let sum = include_str!("../input/4")
        .lines()
        .map(|line| {
            println!("{}", line);
            let mut parts = line.split(',');
            let first = parts.next().unwrap().split('-').collect::<Vec<_>>();
            let second = parts.next().unwrap().split('-').collect::<Vec<_>>();

            // println!("{:?} {:?}", first, second);
            let first = (
                first[0].parse::<u32>().unwrap(),
                first[1].parse::<u32>().unwrap(),
            );
            let second = (
                second[0].parse::<u32>().unwrap(),
                second[1].parse::<u32>().unwrap(),
            );

            if fully_overlapping_ranges(first, second) {
                println!("{:?} {:?}", first, second);
                1
            } else {
                0
            }
        })
        .collect::<Vec<_>>()
        .iter()
        .sum::<i32>();

    println!("Count: {}", sum);

    let sum = include_str!("../input/4")
        .lines()
        .map(|line| {
            println!("{}", line);
            let mut parts = line.split(',');
            let first = parts.next().unwrap().split('-').collect::<Vec<_>>();
            let second = parts.next().unwrap().split('-').collect::<Vec<_>>();

            // println!("{:?} {:?}", first, second);
            let first = (
                first[0].parse::<u32>().unwrap(),
                first[1].parse::<u32>().unwrap(),
            );
            let second = (
                second[0].parse::<u32>().unwrap(),
                second[1].parse::<u32>().unwrap(),
            );

            if overlapping_ranges(first, second) {
                println!("{:?} {:?}", first, second);
                1
            } else {
                0
            }
        })
        .collect::<Vec<_>>()
        .iter()
        .sum::<i32>();
    println!("Count: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(fully_overlapping_ranges((6, 6), (4, 6)));
        assert!(!fully_overlapping_ranges((2, 4), (6, 8)));
    }

    #[test]
    fn test_overlapping() {
        assert!(!overlapping_ranges((2, 3), (4, 5)));
        assert!(overlapping_ranges((2, 4), (4, 5)));
        assert!(overlapping_ranges((6, 6), (4, 6)));
    }
}
