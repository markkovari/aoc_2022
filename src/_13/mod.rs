use std::cmp::Ordering;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::i64, combinator::map,
    multi::separated_list0, sequence::delimited, IResult,
};

#[derive(Debug, Clone)]
enum Packet {
    Int(i64),
    List(Vec<Packet>),
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    alt((
        map(i64, |v| Packet::Int(v)),
        map(
            delimited(tag("["), separated_list0(tag(","), parse_packet), tag("]")),
            |v| Packet::List(v),
        ),
    ))(input)
}

impl std::cmp::PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl std::cmp::Eq for Packet {}

impl std::cmp::PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Packet::Int(l), Packet::Int(r)) => l.partial_cmp(r),
            (Packet::List(_), Packet::Int(_)) => {
                self.partial_cmp(&Packet::List(vec![other.clone()]))
            }
            (Packet::Int(_), Packet::List(_)) => {
                Packet::List(vec![self.clone()]).partial_cmp(other)
            }
            (Packet::List(l), Packet::List(r)) => {
                for (i1, i2) in l.iter().zip(r) {
                    if let Some(result) = i1.partial_cmp(i2) {
                        if result != Ordering::Equal {
                            return Some(result);
                        }
                    }
                }

                l.len().partial_cmp(&r.len())
            }
        }
    }
}

impl std::cmp::Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn get_13_first() -> usize {
    let content = include_str!("../../inputs/13/input.data");
    solve_first(content)
}
pub fn solve_first(input: &str) -> usize {
    let mut part1 = 0;

    for (i, pair) in input.split("\n\n").enumerate() {
        let mut lines = pair.lines();
        let left = parse_packet(lines.next().unwrap()).unwrap().1;
        let right = parse_packet(lines.next().unwrap()).unwrap().1;

        if left < right {
            part1 += i + 1;
        }
    }

    part1
}

pub fn solve_second(input: &str) -> usize {
    let mut packets: Vec<Packet> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| parse_packet(l).unwrap().1)
        .collect();

    let div1 = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
    let div2 = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);
    packets.push(div1.clone());
    packets.push(div2.clone());
    packets.sort();
    let pos1 = packets.iter().position(|e| e == &div1).unwrap() + 1;
    let pos2 = packets.iter().position(|e| e == &div2).unwrap() + 1;

    pos1 * pos2
}

pub fn get_13_second() -> usize {
    let content = include_str!("../../inputs/13/input.data");
    solve_second(content)
}

#[cfg(test)]
mod test_13 {

    use super::*;
    #[test]
    fn test_example_first() {
        let test_content = include_str!("../../inputs/13/input.example");
        assert_eq!(solve_first(test_content), 13);
    }

    #[test]
    fn test_example_second() {
        let test_content = include_str!("../../inputs/13/input.example");
        assert_eq!(solve_second(test_content), 140);
    }

    #[test]
    fn test_example_prod() {
        let test_content = include_str!("../../inputs/13/input.data");
        assert_eq!(solve_first(test_content), 5208);
    }

    #[test]
    fn test_example_second_prod() {
        let test_content = include_str!("../../inputs/13/input.data");
        assert_eq!(solve_second(test_content), 25792);
    }
}
