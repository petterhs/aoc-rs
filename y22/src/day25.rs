fn snafu_char_to_i64(c: char) -> i64 {
    match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => panic!("Invalid char: {}", c),
    }
}

fn i64_from_snafu(s: &str) -> i64 {
    s.chars().rev().enumerate().fold(0, |acc, (i, c)| {
        acc + snafu_char_to_i64(c) * 5_i64.pow(i as u32)
    })
}

fn snafu_from_i64(num: i64) -> String {
    let mut num = num;
    let mut s = String::new();

    if num == 0 {
        return "0".to_string();
    }

    while num != 0 {
        let rem = (num + 2) % 5 - 2;
        num = (num - rem) / 5;

        let c = match rem {
            2 => '2',
            1 => '1',
            0 => '0',
            -1 => '-',
            -2 => '=',
            _ => panic!("Invalid rem: {}", rem),
        };
        s.push(c);
    }
    s.chars().rev().collect()
}

fn part1() {
    let input = include_str!("../input/25");

    let sum = input
        .lines()
        .fold(0, |acc, line| acc + i64_from_snafu(line));

    println!("sum: {}", sum);
    println!("snafu sum: {}", snafu_from_i64(sum));
}

pub fn run() {
    println!("Day 25");
    part1();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_snafu() {
        assert!(i64_from_snafu("0") == 0);
        assert!(i64_from_snafu("1") == 1);
        assert!(i64_from_snafu("2") == 2);
        assert!(i64_from_snafu("-") == -1);
        assert!(i64_from_snafu("1=-0-2") == 1747);
    }

    #[test]
    fn test_to_snafu() {
        assert!(snafu_from_i64(0) == "0");
        assert!(snafu_from_i64(1) == "1");
        assert!(snafu_from_i64(2) == "2");
        assert!(snafu_from_i64(-1) == "-");
        assert!(snafu_from_i64(10) == "20");
        assert!(snafu_from_i64(20) == "1-0");
        assert!(snafu_from_i64(25) == "100");
        assert!(snafu_from_i64(1747) == "1=-0-2");
    }
}
