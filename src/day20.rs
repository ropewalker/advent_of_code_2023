use crate::day20::ModuleType::*;
use crate::day20::PulseType::*;
use aoc_runner_derive::{aoc, aoc_generator};
use num::integer::lcm;
use std::collections::{HashMap, VecDeque};

#[derive(Eq, PartialEq, Clone, Copy)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcast,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum PulseType {
    Low,
    High,
}

#[aoc_generator(day20)]
fn parse_input(module_configuration: &str) -> Vec<(ModuleType, String, Vec<String>)> {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(
        modules:lines(
            type_and_name:{
                "%" name:string(lower+) => (FlipFlop, name),
                "&" name:string(lower+) => (Conjunction, name),
                BROADCAST_MODULE_NAME => (Broadcast, BROADCAST_MODULE_NAME.to_string()),
            } " -> " destinations:repeat_sep(string(lower+), ", ") =>
                (
                    type_and_name.0,
                    type_and_name.1,
                    destinations,
                )
        )
    );

    parser.parse(module_configuration).unwrap()
}

const BROADCAST_MODULE_NAME: &str = "broadcaster";
const BUTTON_MODULE_NAME: &str = "button";
const TERMINAL_MODULE_NAME: &str = "rx";

#[derive(Clone, Debug)]
struct State {
    flip_flop_states: HashMap<String, bool>,
    conjunction_states: HashMap<String, HashMap<String, PulseType>>,
}

impl From<&[(ModuleType, String, Vec<String>)]> for State {
    fn from(module_configuration: &[(ModuleType, String, Vec<String>)]) -> Self {
        let mut flip_flop_states: HashMap<String, bool> = HashMap::new();
        let mut conjunction_states: HashMap<String, HashMap<String, PulseType>> = HashMap::new();

        for (module_type, name, _) in module_configuration {
            match module_type {
                FlipFlop => {
                    flip_flop_states.insert(name.to_owned(), false);
                }
                Conjunction => {
                    conjunction_states.insert(name.to_owned(), HashMap::new());
                }
                Broadcast => (),
            };
        }

        for (_, name, destinations) in module_configuration {
            for destination in destinations.iter() {
                if conjunction_states.contains_key(destination) {
                    let inputs = conjunction_states.get_mut(destination).unwrap();
                    inputs.insert(name.to_owned(), Low);
                }
            }
        }

        State {
            flip_flop_states,
            conjunction_states,
        }
    }
}

fn transform_pulse(
    state: &mut State,
    source: &str,
    destination: &str,
    pulse_type: &PulseType,
) -> Option<PulseType> {
    if let Some(module_state) = state.flip_flop_states.get_mut(destination) {
        if *pulse_type == Low {
            *module_state = !*module_state;

            match *module_state {
                true => Some(High),
                false => Some(Low),
            }
        } else {
            None
        }
    } else if let Some(remembered_pulse_types) = state.conjunction_states.get_mut(destination) {
        let remembered_pulse_type = remembered_pulse_types.get_mut(source).unwrap();
        *remembered_pulse_type = *pulse_type;

        if remembered_pulse_types
            .values()
            .all(|pulse_type| *pulse_type == High)
        {
            Some(Low)
        } else {
            Some(High)
        }
    } else {
        Some(*pulse_type)
    }
}

#[aoc(day20, part1)]
fn part1(module_configuration: &[(ModuleType, String, Vec<String>)]) -> usize {
    let mut state = State::from(module_configuration);
    let mapping = module_configuration
        .iter()
        .map(|(_, name, destinations)| (name.as_str(), destinations))
        .collect::<HashMap<_, _>>();

    let mut pulses_sent = [0usize; 2];

    for _ in 0..1_000 {
        let mut nodes: VecDeque<(&str, &str, PulseType)> =
            VecDeque::from([(BUTTON_MODULE_NAME, BROADCAST_MODULE_NAME, Low)]);
        pulses_sent[Low as usize] += 1;

        while let Some((source, destination, pulse_type)) = nodes.pop_front() {
            if let Some(new_pulse_type) =
                transform_pulse(&mut state, source, destination, &pulse_type)
            {
                if mapping.contains_key(&destination) {
                    for new_destination in mapping.get(&destination).unwrap().iter() {
                        nodes.push_back((destination, new_destination.as_str(), new_pulse_type));
                        pulses_sent[new_pulse_type as usize] += 1;
                    }
                }
            } else {
                continue;
            }
        }
    }

    pulses_sent[Low as usize] * pulses_sent[High as usize]
}

#[aoc(day20, part2)]
fn part2(module_configuration: &[(ModuleType, String, Vec<String>)]) -> usize {
    let mut state = State::from(module_configuration);
    let mapping = module_configuration
        .iter()
        .map(|(_, name, destinations)| (name.as_str(), destinations))
        .collect::<HashMap<_, _>>();

    let last_conjunction = *mapping
        .iter()
        .find(|(_, destinations)| {
            destinations
                .iter()
                .any(|destination| destination == TERMINAL_MODULE_NAME)
        })
        .map(|(module_name, _)| module_name)
        .unwrap();

    let mut cycle_lengths = state
        .conjunction_states
        .get(last_conjunction)
        .unwrap()
        .iter()
        .map(|(module_name, _)| (module_name.to_owned(), None))
        .collect::<HashMap<String, Option<usize>>>();

    let mut pressed_count = 0;

    loop {
        pressed_count += 1;

        let mut nodes: VecDeque<(&str, &str, PulseType)> =
            VecDeque::from([(BUTTON_MODULE_NAME, BROADCAST_MODULE_NAME, Low)]);

        while let Some((source, destination, pulse_type)) = nodes.pop_front() {
            if let Some(new_pulse_type) =
                transform_pulse(&mut state, source, destination, &pulse_type)
            {
                if mapping.contains_key(&destination) {
                    for new_destination in mapping.get(&destination).unwrap().iter() {
                        if new_destination == last_conjunction
                            && new_pulse_type == High
                            && cycle_lengths.get(destination).unwrap().is_none()
                        {
                            cycle_lengths.insert(destination.to_string(), Some(pressed_count));

                            if cycle_lengths.values().all(|cycle_len| cycle_len.is_some()) {
                                return cycle_lengths
                                    .values()
                                    .fold(1, |acc, e| lcm(acc, e.unwrap()));
                            }
                        }

                        nodes.push_back((destination, new_destination.as_str(), new_pulse_type));
                    }
                }
            } else {
                continue;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str = r"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    static TEST_INPUT_2: &str = r"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn part1_example1() {
        assert_eq!(part1(&parse_input(TEST_INPUT_1)), 32_000_000);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(&parse_input(TEST_INPUT_2)), 11_687_500);
    }
}
