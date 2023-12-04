use aoc_runner_derive::{aoc, aoc_generator};

struct Scratchcard {
    winning_numbers: Vec<u32>,
    numbers_you_have: Vec<u32>,
}

#[aoc_generator(day4)]
fn parse_input(scratchcards: &str) -> Vec<Scratchcard> {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(lines(
        "Card" " "+ digit+ ":" " "+
        numbers_you_have:repeat_sep(u32, " "+)
        " "+ "|" " "+
        winning_numbers:repeat_sep(u32, " "+) =>
            Scratchcard {
                numbers_you_have,
                winning_numbers,
            }
    ));
    parser.parse(scratchcards).unwrap()
}

#[aoc(day4, part1)]
fn part1(scratchcards: &[Scratchcard]) -> u32 {
    scratchcards
        .iter()
        .map(|scratchcard| {
            let matched_numbers_count = scratchcard
                .winning_numbers
                .iter()
                .filter(|winning_number| scratchcard.numbers_you_have.contains(winning_number))
                .count();

            if matched_numbers_count > 0 {
                2u32.pow(matched_numbers_count as u32 - 1)
            } else {
                0
            }
        })
        .sum()
}

fn win_cards(scratchcards: &[Scratchcard], copies_count: &mut [usize], card_number: usize) {
    let matched_numbers_count = scratchcards[card_number - 1]
        .winning_numbers
        .iter()
        .filter(|winning_number| {
            scratchcards[card_number - 1]
                .numbers_you_have
                .contains(winning_number)
        })
        .count();

    for next_card_number in card_number + 1..=card_number + matched_numbers_count {
        copies_count[next_card_number - 1] += copies_count[card_number - 1];
    }
}

#[aoc(day4, part2)]
fn part2(scratchcards: &[Scratchcard]) -> usize {
    let mut copies_count = vec![1; scratchcards.len()];

    for card_number in 1..=copies_count.len() {
        win_cards(scratchcards, &mut copies_count, card_number)
    }

    copies_count.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 13);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 30);
    }
}
