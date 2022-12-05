use std::str::FromStr;

use super::{
    instruction::{read_elements_from_line, Instruction},
    stack::{CargoStack, Stack},
};

#[derive(Debug)]
pub struct Shipment {
    pub stacks: Vec<CargoStack>,
}

impl Shipment {
    pub fn apply(&mut self, instructions: Vec<Instruction>) {
        for instruction in instructions.iter() {
            let number_of_moves = instruction.amount;
            for _ in 0..number_of_moves {
                self.transfer_from_to(instruction.from as usize, instruction.to as usize);
            }
        }
    }
    pub fn apply_multiple(&mut self, instructions: Vec<Instruction>) {
        for instruction in instructions.iter() {
            self.transfer_from_to_multiple(
                instruction.from as usize,
                instruction.to as usize,
                instruction.amount,
            );
        }
    }

    pub fn transfer_from_to(&mut self, from: usize, to: usize) {
        let from = &mut self.stacks[from - 1];
        match from.pop().clone() {
            Some(e) => &mut self.stacks[to - 1].push(e),
            _ => &mut (),
        };
    }
    pub fn transfer_from_to_multiple(&mut self, from: usize, to: usize, amount: i32) {
        let from = &mut self.stacks[from - 1];
        let (remaining, transferable) = from
            .elements
            .split_at(from.elements.len() - amount as usize);
        let mut to_transfer = transferable.clone().to_owned().to_vec();
        from.elements = remaining.clone().to_vec();
        (&mut self.stacks[to - 1]).elements.append(&mut to_transfer);
    }
    pub fn get_top(&mut self) -> Vec<String> {
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

#[cfg(test)]
mod test_shipment {

    use super::*;

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
}
