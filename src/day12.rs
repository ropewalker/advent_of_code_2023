use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::slice;
use Condition::*;

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[aoc_generator(day12)]
fn parse_input(condition_records: &str) -> Vec<(Vec<Condition>, Vec<usize>)> {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(
        rule condition: Condition = {
            "." => Operational,
            "#" => Damaged,
            "?" => Unknown,
        };
        lines(
            conditions:condition+ " " damaged_groups:repeat_sep(usize, ",") =>
                (conditions, damaged_groups)
        )
    );

    parser.parse(condition_records).unwrap()
}

fn arrangements_count_cache(
    conditions: &[Condition],
    damaged_groups: &[usize],
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(count) = cache.get(&(conditions.len(), damaged_groups.len())) {
        return *count;
    }

    let mut count = 0;

    if damaged_groups.is_empty() {
        count = if conditions.contains(&Damaged) { 0 } else { 1 };

        cache.insert((conditions.len(), damaged_groups.len()), count);

        return count;
    }

    for offset in 0..conditions.len() {
        if conditions[0..offset].contains(&Damaged) || offset + damaged_groups[0] > conditions.len()
        {
            break;
        }

        if conditions[offset..offset + damaged_groups[0]].contains(&Operational) {
            continue;
        }

        if damaged_groups.len() == 1 {
            if offset + damaged_groups[0] == conditions.len() {
                count += 1;
                break;
            } else {
                count +=
                    arrangements_count_cache(&conditions[offset + damaged_groups[0]..], &[], cache);
                continue;
            };
        } else if offset + damaged_groups[0] + 1 > conditions.len() {
            break;
        } else if conditions[offset + damaged_groups[0]] == Damaged {
            continue;
        }

        count += arrangements_count_cache(
            &conditions[offset + damaged_groups[0] + 1..],
            &damaged_groups[1..],
            cache,
        );
    }

    cache.insert((conditions.len(), damaged_groups.len()), count);

    count
}

fn arrangements_count(conditions: &[Condition], damaged_groups: &[usize]) -> usize {
    arrangements_count_cache(conditions, damaged_groups, &mut HashMap::new())
}

#[aoc(day12, part1)]
fn part1(condition_records: &[(Vec<Condition>, Vec<usize>)]) -> usize {
    condition_records
        .iter()
        .map(|(conditions, damaged_groups)| arrangements_count(conditions, damaged_groups))
        .sum()
}

#[aoc(day12, part2)]
fn part2(condition_records: &[(Vec<Condition>, Vec<usize>)]) -> usize {
    condition_records
        .iter()
        .map(|(conditions, damaged_groups)| {
            arrangements_count(
                &[
                    &conditions[..],
                    slice::from_ref(&Unknown),
                    &conditions[..],
                    slice::from_ref(&Unknown),
                    &conditions[..],
                    slice::from_ref(&Unknown),
                    &conditions[..],
                    slice::from_ref(&Unknown),
                    &conditions[..],
                ]
                .concat(),
                &[
                    &damaged_groups[..],
                    &damaged_groups[..],
                    &damaged_groups[..],
                    &damaged_groups[..],
                    &damaged_groups[..],
                ]
                .concat(),
            )
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 21);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 525_152);
    }
}
