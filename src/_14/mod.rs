mod cave;
mod position;

use cave::Cave;

pub fn get_14_first() -> usize {
    first_part(INPUT)
}

pub fn get_14_second() -> usize {
    second_part(INPUT)
}

const INPUT: &str = include_str!("../../inputs/14/input.data");

pub fn first_part(input: &str) -> usize {
    let mut cave = Cave::new();
    cave.build(input);
    cave.fill();
    cave.count_sand()
}

fn second_part(input: &str) -> usize {
    let mut cave = Cave::new();
    cave.build(input);
    cave.build_floor();
    cave.fill();
    cave.count_sand()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = include_str!("../../inputs/14/input.example");
    const PROD: &str = include_str!("../../inputs/14/input.data");

    #[test]
    fn one_provided() {
        assert_eq!(first_part(PROVIDED), 4);
    }

    #[test]
    fn one_provided_prod() {
        assert_eq!(first_part(PROD), 4);
    }

    #[test]
    fn two_provided() {
        assert_eq!(second_part(PROVIDED), 4);
    }
}
