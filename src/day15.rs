use crate::day15::Operation::*;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day15)]
fn parse_input(initialization_sequence: &str) -> Vec<String> {
    initialization_sequence
        .split(',')
        .map(|s| s.to_owned())
        .collect()
}

fn run_hash_algorithm(input: &str) -> usize {
    input
        .as_bytes()
        .iter()
        .fold(0, |current_value, ascii_code| {
            (current_value + *ascii_code as usize) * 17 % 256
        })
}

#[aoc(day15, part1)]
fn part1(initialization_sequence: &[String]) -> usize {
    initialization_sequence
        .iter()
        .map(|step| run_hash_algorithm(step))
        .sum()
}

#[derive(Eq, PartialEq, Debug)]
enum Operation {
    Dash,
    EqualSign(usize),
}

fn parse_initialization_step(step: &str) -> (String, Operation) {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(
        label:string(alpha+) 
        operation:{
            "-" => Dash, 
            "=" focal_length:digit => EqualSign(focal_length)
        } => 
            (label, operation));

    parser.parse(step).unwrap()
}

fn run_hashmap_algorithm(steps: &[(String, Operation)]) -> [Vec<(String, usize)>; 256] {
    let mut boxes: [Vec<(String, usize)>; 256] =
        std::iter::repeat_with(|| Vec::with_capacity(steps.len()))
            .take(256)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

    for (label, operation) in steps {
        let box_number = run_hash_algorithm(label);

        match operation {
            Dash => {
                if let Some(position) = boxes[box_number].iter().position(|(l, _)| *l == *label) {
                    boxes[box_number].remove(position);
                }
            }
            EqualSign(focal_length) => {
                if let Some(position) = boxes[box_number].iter().position(|(l, _)| *l == *label) {
                    let _ = std::mem::replace(
                        &mut boxes[box_number][position],
                        (label.clone(), *focal_length),
                    );
                } else {
                    boxes[box_number].push((label.clone(), *focal_length));
                }
            }
        }
    }

    boxes
}

fn focusing_power(boxes: &[Vec<(String, usize)>; 256]) -> usize {
    boxes
        .iter()
        .enumerate()
        .flat_map(|(box_number, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(move |(slot_number, (_, focal_length))| {
                    (box_number + 1) * (slot_number + 1) * focal_length
                })
        })
        .sum()
}

#[aoc(day15, part2)]
fn part2(initialization_sequence: &[String]) -> usize {
    focusing_power(&run_hashmap_algorithm(
        &initialization_sequence
            .iter()
            .map(|step| parse_initialization_step(step))
            .collect::<Vec<_>>(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str = "HASH";

    static TEST_INPUT_2: &str = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn hash_algorithm_example() {
        assert_eq!(run_hash_algorithm(TEST_INPUT_1), 52);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT_2)), 1_320);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT_2)), 145);
    }
}
