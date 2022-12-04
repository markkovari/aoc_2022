use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Clone, Copy)]
struct Range {
    from: i32,
    until: i32,
}

impl Range {
    fn fully_contains(&self, other: &Range) -> bool {
        self.from <= other.from && other.until <= self.until
    }
    fn has_common_elements(&self, other: &Range) -> bool {
        let left_set = (self.from..=self.until).collect::<HashSet<i32>>();
        let right_set = (other.from..=other.until).collect::<HashSet<i32>>();
        left_set.intersection(&right_set).count() > 0
    }
    fn one_is_subset(one: Range, another: &Range) -> bool {
        one.fully_contains(another) || another.fully_contains(&one)
    }
}

impl FromStr for Range {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let as_strs = s
            .split("-")
            .filter_map(|e| match e.parse::<i32>() {
                Ok(val) => Some(val),
                Err(_) => None,
            })
            .collect::<Vec<i32>>();
        if as_strs.len() != 2 {
            return Err("Cannot convert, not two pieces".to_owned());
        }
        Ok(Range {
            from: as_strs[0],
            until: as_strs[1],
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct RangePair {
    left: Range,
    right: Range,
}

impl FromStr for RangePair {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges = s
            .split(",")
            .filter_map(|part| match part.parse::<Range>() {
                Ok(e) => Some(e),
                Err(_) => None,
            })
            .collect::<Vec<Range>>();

        if ranges.len() != 2 {
            return Err("Cannot create two ranges based on str".to_owned());
        }
        Ok(RangePair {
            left: ranges[0],
            right: ranges[1],
        })
    }
}

impl RangePair {
    fn has_overlap(&self) -> bool {
        Range::one_is_subset(self.left, &self.right)
    }
    fn has_common_elements(&self) -> bool {
        Range::has_common_elements(&self.left, &self.right)
    }
}

fn read_pairs() -> Vec<RangePair> {
    let content = include_str!("../inputs/4/input.data").lines();
    content
        .into_iter()
        .filter_map(|line| match line.parse::<RangePair>() {
            Err(_) => None,
            Ok(e) => Some(e),
        })
        .collect::<Vec<RangePair>>()
}

pub fn get_4_first() -> u32 {
    let pairs = read_pairs();
    let overlaps = pairs
        .iter()
        .filter(|&pair| pair.has_overlap())
        .collect::<Vec<&RangePair>>();
    overlaps.len() as u32
}

pub fn get_4_second() -> u32 {
    let pairs = read_pairs();
    let overlaps = pairs
        .iter()
        .filter(|&pair| pair.has_common_elements())
        .collect::<Vec<&RangePair>>();
    overlaps.len() as u32
}
