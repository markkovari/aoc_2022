use std::{fmt::format, str::FromStr};

use regex::Regex;

trait Stack<T> {
    fn push(&mut self, element: T);
    fn pop(&mut self) -> Option<T>;
    fn peek(self) -> Option<T>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CargoStack {
    elements: Vec<String>,
}

impl CargoStack {
    pub fn new() -> Self {
        Self { elements: vec![] }
    }
    fn transfer_top(&mut self, to: &mut CargoStack) {
        let item_to_transfer = self.pop();
        if item_to_transfer.is_some() {
            to.push(item_to_transfer.unwrap());
        }
    }
}

impl Stack<String> for CargoStack {
    fn push(self: &mut CargoStack, element: String) {
        self.elements.push(element)
    }
    fn pop(self: &mut CargoStack) -> Option<String> {
        self.elements.pop()
    }
    fn peek(self: CargoStack) -> Option<String> {
        match self.elements.get(self.elements.len() - 1) {
            Some(v) => Some(v.to_string()),
            None => None,
        }
    }
}

#[derive(Debug)]
struct Instruction {
    amount: i32,
    from: i32,
    to: i32,
}

fn read_elements_from_line(line: String) -> Vec<(i32, Option<String>)> {
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

#[derive(Debug)]
struct Shipment {
    stacks: Vec<CargoStack>,
}

impl Shipment {
    fn apply(&mut self, instructions: Vec<Instruction>) {
        for instruction in instructions.iter() {
            let number_of_moves = instruction.amount;
            for _ in 0..number_of_moves {
                self.transfer_from_to(instruction.from as usize, instruction.to as usize);
            }
        }
    }

    fn transfer_from_to(&mut self, from: usize, to: usize) {
        let from = &mut self.stacks[from - 1];
        match from.pop().clone() {
            Some(e) => &mut self.stacks[to - 1].push(e),
            _ => &mut (),
        };
    }
    fn get_top(&mut self) -> Vec<String> {
        self.stacks
            .clone()
            .iter()
            .filter_map(|stack| match stack.clone().peek() {
                Some(v) => Some(v),
                None => None,
            })
            .collect::<Vec<String>>()
    }
}

impl FromStr for Shipment {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            return Err("BRUV, where shipment?".to_owned());
        }
        let reversed_lines = s.lines().rev().collect::<Vec<&str>>();
        let stack_amount = reversed_lines[0]
            .split(" ")
            .filter_map(|e| match e.parse::<i32>() {
                Err(_) => None,
                Ok(e) => Some(e),
            })
            .count();
        let item_lines = reversed_lines
            .iter()
            .skip(1)
            .map(|&a| a)
            .collect::<Vec<&str>>();
        let mut stacks = (0..stack_amount)
            .into_iter()
            .map(|_| CargoStack::new())
            .collect::<Vec<CargoStack>>();
        for line in item_lines {
            for (stack_index, element) in read_elements_from_line(line.to_owned()) {
                match element {
                    Some(val) => stacks[stack_index as usize].push(val),
                    _ => {}
                }
            }
        }
        Ok(Self { stacks })
    }
}

fn read_stack_and_instructions() -> (Shipment, Vec<Instruction>) {
    let content = include_str!("../inputs/5/input.data")
        .split("\n\n")
        .collect::<Vec<&str>>();

    let shipment = content[0].parse::<Shipment>().unwrap();

    let instructions = content[1]
        .to_owned()
        .lines()
        .filter_map(|line| match line.parse::<Instruction>() {
            Err(_) => None,
            Ok(e) => Some(e),
        })
        .collect();

    (shipment, instructions)
}

pub fn get_5_first() -> String {
    let (mut stack, instructions) = read_stack_and_instructions();

    stack.apply(instructions);
    stack.get_top().join("")
}

pub fn get_5_second() -> i32 {
    1
}

#[cfg(test)]
mod tests {
    use std::vec;

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

    #[test]
    fn test_shipment_empty_error() {
        let shipment = "".parse::<Shipment>();
        assert!(shipment.is_err());
    }

    #[test]
    fn test_shipment_one_stack_one_element() {
        let shipment = "[A]\n1".parse::<Shipment>();
        assert!(shipment.is_ok());
    }

    #[test]
    fn test_shipment_two_stack_one_element() {
        let shipment = "[A]\n1 2".parse::<Shipment>();
        assert!(shipment.is_ok());
        assert_eq!(shipment.unwrap().stacks.len(), 2);
    }

    #[test]
    fn test_shipment_two_stack_two_element() {
        let shipment = "[B] [A]\n1 2".parse::<Shipment>();
        assert!(shipment.is_ok());
        assert_eq!(shipment.unwrap().stacks.len(), 2);
    }

    #[test]
    fn test_shipment_with_example_input() {
        let shipment = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3"
            .parse::<Shipment>();
        assert!(shipment.is_ok());
        let first: CargoStack = CargoStack {
            elements: vec!["Z".to_owned(), "N".to_owned()],
        };
        let second: CargoStack = CargoStack {
            elements: vec!["M".to_owned(), "C".to_owned(), "D".to_owned()],
        };
        let third: CargoStack = CargoStack {
            elements: vec!["P".to_owned()],
        };
        let shipment = shipment.unwrap();
        assert_eq!(shipment.stacks.len(), 3);
        assert_eq!(shipment.stacks[0], first);
        assert_eq!(shipment.stacks[1], second);
        assert_eq!(shipment.stacks[2], third);
    }

    #[test]
    fn test_stack_transfer() {
        let mut first: CargoStack = CargoStack {
            elements: vec!["Z".to_owned(), "N".to_owned()],
        };
        let mut second: CargoStack = CargoStack {
            elements: vec!["M".to_owned(), "C".to_owned(), "D".to_owned()],
        };
        first.transfer_top(&mut second);
        assert_eq!(first.elements.len(), 1);
        assert_eq!(second.elements.len(), 4);
        assert_eq!(first.elements[0], "Z");
        assert_eq!(
            second.elements,
            vec![
                "M".to_owned(),
                "C".to_owned(),
                "D".to_owned(),
                "N".to_owned(),
            ]
        );
    }

    #[test]
    fn test_shipment_with_example_input_after_first_instruction() {
        let shipment = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3"
            .parse::<Shipment>();
        assert!(shipment.is_ok());

        let mut shipment = shipment.unwrap();

        let inst: Instruction = Instruction {
            amount: 1,
            from: 1,
            to: 2,
        };
        shipment.apply(vec![inst]);
        assert_eq!(shipment.stacks.len(), 3);
        assert_eq!(shipment.stacks[0].elements, vec!["Z".to_owned()]);
        let first: CargoStack = CargoStack {
            elements: vec!["Z".to_owned()],
        };
        let second: CargoStack = CargoStack {
            elements: vec![
                "M".to_owned(),
                "C".to_owned(),
                "D".to_owned(),
                "N".to_owned(),
            ],
        };
        let third: CargoStack = CargoStack {
            elements: vec!["P".to_owned()],
        };
        assert_eq!(shipment.stacks[0], first);
        assert_eq!(shipment.stacks[1], second);
        assert_eq!(shipment.stacks[2], third);
    }

    #[test]
    fn test_shipment_results() {
        let shipment = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3"
            .parse::<Shipment>();
        assert!(shipment.is_ok());

        let mut shipment = shipment.unwrap();
        assert_eq!(
            shipment.get_top(),
            vec!["N".to_owned(), "D".to_owned(), "P".to_owned(),]
        );
    }
}
