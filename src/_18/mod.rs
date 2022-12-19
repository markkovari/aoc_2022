use std::collections::HashSet;

use itertools::Itertools;

const PROD: &'static str = include_str!("../../inputs/18/input.data");
const EXAMPLE: &'static str = include_str!("../../inputs/18/input.example");

#[derive(Hash, PartialEq, Eq, Clone, Copy, Default)]
struct Coord {
    x: i16,
    y: i16,
    z: i16,
}

impl Coord {
    fn neighbours(&self) -> impl Iterator<Item = Coord> + '_ {
        [Dimension::X, Dimension::Y, Dimension::Z]
            .iter()
            .cartesian_product([-1, 1])
            .map(|(dimension, offset)| match dimension {
                Dimension::X => Coord {
                    x: self.x + offset,
                    ..*self
                },
                Dimension::Y => Coord {
                    y: self.y + offset,
                    ..*self
                },
                Dimension::Z => Coord {
                    z: self.z + offset,
                    ..*self
                },
            })
    }

    fn in_bounds(&self, bounds: &[Self; 2]) -> bool {
        let [mins, maxs] = bounds;
        self.x >= mins.x - 1
            && self.x <= maxs.x + 1
            && self.y >= mins.y - 1
            && self.y <= maxs.y + 1
            && self.z >= mins.z - 1
            && self.z <= maxs.z + 1
    }
}

enum Dimension {
    X,
    Y,
    Z,
}

fn parse(content: &str) -> HashSet<Coord> {
    content
        .lines()
        .map(|line| {
            let mut nums = line.split(",").map(|s| s.parse().unwrap());
            Coord {
                x: nums.next().unwrap(),
                y: nums.next().unwrap(),
                z: nums.next().unwrap(),
            }
        })
        .collect()
}

fn bounds(cubes: &HashSet<Coord>) -> [Coord; 2] {
    cubes.iter().fold(
        [Coord::default(), Coord::default()],
        |[mut mins, mut maxs], cube| {
            mins.x = mins.x.min(cube.x);
            mins.y = mins.y.min(cube.y);
            mins.z = mins.z.min(cube.z);
            maxs.x = maxs.x.max(cube.x);
            maxs.y = maxs.y.max(cube.y);
            maxs.z = maxs.z.max(cube.z);
            [mins, maxs]
        },
    )
}

fn exposed(cubes: &HashSet<Coord>) -> HashSet<Coord> {
    let bounds = bounds(cubes);
    let mut exposed = HashSet::new();

    let start = Coord::default();
    let mut stack = Vec::new();
    let mut seen = HashSet::new();

    stack.push(start);
    seen.insert(start);

    while let Some(coord) = stack.pop() {
        for neighbour in coord.neighbours() {
            if cubes.contains(&neighbour) || !neighbour.in_bounds(&bounds) {
                continue;
            }
            if seen.insert(neighbour) {
                stack.push(neighbour);
                exposed.insert(neighbour);
            }
        }
    }

    exposed
}

pub fn part_1(content: &str) -> usize {
    let cubes = parse(content);

    cubes
        .iter()
        .flat_map(|coord| coord.neighbours())
        // only keep neighbours that are not a cube
        .filter(|coord| !cubes.contains(coord))
        .count()
}

pub fn part_2(content: &str) -> usize {
    let cubes = parse(content);
    let exposed = exposed(&cubes);

    cubes
        .iter()
        .flat_map(|coord| coord.neighbours())
        // only keep neighbours that are also exposed
        .filter(|coord| exposed.contains(coord))
        .count()
}

#[cfg(test)]
mod test_18 {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let example_part_1 = part_1(EXAMPLE);
        assert_eq!(example_part_1, 64);
    }

    #[test]
    fn test_prod_part_1() {
        let prod_part_1 = part_1(PROD);
        assert_eq!(prod_part_1, 4460);
    }

    #[test]
    fn test_example_part_2() {
        let example_part_2 = part_2(EXAMPLE);
        assert_eq!(example_part_2, 58);
    }
    #[test]
    fn test_prod_part_2() {
        let prod_part_2 = part_2(PROD);
        assert_eq!(prod_part_2, 2498);
    }
}
