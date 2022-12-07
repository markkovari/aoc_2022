#[derive(Debug)]
pub enum Command {
    Cd(bool),
    Ls,
    Size(i64),
    Dir,
}

pub fn input_generator(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| {
            if line.as_bytes()[0] == b'$' {
                let mut args = line[2..].split(' ');
                if let Some(cmd) = args.next() {
                    match cmd {
                        "ls" => Command::Ls,
                        "cd" => {
                            Command::Cd(args.next().expect("cd takes too args").to_owned() != "..")
                        }

                        _ => unreachable!("Unknown command: {}", line),
                    }
                } else {
                    unreachable!("Incorrect line: {}", line);
                }
            } else {
                let mut fields = line.split(' ');
                let first = fields.next().expect("Should be two fields");
                if "dir" == first {
                    Command::Dir
                } else {
                    Command::Size(first.parse().unwrap())
                }
            }
        })
        .collect()
}

fn find_dir_sizes(position: &mut usize, data: &[Command], dir_sizes: &mut Vec<i64>) -> i64 {
    let mut total = 0;
    while *position < data.len() {
        *position += 1;
        match &data[*position - 1] {
            Command::Cd(down) => {
                if !down {
                    dir_sizes.push(total);
                    return total;
                } else {
                    total += find_dir_sizes(position, data, dir_sizes);
                }
            }
            Command::Ls => (),
            Command::Dir => (),
            Command::Size(size) => {
                total += size;
            }
        }
    }
    dir_sizes.push(total);
    total
}

pub fn get_7_first() -> i64 {
    let mut dir_sizes = vec![];
    let content = include_str!("../../inputs/7/input.data");
    let _total = find_dir_sizes(&mut 0, &input_generator(content), &mut dir_sizes);
    dir_sizes.iter().filter(|&&dir| dir <= 100_000).sum()
}
pub fn get_7_second() -> i64 {
    let mut dir_sizes = vec![];
    let content = include_str!("../../inputs/7/input.data");

    const TOTAL_DISK: i64 = 70_000_000;
    const NEEDED_DISK: i64 = 30_000_000;
    let total_used = find_dir_sizes(&mut 0, &input_generator(content), &mut dir_sizes);
    let to_delete = total_used - (TOTAL_DISK - NEEDED_DISK);
    *dir_sizes.iter().filter(|&&n| n >= to_delete).min().unwrap()
}

#[cfg(test)]
mod test_7 {
    use super::*;

    #[test]
    fn calculate_example() {
        let mut dir_sizes = vec![];
        let content = include_str!("../../inputs/7/input.example");
        let _total = find_dir_sizes(&mut 0, &input_generator(content), &mut dir_sizes);
        let result: i64 = dir_sizes.iter().filter(|&&dir| dir <= 100_000).sum();
        assert_eq!(result, 95437);
    }
    #[test]
    fn calculate_example_second() {
        let mut dir_sizes = vec![];
        let content = include_str!("../../inputs/7/input.example");

        const TOTAL_DISK: i64 = 70000000;
        const NEEDED_DISK: i64 = 30000000;
        let total_used = find_dir_sizes(&mut 0, &input_generator(content), &mut dir_sizes);
        let to_delete = total_used - (TOTAL_DISK - NEEDED_DISK);
        let result: i64 = *dir_sizes.iter().filter(|&&n| n >= to_delete).min().unwrap();
        assert_eq!(result, 24933642);
    }
}
