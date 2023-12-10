use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

type Position = (i32, i32);
type Direction = (i32, i32);
type Pipe = [Direction; 2];

#[derive(Clone)]
struct Sketch {
    starting_position: Position,
    pipes: HashMap<Position, Pipe>,
    bottom_right_position: Position,
}

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Sketch {
    let mut sketch = Sketch {
        starting_position: (0, 0),
        pipes: HashMap::new(),
        bottom_right_position: (
            input.lines().next().unwrap().chars().count() as i32 - 1,
            input.lines().count() as i32 - 1,
        ),
    };

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, tile)| match tile {
            'S' => sketch.starting_position = (x as i32, y as i32),
            '.' => (),
            _ => {
                sketch.pipes.insert(
                    (x as i32, y as i32),
                    match tile {
                        '|' => [(0, -1), (0, 1)],
                        '-' => [(-1, 0), (1, 0)],
                        'L' => [(0, -1), (1, 0)],
                        'J' => [(0, -1), (-1, 0)],
                        '7' => [(-1, 0), (0, 1)],
                        'F' => [(1, 0), (0, 1)],
                        _ => unreachable!(),
                    },
                );
            }
        })
    });

    sketch
}

fn starting_pipe(sketch: &Sketch) -> Pipe {
    let starting_position = sketch.starting_position;
    let mut starting_directions = Vec::with_capacity(2);

    for (x, y) in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ] {
        let pipe_position = (starting_position.0 + x, starting_position.1 + y);

        if let Some(pipe) = sketch.pipes.get(&pipe_position) {
            for direction in pipe.iter() {
                if (pipe_position.0 + direction.0, pipe_position.1 + direction.1)
                    == starting_position
                {
                    starting_directions.push((-direction.0, -direction.1));
                    break;
                }
            }

            if starting_directions.len() == 2 {
                break;
            }
        }
    }

    starting_directions.try_into().unwrap()
}

fn main_loop(sketch: &Sketch) -> HashSet<Position> {
    let mut current_position = sketch.starting_position;
    let mut main_loop = HashSet::from([current_position]);

    'main_loop: loop {
        if let Some(pipe) = sketch
            .pipes
            .get(&current_position)
            .or(Some(&starting_pipe(sketch)))
        {
            for direction in pipe {
                let adjacent_pipe_position = (
                    current_position.0 + direction.0,
                    current_position.1 + direction.1,
                );

                if !main_loop.contains(&adjacent_pipe_position) {
                    current_position = adjacent_pipe_position;
                    main_loop.insert(current_position);
                    continue 'main_loop;
                }
            }

            return main_loop;
        }
    }
}

#[aoc(day10, part1)]
fn part1(sketch: &Sketch) -> usize {
    main_loop(sketch).len() / 2
}

#[aoc(day10, part2)]
fn part2(sketch: &Sketch) -> usize {
    let main_loop = main_loop(sketch);
    let starting_pipe = starting_pipe(sketch);
    let mut inside_count = 0;

    for y in 0..=sketch.bottom_right_position.1 {
        let mut outside = true;

        for x in 0..=sketch.bottom_right_position.0 {
            if let Some(position) = main_loop.get(&(x, y)) {
                let directions = sketch.pipes.get(position).unwrap_or(&starting_pipe);

                if directions.contains(&(0, 1)) {
                    outside = !outside;
                }
            } else if !outside {
                inside_count += 1;
            }
        }
    }

    inside_count
}

#[cfg(test)]
mod tests {
    use super::*;

    static PART_1_TEST_INPUT_1: &str = r".....
.S-7.
.|.|.
.L-J.
.....";

    static PART_1_TEST_INPUT_2: &str = r"..F7.
.FJ|.
SJ.L7
|F--J
LJ..";

    static PART_2_TEST_INPUT_1: &str = r"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    static PART_2_TEST_INPUT_2: &str = r"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

    static PART_2_TEST_INPUT_3: &str = r".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    static PART_2_TEST_INPUT_4: &str = r"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn part1_example1() {
        assert_eq!(part1(&parse_input(PART_1_TEST_INPUT_1)), 4);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(&parse_input(PART_1_TEST_INPUT_2)), 8);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(part2(&parse_input(PART_2_TEST_INPUT_1)), 4);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(part2(&parse_input(PART_2_TEST_INPUT_2)), 4);
    }

    #[test]
    fn part2_example3() {
        assert_eq!(part2(&parse_input(PART_2_TEST_INPUT_3)), 8);
    }

    #[test]
    fn part2_example4() {
        assert_eq!(part2(&parse_input(PART_2_TEST_INPUT_4)), 10);
    }
}
