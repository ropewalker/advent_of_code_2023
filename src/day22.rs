use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

type Coordinates2 = (i64, i64);
type Coordinates3 = (i64, i64, i64);

type Rectangle = (Coordinates2, Coordinates2);
type Brick = (Coordinates3, Coordinates3);

#[aoc_generator(day22)]
fn parse_input(snapshot: &str) -> Vec<Brick> {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(lines(
        x0:i64 "," y0:i64 "," z0:i64 "~" x1:i64 "," y1:i64 "," z1:i64 =>
            ((x0, y0, z0), (x1, y1, z1))
    ));

    parser.parse(snapshot).unwrap()
}

fn overlap(rectangle1: Rectangle, rectangle2: Rectangle) -> bool {
    rectangle1.0 .0 <= rectangle2.1 .0
        && rectangle1.1 .0 >= rectangle2.0 .0
        && rectangle1.0 .1 <= rectangle2.1 .1
        && rectangle1.1 .1 >= rectangle2.0 .1
}

fn base(brick: &Brick) -> Rectangle {
    ((brick.0 .0, brick.0 .1), (brick.1 .0, brick.1 .1))
}

fn vertical_shift(brick: &Brick, z: i64) -> Brick {
    (
        (brick.0 .0, brick.0 .1, z),
        (brick.1 .0, brick.1 .1, brick.1 .2 - brick.0 .2 + z),
    )
}

fn land(snapshot: &[Brick]) -> Vec<Brick> {
    let mut landed_bricks = snapshot.to_vec();
    landed_bricks.sort_unstable_by(|(bottom1, _), (bottom2, _)| bottom1.2.cmp(&bottom2.2));

    for index in 0..landed_bricks.len() {
        let brick = landed_bricks[index];

        let max_z = landed_bricks[0..index]
            .iter()
            .filter_map(|overlapping_brick| {
                if overlap(base(&brick), base(overlapping_brick)) {
                    Some(overlapping_brick.1 .2)
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(0);

        landed_bricks[index] = vertical_shift(&brick, max_z + 1);
    }

    landed_bricks
}

fn supporting_bricks(bricks: &[Brick]) -> HashMap<Brick, HashSet<Brick>> {
    bricks
        .iter()
        .map(|brick| {
            (
                *brick,
                bricks
                    .iter()
                    .filter_map(|supporting_brick| {
                        if overlap(base(brick), base(supporting_brick))
                            && brick.0 .2 == supporting_brick.1 .2 + 1
                        {
                            Some(*supporting_brick)
                        } else {
                            None
                        }
                    })
                    .collect::<HashSet<_>>(),
            )
        })
        .collect::<HashMap<_, _>>()
}

#[aoc(day22, part1)]
fn part1(snapshot: &[Brick]) -> usize {
    let landed_bricks = land(snapshot);

    let result: HashSet<Brick> = supporting_bricks(&landed_bricks)
        .values()
        .filter_map(|bricks| {
            if bricks.len() == 1 {
                Some(*bricks.iter().next().unwrap())
            } else {
                None
            }
        })
        .collect::<HashSet<_>>();

    landed_bricks.len() - result.len()
}

#[aoc(day22, part2)]
fn part2(snapshot: &[(Coordinates3, Coordinates3)]) -> usize {
    let landed_bricks = land(snapshot);
    let supporting_bricks = supporting_bricks(&landed_bricks);

    let mut result = 0;

    let mut cache: HashMap<Brick, HashSet<Brick>> = HashMap::new();

    for brick in landed_bricks.into_iter().rev() {
        let mut removed_bricks = HashSet::from([brick]);

        loop {
            let mut new_removed_bricks = HashSet::new();

            for (supported_brick, supporting_bricks) in supporting_bricks.iter() {
                if !removed_bricks.contains(supported_brick)
                    && !supporting_bricks.is_empty()
                    && supporting_bricks.difference(&removed_bricks).count() == 0
                {
                    new_removed_bricks.insert(*supported_brick);
                    new_removed_bricks.extend(
                        cache
                            .get(supported_brick)
                            .unwrap_or(&HashSet::with_capacity(0)),
                    );
                }
            }

            if !new_removed_bricks.is_empty() {
                result += new_removed_bricks.len();
                removed_bricks.extend(new_removed_bricks);
            } else {
                cache.insert(brick, removed_bricks);
                break;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str = r"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn part1_example1() {
        assert_eq!(part1(&parse_input(TEST_INPUT_1)), 5);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(part2(&parse_input(TEST_INPUT_1)), 7);
    }
}
