use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn parse_input(report: &str) -> Vec<Vec<i32>> {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(lines(repeat_sep(i32, " ")));
    parser.parse(report).unwrap()
}

fn derivatives(history: &[i32]) -> Vec<Vec<i32>> {
    let mut derivatives = vec![history.to_vec()];

    loop {
        let differences: Vec<i32> = derivatives
            .last()
            .unwrap()
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .collect();

        let all_zeroes = differences.iter().all(|step| *step == 0);
        let domain_len = differences.len();

        derivatives.push(differences);

        if all_zeroes || domain_len < 2 {
            break;
        }
    }

    derivatives
}

#[aoc(day9, part1)]
fn part1(report: &[Vec<i32>]) -> i32 {
    report
        .iter()
        .map(|history| {
            derivatives(history)
                .iter()
                .map(|steps| *steps.last().unwrap())
                .rev()
                .reduce(|extrapolation, last_difference| extrapolation + last_difference)
                .unwrap()
        })
        .sum()
}

#[aoc(day9, part2)]
fn part2(report: &[Vec<i32>]) -> i32 {
    report
        .iter()
        .map(|history| {
            derivatives(history)
                .iter()
                .map(|steps| *steps.first().unwrap())
                .rev()
                .reduce(|extrapolation, first_difference| first_difference - extrapolation)
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 114);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 2);
    }
}
