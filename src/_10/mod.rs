use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Command {
    Noop,
    AddX(i32),
}

impl FromStr for Command {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split(' ').collect::<Vec<&str>>();
        if words.len() == 1 {
            if words[0] == "noop" {
                return Ok(Command::Noop);
            } else {
                return Err("dear dairy I want you to be lactose free");
            }
        }
        match words[1].parse::<i32>() {
            Err(_) => Err("hello police it is illegal"),
            Ok(e) => Ok(Command::AddX(e)),
        }
    }
}

pub fn get_10_first() -> i32 {
    let content = include_str!("../../inputs/10/input.example");
    println!("{}", content);
    3
}

pub fn get_10_second() -> i32 {
    let content = include_str!("../../inputs/10/input.example");
    println!("{}", content);
    3
}

#[cfg(test)]
mod test_10 {
    use super::*;

    #[test]
    fn test_parse_command() {
        let noop = "noop".parse::<Command>();
        assert!(noop.is_ok());
    }

    #[test]
    fn test_parse_command_invalid() {
        let noop = "nooper".parse::<Command>();
        assert!(noop.is_err());
    }

    #[test]
    fn test_parse_int_command_valid() {
        let noop = "addx 1".parse::<Command>();
        assert!(noop.is_err());
    }
}
