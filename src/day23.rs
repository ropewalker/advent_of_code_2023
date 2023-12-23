use crate::day23::Tile::*;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};
use std::ops::Add;
use Direction::*;

type Coordinates = (i32, i32);

#[derive(Eq, PartialEq, Copy, Clone, Hash, Ord, PartialOrd)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl From<Direction> for Coordinates {
    fn from(value: Direction) -> Self {
        match value {
            Left => (-1, 0),
            Right => (1, 0),
            Up => (0, -1),
            Down => (0, 1),
        }
    }
}

impl Add<Direction> for Coordinates {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self {
        let rhs: (i32, i32) = rhs.into();

        (self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

#[aoc_generator(day23)]
fn parse_input(hiking_map: &str) -> Vec<Vec<Tile>> {
    hiking_map
        .lines()
        .map(|line| {
            line.chars()
                .map(|tile| match tile {
                    '.' => Path,
                    '#' => Forest,
                    '^' => Slope(Up),
                    '>' => Slope(Right),
                    'v' => Slope(Down),
                    '<' => Slope(Left),
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn adjacent_positions(
    position: Coordinates,
    hiking_map: &[Vec<Tile>],
    ignore_slopes: bool,
) -> Vec<Coordinates> {
    let bottom_right = bottom_right(hiking_map);

    [Right, Down, Left, Up]
        .into_iter()
        .filter_map(|direction| {
            let adjacent_position = position + direction;
            if adjacent_position.0 >= 0
                && adjacent_position.0 <= bottom_right.0
                && adjacent_position.1 >= 0
                && adjacent_position.1 <= bottom_right.1
                && match hiking_map[adjacent_position.1 as usize][adjacent_position.0 as usize] {
                    Path => true,
                    Slope(slope_direction) if slope_direction == direction || ignore_slopes => true,
                    _ => false,
                }
            {
                Some(adjacent_position)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

fn get_connected_vertices(
    vertex: Coordinates,
    hiking_map: &[Vec<Tile>],
    ignore_slopes: bool,
) -> HashMap<Coordinates, usize> {
    let mut connected_vertices = HashMap::new();

    if vertex == finishing_point(hiking_map) {
        return connected_vertices;
    }

    for starting_point in adjacent_positions(vertex, hiking_map, ignore_slopes) {
        let mut visited = HashSet::from([vertex]);
        let mut nodes = vec![(starting_point, 1)];

        while let Some((position, distance)) = nodes.pop() {
            visited.insert(position);

            let adjacent_positions = adjacent_positions(position, hiking_map, ignore_slopes)
                .into_iter()
                .filter(|new_position| !visited.contains(new_position))
                .collect::<Vec<_>>();

            if adjacent_positions.len() == 1 {
                nodes.push((*adjacent_positions.first().unwrap(), distance + 1));
            } else {
                let prev_distance = connected_vertices.entry(position).or_default();
                *prev_distance = usize::max(*prev_distance, distance);
            }
        }
    }

    connected_vertices
}

fn topological_sort(
    hiking_map: &[Vec<Tile>],
    current_vertex: Coordinates,
    sorted: &mut Vec<Coordinates>,
    edges: &mut HashMap<Coordinates, HashMap<Coordinates, usize>>,
    visited: &mut HashSet<Coordinates>,
) {
    let connected_vertices = edges
        .entry(current_vertex)
        .or_insert(get_connected_vertices(current_vertex, hiking_map, false))
        .clone();

    for (connected_vertex, _) in connected_vertices {
        if !visited.contains(&connected_vertex) {
            topological_sort(hiking_map, connected_vertex, sorted, edges, visited);
        }
    }

    visited.insert(current_vertex);
    sorted.push(current_vertex);
}

fn bottom_right(hiking_map: &[Vec<Tile>]) -> Coordinates {
    (
        hiking_map.first().unwrap().len() as i32 - 1,
        hiking_map.len() as i32 - 1,
    )
}

fn starting_point(hiking_map: &[Vec<Tile>]) -> Coordinates {
    (
        hiking_map
            .first()
            .unwrap()
            .iter()
            .enumerate()
            .find(|(_x, tile)| **tile == Path)
            .unwrap()
            .0 as i32,
        0,
    )
}

fn finishing_point(hiking_map: &[Vec<Tile>]) -> Coordinates {
    (
        hiking_map
            .last()
            .unwrap()
            .iter()
            .enumerate()
            .find(|(_x, tile)| **tile == Path)
            .unwrap()
            .0 as i32,
        bottom_right(hiking_map).1,
    )
}

fn calculate_edges(
    hiking_map: &[Vec<Tile>],
    ignore_slopes: bool,
) -> HashMap<Coordinates, HashMap<Coordinates, usize>> {
    let starting_point = starting_point(hiking_map);

    let mut vertices = vec![starting_point];
    let mut edges = HashMap::new();

    while let Some(vertex) = vertices.pop() {
        let connected_vertices = get_connected_vertices(vertex, hiking_map, ignore_slopes);

        vertices.extend(
            connected_vertices
                .keys()
                .filter(|connected_vertex| !edges.contains_key(*connected_vertex)),
        );

        edges.insert(vertex, connected_vertices);
    }

    edges
}

fn longest_path_with_slopes(hiking_map: &[Vec<Tile>]) -> usize {
    let starting_point = starting_point(hiking_map);

    let mut distances_from_start = HashMap::from([(starting_point, 0)]);
    let mut sorted_vertices = Vec::new();
    let mut edges = HashMap::new();
    let mut visited_vertices = HashSet::from([starting_point]);

    topological_sort(
        hiking_map,
        starting_point,
        &mut sorted_vertices,
        &mut edges,
        &mut visited_vertices,
    );

    for vertex in sorted_vertices.into_iter().rev() {
        let distance_to_vertex = *distances_from_start.entry(vertex).or_default();

        for (connected_vertex, distance) in edges.get(&vertex).unwrap() {
            let prev_distance = distances_from_start.entry(*connected_vertex).or_default();

            *prev_distance = usize::max(*prev_distance, distance_to_vertex + *distance);
        }
    }

    *distances_from_start
        .get(&finishing_point(hiking_map))
        .unwrap()
}

#[aoc(day23, part1)]
fn part1(hiking_map: &[Vec<Tile>]) -> usize {
    longest_path_with_slopes(hiking_map)
}

fn check_all_paths(
    current_point: Coordinates,
    finishing_point: Coordinates,
    path: &mut Vec<Coordinates>,
    path_len: usize,
    edges: &HashMap<Coordinates, HashMap<Coordinates, usize>>,
    max_path_len: &mut usize,
) {
    if path.contains(&current_point) {
        return;
    }

    path.push(current_point);

    if current_point == finishing_point {
        *max_path_len = usize::max(path_len, *max_path_len);
        path.pop();
        return;
    }

    for (connected_vertex, distance) in edges.get(&current_point).unwrap().clone() {
        check_all_paths(
            connected_vertex,
            finishing_point,
            path,
            path_len + distance,
            edges,
            max_path_len,
        );
    }

    path.pop();
}

fn longest_path_without_slopes(hiking_map: &[Vec<Tile>]) -> usize {
    let starting_point = starting_point(hiking_map);
    let finishing_point = finishing_point(hiking_map);

    let edges = calculate_edges(hiking_map, true);

    let mut path = Vec::new();
    let mut max_path_len = 0;

    check_all_paths(
        starting_point,
        finishing_point,
        &mut path,
        0,
        &edges,
        &mut max_path_len,
    );

    max_path_len
}

#[aoc(day23, part2)]
fn part2(hiking_map: &[Vec<Tile>]) -> usize {
    longest_path_without_slopes(hiking_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str = r"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn part1_example1() {
        assert_eq!(part1(&parse_input(TEST_INPUT_1)), 94);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(part2(&parse_input(TEST_INPUT_1)), 154);
    }
}
