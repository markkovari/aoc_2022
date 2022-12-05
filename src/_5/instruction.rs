use std::str::FromStr;

use regex::Regex;

#[derive(Debug)]
pub struct Instruction {
    pub amount: i32,
    pub from: i32,
    pub to: i32,
}

pub fn read_elements_from_line(line: String) -> Vec<(i32, Option<String>)> {
    let mut result: Vec<(i32, Option<String>)> = vec![];
    let mut remaining = line.clone();
    let mut index = 0;
    while remaining.len() >= 3 {
        let (current, rem) = remaining.split_at(3);
        if current.eq("   ") {
            result.push((index, None));
        } else {
            result.push((index, Some(format!("{}", current.chars().nth(1).unwrap()))));
        }
        index += 1;
        if remaining.len() > 3 {
            remaining = rem.split_at(1).1.to_owned();
        } else {
            break;
        }
    }
    result
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = Regex::new(r"(\d+)(.*)(\d+)(.*)(\d+)").unwrap();
        let found = pattern.captures(s);
        // If it works it ain't stupid
        // I will talk about this code with my therapist tho
        match found {
            Some(f) => Ok(Self {
                amount: f
                    .get(1)
                    .unwrap()
                    .to_owned()
                    .as_str()
                    .parse::<i32>()
                    .unwrap(),
                from: f
                    .get(3)
                    .unwrap()
                    .to_owned()
                    .as_str()
                    .parse::<i32>()
                    .unwrap(),
                to: f
                    .get(5)
                    .unwrap()
                    .to_owned()
                    .as_str()
                    .parse::<i32>()
                    .unwrap(),
            }),
            None => Err("Cannot convert to instruction".to_owned()),
        }
    }
}

#[cfg(test)]
mod test_instructions {
    use super::*;

    #[test]
    fn test() {
        let some = "move 1 from 2 to 3".parse::<Instruction>();
        assert!(some.is_ok());
        let result = some.unwrap();
        assert_eq!(result.amount, 1);
        assert_eq!(result.from, 2);
        assert_eq!(result.to, 3);
    }

    #[test]
    fn test_stack_line_reader_one_element() {
        let elements_with_positions = read_elements_from_line("[Z]".to_string());

        assert_eq!(elements_with_positions.len(), 1);
        assert_eq!(elements_with_positions[0], (0, Some("Z".to_owned())));
    }

    #[test]
    fn test_stack_line_reader_two_elements() {
        let elements_with_positions = read_elements_from_line("[Z] [X]".to_string());

        assert_eq!(elements_with_positions.len(), 2);
        assert_eq!(elements_with_positions[0], (0, Some("Z".to_owned())));
        assert_eq!(elements_with_positions[1], (1, Some("X".to_owned())));
    }

    #[test]
    fn test_stack_line_reader_three_elements() {
        let elements_with_positions = read_elements_from_line("[Z] [M] [P]".to_string());

        assert_eq!(elements_with_positions.len(), 3);
        assert_eq!(elements_with_positions[0], (0, Some("Z".to_owned())));
        assert_eq!(elements_with_positions[1], (1, Some("M".to_owned())));
        assert_eq!(elements_with_positions[2], (2, Some("P".to_owned())));
    }

    #[test]
    fn test_stack_line_reader_one_element_one_none_element() {
        let elements_with_positions = read_elements_from_line("    [D]".to_string());

        assert_eq!(elements_with_positions.len(), 2);
        assert_eq!(elements_with_positions[0], (0, None));
        assert_eq!(elements_with_positions[1], (1, Some("D".to_owned())));
    }

    #[test]
    fn test_stack_line_reader_one_element_two_none_element() {
        let elements_with_positions = read_elements_from_line("        [D]".to_string());

        assert_eq!(elements_with_positions.len(), 3);
        assert_eq!(elements_with_positions[0], (0, None));
        assert_eq!(elements_with_positions[1], (1, None));
        assert_eq!(elements_with_positions[2], (2, Some("D".to_owned())));
    }
}
