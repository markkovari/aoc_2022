use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

// 498,4 -> 498,6 -> 496,6
// 503,4 -> 502,4 -> 502,9 -> 494,9
impl FromStr for Position {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(",").expect("Should be separated by ,");
        let x = left.parse::<usize>().expect("should be positive number");
        let y = right.parse::<usize>().expect("should be positive number");
        return Ok(Self { x, y });
    }
}
