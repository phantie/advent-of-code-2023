#![allow(unused)]

mod part_one {
    use super::*;

    pub fn part_one() -> u32 {
        let mut hands = read_input()
            .map(Result::unwrap)
            .map(parse_line)
            .collect::<Vec<_>>();

        hands.sort_by(|(lhs, _), (rhs, _)| lhs.kind().cmp(&rhs.kind()));
        hands.sort_by(|(lhs, _), (rhs, _)| lhs.label_cmp(&rhs));

        hands
            .into_iter()
            .enumerate()
            // from weakest to strongest hand order
            .fold(0, |acc, (i, (hand, bid))| acc + ((i + 1) * bid as usize)) as u32
    }

    #[cfg(test)]
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 254024898);
    }
}

mod part_two {
    use super::*;

    pub fn part_two() -> u32 {
        let mut hands = read_input()
            .map(Result::unwrap)
            .map(parse_line)
            .collect::<Vec<_>>();

        hands.sort_by(|(lhs, _), (rhs, _)| lhs.joker_kind().cmp(&rhs.joker_kind()));
        hands.sort_by(|(lhs, _), (rhs, _)| lhs.joker_label_cmp(&rhs));

        hands
            .into_iter()
            .enumerate()
            // from weakest to strongest hand order
            .fold(0, |acc, (i, (hand, bid))| acc + ((i + 1) * bid as usize)) as u32
    }

    #[cfg(test)]
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 254115617);
    }
}

fn main() {
    dbg!(part_one::part_one());
    dbg!(part_two::part_two());
}

fn read_input() -> utils::ReadLines {
    let filename = "input.txt";
    utils::read_lines(filename).unwrap()
}

pub fn parse_line(value: String) -> (Hand, Bid) {
    let (hand, bid) = {
        let mut s = value.split(" ");
        (s.next().unwrap(), s.next().unwrap())
    };

    (Hand::new(hand), bid.parse().unwrap())
}

use std::cmp::Ordering;

fn label_to_weight(value: char) -> u32 {
    let labels = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];

    (labels.len() - labels.clone().into_iter().position(|v| v == value).unwrap()) as u32
}

fn joker_label_to_weight(value: char) -> u32 {
    let labels = [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];

    (labels.len() - labels.clone().into_iter().position(|v| v == value).unwrap()) as u32
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum HandKind {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandKind {
    pub fn weight(&self) -> u32 {
        use HandKind::*;
        match self {
            FiveOfAKind => 6,
            FourOfAKind => 5,
            FullHouse => 4,
            ThreeOfAKind => 3,
            TwoPair => 2,
            OnePair => 1,
            HighCard => 0,
        }
    }
}

impl Ord for HandKind {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight().cmp(&other.weight())
    }
}

#[derive(Debug)]
pub struct Hand(String);

impl AsRef<str> for Hand {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Hand {
    pub fn new(value: &str) -> Self {
        Self(value.into())
    }

    pub fn label_cmp(&self, other: &Self) -> Ordering {
        if self.kind() == other.kind() {
            self.as_ref()
                .chars()
                .zip(other.as_ref().chars())
                .find_map(|(self_c, other_c)| {
                    match label_to_weight(self_c).cmp(&label_to_weight(other_c)) {
                        Ordering::Equal => None,
                        v => Some(v),
                    }
                })
                .unwrap()
        } else {
            Ordering::Equal
        }
    }

    pub fn joker_label_cmp(&self, other: &Self) -> Ordering {
        if self.joker_kind() == other.joker_kind() {
            self.as_ref()
                .chars()
                .zip(other.as_ref().chars())
                .find_map(|(self_c, other_c)| {
                    match joker_label_to_weight(self_c).cmp(&joker_label_to_weight(other_c)) {
                        Ordering::Equal => None,
                        v => Some(v),
                    }
                })
                .unwrap()
        } else {
            Ordering::Equal
        }
    }

    pub fn joker_kind(&self) -> HandKind {
        let j_count = self.as_ref().chars().filter(|c| *c == 'J').count();

        if j_count == 0 {
            return self.kind();
        }

        use HandKind::*;
        match (self.kind(), j_count) {
            (FiveOfAKind, 5) => FiveOfAKind,

            (FourOfAKind, 4) => FiveOfAKind,
            (FourOfAKind, 1) => FiveOfAKind,

            (FullHouse, 3) => FiveOfAKind,
            (FullHouse, 2) => FiveOfAKind,

            (ThreeOfAKind, 3) => FourOfAKind,
            (ThreeOfAKind, 1) => FourOfAKind,

            (TwoPair, 2) => FourOfAKind,
            (TwoPair, 1) => FullHouse,

            (OnePair, 2) => ThreeOfAKind,
            (OnePair, 1) => ThreeOfAKind,

            (HighCard, 1) => OnePair,

            _ => unreachable!(),
        }
    }

    pub fn kind(&self) -> HandKind {
        use std::collections::HashMap;
        let char_count = self.as_ref().chars().fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        });

        let mut counts = char_count.values().collect::<Vec<_>>();
        counts.sort();
        counts.reverse();

        use HandKind::*;

        match counts[..] {
            [5] => FiveOfAKind,
            [4, 1] => FourOfAKind,
            [3, 2] => FullHouse,
            [3, 1, 1] => ThreeOfAKind,
            [2, 2, 1] => TwoPair,
            [2, 1, 1, 1] => OnePair,
            [1, 1, 1, 1, 1] => HighCard,
            _ => unreachable!(),
        }
    }
}

pub type Bid = u32;
