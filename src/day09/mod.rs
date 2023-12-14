#![allow(dead_code)]

use crate::utils;
use std::time::Instant;

fn is_all_zeroes(numbers: &[i32]) -> bool {
    for number in numbers {
        if *number != 0 {
            return false;
        }
    }

    true
}

fn get_next_value(history: &[i32]) -> i32 {
    let mut sequences: Vec<Vec<i32>> = vec![history.to_vec()];

    while sequences.last().unwrap().is_empty() || !is_all_zeroes(sequences.last().unwrap()) {
        let mut next_sequence: Vec<i32> = Vec::new();
        for (i, number) in sequences.last().unwrap().iter().skip(1).enumerate() {
            let difference = number - sequences.last().unwrap()[i];
            next_sequence.push(difference);
        }
        sequences.push(next_sequence.to_vec());
    }

    let mut next_value = 0;
    for sequence in sequences.iter_mut().rev() {
        let last_value = sequence[sequence.len() - 1];
        next_value += last_value;
        sequence.push(next_value);
    }

    sequences[0][sequences[0].len() - 1]
}

fn get_next_previous(history: &[i32]) -> i32 {
    let mut sequences: Vec<Vec<i32>> = vec![history.to_vec()];

    while sequences.last().unwrap().is_empty() || !is_all_zeroes(sequences.last().unwrap()) {
        let mut next_sequence: Vec<i32> = Vec::new();
        for (i, number) in sequences.last().unwrap().iter().skip(1).enumerate() {
            let difference = number - sequences.last().unwrap()[i];
            next_sequence.push(difference);
        }
        sequences.push(next_sequence.to_vec());
    }

    let mut previous_value = 0;
    for sequence in sequences.iter_mut().rev() {
        let first_value = sequence[0];

        previous_value = first_value - previous_value;
        sequence.insert(0, previous_value);
    }

    sequences[0][0]
}

pub fn puzzle1() {
    let start = Instant::now();
    println!("Day 9, puzzle 1");
    let input = utils::file::read_input("src/day09/input.txt");
    let lines = input.lines().collect::<Vec<&str>>();

    let mut next_value = 0;

    for line in lines {
        let numbers = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        next_value += get_next_value(&numbers);
    }

    println!("Next value: {}", next_value);
    println!("Time elapsed: {:?}", start.elapsed());
}

pub fn puzzle2() {
    let start = Instant::now();
    println!("Day 9, puzzle 2");
    let input = utils::file::read_input("src/day09/input.txt");
    let lines = input.lines().collect::<Vec<&str>>();

    let mut previous_value = 0;

    for line in lines {
        let numbers = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        previous_value += get_next_previous(&numbers);
    }

    println!("Previous value: {}", previous_value);

    println!("Time elapsed: {:?}", start.elapsed());
}
