use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
pub struct Rucksack {
    items: HashSet<String>,
}

impl Rucksack {
    fn new(items: HashSet<String>) -> Self {
        Self { items }
    }

    pub fn split_half(from_string: String) -> (Rucksack, Rucksack) {
        let (fist_half, second_half) = from_string.split_at(from_string.len() / 2);
        (fist_half.parse().unwrap(), second_half.parse().unwrap())
    }

    pub fn get_intersection(&self, other: Rucksack) -> HashSet<String> {
        self.items
            .intersection(&other.items)
            .map(|e| e.to_owned())
            .collect()
    }
}

impl FromStr for Rucksack {
    type Err = &'static str;

    fn from_str(from: &str) -> Result<Self, Self::Err> {
        Ok(Rucksack::new(
            from.to_owned()
                .split("")
                .filter(|e| !e.is_empty())
                .map(|e| e.to_owned())
                .collect::<HashSet<String>>(),
        ))
    }
}
pub fn get_value(char: char) -> u32 {
    match char {
        'a'..='z' => char as u32 - 97 + 1,
        'A'..='Z' => char as u32 - 39 + 1,
        _ => 0,
    }
}
pub fn get_intersections(of_texts: String) -> Vec<HashSet<String>> {
    of_texts
        .lines()
        .filter_map(|line| Some(Rucksack::split_half(line.to_owned())))
        .map(|(a, b)| a.get_intersection(b))
        .filter(|a| a.len() > 0)
        .collect::<Vec<HashSet<String>>>()
}

pub fn get_3_first() -> u32 {
    let intersections = get_intersections(include_str!("../inputs/3/input.data").to_owned());
    let sum_of_intersection_values = intersections
        .iter()
        .map(|diffs| {
            diffs
                .iter()
                .map(|diff| {
                    diff.chars()
                        .into_iter()
                        .map(|c| get_value(c))
                        .fold(0, |a, b| a + b)
                })
                .fold(0, |a, b| a + b)
        })
        .fold(0, |a, b| a + b);
    sum_of_intersection_values
}

fn get_commons(of: HashSet<char>, other: HashSet<char>) -> Vec<char> {
    of.intersection(&other).map(|&a| a).collect::<Vec<char>>()
}

fn get_unique_letters(from: &str) -> HashSet<char> {
    from.chars().collect()
}

fn get_commons_of_three(
    of: HashSet<char>,
    other: HashSet<char>,
    and_other: HashSet<char>,
) -> Vec<char> {
    let first_inter = of
        .intersection(&other.clone())
        .map(|&a| a)
        .collect::<HashSet<char>>();
    get_commons(first_inter, and_other)
}

pub fn get_3_second() -> u32 {
    let blocks = include_str!("../inputs/2/input.data")
        .lines()
        .collect::<Vec<&str>>();
    let mut unique_chars: Vec<Vec<char>> = vec![];
    for i in 0..blocks.len() / 3 {
        match &blocks[3 * i..(3 * i) + 3] {
            &[first, second, third] => {
                let uniques = get_commons_of_three(
                    get_unique_letters(first),
                    get_unique_letters(second),
                    get_unique_letters(third),
                );
                unique_chars.push(uniques.clone());
            }
            _ => {
                // println!("{:?}", &blocks[3 * i..(3 * i) + 2]);
            }
        }
    }
    unique_chars
        .iter()
        .map(|block_uniques| {
            block_uniques
                .iter()
                .map(|&e| get_value(e))
                .fold(0, |a, b| a + b)
        })
        .fold(0, |a, b| a + b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let some: Rucksack = "".parse::<Rucksack>().unwrap();
        let empty_set: HashSet<String> = HashSet::new();
        assert_eq!(some.items, empty_set);
    }

    #[test]
    fn test_of_two_element() {
        let some = "ab".parse::<Rucksack>().unwrap();
        assert_eq!(some.items.len(), 2);
    }

    #[test]
    fn test_of_four_element_no_intersection() {
        let some = "abcd".parse::<Rucksack>().unwrap();
        assert_eq!(some.items.len(), 4);
    }

    #[test]
    fn test_of_four_element_full_intersection() {
        let some = "aaaa".parse::<Rucksack>().unwrap();
        assert_eq!(some.items.len(), 1);
    }

    #[test]
    fn test_of_four_element_full_intersection_separation() {
        let (a, b) = Rucksack::split_half("abcd".to_owned());
        assert_eq!(a.items.len(), b.items.len());
    }
    #[test]
    fn test_of_four_element_separation_one_intersection() {
        let (a, b) = Rucksack::split_half("abca".to_owned());
        assert_eq!(a.items.len(), b.items.len());
        let intersection = a.get_intersection(b);
        let mut expected_intersection: HashSet<String> = HashSet::new();
        expected_intersection.insert("a".to_owned());
        assert_eq!(expected_intersection, intersection);
    }

    #[test]
    fn test_of_one_one_intersection() {
        let intersections = get_intersections("abca".to_owned());
        let mut diffs: HashSet<String> = HashSet::new();
        diffs.insert("a".to_owned());
        let expected_intersection: Vec<HashSet<String>> = vec![diffs];
        assert_eq!(expected_intersection, intersections);
    }

    #[test]
    fn test_of_two_lines_one_intersection() {
        let intersections = get_intersections("abca\nabcdef".to_owned());
        let mut diffs: HashSet<String> = HashSet::new();
        diffs.insert("a".to_owned());
        let expected_intersection: Vec<HashSet<String>> = vec![diffs];
        assert_eq!(expected_intersection, intersections);
    }

    #[test]
    fn test_of_two_lines_two_intersections() {
        let intersections = get_intersections("abca\nxbcdafbb".to_owned());
        let mut diffs: HashSet<String> = HashSet::new();
        diffs.insert("a".to_owned());
        let mut expected_intersection: Vec<HashSet<String>> = vec![diffs];
        let mut diffs: HashSet<String> = HashSet::new();
        diffs.insert("b".to_owned());
        expected_intersection.push(diffs);
        assert_eq!(expected_intersection, intersections);
    }
    #[test]
    fn test_intersection_values() {
        assert_eq!(get_value('a'), 1);
        assert_eq!(get_value('z'), 26);
        assert_eq!(get_value('A'), 27);
        assert_eq!(get_value('Z'), 52);
        assert_eq!(get_value('p'), 16);
        assert_eq!(get_value('L'), 38);
    }
}
