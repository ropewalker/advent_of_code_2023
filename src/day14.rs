use crate::day14::Tile::*;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Eq, PartialEq, Clone, Copy)]
enum Tile {
    Rounded,
    CubeShaped,
    EmptySpace,
}

#[derive(Eq, PartialEq, Clone)]
struct Platform(Vec<Vec<Tile>>);

impl Platform {
    fn tilt_north(&self) -> Self {
        let mut tilted_platform = self.clone();

        for column in 0..self.0.first().unwrap().len() {
            let mut cluster_start = 0;
            let mut cluster_len = 0;

            for row in 0..self.0.len() {
                match self.0[row][column] {
                    Rounded => {
                        cluster_len += 1;
                        tilted_platform.0[row][column] = EmptySpace;
                    }
                    CubeShaped => {
                        for y in cluster_start..cluster_start + cluster_len {
                            tilted_platform.0[y][column] = Rounded;
                        }

                        cluster_start = row + 1;
                        cluster_len = 0;
                    }
                    EmptySpace => {
                        tilted_platform.0[row][column] = EmptySpace;
                    }
                }
            }

            for y in cluster_start..cluster_start + cluster_len {
                tilted_platform.0[y][column] = Rounded;
            }
        }

        tilted_platform
    }

    fn tilt_west(&self) -> Self {
        let mut tilted_platform = self.clone();

        for row in 0..self.0.len() {
            let mut cluster_start = 0;
            let mut cluster_len = 0;

            for column in 0..self.0.first().unwrap().len() {
                match self.0[row][column] {
                    Rounded => {
                        cluster_len += 1;
                        tilted_platform.0[row][column] = EmptySpace;
                    }
                    CubeShaped => {
                        for x in cluster_start..cluster_start + cluster_len {
                            tilted_platform.0[row][x] = Rounded;
                        }

                        cluster_start = column + 1;
                        cluster_len = 0;
                    }
                    EmptySpace => {
                        tilted_platform.0[row][column] = EmptySpace;
                    }
                }
            }

            for x in cluster_start..cluster_start + cluster_len {
                tilted_platform.0[row][x] = Rounded;
            }
        }

        tilted_platform
    }

    fn tilt_south(&self) -> Self {
        let mut tilted_platform = self.clone();

        for column in 0..self.0.first().unwrap().len() {
            let mut cluster_start = self.0.len() - 1;
            let mut cluster_len = 0;

            for row in (0..self.0.len()).rev() {
                match self.0[row][column] {
                    Rounded => {
                        cluster_len += 1;
                        tilted_platform.0[row][column] = EmptySpace;
                    }
                    CubeShaped => {
                        for y in (cluster_start + 1 - cluster_len..=cluster_start).rev() {
                            tilted_platform.0[y][column] = Rounded;
                        }

                        cluster_start = if row >= 1 { row - 1 } else { 0 };
                        cluster_len = 0;
                    }
                    EmptySpace => {
                        tilted_platform.0[row][column] = EmptySpace;
                    }
                }
            }

            for y in (cluster_start + 1 - cluster_len..=cluster_start).rev() {
                tilted_platform.0[y][column] = Rounded;
            }
        }

        tilted_platform
    }

    fn tilt_east(&self) -> Self {
        let mut tilted_platform = self.clone();

        for row in 0..self.0.len() {
            let mut cluster_start = self.0.first().unwrap().len() - 1;
            let mut cluster_len = 0;

            for column in (0..self.0.first().unwrap().len()).rev() {
                match self.0[row][column] {
                    Rounded => {
                        cluster_len += 1;
                        tilted_platform.0[row][column] = EmptySpace;
                    }
                    CubeShaped => {
                        for x in (cluster_start + 1 - cluster_len..=cluster_start).rev() {
                            tilted_platform.0[row][x] = Rounded;
                        }

                        cluster_start = if column >= 1 { column - 1 } else { 0 };
                        cluster_len = 0;
                    }
                    EmptySpace => {
                        tilted_platform.0[row][column] = EmptySpace;
                    }
                }
            }

            for x in (cluster_start + 1 - cluster_len..=cluster_start).rev() {
                tilted_platform.0[row][x] = Rounded;
            }
        }

        tilted_platform
    }

    fn cycle(&self) -> Self {
        self.tilt_north().tilt_west().tilt_south().tilt_east()
    }

    fn load(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter().filter(|tile| **tile == Rounded).count() * (self.0.len() - y)
            })
            .sum()
    }
}

#[aoc_generator(day14)]
fn parse_input(platform: &str) -> Platform {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(platform:lines((tile:any_char => match tile {
        '#' => CubeShaped,
        'O' => Rounded,
        '.' => EmptySpace,
        _ => unreachable!(),
    })+) => Platform(platform));

    parser.parse(platform).unwrap()
}

#[aoc(day14, part1)]
fn part1(platform: &Platform) -> usize {
    platform.tilt_north().load()
}

const CYCLES: usize = 1_000_000_000;

#[aoc(day14, part2)]
fn part2(platform: &Platform) -> usize {
    let mut states = vec![platform.to_owned()];

    for cycle in 1..=CYCLES {
        let new_state = states.last().unwrap().cycle();

        if let Some(first_occurrence) = states.iter().position(|state| *state == new_state) {
            let cycle_len = cycle - first_occurrence;

            return states[(CYCLES - first_occurrence) % cycle_len + first_occurrence].load();
        } else {
            states.push(new_state);
        }
    }

    states.last().unwrap().load()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 136);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 64);
    }
}
