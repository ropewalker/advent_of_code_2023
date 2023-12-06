use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
fn parse_input(times_and_distances: &str) -> Vec<(u64, u64)> {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(
        "Time:" " "+ times:repeat_sep(u64, " "+) "\n"
        "Distance:" " "+ distances:repeat_sep(u64, " "+) =>
            (times, distances)
    );

    let (times, distances) = parser.parse(times_and_distances).unwrap();

    times.into_iter().zip(distances).collect()
}

fn ways_to_beat_the_record(time: u64, record_distance: u64) -> usize {
    let discriminant = time * time - 4 * record_distance;

    let min_root = ((time as f64 - (discriminant as f64).sqrt()) / 2.0).floor() as usize + 1;
    let max_root = ((time as f64 + (discriminant as f64).sqrt()) / 2.0).ceil() as usize - 1;

    max_root - min_root + 1
}

#[aoc(day6, part1)]
fn part1(times_and_distances: &[(u64, u64)]) -> usize {
    times_and_distances
        .iter()
        .map(|(time, distance)| ways_to_beat_the_record(*time, *distance))
        .product()
}

#[aoc(day6, part2)]
fn part2(times_and_distances: &[(u64, u64)]) -> usize {
    let (time, distance) = times_and_distances.iter().fold((0, 0), |acc, e| {
        (
            acc.0 * 10u64.pow(e.0.checked_ilog10().unwrap() + 1) + e.0,
            acc.1 * 10u64.pow(e.1.checked_ilog10().unwrap() + 1) + e.1,
        )
    });

    ways_to_beat_the_record(time, distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 288);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 71_503);
    }
}
