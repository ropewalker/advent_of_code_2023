use crate::day19::Destination::*;
use crate::day19::Rule::*;
use crate::day19::Verdict::*;
use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;
use std::cmp::Ordering::{Greater, Less};
use std::collections::HashMap;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Verdict {
    Accepted,
    Rejected,
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum Destination {
    Decision(Verdict),
    NextWorkflow(WorkflowName),
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
enum Category {
    ExtremelyCoolLooking,
    Musical,
    Aerodynamic,
    Shiny,
}

type WorkflowName = String;
type Workflows = HashMap<WorkflowName, Vec<Rule>>;

#[derive(Eq, PartialEq, Clone, Debug)]
enum Rule {
    Conditional(Category, Ordering, u64, Destination),
    NonConditional(Destination),
}

type Part = [u64; 4];

#[aoc_generator(day19)]
fn parse_input(system: &str) -> (Workflows, Vec<Part>) {
    use aoc_parse::{parser, prelude::*};

    use Category::*;
    use Destination::*;
    use Ordering::*;
    use Rule::*;
    use Verdict::*;

    let parser = parser!(
        rule category: Category = {
            "x" => ExtremelyCoolLooking,
            "m" => Musical,
            "a" => Aerodynamic,
            "s" => Shiny,
        };

        rule comparison: Ordering = {
            "<" => Less,
            ">" => Greater,
        };

        rule destination: Destination = {
            "A" => Decision(Accepted),
            "R" => Decision(Rejected),
            w:string(lower+) => NextWorkflow(w),
        };

        rule workflow_rule: Rule = {
            c:category o:comparison n:u64 ":" d:destination => Conditional(c, o, n, d),
            d:destination => NonConditional(d),
        };

        rule workflow: (WorkflowName, Vec<Rule>) =
            name:string(lower+) "{" rules:repeat_sep(workflow_rule, ",") "}" =>
                (name, rules);

        w:section(lines(workflow))
        p:section(lines("{" ratings:repeat_sep(cat:category "=" value:u64, ",") "}")) =>
            (
                w.into_iter().collect::<HashMap<_, _>>(),
                p
                    .into_iter()
                    .map(|part| part
                        .into_iter()
                        .map(|(_, value)| value)
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap()
                    )
                    .collect::<Vec<_>>())
    );

    parser.parse(system).unwrap()
}

const STARTING_WORKFLOW: &str = "in";

#[aoc(day19, part1)]
fn part1((workflows, parts): &(Workflows, Vec<Part>)) -> u64 {
    use Destination::*;
    use Rule::*;
    use Verdict::*;

    let mut total_rating = 0;

    'parts: for part in parts {
        let mut workflow_name = STARTING_WORKFLOW;

        'workflows: loop {
            let rules = workflows.get(workflow_name).unwrap();

            for rule in rules {
                let destination = match rule {
                    Conditional(category, ordering, value, destination) => {
                        if part[*category as usize].cmp(value) == *ordering {
                            Some(destination)
                        } else {
                            None
                        }
                    }
                    NonConditional(destination) => Some(destination),
                };

                if let Some(destination) = destination {
                    match destination {
                        Decision(Accepted) => {
                            total_rating += part.iter().sum::<u64>();
                            continue 'parts;
                        }
                        Decision(Rejected) => {
                            continue 'parts;
                        }
                        NextWorkflow(next_workflow_name) => {
                            workflow_name = next_workflow_name;
                            continue 'workflows;
                        }
                    }
                }
            }
        }
    }

    total_rating
}

type RatingRanges = [(u64, u64); 4];

fn combinations_count(rating_ranges: RatingRanges) -> u64 {
    rating_ranges
        .iter()
        .map(|(min, max)| (*min..=*max).count() as u64)
        .product::<u64>()
}

const MIN_RATING: u64 = 1;
const MAX_RATING: u64 = 4_000;

#[aoc(day19, part2)]
fn part2((workflows, _): &(Workflows, Vec<Part>)) -> u64 {
    let mut combinations = 0;

    let rating_ranges = [(MIN_RATING, MAX_RATING); 4];
    let mut nodes = vec![(rating_ranges, NextWorkflow(STARTING_WORKFLOW.to_string()))];

    while let Some((mut rating_ranges, destination)) = nodes.pop() {
        let workflow_name = match destination {
            Decision(Accepted) => {
                combinations += combinations_count(rating_ranges);
                continue;
            }
            Decision(Rejected) => continue,
            NextWorkflow(workflow_name) => workflow_name,
        };

        for rule in workflows.get(&workflow_name).unwrap() {
            match rule {
                Conditional(category, ordering, value, destination) => {
                    let mut matching_sub_ranges = rating_ranges;
                    let (min, max) = matching_sub_ranges[*category as usize];

                    matching_sub_ranges[*category as usize] = match ordering {
                        Less => (min, u64::min(max, value - 1)),
                        Greater => (u64::max(min, value + 1), max),
                        _ => unreachable!(),
                    };

                    nodes.push((matching_sub_ranges, destination.clone()));

                    let (min, max) = rating_ranges[*category as usize];

                    rating_ranges[*category as usize] = match ordering {
                        Greater => (min, u64::min(max, *value)),
                        Less => (u64::max(min, *value), max),
                        _ => unreachable!(),
                    };

                    if combinations_count(rating_ranges) == 0 {
                        break;
                    }
                }
                NonConditional(destination) => {
                    nodes.push((rating_ranges, destination.clone()));
                }
            }
        }
    }

    combinations
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 19_114);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 167_409_079_868_000);
    }
}
