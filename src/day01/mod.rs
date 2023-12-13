#![allow(dead_code)]

use crate::utils;
use std::time::Instant;

pub fn puzzle1() {
    let start = Instant::now();
    println!("Day 1, puzzle 1");

    let input = utils::file::read_input("src/day01/input.txt");
    let lines = input.lines().collect::<Vec<&str>>();
    let mut sum: u32 = 0;

    for line in lines {
        let chars = line.chars().collect::<Vec<char>>();
        let mut digits: Vec<u32> = Vec::new();
        for c in chars {
            if c.is_ascii_digit() {
                digits.push(c.to_digit(10).unwrap());
            }
        }

        let first = digits[0];
        let last = digits[digits.len() - 1];
        let number = first * 10 + last;
        sum += number;
    }

    println!("Sum: {}", sum);

    println!("Time elapsed: {:?}", start.elapsed());
}

pub fn puzzle2() {
    let start = Instant::now();
    println!("Day 1, puzzle 2");

    let input = utils::file::read_input("src/day01/input.txt");
    let lines = input.lines().collect::<Vec<&str>>();
    let mut sum: u32 = 0;

    let number_words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for line in lines {
        let mut digits: Vec<u32> = Vec::new();

        for i in 0..line.len() {
            let c = line.chars().nth(i).unwrap();

            if c.is_ascii_digit() {
                digits.push(c.to_digit(10).unwrap());
            } else {
                for (j, number_word) in number_words.iter().enumerate() {
                    if line[i..].starts_with(number_word) {
                        digits.push((j as u32) + 1);
                        break;
                    }
                }
            }
        }

        if !digits.is_empty() {
            let first = digits[0];
            let last = digits[digits.len() - 1];
            let number = first * 10 + last;
            sum += number;
        }
    }

    println!("Sum: {}", sum);

    println!("Time elapsed: {:?}", start.elapsed());
}
