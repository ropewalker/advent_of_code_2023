use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone)]
struct MapLine {
    destination_range_start: u32,
    source_range_start: u32,
    range_length: u32,
}

#[derive(Clone)]
struct CategoryMap(Vec<MapLine>);

struct Almanac {
    seeds: Vec<u32>,
    maps: Vec<CategoryMap>,
}

#[aoc_generator(day5)]
fn parse_input(almanac: &str) -> Almanac {
    use aoc_parse::{parser, prelude::*};

    let almanac_parser = parser!(
        rule map_line: MapLine = destination_range_start:u32 " " source_range_start:u32 " " range_length:u32 =>
            MapLine {
                destination_range_start,
                source_range_start,
                range_length,
            };

        rule category_map: CategoryMap =
            line(string(any_char+))
            map_lines:lines(map_line) =>
                CategoryMap(map_lines);

        seeds:section(line("seeds: " repeat_sep(u32, " ")))
        maps:sections(category_map) =>
            Almanac {
                seeds,
                maps,
            }
    );

    almanac_parser.parse(almanac).unwrap()
}

fn location(seed: &u32, maps: &[CategoryMap]) -> u32 {
    let mut id = *seed;

    for category_map in maps.iter() {
        for map_line in category_map.0.iter() {
            if id >= map_line.source_range_start
                && id < map_line.source_range_start + map_line.range_length
            {
                id = map_line.destination_range_start + (id - map_line.source_range_start);
                break;
            }
        }
    }

    id
}

#[aoc(day5, part1)]
fn part1(almanac: &Almanac) -> u32 {
    almanac
        .seeds
        .iter()
        .map(|seed| location(seed, &almanac.maps))
        .min()
        .unwrap()
}

#[aoc(day5, part2)]
fn part2(almanac: &Almanac) -> u32 {
    let mut source_ranges = Vec::new();

    for seed_range in almanac.seeds.chunks_exact(2) {
        source_ranges.push((seed_range[0], seed_range[0] + seed_range[1] - 1));
    }

    for category_map in almanac.maps.iter() {
        let mut destination_ranges = Vec::new();

        'source_ranges: while let Some(source_range) = source_ranges.pop() {
            for map_line in category_map.0.iter() {
                let line_source_start = map_line.source_range_start;
                let line_source_end = map_line.source_range_start + map_line.range_length - 1;

                let line_destination_start = map_line.destination_range_start;

                if line_source_start <= source_range.1 && line_source_end >= source_range.0 {
                    if source_range.0 < line_source_start {
                        source_ranges.push((source_range.0, line_source_start - 1));
                    }

                    if source_range.1 > line_source_end {
                        source_ranges.push((line_source_end + 1, source_range.1));
                    }

                    destination_ranges.push((
                        u32::max(line_source_start, source_range.0) - line_source_start
                            + line_destination_start,
                        u32::min(line_source_end, source_range.1) - line_source_start
                            + line_destination_start,
                    ));

                    continue 'source_ranges;
                }
            }

            destination_ranges.push((source_range.0, source_range.1));
        }

        source_ranges = destination_ranges;
    }

    source_ranges
        .iter()
        .map(|(range_min, _)| *range_min)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 35);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 46);
    }
}
