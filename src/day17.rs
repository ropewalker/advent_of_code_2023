use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::ops::{Add, AddAssign};
use Direction::*;

type Coordinates = (i32, i32);

#[derive(Eq, PartialEq, Copy, Clone, Hash, Ord, PartialOrd)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
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

impl TryFrom<Coordinates> for Direction {
    type Error = ();

    fn try_from(value: Coordinates) -> Result<Self, Self::Error> {
        match value {
            (-1, 0) => Ok(Left),
            (1, 0) => Ok(Right),
            (0, -1) => Ok(Up),
            (0, 1) => Ok(Down),
            _ => Err(()),
        }
    }
}

impl Add<Direction> for Coordinates {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self {
        let rhs: (i32, i32) = rhs.into();

        (self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign<Direction> for Coordinates {
    fn add_assign(&mut self, rhs: Direction) {
        let rhs: (i32, i32) = rhs.into();

        *self = (self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Direction {
    fn turn_right(&self) -> Self {
        let coordinates: Coordinates = (*self).into();
        (coordinates.1, coordinates.0).try_into().unwrap()
    }

    fn turn_left(&self) -> Self {
        let coordinates: Coordinates = (*self).into();
        (-coordinates.1, -coordinates.0).try_into().unwrap()
    }
}

#[aoc_generator(day17)]
fn parse_input(heat_loss_map: &str) -> Vec<Vec<usize>> {
    use aoc_parse::{parser, prelude::*};
    parser!(lines(digit+)).parse(heat_loss_map).unwrap()
}

#[derive(Eq, PartialEq)]
struct State {
    coordinates: Coordinates,
    direction: Direction,
    heat_loss: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .heat_loss
            .cmp(&self.heat_loss)
            .then_with(|| {
                (self.coordinates.0 + self.coordinates.1)
                    .cmp(&(other.coordinates.0 + other.coordinates.1))
            })
            .then_with(|| other.direction.cmp(&self.direction))
            .then_with(|| self.coordinates.0.cmp(&other.coordinates.0))
            .then_with(|| self.coordinates.1.cmp(&other.coordinates.1))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn min_heat_loss(
    heat_loss_map: &[Vec<usize>],
    min_consecutive: usize,
    max_consecutive: usize,
) -> usize {
    let max_x = heat_loss_map[0].len() as i32 - 1;
    let max_y = heat_loss_map.len() as i32 - 1;

    let mut visited: HashMap<(Coordinates, Direction), usize> = HashMap::new();

    let mut nodes: BinaryHeap<State> = BinaryHeap::new();

    nodes.push(State {
        coordinates: (1, 0),
        direction: Right,
        heat_loss: 0,
    });

    nodes.push(State {
        coordinates: (0, 1),
        direction: Down,
        heat_loss: 0,
    });

    let mut min_heat_loss: Option<usize> = None;

    while let Some(State {
        coordinates,
        direction,
        heat_loss,
    }) = nodes.pop()
    {
        let prev_heat_loss = visited
            .entry((coordinates, direction))
            .or_insert(heat_loss + 1);

        if *prev_heat_loss > heat_loss {
            *prev_heat_loss = heat_loss;
        } else {
            continue;
        }

        let mut heat_loss = heat_loss;
        let mut coordinates = coordinates;

        for step in 0..max_consecutive as i32 {
            if coordinates.0 < 0
                || coordinates.0 > max_x
                || coordinates.1 < 0
                || coordinates.1 > max_y
            {
                break;
            }

            heat_loss += heat_loss_map[coordinates.1 as usize][coordinates.0 as usize];

            if coordinates == (max_x, max_y) {
                if min_heat_loss.unwrap_or(heat_loss + 1) > heat_loss
                    && step >= min_consecutive as i32 - 1
                {
                    min_heat_loss = Some(heat_loss);
                }

                break;
            }

            if step >= min_consecutive as i32 - 1 {
                nodes.push(State {
                    coordinates: coordinates + direction.turn_left(),
                    direction: direction.turn_left(),
                    heat_loss,
                });
                nodes.push(State {
                    coordinates: coordinates + direction.turn_right(),
                    direction: direction.turn_right(),
                    heat_loss,
                });
            }

            coordinates += direction;
        }
    }

    min_heat_loss.unwrap_or(0)
}

#[aoc(day17, part1)]
fn part1(heat_loss_map: &[Vec<usize>]) -> usize {
    min_heat_loss(heat_loss_map, 0, 3)
}

#[aoc(day17, part2)]
fn part2(heat_loss_map: &[Vec<usize>]) -> usize {
    min_heat_loss(heat_loss_map, 4, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    static TEST_INPUT_1: &str = r"111111111111
999999999991
999999999991
999999999991
999999999991";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 102);
    }

    #[test]
    fn part2_example_1() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 94);
    }

    #[test]
    fn part2_example_2() {
        assert_eq!(part2(&parse_input(TEST_INPUT_1)), 71);
    }
}
