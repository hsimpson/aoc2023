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

    let input = utils::file::read_input("src/day07/test.txt");

    println!("Time elapsed: {:?}", start.elapsed());
}
