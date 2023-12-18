use aoc_runner_derive::{aoc, aoc_generator};
use std::ops::{Add, AddAssign, Mul};
use Direction::*;

type Coordinates = (i64, i64);

#[derive(Eq, PartialEq, Copy, Clone, Hash, Ord, PartialOrd)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl From<Direction> for Coordinates {
    fn from(value: Direction) -> Self {
        match value {
            Left => (-1, 0),
            Right => (1, 0),
            Up => (0, -1),
            Down => (0, 1),
        }
    }
}

impl Add<Direction> for Coordinates {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self {
        let rhs: (i64, i64) = rhs.into();

        (self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign<Direction> for Coordinates {
    fn add_assign(&mut self, rhs: Direction) {
        let rhs: (i64, i64) = rhs.into();

        *self = (self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul<i64> for Direction {
    type Output = Coordinates;

    fn mul(self, rhs: i64) -> Self::Output {
        let coordinates: Coordinates = self.into();
        (coordinates.0 * rhs, coordinates.1 * rhs)
    }
}

fn lagoon_volume(dig_plan: &[(Direction, i64)]) -> i64 {
    let mut coordinates = (0, 0);
    let mut double_area = 0;
    let mut perimeter = 0;

    let multiplier = 2;

    for (direction, length) in dig_plan {
        let shift = *direction * (*length * multiplier);
        let new_coordinates = (coordinates.0 + shift.0, coordinates.1 + shift.1);

        double_area += coordinates.0 * new_coordinates.1 - coordinates.1 * new_coordinates.0;
        perimeter += i64::abs(coordinates.0 - new_coordinates.0)
            + i64::abs(coordinates.1 - new_coordinates.1);

        coordinates = new_coordinates;
    }

    (double_area / 2 + perimeter + 4) / (multiplier * multiplier)
}

#[aoc_generator(day18)]
fn parse_input(dig_plan: &str) -> Vec<(Direction, i64, i64, Direction)> {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(lines(
        {
            "U" => Up, 
            "L" => Left, 
            "R" => Right, 
            "D" => Down
        }
        " " i64
        " (#" (
            length:string(digit_hex digit_hex digit_hex digit_hex digit_hex) =>
                i64::from_str_radix(&length, 16).unwrap()
        )
        {
            "0" => Right,
            "1" => Down,
            "2" => Left,
            "3" => Up,
        } ")"));

    parser.parse(dig_plan).unwrap()
}

#[aoc(day18, part1)]
fn part1(dig_plan: &[(Direction, i64, i64, Direction)]) -> i64 {
    let dig_plan = dig_plan
        .iter()
        .map(|(direction, length, _, _)| (*direction, *length))
        .collect::<Vec<_>>();

    lagoon_volume(&dig_plan)
}

#[aoc(day18, part2)]
fn part2(dig_plan: &[(Direction, i64, i64, Direction)]) -> i64 {
    let dig_plan = dig_plan
        .iter()
        .map(|(_, _, length, direction)| (*direction, *length))
        .collect::<Vec<_>>();

    lagoon_volume(&dig_plan)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 62);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 952_408_144_115);
    }
}
