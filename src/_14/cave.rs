use super::position::Position;

const WIDTH: usize = 1000;
const HEIGHT: usize = 200;

#[derive(Debug)]
pub struct Cave {
    grid_xy: [[u8; HEIGHT]; WIDTH],
    max_y_of_rock: usize,
}

impl Cave {
    pub fn new() -> Self {
        Self {
            grid_xy: [[b' '; HEIGHT]; WIDTH],
            max_y_of_rock: 0,
        }
    }

    pub fn build(&mut self, input: &str) {
        for line in input.lines() {
            let mut coordinates = line
                .split(" -> ")
                .map(|pos| pos.parse::<Position>().unwrap());
            if let Some(head) = coordinates.next() {
                let mut from: Position = head;
                for to in coordinates {
                    if from.y == to.y {
                        let y = from.y;
                        let x0 = std::cmp::min(from.x, to.x);
                        let x1 = std::cmp::max(from.x, to.x);
                        for x in x0..=x1 {
                            self.grid_xy[x][y] = b'#';
                        }
                        if y > self.max_y_of_rock {
                            self.max_y_of_rock = y;
                        }
                    } else if from.x == to.x {
                        let x = from.x;
                        let y0 = std::cmp::min(from.y, to.y);
                        let y1 = std::cmp::max(from.y, to.y);
                        for y in y0..=y1 {
                            self.grid_xy[x][y] = b'#';
                        }
                        if y1 > self.max_y_of_rock {
                            self.max_y_of_rock = y1;
                        }
                    } else {
                        panic!("diagonal");
                    }
                    from = to;
                }
            }
        }
    }

    pub fn build_floor(&mut self) {
        let y = 2 + self.max_y_of_rock;
        for x in 0..WIDTH {
            self.grid_xy[x][y] = b'=';
        }
    }

    pub fn fill(&mut self) {
        while let Some(final_pos) = self.drop_grain() {
            self.grid_xy[final_pos.x][final_pos.y] = b'o';
        }
    }

    fn drop_grain(&mut self) -> Option<Position> {
        let mut pos = Position { x: WIDTH / 2, y: 0 };
        if self.grid_xy[pos.x][pos.y] != b' ' {
            return None;
        }
        while pos.y + 1 < HEIGHT {
            if self.grid_xy[pos.x][pos.y + 1] == b' ' {
                pos.y += 1;
            } else if self.grid_xy[pos.x - 1][pos.y + 1] == b' ' {
                pos.x -= 1;
                pos.y += 1;
            } else if self.grid_xy[pos.x + 1][pos.y + 1] == b' ' {
                pos.x += 1;
                pos.y += 1;
            }
            return Some(pos);
        }
        return None;
    }

    pub fn count_sand(&self) -> usize {
        self.grid_xy
            .iter()
            .map(|row| row.iter().filter(|a| **a == b'o').count())
            .sum()
    }
}
