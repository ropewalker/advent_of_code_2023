use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input(_input: &str) -> Vec<u32> {
    unimplemented!()
}

#[aoc(day1, part1)]
fn part1(_parsed_input: &[u32]) -> u32 {
    unimplemented!()
}

#[aoc(day1, part2)]
fn part2(_parsed_input: &[u32]) -> u32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 0);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 0);
    }
}
