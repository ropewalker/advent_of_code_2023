use crate::day08::Instruction::{Left, Right};
use aoc_runner_derive::{aoc, aoc_generator};
use num::integer::lcm;
use std::collections::HashMap;

enum Instruction {
    Left,
    Right,
}

struct Network(HashMap<String, (String, String)>);

struct Document {
    instructions: Vec<Instruction>,
    network: Network,
}

#[aoc_generator(day8)]
fn parse_input(document: &str) -> Document {
    use aoc_parse::{parser, prelude::*};
    use Instruction::*;

    let parser = parser!(
        instructions:section(line({"L" => Left, "R" => Right}+))
        network:section(
            nodes:lines(
                key:string(alnum+) " = (" left:string(alnum+) ", " right:string(alnum+) ")" =>
                    (key, (left, right))
            ) =>
                Network(nodes.into_iter().collect())
        ) =>
            Document {
                instructions,
                network
            }
    );

    parser.parse(document).unwrap()
}

/*
This method only works for specific inputs, namely, where the path starting at each node that ends
with A goes through a single unique node that ends with Z, and the number of steps needed to reach
that node for the first time is the same as the number of steps required to reach it every
consecutive time. Both the example and the actual input satisfy this condition. It is easy to
demonstrate that it is not true in the general case.
*/
#[aoc(day8, part1)]
fn part1(document: &Document) -> usize {
    let mut steps = 0;
    let mut current_node = "AAA";

    for instruction in document.instructions.iter().cycle() {
        steps += 1;

        let next_nodes = document.network.0.get(current_node).unwrap();

        current_node = match instruction {
            Left => &next_nodes.0,
            Right => &next_nodes.1,
        };

        if current_node == "ZZZ" {
            return steps;
        }
    }

    unreachable!()
}

#[aoc(day8, part2)]
fn part2(document: &Document) -> usize {
    let starting_nodes: Vec<_> = document
        .network
        .0
        .keys()
        .filter(|node| node.ends_with('A'))
        .collect();

    let mut steps = 1;

    for starting_node in starting_nodes.into_iter() {
        let mut node_steps = 0;
        let mut current_node = starting_node;

        for instruction in document.instructions.iter().cycle() {
            node_steps += 1;

            let next_nodes = document.network.0.get(current_node).unwrap();

            current_node = match instruction {
                Left => &next_nodes.0,
                Right => &next_nodes.1,
            };

            if current_node.ends_with('Z') {
                steps = lcm(node_steps, steps);
                break;
            }
        }
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    static PART_1_TEST_INPUT_1: &str = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    static PART_1_TEST_INPUT_2: &str = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    static PART_2_TEST_INPUT: &str = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn part1_example1() {
        assert_eq!(part1(&parse_input(PART_1_TEST_INPUT_1)), 2);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(&parse_input(PART_1_TEST_INPUT_2)), 6);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(PART_2_TEST_INPUT)), 6);
    }
}
