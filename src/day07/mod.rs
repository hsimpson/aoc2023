#![allow(dead_code)]

use crate::utils;
use std::{cmp::Ordering, time::Instant};

const CARDS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

#[derive(Debug, Eq, PartialEq)]
enum HandType {
    FifeOfAKind,
    ForOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

const TYPE_RANK: [HandType; 7] = [
    HandType::FifeOfAKind,
    HandType::ForOfAKind,
    HandType::FullHouse,
    HandType::ThreeOfAKind,
    HandType::TwoPair,
    HandType::OnePair,
    HandType::HighCard,
];

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    hand_type: HandType,
    bid: u32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type != other.hand_type {
            let type_rank = TYPE_RANK.iter().position(|t| *t == self.hand_type).unwrap();
            let other_type_rank = TYPE_RANK
                .iter()
                .position(|t| *t == other.hand_type)
                .unwrap();
            type_rank.cmp(&other_type_rank)
        } else {
            // if both hands have the same type, compare the cards
            for i in 0..5 {
                let self_card = self.cards[i];
                let other_card = other.cards[i];
                let self_card_rank = CARDS.iter().position(|&c| c == self_card).unwrap();
                let other_card_rank = CARDS.iter().position(|&c| c == other_card).unwrap();
                if self_card_rank != other_card_rank {
                    return self_card_rank.cmp(&other_card_rank);
                }
            }

            Ordering::Equal
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

fn hand_type(cards: &Vec<char>) -> HandType {
    let mut counts = [0; 13];
    for card in cards {
        let index = CARDS.iter().position(|&c| c == *card).unwrap();
        counts[index] += 1;
    }

    let mut fives = 0;
    let mut fours = 0;
    let mut threes = 0;
    let mut twos = 0;

    for count in counts {
        match count {
            5 => fives += 1,
            4 => fours += 1,
            3 => threes += 1,
            2 => twos += 1,
            _ => (),
        }
    }

    if fives > 0 {
        HandType::FifeOfAKind
    } else if fours > 0 {
        HandType::ForOfAKind
    } else if threes > 0 && twos > 0 {
        HandType::FullHouse
    } else if threes > 0 {
        HandType::ThreeOfAKind
    } else if twos > 1 {
        HandType::TwoPair
    } else if twos > 0 {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}

fn parse_hand(line: &str) -> Hand {
    let split = line.split(' ').collect::<Vec<&str>>();
    let cards = split[0].chars().collect::<Vec<char>>();
    let bid = split[1].parse::<u32>().unwrap();
    let hand_type = hand_type(&cards);

    Hand {
        cards,
        hand_type,
        bid,
    }
}

fn hand_type2(cards: &Vec<char>) -> HandType {
    let mut counts = [0; 13];
    for card in cards {
        if card == &'J' {
            continue;
        }
        let index = CARDS.iter().position(|&c| c == *card).unwrap();
        counts[index] += 1;
    }

    let mut fives = 0;
    let mut fours = 0;
    let mut threes = 0;
    let mut twos = 0;

    for count in counts {
        match count {
            5 => fives += 1,
            4 => fours += 1,
            3 => threes += 1,
            2 => twos += 1,
            _ => (),
        }
    }

    let mut hand_type: HandType;

    if fives > 0 {
        hand_type = HandType::FifeOfAKind
    } else if fours > 0 {
        hand_type = HandType::ForOfAKind
    } else if threes > 0 && twos > 0 {
        hand_type = HandType::FullHouse
    } else if threes > 0 {
        hand_type = HandType::ThreeOfAKind
    } else if twos > 1 {
        hand_type = HandType::TwoPair
    } else if twos > 0 {
        hand_type = HandType::OnePair
    } else {
        hand_type = HandType::HighCard
    }

    let jocker_count = cards.iter().filter(|&c| c == &'J').count();
    for _ in 0..jocker_count {
        match hand_type {
            HandType::FifeOfAKind => hand_type = HandType::FifeOfAKind,
            HandType::ForOfAKind => hand_type = HandType::FifeOfAKind,
            HandType::FullHouse => hand_type = HandType::ForOfAKind,
            HandType::ThreeOfAKind => hand_type = HandType::ForOfAKind,
            HandType::TwoPair => hand_type = HandType::FullHouse,
            HandType::OnePair => hand_type = HandType::ThreeOfAKind,
            HandType::HighCard => hand_type = HandType::OnePair,
        }
    }

    hand_type
}

fn parse_hand2(line: &str) -> Hand {
    let split = line.split(' ').collect::<Vec<&str>>();
    let cards = split[0].chars().collect::<Vec<char>>();
    let bid = split[1].parse::<u32>().unwrap();
    let hand_type = hand_type2(&cards);

    Hand {
        cards,
        hand_type,
        bid,
    }
}

pub fn puzzle1() {
    let start = Instant::now();
    println!("Day 7, puzzle 1");

    let input = utils::file::read_input("src/day07/input.txt");
    let lines = input.lines().collect::<Vec<&str>>();
    let mut hands: Vec<Hand> = Vec::new();

    for line in lines {
        let hand = parse_hand(line);
        hands.push(hand);
    }

    hands.sort();

    let mut total_winnings = 0;
    let mut rank: u32 = hands.len() as u32;
    for hand in hands {
        total_winnings += hand.bid * rank;
        rank -= 1;
    }

    println!("total winnings: {:?}", total_winnings);
    println!("Time elapsed: {:?}", start.elapsed());
}

pub fn puzzle2() {
    let start = Instant::now();
    println!("Day 7, puzzle 2");

    let input = utils::file::read_input("src/day07/input.txt");
    let lines = input.lines().collect::<Vec<&str>>();
    let mut hands: Vec<Hand> = Vec::new();

    for line in lines {
        let hand = parse_hand2(line);
        hands.push(hand);
    }

    const CARDS2: [char; 13] = [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];

    hands.sort_by(|a, b| {
        if a.hand_type != b.hand_type {
            let a_rank = TYPE_RANK.iter().position(|t| *t == a.hand_type).unwrap();
            let b_rank = TYPE_RANK.iter().position(|t| *t == b.hand_type).unwrap();

            return a_rank.cmp(&b_rank);
        } else {
            for i in 0..5 {
                let a_card = a.cards[i];
                let b_card = b.cards[i];
                let a_card_rank = CARDS2.iter().position(|&c| c == a_card).unwrap();
                let b_card_rank = CARDS2.iter().position(|&c| c == b_card).unwrap();
                if a_card_rank != b_card_rank {
                    return a_card_rank.cmp(&b_card_rank);
                }
            }
        }

        Ordering::Equal
    });

    let mut total_winnings = 0;
    let mut rank: u32 = hands.len() as u32;
    for hand in hands {
        total_winnings += hand.bid * rank;
        rank -= 1;
    }

    println!("total winnings: {:?}", total_winnings);
    println!("Time elapsed: {:?}", start.elapsed());
}
