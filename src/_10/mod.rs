use std::str::FromStr;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

impl FromStr for Instruction {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split(' ').collect::<Vec<&str>>();
        if words.len() == 1 {
            if words[0] == "noop" {
                return Ok(Instruction::Noop);
            } else {
                return Err("dear dairy I want you to be lactose free");
            }
        }
        match words[1].parse::<i32>() {
            Err(_) => Err("hello police it is illegal"),
            Ok(e) => Ok(Instruction::AddX(e)),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Computer {
    iteration_values: Vec<(usize, i32)>,
}

impl Default for Computer {
    fn default() -> Self {
        Self::new()
    }
}

impl Computer {
    pub fn new() -> Self {
        Self {
            iteration_values: vec![(0, 1)],
        }
    }

    pub fn apply(&mut self, command: Instruction) {
        match command {
            Instruction::Noop => {
                let iteration_len = self.iteration_values.len();
                let new_value = self.iteration_values[iteration_len - 1].1;
                self.iteration_values.push((iteration_len + 1, new_value));
            }
            Instruction::AddX(e) => {
                self.apply(Instruction::Noop);
                let iteration_len = self.iteration_values.len();
                let new_value = self.iteration_values[iteration_len - 1].1 + e;
                self.iteration_values.push((iteration_len + 1, new_value));
            }
        }
    }

    fn apply_multiple(&mut self, commands: Vec<Instruction>) {
        for command in commands {
            self.apply(command)
        }
    }

    fn get_signal_strength(&self, at: usize) -> Option<i32> {
        match self.iteration_values.iter().find(|(e, _)| *e == at) {
            Some((k, v)) => Some((*k as i32) * *v),
            None => None,
        }
    }
    fn get_every_fortieth_sum(&self) -> i32 {
        [20, 60, 100, 140, 180, 220]
            .iter()
            .map(|e| self.get_signal_strength(*e).unwrap())
            .fold(0, |a, b| a + b)
    }

    fn render(&self) -> String {
        self.iteration_values
            .chunks(40)
            .into_iter()
            .flat_map(|row| {
                row.iter()
                    .enumerate()
                    .map(|(i, x)| {
                        if x.1.abs_diff(i as i32) <= 1 {
                            '\u{2588}'
                        } else {
                            ' '
                        }
                    })
                    .chain(std::iter::once('\n'))
            })
            .collect::<String>()
    }
}

pub fn get_10_first() -> i32 {
    let content = include_str!("../../inputs/10/input.data");
    let instructions = content
        .lines()
        .filter_map(|line| match line.parse() {
            Err(_) => None,
            Ok(e) => Some(e),
        })
        .collect::<Vec<Instruction>>();
    let mut computer = Computer::default();
    computer.apply_multiple(instructions);
    computer.get_every_fortieth_sum()
}

pub fn get_10_second() -> String {
    let content = include_str!("../../inputs/10/input.data");
    let instructions = content
        .lines()
        .filter_map(|line| match line.parse() {
            Err(_) => None,
            Ok(e) => Some(e),
        })
        .collect::<Vec<Instruction>>();
    let mut computer = Computer::default();
    computer.apply_multiple(instructions);
    computer.render()
}

#[cfg(test)]
mod test_10 {
    use super::*;

    fn read_instructions(content: &str) -> Vec<Instruction> {
        content
            .lines()
            .filter_map(|line| match line.parse::<Instruction>() {
                Ok(e) => Some(e),
                Err(_) => None,
            })
            .collect()
    }

    #[test]
    fn test_parse_command() {
        let noop = "noop".parse::<Instruction>();
        assert!(noop.is_ok());
    }

    #[test]
    fn test_parse_command_invalid() {
        let noop = "nooper".parse::<Instruction>();
        assert!(noop.is_err());
    }

    #[test]
    fn test_parse_int_command_valid() {
        let noop = "addx 1".parse::<Instruction>();
        assert!(noop.is_ok());
    }

    #[test]
    fn computer_is_initially_one() {
        let computer = Computer::default();
        assert_eq!(computer.iteration_values.len(), 1);
    }

    #[test]
    fn command_list_is_valid() {
        let command_text = "noop\naddx 3\naddx -5";
        let commands = read_instructions(command_text);
        assert_eq!(commands.len(), 3);
        assert_eq!(commands[0], Instruction::Noop);
        assert_eq!(commands[1], Instruction::AddX(3));
        assert_eq!(commands[2], Instruction::AddX(-5));
    }

    #[test]
    fn command_list_is_applied() {
        let command_text = "noop\naddx 3\naddx -5";
        let commands = read_instructions(command_text);
        let mut computer = Computer::default();
        computer.apply_multiple(commands);
        assert_eq!(computer.iteration_values[0].1, 1);
    }

    #[test]
    fn command_list_is_applied_first_example() {
        let commands = include_str!("../../inputs/10/input.example")
            .lines()
            .filter_map(|line| match line.parse::<Instruction>() {
                Err(_) => None,
                Ok(e) => Some(e),
            })
            .collect::<Vec<Instruction>>();
        let mut computer = Computer::default();
        computer.apply_multiple(commands);
        let at_20 = computer.get_signal_strength(20);
        assert!(at_20.is_some());
        assert_eq!(at_20.unwrap(), 420);
        let at_60 = computer.get_signal_strength(60);
        assert!(at_60.is_some());
        assert_eq!(at_60.unwrap(), 1140);
        let at_100 = computer.get_signal_strength(100);
        assert!(at_100.is_some());
        assert_eq!(at_100.unwrap(), 1800);
        let at_140 = computer.get_signal_strength(140);
        assert!(at_140.is_some());
        assert_eq!(at_140.unwrap(), 2940);
        let at_180 = computer.get_signal_strength(180);
        assert!(at_180.is_some());
        assert_eq!(at_180.unwrap(), 2880);
        let at_220 = computer.get_signal_strength(220);
        assert!(at_220.is_some());
        assert_eq!(at_220.unwrap(), 3960);
    }

    #[test]
    fn command_list_is_applied_first_example_get_sum() {
        let commands = include_str!("../../inputs/10/input.example")
            .lines()
            .filter_map(|line| match line.parse::<Instruction>() {
                Err(_) => None,
                Ok(e) => Some(e),
            })
            .collect::<Vec<Instruction>>();
        let mut computer = Computer::default();
        computer.apply_multiple(commands);
        let result = computer.get_every_fortieth_sum();
        assert_eq!(result, 13140);
    }

    #[test]
    fn command_list_is_applied_rendered() {
        let commands = include_str!("../../inputs/10/input.data")
            .lines()
            .filter_map(|line| match line.parse::<Instruction>() {
                Err(_) => None,
                Ok(e) => Some(e),
            })
            .collect::<Vec<Instruction>>();
        let mut computer = Computer::default();
        computer.apply_multiple(commands);
        let result = computer.render();

        /* some ascii art

        ███  ████ █  █ ████  ██  ███  ████ ████
        █  █ █    █ █     █ █  █ █  █ █    █
        █  █ ███  ██     █  █    █  █ ███  ███
        ███  █    █ █   █   █    ███  █    █
        █ █  █    █ █  █    █  █ █    █    █
        █  █ █    █  █ ████  ██  █    ████ █

        */
        let expected_string = "███  ████ █  █ ████  ██  ███  ████ ████ \n█  █ █    █ █     █ █  █ █  █ █    █    \n█  █ ███  ██     █  █    █  █ ███  ███  \n███  █    █ █   █   █    ███  █    █    \n█ █  █    █ █  █    █  █ █    █    █    \n█  █ █    █  █ ████  ██  █    ████ █    \n \n";
        assert_eq!(result.to_owned(), expected_string.to_owned());
    }
}
