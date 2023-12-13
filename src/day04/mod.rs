#![allow(dead_code)]

use crate::utils;
use std::time::Instant;

struct Card {
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
}

struct Card2 {
    wins: u32,
}

fn parse_card(line: &str) -> Card {
    let first_split = line.split(':').collect::<Vec<&str>>();
    let second_split = first_split[1].split('|').collect::<Vec<&str>>();

    let winning_numbers = second_split[0]
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let my_numbers = second_split[1]
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    Card {
        winning_numbers,
        my_numbers,
    }
}

fn calc_card_worth(card: &Card) -> u32 {
    let mut worth = 0;

    for winning_number in &card.winning_numbers {
        if card.my_numbers.contains(winning_number) {
            if worth == 0 {
                worth = 1;
            } else {
                worth *= 2;
            }
        }
    }

    worth
}

pub fn puzzle1() {
    let start = Instant::now();
    println!("Day 4, puzzle 1");

    let input = utils::file::read_input("src/day04/input.txt");
    let lines = input.lines().collect::<Vec<&str>>();

    let mut sum = 0;

    for line in lines {
        let card = parse_card(line);
        sum += calc_card_worth(&card);
    }

    println!("Sum: {}", sum);

    println!("Time elapsed: {:?}", start.elapsed());
}

fn parse_card2(line: &str) -> Card2 {
    let card = parse_card(line);

    let mut wins = 0;

    for winning_number in card.winning_numbers {
        if card.my_numbers.contains(&winning_number) {
            wins += 1;
        }
    }

    Card2 {
        // number: card.number,
        wins,
    }
}

pub fn puzzle2() {
    let start = Instant::now();
    println!("Day 4, puzzle 2");

    let input = utils::file::read_input("src/day04/input.txt");
    let lines = input.lines().collect::<Vec<&str>>();

    let mut parsed_cards: Vec<Card2> = Vec::new();
    let mut card_counts: Vec<u32> = vec![0; lines.len()];

    for line in lines {
        let card = parse_card2(line);
        parsed_cards.push(card);
    }

    let mut sum = 0;

    for (i, parsed_card) in parsed_cards.iter().enumerate() {
        let mut idx = i;
        card_counts[idx] += 1;
        sum += 1;

        if parsed_card.wins > 0 {
            let card_count = card_counts[i];

            for _ in 0..parsed_card.wins {
                idx += 1;
                card_counts[idx] += card_count;
                sum += card_count;
            }
        }
    }

    println!("Sum: {}", sum);

    println!("Time elapsed: {:?}", start.elapsed());
}
