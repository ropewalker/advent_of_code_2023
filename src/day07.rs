use crate::day07::Card::*;
use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        (*other as usize).cmp(&(*self as usize))
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const HAND_SIZE: usize = 5;

type HandType = (usize, usize);

#[derive(PartialEq, Eq, Clone, Debug)]
struct Hand([Card; HAND_SIZE]);

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut card_counts: HashMap<Card, usize> = HashMap::new();

        for card in self.0.iter() {
            *card_counts.entry(*card).or_insert(0) += 1;
        }

        let joker_count = card_counts.get(&Joker).unwrap_or(&0);

        let (most_frequent_card, largest_count) = card_counts
            .iter()
            .filter(|(card, _)| **card != Joker)
            .max_by(|(_, count_a), (_, count_b)| count_a.cmp(count_b))
            .unwrap_or((&Joker, &0));

        let second_largest_count = card_counts
            .iter()
            .filter(|(card, _)| **card != Joker && **card != *most_frequent_card)
            .max_by(|(_, count_a), (_, count_b)| count_a.cmp(count_b))
            .map(|(_, count)| count)
            .unwrap_or(&0);

        (*largest_count + *joker_count, *second_largest_count)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type()
            .cmp(&other.hand_type())
            .then(self.0.cmp(&other.0))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type Bid = u32;

#[aoc_generator(day7)]
fn parse_input(camel_cards: &str) -> Vec<(Hand, Bid)> {
    use aoc_parse::{parser, prelude::*};
    use Card::*;

    let parser = parser!(
        rule card: Card = {
            "A" => Ace,
            "K" => King,
            "Q" => Queen,
            "J" => Jack,
            "T" => Ten,
            "9" => Nine,
            "8" => Eight,
            "7" => Seven,
            "6" => Six,
            "5" => Five,
            "4" => Four,
            "3" => Three,
            "2" => Two,
        };

        rule hand: Hand = cards:card+ => Hand(cards.try_into().unwrap());

        lines(hand " " bid:u32)
    );

    parser.parse(camel_cards).unwrap()
}

#[aoc(day7, part1)]
fn part1(camel_cards: &[(Hand, Bid)]) -> u32 {
    let mut camel_cards = camel_cards.to_vec();

    camel_cards.sort_by(|(hand_a, _), (hand_b, _)| hand_a.cmp(hand_b));

    camel_cards
        .iter()
        .enumerate()
        .map(|(rank, (_, bid))| *bid * (rank as u32 + 1))
        .sum()
}

#[aoc(day7, part2)]
fn part2(camel_cards: &[(Hand, Bid)]) -> u32 {
    let mut camel_cards = camel_cards
        .iter()
        .map(|(hand, bid)| {
            (
                Hand(
                    hand.0
                        .iter()
                        .map(|card| if *card == Jack { Joker } else { *card })
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap(),
                ),
                bid,
            )
        })
        .collect::<Vec<_>>();

    camel_cards.sort_by(|(hand_a, _), (hand_b, _)| hand_a.cmp(hand_b));

    camel_cards
        .iter()
        .enumerate()
        .map(|(rank, (_, bid))| *bid * (rank as u32 + 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 6_440);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 5_905);
    }
}
