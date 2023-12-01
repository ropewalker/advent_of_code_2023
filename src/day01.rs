use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input(calibration_document: &str) -> Vec<String> {
    calibration_document.lines().map(|l| l.to_owned()).collect()
}

fn calibration_value(line: &str) -> u32 {
    let first_digit = line
        .chars()
        .find(|c| c.is_ascii_digit())
        .unwrap()
        .to_digit(10)
        .unwrap();

    let second_digit = line
        .chars()
        .rev()
        .find(|c| c.is_ascii_digit())
        .unwrap()
        .to_digit(10)
        .unwrap();

    10 * first_digit + second_digit
}

#[aoc(day1, part1)]
fn part1(calibration_document: &[String]) -> u32 {
    calibration_document
        .iter()
        .map(|l| calibration_value(l))
        .sum()
}

const ONE: &str = "one";
const TWO: &str = "two";
const THREE: &str = "three";
const FOUR: &str = "four";
const FIVE: &str = "five";
const SIX: &str = "six";
const SEVEN: &str = "seven";
const EIGHT: &str = "eight";
const NINE: &str = "nine";

fn calibration_value_spelled_out_with_letters(line: &str) -> u32 {
    let spelled_out_digits = [ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE];

    let mut first_digit = 0;
    let mut current_letter_indices = [0; 9];

    'line_characters: for character in line.chars() {
        if character.is_ascii_digit() {
            first_digit = character.to_digit(10).unwrap();
            break;
        }

        for digit in 1..=9 {
            let current_letter_index = current_letter_indices[digit - 1];
            let spelled_out_digit = spelled_out_digits[digit - 1];

            if spelled_out_digit.chars().nth(current_letter_index).unwrap() == character {
                if current_letter_index == spelled_out_digit.len() - 1 {
                    first_digit = digit as u32;
                    break 'line_characters;
                } else {
                    current_letter_indices[digit - 1] += 1;
                }
            } else if spelled_out_digit.chars().next().unwrap() == character {
                current_letter_indices[digit - 1] = 1;
            } else {
                current_letter_indices[digit - 1] = 0;
            }
        }
    }

    let mut last_digit = 0;
    let mut current_letter_indices = [0; 9];

    'line_characters: for character in line.chars().rev() {
        if character.is_ascii_digit() {
            last_digit = character.to_digit(10).unwrap();
            break;
        }

        for digit in 1..=9 {
            let current_letter_index = current_letter_indices[digit - 1];
            let spelled_out_digit = spelled_out_digits[digit - 1];

            if spelled_out_digit
                .chars()
                .rev()
                .nth(current_letter_index)
                .unwrap()
                == character
            {
                if current_letter_index == spelled_out_digit.len() - 1 {
                    last_digit = digit as u32;
                    break 'line_characters;
                } else {
                    current_letter_indices[digit - 1] += 1;
                }
            } else if spelled_out_digit.chars().next_back().unwrap() == character {
                current_letter_indices[digit - 1] = 1;
            } else {
                current_letter_indices[digit - 1] = 0;
            }
        }
    }

    10 * first_digit + last_digit
}

#[aoc(day1, part2)]
fn part2(calibration_document: &[String]) -> u32 {
    calibration_document
        .iter()
        .map(|l| calibration_value_spelled_out_with_letters(l))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    static TEST_INPUT_2: &str = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT_1)), 142);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT_2)), 281);
    }
}
