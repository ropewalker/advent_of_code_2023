use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};
use std::ops::Add;
use Direction::*;
use MirrorOrSplitter::*;

#[derive(Eq, PartialEq, Copy, Clone)]
enum MirrorOrSplitter {
    NegativeMirror,
    PositiveMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

type Coordinates = (i32, i32);

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn positive_turn(&self) -> Self {
        let coordinates: Coordinates = (*self).into();
        (coordinates.1, coordinates.0).try_into().unwrap()
    }

    fn negative_turn(&self) -> Self {
        let coordinates: Coordinates = (*self).into();
        (-coordinates.1, -coordinates.0).try_into().unwrap()
    }

    fn transform(&self, mirror_or_splitter: &MirrorOrSplitter) -> Vec<Self> {
        match mirror_or_splitter {
            NegativeMirror => vec![self.negative_turn()],
            PositiveMirror => vec![self.positive_turn()],
            VerticalSplitter => match self {
                Up | Down => vec![*self],
                Left | Right => vec![self.positive_turn(), self.negative_turn()],
            },
            HorizontalSplitter => match self {
                Up | Down => vec![self.positive_turn(), self.negative_turn()],
                Left | Right => vec![*self],
            },
        }
    }
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

struct Layout {
    mirrors: HashMap<Coordinates, MirrorOrSplitter>,
    bottom_right: Coordinates,
}

#[aoc_generator(day16)]
fn parse_input(layout: &str) -> Layout {
    Layout {
        mirrors: layout
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(x, tile)| match tile {
                        '/' => Some(((x as i32, y as i32), NegativeMirror)),
                        '\\' => Some(((x as i32, y as i32), PositiveMirror)),
                        '|' => Some(((x as i32, y as i32), VerticalSplitter)),
                        '-' => Some(((x as i32, y as i32), HorizontalSplitter)),
                        _ => None,
                    })
            })
            .collect(),
        bottom_right: (
            layout.lines().next().unwrap().len() as i32 - 1,
            layout.lines().count() as i32 - 1,
        ),
    }
}

fn energized_count(
    layout: &Layout,
    starting_coordinates: Coordinates,
    starting_direction: Direction,
) -> usize {
    let mut visited = HashSet::from([(starting_coordinates, starting_direction)]);
    let mut current_states = vec![(starting_coordinates, starting_direction)];

    while let Some((coordinates, direction)) = current_states.pop() {
        let new_states = if let Some(mirror_or_splinter) = layout.mirrors.get(&coordinates) {
            direction
                .transform(mirror_or_splinter)
                .into_iter()
                .map(|direction| (coordinates + direction, direction))
                .collect::<Vec<_>>()
        } else {
            vec![(coordinates + direction, direction)]
        };

        for new_state in new_states.into_iter() {
            if !visited.contains(&new_state)
                && new_state.0 .0 >= 0
                && new_state.0 .0 <= layout.bottom_right.0
                && new_state.0 .1 >= 0
                && new_state.0 .1 <= layout.bottom_right.1
            {
                visited.insert(new_state);
                current_states.push(new_state);
            }
        }
    }

    visited
        .into_iter()
        .map(|(coordinates, _)| coordinates)
        .collect::<HashSet<_>>()
        .len()
}

#[aoc(day16, part1)]
fn part1(layout: &Layout) -> usize {
    energized_count(layout, (0, 0), Right)
}

#[aoc(day16, part2)]
fn part2(layout: &Layout) -> usize {
    let mut max_energized_count = 0;

    for x in 0..=layout.bottom_right.0 {
        let down_energized_count = energized_count(layout, (x, 0), Down);
        let up_energized_count = energized_count(layout, (x, layout.bottom_right.1), Up);

        max_energized_count = usize::max(
            usize::max(down_energized_count, up_energized_count),
            max_energized_count,
        );
    }

    for y in 0..=layout.bottom_right.1 {
        let down_energized_count = energized_count(layout, (0, y), Right);
        let up_energized_count = energized_count(layout, (layout.bottom_right.0, y), Left);

        max_energized_count = usize::max(
            usize::max(down_energized_count, up_energized_count),
            max_energized_count,
        );
    }

    max_energized_count
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 46);
    }
}
