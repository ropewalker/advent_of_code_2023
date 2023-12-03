use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash)]
struct SymbolCoordinates {
    x: i32,
    y: i32,
}

#[derive(Eq, PartialEq, Hash)]
struct NumberCoordinates {
    x_min: i32,
    x_max: i32,
    y: i32,
}

struct EngineSchematic {
    symbols: HashMap<SymbolCoordinates, char>,
    numbers: HashMap<NumberCoordinates, u32>,
}

#[aoc_generator(day3)]
fn parse_input(engine_schematic: &str) -> EngineSchematic {
    let mut symbols = HashMap::new();
    let mut numbers = HashMap::new();

    for (y, line) in engine_schematic.lines().enumerate() {
        let mut is_number = false;
        let mut number = 0;
        let mut number_x_min = 0;
        let mut number_x_max = 0;

        for (x, character) in line.chars().enumerate() {
            match character {
                digit if digit.is_ascii_digit() => {
                    if is_number {
                        number_x_max = x as i32;
                    } else {
                        number_x_min = x as i32;
                        number_x_max = x as i32;
                        is_number = true;
                    }

                    number = 10 * number + digit.to_digit(10).unwrap();
                }
                symbol => {
                    if is_number {
                        numbers.insert(
                            NumberCoordinates {
                                x_min: number_x_min,
                                x_max: number_x_max,
                                y: y as i32,
                            },
                            number,
                        );
                        is_number = false;
                        number = 0;
                    }

                    match symbol {
                        '.' => {}
                        _ => {
                            symbols.insert(
                                SymbolCoordinates {
                                    x: x as i32,
                                    y: y as i32,
                                },
                                symbol,
                            );
                        }
                    }
                }
            }
        }

        if is_number {
            numbers.insert(
                NumberCoordinates {
                    x_min: number_x_min,
                    x_max: number_x_max,
                    y: y as i32,
                },
                number,
            );
        }
    }

    EngineSchematic { symbols, numbers }
}

#[aoc(day3, part1)]
fn part1(engine_schematic: &EngineSchematic) -> u32 {
    let mut part_numbers_sum = 0;

    'numbers: for (number_coordinates, number) in engine_schematic.numbers.iter() {
        for y in number_coordinates.y - 1..=number_coordinates.y + 1 {
            for x in number_coordinates.x_min - 1..=number_coordinates.x_max + 1 {
                if engine_schematic
                    .symbols
                    .contains_key(&SymbolCoordinates { x, y })
                {
                    part_numbers_sum += number;
                    continue 'numbers;
                }
            }
        }
    }

    part_numbers_sum
}

#[aoc(day3, part2)]
fn part2(engine_schematic: &EngineSchematic) -> u32 {
    let mut gear_ratios_sum = 0;

    for gear_coordinates in engine_schematic
        .symbols
        .iter()
        .filter(|(_, symbol)| **symbol == '*')
        .map(|(coordinates, _)| coordinates)
    {
        let numbers: Vec<_> = engine_schematic
            .numbers
            .iter()
            .filter(|(number_coordinates, _)| {
                gear_coordinates.x >= number_coordinates.x_min - 1
                    && gear_coordinates.x <= number_coordinates.x_max + 1
                    && gear_coordinates.y >= number_coordinates.y - 1
                    && gear_coordinates.y <= number_coordinates.y + 1
            })
            .map(|(_, number)| number)
            .collect();

        if numbers.len() == 2 {
            gear_ratios_sum += **numbers.first().unwrap() * **numbers.last().unwrap();
        }
    }

    gear_ratios_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 4_361);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 467_835);
    }
}
