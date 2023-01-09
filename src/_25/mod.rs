use std::io::Error;
const INPUT: &str = include_str!("../../inputs/25/input.example");
const PROD: &str = include_str!("../../inputs/25/input.data");

fn main() -> Result<(), Error> {
    let data = include_str!("../../inputs/25/input.example");
    let total = data.lines().map(snafu_to_decimal).sum::<i64>();
    println!("{}", decimal_to_snafu(total));
    Ok(())
}
fn snafu_to_decimal(snafu: &str) -> i64 {
    let mut total = 0;
    for (i, c) in snafu.chars().rev().enumerate() {
        let column = match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '-' => -1,
            '=' => -2,
            _ => unimplemented!(),
        };
        total += column * i64::pow(5, i as u32);
    }
    total
}

fn decimal_to_snafu(mut number: i64) -> String {
    let mut snafu_digits = Vec::new();
    while number > 0 {
        let remainder = number % 5;
        snafu_digits.push(match remainder {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '=',
            4 => '-',
            _ => unreachable!(),
        });
        if remainder >= 3 {
            number += 5
        }
        number /= 5;
    }
    snafu_digits.iter().rev().collect::<String>()
}

#[cfg(test)]
mod tests_25 {
    use super::*;

    #[test]

    fn test_input() {
        let total = INPUT.lines().map(snafu_to_decimal).sum::<i64>();
        assert_eq!(decimal_to_snafu(total), "2=-1=0");
    }
    #[test]

    fn test_prod() {
        let total = PROD.lines().map(snafu_to_decimal).sum::<i64>();
        assert_eq!(decimal_to_snafu(total), "20===-20-020=0001-02");
    }
}
