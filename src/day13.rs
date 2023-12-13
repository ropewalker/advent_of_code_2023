use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

type Coordinates = (i32, i32);

#[aoc_generator(day13)]
fn parse_input(map: &str) -> Vec<HashSet<Coordinates>> {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(
        patterns:sections(
            rows:lines(any_char+) =>
                rows.iter().enumerate().flat_map(|(y, row)|
                    row
                        .iter()
                        .enumerate()
                        .filter_map(move |(x, element)|
                            if *element == '#' {
                                Some((x as i32 + 1, y as i32 + 1))
                            } else {
                                None
                            })
                ).collect::<HashSet<Coordinates>>()
    ));

    parser.parse(map).unwrap()
}

fn has_reflection(
    (x, y): &Coordinates,
    pattern: &HashSet<Coordinates>,
    max_x: i32,
    max_y: i32,
    axis: &i32,
    axis_is_column: bool,
) -> bool {
    if axis_is_column {
        x <= axis && (2 * axis - x + 1 > max_x || pattern.contains(&(2 * axis - x + 1, *y)))
            || x > axis && (2 * axis - x + 1 < 1 || pattern.contains(&(2 * axis - x + 1, *y)))
    } else {
        y <= axis && (2 * axis - y + 1 > max_y || pattern.contains(&(*x, 2 * axis - y + 1)))
            || y > axis && (2 * axis - y + 1 < 1 || pattern.contains(&(*x, 2 * axis - y + 1)))
    }
}

fn summary(pattern: &HashSet<Coordinates>) -> usize {
    let max_x = pattern.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let max_y = pattern.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

    ((1..max_x)
        .find(|column| {
            pattern
                .iter()
                .all(|coordinates| has_reflection(coordinates, pattern, max_x, max_y, column, true))
        })
        .unwrap_or(0)
        + 100
            * (1..max_y)
                .find(|row| {
                    pattern.iter().all(|coordinates| {
                        has_reflection(coordinates, pattern, max_x, max_y, row, false)
                    })
                })
                .unwrap_or(0)) as usize
}

#[aoc(day13, part1)]
fn part1(patterns: &[HashSet<Coordinates>]) -> usize {
    patterns.iter().map(summary).sum()
}

fn corrected_summary(pattern: &HashSet<Coordinates>) -> usize {
    let max_x = pattern.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let max_y = pattern.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

    ((1..max_x)
        .find(|column| {
            pattern
                .iter()
                .filter(|coordinates| {
                    has_reflection(coordinates, pattern, max_x, max_y, column, true)
                })
                .count()
                == pattern.len() - 1
        })
        .unwrap_or(0)
        + 100
            * (1..max_y)
                .find(|row| {
                    pattern
                        .iter()
                        .filter(|coordinates| {
                            has_reflection(coordinates, pattern, max_x, max_y, row, false)
                        })
                        .count()
                        == pattern.len() - 1
                })
                .unwrap_or(0)) as usize
}

#[aoc(day13, part2)]
fn part2(patterns: &[HashSet<Coordinates>]) -> usize {
    patterns.iter().map(corrected_summary).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 405);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 400);
    }
}
