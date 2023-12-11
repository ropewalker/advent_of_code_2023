use aoc_runner_derive::{aoc, aoc_generator};

type Coordinates = (i64, i64);

#[aoc_generator(day11)]
fn parse_input(image: &str) -> Vec<Coordinates> {
    image
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars().enumerate().filter_map(move |(x, char)| {
                if char == '#' {
                    Some((x as i64, y as i64))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn sum_of_distances(image: &[Coordinates], expansion_factor: i64) -> i64 {
    let mut expanded_image = image.to_vec();

    let max_x = *image
        .iter()
        .max_by(|a, b| a.0.cmp(&b.0))
        .map(|(x, _)| x)
        .unwrap();
    let max_y = *image
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(_, y)| y)
        .unwrap();

    (0..=max_x)
        .rev()
        .filter(|x| !image.iter().any(|coordinates| coordinates.0 == *x))
        .for_each(|x| {
            expanded_image
                .iter_mut()
                .filter(|coordinates| coordinates.0 >= x)
                .for_each(|coordinates| coordinates.0 += expansion_factor - 1)
        });

    (0..=max_y)
        .rev()
        .filter(|y| !image.iter().any(|coordinates| coordinates.1 == *y))
        .for_each(|y| {
            expanded_image
                .iter_mut()
                .filter(|coordinates| coordinates.1 >= y)
                .for_each(|coordinates| coordinates.1 += expansion_factor - 1)
        });

    let mut distance_sum = 0;

    for i in 0..expanded_image.len() - 1 {
        for j in i + 1..expanded_image.len() {
            let coordinates_i = expanded_image[i];
            let coordinates_j = expanded_image[j];

            distance_sum += (coordinates_j.0 - coordinates_i.0).abs()
                + (coordinates_j.1 - coordinates_i.1).abs();
        }
    }

    distance_sum
}

#[aoc(day11, part1)]
fn part1(image: &[Coordinates]) -> i64 {
    sum_of_distances(image, 2)
}

#[aoc(day11, part2)]
fn part2(image: &[Coordinates]) -> i64 {
    sum_of_distances(image, 1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 374);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(sum_of_distances(&parse_input(TEST_INPUT), 10), 1_030);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(sum_of_distances(&parse_input(TEST_INPUT), 100), 8_410);
    }
}
