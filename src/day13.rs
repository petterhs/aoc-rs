use std::{collections::VecDeque, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Value(i32),
    Packet(Vec<Packet>),
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Value(v) => write!(f, "{}", v),
            Packet::Packet(p) => {
                write!(f, "[")?;
                for (i, packet) in p.iter().enumerate() {
                    if i == 0 {
                        write!(f, "{}", packet)?;
                        continue;
                    }
                    write!(f, ",{}", packet)?;
                }
                write!(f, "]")
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct PacketPair {
    left: Packet,
    right: Packet,
}

fn parse_input(input: &str) -> Vec<PacketPair> {
    let mut vec = Vec::new();

    let mut left = None;
    let mut right = None;
    for line in input.lines() {
        match line {
            "" => {
                left = None;
                right = None;
            }
            line => {
                let packet = parse_packet(line);
                match (&left, &right) {
                    (None, None) => left = Some(packet),
                    (Some(l), None) => {
                        right = Some(packet.clone());
                        vec.push(PacketPair {
                            left: l.clone(),
                            right: packet.clone(),
                        });
                    }
                    _ => panic!("Invalid input"),
                }
            }
        }
    }
    vec
}

fn parse_packet(line: &str) -> Packet {
    //Remove first and last characters
    let line = &line[1..line.len() - 1];

    let mut data = line.split(",").collect::<VecDeque<&str>>();
    // println!("Data: {:?}", data);

    let mut packet = Vec::new();

    if data.len() == 1 {
        if let Ok(value) = data[0].parse::<i32>() {
            packet.push(Packet::Value(value));
            return Packet::Packet(packet);
        }
    }

    while let Some(datum) = data.pop_front() {
        if datum.matches('[').count() == datum.matches(']').count()
            && datum.matches('[').count() != 0
        {
            packet.push(parse_packet(datum));
            continue;
        }
        if datum.parse::<i32>().is_ok() {
            let value = datum.parse::<i32>().unwrap();
            packet.push(Packet::Value(value));
        } else if datum.starts_with("[") {
            let mut count = datum.matches("[").count() - datum.matches("]").count();

            let mut packet_str = "".to_string();
            packet_str.push_str(datum);

            'outer: while let Some(datum) = data.pop_front() {
                packet_str += ",";
                packet_str.push_str(datum);
                count += datum.matches("[").count();

                for _ in 0..datum.matches("]").count() {
                    count -= 1;
                    if count == 0 {
                        packet.push(parse_packet(&packet_str));
                        break 'outer;
                    }
                }
            }
        } else if datum == "" {
            continue;
        } else {
            panic!("Invalid input: {}", datum);
        }
    }
    Packet::Packet(packet)
}

#[derive(Debug, PartialEq)]
enum Order {
    Correct,
    Incorrect,
    Equal,
}

fn check_packet_order(left: Packet, right: Packet) -> Order {
    match (left, right) {
        (Packet::Value(l), Packet::Value(r)) => {
            if l < r {
                Order::Correct
            } else if l > r {
                Order::Incorrect
            } else {
                Order::Equal
            }
        }
        (Packet::Value(l), Packet::Packet(r)) => {
            return check_packet_order(Packet::Packet(vec![Packet::Value(l)]), Packet::Packet(r));
        }
        (Packet::Packet(l), Packet::Value(r)) => {
            return check_packet_order(Packet::Packet(l), Packet::Packet(vec![Packet::Value(r)]));
        }
        (Packet::Packet(l), Packet::Packet(r)) => {
            if l.len() == 0 && r.len() == 0 {
                return Order::Equal;
            }
            for i in 0..l.len() {
                let li = l.get(i).unwrap();
                if let Some(ri) = r.get(i) {
                    let order = check_packet_order(li.clone(), ri.clone());
                    match order {
                        Order::Correct => {
                            return Order::Correct;
                        }
                        Order::Incorrect => {
                            return Order::Incorrect;
                        }
                        Order::Equal => {
                            continue;
                        }
                    }
                } else {
                    return Order::Incorrect;
                }
            }

            Order::Correct
        }
    }
}

fn part1() {
    let input = include_str!("../input/13");

    let packet_pairs = parse_input(input);

    let sum_of_correct_indices = packet_pairs
        .iter()
        .enumerate()
        .map(
            |(i, pair)| match check_packet_order(pair.left.clone(), pair.right.clone()) {
                Order::Incorrect => 0,
                _ => i + 1,
            },
        )
        .collect::<Vec<usize>>()
        .iter()
        .sum::<usize>();

    println!("Sum of correct indices: {}", sum_of_correct_indices);
}

pub fn run() {
    println!("Day 13");
    part1();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pop() {
        let mut vec = vec![1, 2, 3];

        let mut new_vec = Vec::new();

        while let Some(value) = vec.pop() {
            new_vec.push(value);
        }

        assert_eq!(new_vec, vec![3, 2, 1]);
    }

    #[test]
    fn test_eq() {
        let left = Packet::Packet(vec![Packet::Value(1), Packet::Value(2), Packet::Value(3)]);
        let right = Packet::Packet(vec![Packet::Value(1), Packet::Value(2), Packet::Value(3)]);
        assert_eq!(left, right);
    }

    #[test]
    fn test_parse_packet() {
        let input = "[1,2,3]";
        let expected = Packet::Packet(vec![Packet::Value(1), Packet::Value(2), Packet::Value(3)]);
        assert_eq!(parse_packet(input), expected);

        let input = "[3]";
        let expected = Packet::Packet(vec![Packet::Value(3)]);
        assert_eq!(parse_packet(input), expected);
    }

    #[test]
    fn test_parse_packet2() {
        let input = "[1,[2,3],[4],5]";
        let expected = Packet::Packet(vec![
            Packet::Value(1),
            Packet::Packet(vec![Packet::Value(2), Packet::Value(3)]),
            Packet::Packet(vec![Packet::Value(4)]),
            Packet::Value(5),
        ]);
        assert_eq!(parse_packet(input), expected);
    }

    #[test]
    fn test_parse_packet3() {
        let input = "[1,[2,3,[4,5],6],7]";
        let expected = Packet::Packet(vec![
            Packet::Value(1),
            Packet::Packet(vec![
                Packet::Value(2),
                Packet::Value(3),
                Packet::Packet(vec![Packet::Value(4), Packet::Value(5)]),
                Packet::Value(6),
            ]),
            Packet::Value(7),
        ]);
        assert_eq!(parse_packet(input), expected);
    }

    #[test]
    fn test_parse_packet4() {
        let input = "[1,[2,[3,[4,[5,6,7]]]],8,9]";
        let expected = Packet::Packet(vec![
            Packet::Value(1),
            Packet::Packet(vec![
                Packet::Value(2),
                Packet::Packet(vec![
                    Packet::Value(3),
                    Packet::Packet(vec![
                        Packet::Value(4),
                        Packet::Packet(vec![Packet::Value(5), Packet::Value(6), Packet::Value(7)]),
                    ]),
                ]),
            ]),
            Packet::Value(8),
            Packet::Value(9),
        ]);

        assert_eq!(parse_packet(input), expected);
    }

    #[test]
    fn test_parse_packet5() {
        let input = "[2,[4,[5]],6]";

        let expected = Packet::Packet(vec![
            Packet::Value(2),
            Packet::Packet(vec![
                Packet::Value(4),
                Packet::Packet(vec![Packet::Value(5)]),
            ]),
            Packet::Value(6),
        ]);

        assert_eq!(parse_packet(input), expected);
    }

    #[test]
    fn test_parse_packet6() {
        let input = "[[[5,1,[],[8,1,3],6]]]";

        let expected = Packet::Packet(vec![Packet::Packet(vec![Packet::Packet(vec![
            Packet::Value(5),
            Packet::Value(1),
            Packet::Packet(vec![]),
            Packet::Packet(vec![Packet::Value(8), Packet::Value(1), Packet::Value(3)]),
            Packet::Value(6),
        ])])]);

        assert_eq!(parse_packet(input), expected);
    }

    #[test]
    fn test_parse_packet7() {
        let input = "[[6],[[],7,7]]";

        let expected = Packet::Packet(vec![
            Packet::Packet(vec![Packet::Value(6)]),
            Packet::Packet(vec![
                Packet::Packet(vec![]),
                Packet::Value(7),
                Packet::Value(7),
            ]),
        ]);

        assert_eq!(parse_packet(input), expected);
    }

    #[test]
    fn test_parse_input() {
        let input = include_str!("../input/test13");

        let packets = parse_input(input);

        let expected_first_pair = PacketPair {
            left: Packet::Packet(vec![
                Packet::Value(1),
                Packet::Value(1),
                Packet::Value(3),
                Packet::Value(1),
                Packet::Value(1),
            ]),
            right: Packet::Packet(vec![
                Packet::Value(1),
                Packet::Value(1),
                Packet::Value(5),
                Packet::Value(1),
                Packet::Value(1),
            ]),
        };

        assert_eq!(packets[0], expected_first_pair);

        let expected_6th_pair = PacketPair {
            left: Packet::Packet(vec![]),
            right: Packet::Packet(vec![Packet::Value(3)]),
        };

        assert_eq!(packets[5], expected_6th_pair);
    }

    #[test]
    fn part1() {
        let input = include_str!("../input/test13");

        let packet_pairs = parse_input(input);

        let sum_of_correct_indices = packet_pairs
            .iter()
            .enumerate()
            .map(|(i, pair)| {
                if check_packet_order(pair.left.clone(), pair.right.clone()) == Order::Incorrect {
                    0
                } else {
                    println!("Correct: {}", i + 1);
                    i + 1
                }
            })
            .collect::<Vec<usize>>()
            .iter()
            .sum::<usize>();

        assert_eq!(sum_of_correct_indices, 13);
    }

    #[test]

    fn test_pair18() {
        let input = include_str!("../input/13");

        let packet_pairs = parse_input(input);

        let pair_18 = packet_pairs.get(32).unwrap();

        println!("Pair 18: {}", pair_18.left);
        println!("Pair 18: {}", pair_18.right);

        assert_eq!(
            check_packet_order(pair_18.left.clone(), pair_18.right.clone()),
            Order::Incorrect
        );
    }
}
