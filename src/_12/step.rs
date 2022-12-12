use std::fmt::Display;

use super::height::{Accessability, Direction, HasAccessTo, Height};

#[derive(PartialEq, PartialOrd, Eq, Copy, Clone)]
pub struct Step {
    pub from: Height,
    pub to: Height,
    pub direction: Direction,
    pub accessibility: Accessability,
}

impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dir = &self.direction;
        let access = &self.accessibility;
        match (dir, access) {
            (_, Accessability::InAccessible) => f.write_str("."),
            (dir, _) => match dir {
                Direction::D => f.write_str("v"),
                Direction::U => f.write_str("^"),
                Direction::L => f.write_str("<"),
                Direction::R => f.write_str(">"),
            },
        }
    }
}

impl Step {
    pub fn new(mut from: Height, to: Height, direction: Direction) -> Self {
        Self {
            from,
            to,
            direction,
            accessibility: from.accessible(to),
        }
    }
}

#[cfg(test)]
mod test_steps {

    use super::*;
    #[test]
    fn accessible_check_with_neighbors_valid() {
        let current_height = Height('m');
        let new_height = Height('n');
        let step: Step = Step::new(current_height, new_height, Direction::R);
        assert_eq!(step.accessibility, Accessability::Accessible);
        assert_eq!(step.direction, Direction::R);
        let as_str = format!("{}", step);
        assert_eq!(as_str, ">");
    }

    #[test]
    fn accessible_check_with_neighbors_invalid() {
        let current_height = Height('m');
        let new_height = Height('o');
        let step: Step = Step::new(current_height, new_height, Direction::R);
        assert_eq!(step.accessibility, Accessability::InAccessible);
        assert_eq!(step.direction, Direction::R);
        let as_str = format!("{}", step);
    }
}
