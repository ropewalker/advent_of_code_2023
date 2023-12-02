use aoc_runner_derive::{aoc, aoc_generator};

struct Configuration {
    red: u32,
    green: u32,
    blue: u32,
}

type Game = Vec<Configuration>;
type GameId = usize;

#[aoc_generator(day2)]
fn parse_input(record: &str) -> Vec<(GameId, Game)> {
    use aoc_parse::{parser, prelude::*};

    let color_parser = parser!({
        red:u32 " red" => [red, 0, 0],
        blue:u32 " blue" => [0, blue, 0],
        green:u32 " green" => [0, 0, green],
    });

    let configuration_parser = parser!(colors:repeat_sep(color:color_parser, ", ") =>
    Configuration {
        red: colors.iter().map(|[red, _, _]| red).sum(),
        blue: colors.iter().map(|[_, blue, _]| blue).sum(),
        green: colors.iter().map(|[_, _, green]| green).sum(),
    });

    let games_parser = parser!(lines(
        "Game " game_id:usize ": " games:repeat_sep(configuration_parser, "; ") => (game_id, games)
    ));

    games_parser.parse(record).unwrap()
}

#[aoc(day2, part1)]
fn part1(record: &[(GameId, Game)]) -> usize {
    const TEST_CONFIGURATION: Configuration = Configuration {
        red: 12,
        green: 13,
        blue: 14,
    };

    record
        .iter()
        .filter(|(_, game)| {
            game.iter().all(|configuration| {
                configuration.red <= TEST_CONFIGURATION.red
                    && configuration.green <= TEST_CONFIGURATION.green
                    && configuration.blue <= TEST_CONFIGURATION.blue
            })
        })
        .map(|(game_id, _)| game_id)
        .sum()
}

fn power(game: &Game) -> u32 {
    let max_red = game
        .iter()
        .map(|configuration| configuration.red)
        .max()
        .unwrap();
    let max_green = game
        .iter()
        .map(|configuration| configuration.green)
        .max()
        .unwrap();
    let max_blue = game
        .iter()
        .map(|configuration| configuration.blue)
        .max()
        .unwrap();

    max_red * max_green * max_blue
}

#[aoc(day2, part2)]
fn part2(record: &[(GameId, Game)]) -> u32 {
    record.iter().map(|(_, game)| power(game)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 8);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 2_286);
    }
}
