use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
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

    let mut horizontal_edges: HashMap<i64, Vec<(i64, i64)>> = HashMap::new();
    let mut vertical_edges: HashMap<i64, Vec<(i64, i64)>> = HashMap::new();
    let mut vertices = vec![coordinates];

    for (direction, length) in dig_plan {
        let shift = *direction * *length;
        let new_coordinates = (coordinates.0 + shift.0, coordinates.1 + shift.1);

        match direction {
            Right | Left => horizontal_edges.entry(coordinates.1).or_default().push((
                i64::min(coordinates.0, new_coordinates.0),
                i64::max(coordinates.0, new_coordinates.0),
            )),
            Down | Up => vertical_edges.entry(coordinates.0).or_default().push((
                i64::min(coordinates.1, new_coordinates.1),
                i64::max(coordinates.1, new_coordinates.1),
            )),
        };

        coordinates = new_coordinates;
        vertices.push(coordinates);
    }

    let mut count = 0;

    let mut ordinates = vertices.iter().map(|(_x, y)| *y).collect::<Vec<_>>();
    ordinates.sort_unstable();
    ordinates.dedup();

    let mut abscissas = vertices.iter().map(|(x, _y)| *x).collect::<Vec<_>>();
    abscissas.sort_unstable();
    abscissas.dedup();

    for y_window in ordinates.windows(2) {
        let y0 = y_window[0];
        let y1 = y_window[1];

        let mut inside = false;

        for x_window in abscissas.windows(2) {
            let x0 = x_window[0];
            let x1 = x_window[1];

            if vertical_edges
                .get(&x0)
                .unwrap()
                .iter()
                .any(|(start_y, end_y)| *start_y <= y0 && *end_y >= y1)
            {
                inside = !inside;
            }

            if inside {
                count += i64::max(0, x1 - x0 - 1) * i64::max(0, y1 - y0 - 1);

                if !horizontal_edges
                    .get(&y0)
                    .unwrap()
                    .iter()
                    .any(|(start_x, end_x)| *start_x <= x0 && *end_x >= x1)
                {
                    count += i64::max(0, x1 - x0 - 1);
                }

                if !vertical_edges
                    .get(&x0)
                    .unwrap()
                    .iter()
                    .any(|(start_y, end_y)| *start_y <= y0 && *end_y >= y1)
                {
                    count += i64::max(0, y1 - y0 - 1);
                }

                if !horizontal_edges
                    .get(&y0)
                    .unwrap()
                    .iter()
                    .any(|(start_x, end_x)| *start_x <= x0 && *end_x >= x0)
                    && !vertical_edges
                        .get(&x0)
                        .unwrap()
                        .iter()
                        .any(|(start_y, end_y)| *start_y <= y0 && *end_y >= y0)
                {
                    count += 1;
                }
            }
        }
    }

    count += horizontal_edges
        .values()
        .flat_map(|v| v.iter().map(|(x0, x1)| x1 - x0))
        .sum::<i64>();

    count += vertical_edges
        .values()
        .flat_map(|v| v.iter().map(|(y0, y1)| y1 - y0))
        .sum::<i64>();

    count
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
