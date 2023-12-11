use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

type Coordinates = (i64, i64);

#[aoc_generator(day11)]
fn parse_input(image: &str) -> Vec<Coordinates> {
    image
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars().enumerate().filter_map(move |(x, char)| {
                if char == '#' {
                    Some((x as i64, y as i64))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn sum_of_distances(image: &[Coordinates], expansion_factor: i64) -> i64 {
    let max_x = *image
        .iter()
        .max_by(|a, b| a.0.cmp(&b.0))
        .map(|(x, _)| x)
        .unwrap();
    let max_y = *image
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(_, y)| y)
        .unwrap();

    let empty_columns = (0..max_x)
        .filter(|x| !image.iter().any(|coordinates| coordinates.0 == *x))
        .collect::<HashSet<_>>();
    let empty_rows = (0..max_y)
        .filter(|y| !image.iter().any(|coordinates| coordinates.1 == *y))
        .collect::<HashSet<_>>();

    let mut distance_sum = 0;

    for i in 0..image.len() - 1 {
        for j in i + 1..image.len() {
            let coordinates_i = image[i];
            let coordinates_j = image[j];

            distance_sum += (coordinates_j.0 - coordinates_i.0).abs()
                + (coordinates_j.1 - coordinates_i.1).abs()
                + ((empty_columns
                    .iter()
                    .filter(|x| {
                        **x > i64::min(coordinates_i.0, coordinates_j.0)
                            && **x < i64::max(coordinates_i.0, coordinates_j.0)
                    })
                    .count() as i64)
                    + (empty_rows
                        .iter()
                        .filter(|y| {
                            **y > i64::min(coordinates_i.1, coordinates_j.1)
                                && **y < i64::max(coordinates_i.1, coordinates_j.1)
                        })
                        .count() as i64))
                    * (expansion_factor - 1);
        }
    }

    distance_sum
}

#[aoc(day11, part1)]
fn part1(image: &[Coordinates]) -> i64 {
    sum_of_distances(image, 2)
}

#[aoc(day11, part2)]
fn part2(image: &[Coordinates]) -> i64 {
    sum_of_distances(image, 1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 374);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(sum_of_distances(&parse_input(TEST_INPUT), 10), 1_030);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(sum_of_distances(&parse_input(TEST_INPUT), 100), 8_410);
    }
}
