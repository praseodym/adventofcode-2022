use std::cmp::Ordering;
use std::collections::VecDeque;
use Packet::{Integer, List};

#[derive(Debug, Clone, Eq)]
enum Packet {
    Integer(u8),
    List(VecDeque<Packet>),
}

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (usize, usize) {
    let mut part1_answer: usize = 0;
    let pairs = parse_input(input);
    for (i, pair) in pairs.into_iter().enumerate() {
        if pair.0 < pair.1 {
            part1_answer += i + 1;
        }
    }

    let part2_answer: usize = 0;

    (part1_answer, part2_answer)
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Integer(left), Integer(right)) => left.cmp(right),
            (List(ref left), List(ref right)) => {
                let mut left = left.clone();
                let mut right = right.clone();
                loop {
                    return match (left.pop_front(), right.pop_front()) {
                        (Some(a), Some(b)) => {
                            let order = a.cmp(&b);
                            if order == Ordering::Equal {
                                continue;
                            }
                            order
                        }
                        (None, Some(_)) => Ordering::Less,
                        (Some(_), None) => Ordering::Greater,
                        (None, None) => Ordering::Equal,
                    };
                }
            }
            (Integer(left), List(ref right)) => {
                List(VecDeque::from([Integer(*left)])).cmp(&List(right.clone()))
            }
            (List(ref left), Integer(right)) => {
                List(left.clone()).cmp(&List(VecDeque::from([Integer(*right)])))
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

fn read_packet(raw_packet: &str) -> Packet {
    let mut stack: VecDeque<Packet> = VecDeque::new();
    let mut chars = raw_packet.chars().peekable();
    while chars.peek().is_some() {
        let c = chars.next().unwrap();
        if c == '[' {
            stack.push_back(List(VecDeque::new()));
        } else if c == ']' {
            if stack.len() > 1 {
                let s = stack.pop_back().unwrap();
                if let List(l) = stack.back_mut().unwrap() {
                    l.push_back(s);
                }
            }
        } else if c.is_ascii_digit() {
            let mut s = c.to_string();
            if chars.peek().unwrap().is_ascii_digit() {
                s.push(chars.next().unwrap());
            }
            if let List(l) = stack.back_mut().unwrap() {
                l.push_back(Integer(s.parse::<u8>().unwrap()))
            }
        } else if c == ',' {
            continue;
        } else {
            panic!("unknown char: {}", c);
        }
    }
    assert_eq!(stack.len(), 1);
    stack.pop_back().unwrap()
}

fn parse_input(input: &'static str) -> Vec<(Packet, Packet)> {
    let mut pairs = Vec::new();
    for pair in input.trim_end().split("\n\n") {
        let mut lines = pair.split_whitespace();
        let a = read_packet(lines.next().unwrap());
        let b = read_packet(lines.next().unwrap());
        pairs.push((a, b))
    }
    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_parse() {
        let pairs = parse_input(include_str!("../input-example"));
        assert_eq!(pairs.len(), 8);
    }

    fn test_packets(packets: (&str, &str)) -> Ordering {
        read_packet(packets.0)
            .partial_cmp(&read_packet(packets.1))
            .unwrap()
    }

    #[test]
    fn test_order() {
        assert_eq!(test_packets(("[1,1]", "[1,1,1]")), Ordering::Less);
        assert_eq!(test_packets(("[1,1,1]", "[1,1]")), Ordering::Greater);
        assert_eq!(test_packets(("[]", "[]")), Ordering::Equal);
        assert_eq!(test_packets(("[1]", "[2]")), Ordering::Less);
        assert_eq!(test_packets(("[2]", "[1]")), Ordering::Greater);
        assert_eq!(test_packets(("[1]", "[[2]]")), Ordering::Less);
        assert_eq!(test_packets(("[[1]", "[2]")), Ordering::Less);
        assert_eq!(test_packets(("[1]", "[[1],1]")), Ordering::Less);
        assert_eq!(test_packets(("[[],4]", "[[],3]")), Ordering::Greater);
        assert_eq!(test_packets(("[[0,0],2]", "[[0,0],1]")), Ordering::Greater);
        assert_eq!(test_packets(("[1]", "[[1,2,3]]")), Ordering::Less);
        assert_eq!(test_packets(("[1]", "[[0,2,3]]")), Ordering::Greater);
        assert_eq!(test_packets(("[10]", "[2]")), Ordering::Greater);
    }

    #[test]
    fn test_example_answer() {
        let (part1_answer, _part2_answer) = run(include_str!("../input-example"));
        assert_eq!(part1_answer, 13);
        // assert_eq!(part2_answer, 0);
    }

    #[test]
    fn test_input_answer() {
        let (part1_answer, _part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 5623);
        // assert_eq!(part2_answer, 0);
    }
}
