use std::collections::HashSet;

use iterwindows::IterArrayWindows;

fn are_unique(elements: Vec<char>) -> bool {
    let mut unique_elements: HashSet<char> = HashSet::new();
    for &element in elements.iter() {
        if unique_elements.contains(&element) {
            return false;
        }
        unique_elements.insert(element);
    }
    return true;
}

pub fn get_first_unique_quartet(input: String) -> Option<i32> {
    let quartet = input.chars().into_iter().array_windows::<4>();
    let mut index = 0;
    for window in quartet {
        if are_unique(window.to_vec()) {
            return Some(index + 4);
        }
        index += 1;
    }
    None
}
pub fn get_first_unique_quattuordecim(input: String) -> Option<i32> {
    let quartet = input.chars().into_iter().array_windows::<14>();
    let mut index = 0;
    for window in quartet {
        if are_unique(window.to_vec()) {
            return Some(index + 14);
        }
        index += 1;
    }
    None
}

pub fn get_6_first() -> i32 {
    let content = include_str!("../../inputs/6/input.data");
    get_first_unique_quartet(content.to_owned()).unwrap()
}
pub fn get_6_second() -> i32 {
    let content = include_str!("../../inputs/6/input.data");
    get_first_unique_quattuordecim(content.to_owned()).unwrap()
}

#[cfg(test)]
mod test_6 {
    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!(get_first_unique_quartet("asda".to_owned()), None);
    }

    #[test]
    fn test_are_unique() {
        assert!(are_unique(vec!['a', 'b', 'c', 'd']));
        assert!(!are_unique(vec!['a', 'b', 'a', 'd']));
    }

    #[test]
    fn test_examples() {
        assert_eq!(
            get_first_unique_quartet("bvwbjplbgvbhsrlpgdmjqwftvncz".to_owned()),
            Some(5)
        );
        assert_eq!(
            get_first_unique_quartet("nppdvjthqldpwncqszvftbrmjlhg".to_owned()),
            Some(6)
        );
        assert_eq!(
            get_first_unique_quartet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_owned()),
            Some(10)
        );
        assert_eq!(
            get_first_unique_quartet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_owned()),
            Some(11)
        );
    }

    #[test]
    fn test_examples_a_lot() {
        assert_eq!(
            get_first_unique_quattuordecim("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_owned()),
            Some(19)
        );
        assert_eq!(
            get_first_unique_quattuordecim("bvwbjplbgvbhsrlpgdmjqwftvncz".to_owned()),
            Some(23)
        );
        assert_eq!(
            get_first_unique_quattuordecim("nppdvjthqldpwncqszvftbrmjlhg".to_owned()),
            Some(23)
        );
        assert_eq!(
            get_first_unique_quattuordecim("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_owned()),
            Some(29)
        );
        assert_eq!(
            get_first_unique_quattuordecim("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_owned()),
            Some(26)
        );
    }
}
