use std::collections::VecDeque;

fn has_duplicates(queue: &VecDeque<char>) -> bool {
    for i in 0..queue.len() {
        for j in 0..queue.len() {
            if i != j && queue[i] == queue[j] {
                return true;
            }
        }
    }
    return false;
}

fn start_of_packet(packet: &str, distinct_chars: usize) -> usize {
    let mut queue: VecDeque<char> = packet.chars().take(distinct_chars).collect();

    if !has_duplicates(&queue) {
        return distinct_chars;
    }

    for i in distinct_chars..packet.len() {
        let _ = queue.pop_front();
        queue.push_back(packet.chars().nth(i).unwrap());
        if !has_duplicates(&queue) {
            return i + 1;
        }
    }

    return 0;
}

fn part1() {
    for line in include_str!("../input/6").lines() {
        let start = start_of_packet(&line, 4);

        println!("Packet start: {}", start);
    }
}
fn part2() {
    for line in include_str!("../input/6").lines() {
        let start = start_of_packet(&line, 14);

        println!("Packet start: {}", start);
    }
}
pub fn run() {
    println!("Day 6");
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let s1 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let s2 = "nppdvjthqldpwncqszvftbrmjlhg";
        let s3 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let s4 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(start_of_packet(s1, 4), 5);
        assert_eq!(start_of_packet(s2, 4), 6);
        assert_eq!(start_of_packet(s3, 4), 10);
        assert_eq!(start_of_packet(s4, 4), 11);
    }
    #[test]
    fn test_part2() {
        let s1 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let s2 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let s3 = "nppdvjthqldpwncqszvftbrmjlhg";
        let s4 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let s5 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(start_of_packet(s1, 14), 19);
        assert_eq!(start_of_packet(s2, 14), 23);
        assert_eq!(start_of_packet(s3, 14), 23);
        assert_eq!(start_of_packet(s4, 14), 29);
        assert_eq!(start_of_packet(s5, 14), 26);
    }
}
