mod instruction;
mod shipment;
mod stack;

use self::{instruction::Instruction, shipment::Shipment};

fn read_stack_and_instructions() -> (Shipment, Vec<Instruction>) {
    let content = include_str!("../../inputs/5/input.data")
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

pub fn get_5_second() -> String {
    let (mut stack, instructions) = read_stack_and_instructions();

    stack.apply_multiple(instructions);
    stack.get_top().join("")
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::_5::stack::CargoStack;

    use super::*;

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
