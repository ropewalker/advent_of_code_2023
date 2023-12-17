use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, VecDeque};
use std::ops::Add;
use Direction::*;

type Coordinates = (i32, i32);

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
enum Direction {
    Up,
    Down,
    Left,
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

type State = (Coordinates, Direction, usize);
type Node = (State, usize);

fn min_heat_loss(
    heat_loss_map: &[Vec<usize>],
    min_consecutive: usize,
    max_consecutive: usize,
) -> usize {
    let max_x = heat_loss_map[0].len() as i32 - 1;
    let max_y = heat_loss_map.len() as i32 - 1;

    let mut visited: HashMap<State, usize> = HashMap::new();
    let mut nodes: VecDeque<Node> =
        VecDeque::from([(((0, 0), Right, 0), 0), (((0, 0), Down, 0), 0)]);

    let mut min_heat_loss: Option<usize> = None;

    while let Some(((coordinates, direction, straight_line_len), heat_loss)) = nodes.pop_front() {
        let mut new_states = Vec::new();

        if straight_line_len < max_consecutive {
            new_states.push((coordinates + direction, direction, straight_line_len + 1));
        }

        if straight_line_len >= min_consecutive {
            new_states.push((
                coordinates + direction.turn_left(),
                direction.turn_left(),
                1,
            ));
            new_states.push((
                coordinates + direction.turn_right(),
                direction.turn_right(),
                1,
            ));
        }

        for new_state in new_states.into_iter() {
            let (new_coordinates, new_direction, new_straight_line_len) = new_state;

            if new_coordinates.0 >= 0
                && new_coordinates.0 <= max_x
                && new_coordinates.1 >= 0
                && new_coordinates.1 <= max_y
            {
                let new_heat_loss = heat_loss
                    + heat_loss_map[new_coordinates.1 as usize][new_coordinates.0 as usize];

                let prev_heat_loss = visited.entry(new_state).or_insert(new_heat_loss + 1);

                if *prev_heat_loss > new_heat_loss {
                    *prev_heat_loss = new_heat_loss;

                    if new_coordinates == (max_x, max_y)
                        && new_straight_line_len >= min_consecutive
                        && min_heat_loss.unwrap_or(new_heat_loss + 1) > new_heat_loss
                    {
                        min_heat_loss = Some(new_heat_loss);
                    }

                    nodes.push_back((
                        (new_coordinates, new_direction, new_straight_line_len),
                        new_heat_loss,
                    ));
                }
            }
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
