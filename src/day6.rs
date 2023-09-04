use std::collections::VecDeque;

fn has_duplicates(queue: &VecDeque<char>) -> bool {
    for i in 0..4 {
        for j in 0..4 {
            if i != j && queue[i] == queue[j] {
                return true;
            }
        }
    }
    return false;
}

fn start_of_packet(packet: &str) -> u32 {
    let mut queue: VecDeque<char> = packet.chars().take(4).collect();

    //check if queue is distinct
    if !has_duplicates(&queue) {
        return 4;
    }

    for i in 4..packet.len() {
        //remove first char
        let _ = queue.pop_front();
        queue.push_back(packet.chars().nth(i).unwrap());
        if !has_duplicates(&queue) {
            return i as u32 + 1;
        }
    }

    return 0;
}

fn part1() {
    for line in include_str!("../input/6").lines() {
        let start = start_of_packet(&line);

        println!("Packet start: {}", start);
    }
}

pub fn run() {
    println!("Day 6");
    part1();
    // part2();
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
        assert_eq!(start_of_packet(s1), 5);
        assert_eq!(start_of_packet(s2), 6);
        assert_eq!(start_of_packet(s3), 10);
        assert_eq!(start_of_packet(s4), 11);
    }
}
