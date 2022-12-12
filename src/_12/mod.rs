use std::{char, fmt::Debug, ops::Add};

pub fn get_12_first() -> usize {
    2
}
pub fn get_12_second() -> usize {
    2
}

#[derive(PartialEq, PartialOrd, Eq, Debug)]
struct Height(char);

#[derive(PartialEq, PartialOrd, Eq, Debug)]
enum Accessability {
    UnAccessible,
    Accessible,
}

#[derive(PartialEq, PartialOrd, Eq, Debug)]
enum Direction {
    L,
    R,
    U,
    D,
}

#[derive(PartialEq, PartialOrd, Eq)]
struct Step {
    from: Height,
    to: Height,
    direction: Direction,
    accessibility: Accessability,
}

impl Debug for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dir = &self.direction;
        let access = &self.accessibility;
        match (dir, access) {
            (_, Accessability::UnAccessible) => f.write_str("."),
            (dir, _) => match dir {
                Direction::D => f.write_str("v"),
                Direction::U => f.write_str("^"),
                Direction::L => f.write_str("<"),
                Direction::R => f.write_str(">"),
            },
        }
    }
}

trait HasAccessTo {
    fn accessible(&mut self, other: &Height) -> Accessability;
}

impl HasAccessTo for Height {
    fn accessible(&mut self, other: &Height) -> Accessability {
        let current_height = self.0 as u32;
        let other_char = other.0 as u32;
        match other_char - current_height {
            1 => Accessability::Accessible,
            _ => Accessability::UnAccessible,
        }
    }
}

#[cfg(test)]
mod test_12 {

    use super::*;

    #[test]
    fn accessible_check_valid() {
        let mut current_height = Height('a');
        let new_height = Height('b');
        assert_eq!(
            current_height.accessible(&new_height),
            Accessability::Accessible
        )
    }

    #[test]
    fn accessible_check_invalid() {
        let mut current_height = Height('m');
        let new_height = Height('n');
        let unaccessible_new_height = Height('o');
        assert_eq!(
            current_height.accessible(&new_height),
            Accessability::Accessible
        );
        assert_eq!(
            current_height.accessible(&unaccessible_new_height),
            Accessability::UnAccessible
        )
    }

    #[test]
    fn accessible_check_with_neighbors_valid() {
        let mut current_height = Height('m');
        let new_height = Height('n');

        assert_eq!(
            current_height.accessible(&new_height),
            Accessability::Accessible
        );
    }
}
