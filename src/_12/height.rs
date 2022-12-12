#[derive(PartialEq, PartialOrd, Eq, Debug, Copy, Clone)]
pub struct Height(pub char);

#[derive(PartialEq, PartialOrd, Eq, Debug, Copy, Clone)]
pub enum Accessability {
    InAccessible,
    Accessible,
}

#[derive(PartialEq, PartialOrd, Eq, Debug, Copy, Clone)]
pub enum Direction {
    L,
    R,
    U,
    D,
}

pub trait HasAccessTo {
    fn accessible(&mut self, other: Height) -> Accessability;
}

impl HasAccessTo for Height {
    fn accessible(&mut self, other: Height) -> Accessability {
        let current_height = self.0 as u32;
        let other_char = other.0 as u32;
        match other_char - current_height {
            1 => Accessability::Accessible,
            _ => Accessability::InAccessible,
        }
    }
}

#[cfg(test)]
mod test_height {

    use super::*;

    #[test]
    fn accessible_check_valid() {
        let mut current_height = Height('a');
        let new_height = Height('b');
        assert_eq!(
            current_height.accessible(new_height),
            Accessability::Accessible
        )
    }

    #[test]
    fn accessible_check_invalid() {
        let mut current_height = Height('m');
        let new_height = Height('n');
        let unaccessible_new_height = Height('o');
        assert_eq!(
            current_height.accessible(new_height),
            Accessability::Accessible
        );
        assert_eq!(
            current_height.accessible(unaccessible_new_height),
            Accessability::InAccessible
        )
    }
}
