fn hash(input: &str) -> u8 {
    let mut result = 0u32;
    input.chars().for_each(|c| {
        result += c as u32;
        result *= 17;
        result %= 256;
    });

    result as u8
}

fn part1(input: &str) -> usize {
    input
        .split(',')
        .map(|line| line.trim())
        .fold(0, |acc, line| acc + hash(line) as usize)
}

fn part2(input: &str) -> usize {
    let mut boxes = vec![Vec::new(); 256];
    input.split(',').map(|line| line.trim()).for_each(|line| {
        if line.contains('-') {
            let label = line.split(&['-']).next().unwrap();
            let hash = hash(label) as usize;
            if let Some(index) = boxes[hash]
                .iter()
                .position(|x: &(&str, usize)| x.0 == label)
            {
                boxes[hash].remove(index);
            }
        } else if line.contains('=') {
            let mut lens = line.split(&['=']);
            let label = lens.next().unwrap();
            let focal = lens.next().unwrap().parse::<usize>().unwrap();
            let hash = hash(label) as usize;

            if let Some(index) = boxes[hash]
                .iter()
                .position(|x: &(&str, usize)| x.0 == label)
            {
                boxes[hash].remove(index);
                boxes[hash as usize].insert(index, (label, focal));
            } else {
                boxes[hash as usize].push((label, focal));
            }
        }
    });

    boxes
        .iter()
        .enumerate()
        .filter(|(_i, boxx)| !boxx.is_empty())
        .fold(0, |acc, (i, lenses)| {
            acc + lenses
                .iter()
                .enumerate()
                .fold(0, |acc, (j, (_, focal))| acc + (i + 1) * (j + 1) * focal)
        })
}

fn main() {
    println!("AoC 2023 - Day 1");
    let input = include_str!("input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("test");
        assert_eq!(part1(input), 1320);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("test");
        assert_eq!(part2(input), 145);
    }
}
