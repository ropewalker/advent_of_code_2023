use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::ops::{Add, AddAssign};
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

#[aoc_generator(day21)]
fn parse_input(garden_map: &str) -> (HashSet<Coordinates>, i64) {
    let mut rocks_positions = HashSet::new();

    garden_map.lines().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, tile)| {
            if tile == '#' {
                rocks_positions.insert((x as i64, y as i64));
            }
        })
    });

    (rocks_positions, garden_map.lines().count() as i64)
}

fn accessible_plots_count(
    rocks_positions: &HashSet<Coordinates>,
    plot_size: i64,
    steps_remaining: i64,
) -> usize {
    let starting_position = (plot_size / 2, plot_size / 2);
    let mut frontier = HashSet::from([starting_position]);
    let mut visited = frontier.clone();

    for _ in 1..=steps_remaining {
        let mut new_frontier = HashSet::new();

        for position in frontier.iter() {
            [Right, Down, Left, Up].iter().for_each(|direction| {
                let new_coordinates = *position + *direction;
                if !rocks_positions.contains(&(
                    (new_coordinates.0 % plot_size + plot_size) % plot_size,
                    (new_coordinates.1 % plot_size + plot_size) % plot_size,
                )) && !visited.contains(&new_coordinates)
                {
                    new_frontier.insert(new_coordinates);
                    visited.insert(new_coordinates);
                }
            });
        }

        frontier = new_frontier;
    }

    visited
        .iter()
        .filter(|coordinates| (coordinates.0 + coordinates.1) % 2 == steps_remaining % 2)
        .count()
}

#[aoc(day21, part1)]
fn part1((rocks_positions, plot_size): &(HashSet<Coordinates>, i64)) -> usize {
    accessible_plots_count(rocks_positions, *plot_size, 64)
}

fn calculate_distances(
    rocks_positions: &HashSet<Coordinates>,
    plot_size: i64,
    max_distance: i64,
) -> HashMap<Coordinates, i64> {
    let starting_position = (plot_size / 2, plot_size / 2);

    let mut queue = VecDeque::from([(starting_position, 0)]);
    let mut visited = HashMap::from([(starting_position, 0)]);

    while let Some((position, distance)) = queue.pop_front() {
        [Right, Down, Left, Up].into_iter().for_each(|direction| {
            let new_coordinates = position + direction;

            if !rocks_positions.contains(&(
                (new_coordinates.0 % plot_size + plot_size) % plot_size,
                (new_coordinates.1 % plot_size + plot_size) % plot_size,
            )) && !visited.contains_key(&new_coordinates)
                && distance < max_distance
            {
                queue.push_back((new_coordinates, distance + 1));
                visited.insert(new_coordinates, distance + 1);
            }
        });
    }

    visited
}

#[aoc(day21, part2)]
fn part2((rocks_positions, plot_size): &(HashSet<Coordinates>, i64)) -> i64 {
    let n = (26_501_365 - plot_size / 2) / plot_size;

    let distances = calculate_distances(rocks_positions, *plot_size, plot_size / 2 + plot_size * 2);

    let y0 = distances
        .values()
        .filter(|distance| **distance <= plot_size / 2 && *distance % 2 != 0)
        .count() as i64;

    let y1 = distances
        .values()
        .filter(|distance| **distance <= plot_size / 2 + plot_size && *distance % 2 == 0)
        .count() as i64
        - y0;

    let y2 = distances
        .values()
        .filter(|distance| **distance <= plot_size / 2 + plot_size * 2 && *distance % 2 != 0)
        .count() as i64
        - y0;

    let a = (y2 - 2 * y1) / 2;
    let b = y1 - a;

    a * n * n + b * n + y0
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str = r"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    fn initialize() -> (HashSet<Coordinates>, i64) {
        parse_input(TEST_INPUT_1)
    }

    #[test]
    fn example1() {
        let (rocks_positions, plot_size) = initialize();

        assert_eq!(accessible_plots_count(&rocks_positions, plot_size, 6), 16);
    }

    #[test]
    fn example2() {
        let (rocks_positions, plot_size) = initialize();

        assert_eq!(accessible_plots_count(&rocks_positions, plot_size, 10), 50);
    }

    #[test]
    fn example3() {
        let (rocks_positions, plot_size) = initialize();

        assert_eq!(
            accessible_plots_count(&rocks_positions, plot_size, 50),
            1_594
        );
    }

    #[test]
    fn example4() {
        let (rocks_positions, plot_size) = initialize();

        assert_eq!(
            accessible_plots_count(&rocks_positions, plot_size, 100),
            6_536
        );
    }

    #[test]
    fn example5() {
        let (rocks_positions, plot_size) = initialize();

        assert_eq!(
            accessible_plots_count(&rocks_positions, plot_size, 500),
            167_004
        );
    }

    #[test]
    fn example6() {
        let (rocks_positions, plot_size) = initialize();

        assert_eq!(
            accessible_plots_count(&rocks_positions, plot_size, 1_000),
            668_697
        );
    }

    #[test]
    fn example7() {
        let (rocks_positions, plot_size) = initialize();

        assert_eq!(
            accessible_plots_count(&rocks_positions, plot_size, 5_000),
            16_733_044
        );
    }
}
