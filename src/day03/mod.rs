use crate::utils;
use std::time::Instant;

#[derive(Debug, Copy, Clone)]
struct Number {
    value: i32,
    start: usize,
    length: usize,
    line: usize,
}

fn find_numbers(line_str: &str, line: usize) -> Vec<Number> {
    let mut numbers = Vec::new();
    let mut number_string = String::new();
    for (i, c) in line_str.chars().enumerate() {
        if c.is_ascii_digit() {
            number_string.push(c);
        } else if !number_string.is_empty() {
            let number = number_string.parse::<i32>().unwrap();
            numbers.push(Number {
                value: number,
                start: i - number_string.len(),
                length: number_string.len(),
                line,
            });
            number_string.clear();
        }
    }
    if !number_string.is_empty() {
        let number = number_string.parse::<i32>().unwrap();
        numbers.push(Number {
            value: number,
            start: line_str.len() - number_string.len(),
            length: number_string.len(),
            line,
        });
    }

    numbers
}

fn check_adjacent(
    number: &Number,
    line_count: usize,
    line_length: usize,
    complete_array: &[char],
) -> bool {
    let start_idx = if number.start > 0 {
        number.start - 1
    } else {
        0
    };

    let end_idx = if number.start + number.length < line_length {
        number.start + number.length
    } else {
        line_length - 1
    };

    // println!(
    //     "number: {}, start_idx: {}, end_idx: {}",
    //     number.value, start_idx, end_idx
    // );

    // check top
    if number.line > 0 {
        #[allow(clippy::needless_range_loop)]
        for i in start_idx..=end_idx {
            let idx = i + ((number.line - 1) * line_length);
            if complete_array[idx] != '.' {
                // println!("top match");
                return true;
            }
        }
    }

    // check left
    if start_idx > 0 && complete_array[(number.line * line_length) + start_idx] != '.' {
        // println!("left match");
        return true;
    }

    // check right
    if number.start + number.length < line_length
        && complete_array[(number.line * line_length) + end_idx] != '.'
    {
        // println!("right match");
        return true;
    }

    // check bottom
    if number.line < line_count - 1 {
        #[allow(clippy::needless_range_loop)]
        for i in start_idx..=end_idx {
            let idx = i + ((number.line + 1) * line_length);
            if complete_array[idx] != '.' {
                // println!("bottom match");
                return true;
            }
        }
    }

    false
}

pub fn puzzle1() {
    let start = Instant::now();
    println!("Day 3, puzzle 1");

    let input = utils::file::read_input("src/day03/input.txt");
    let lines = input.lines().collect::<Vec<&str>>();
    let line_count = lines.len();
    let line_length = lines[0].len();
    let complete_array: Vec<char> = lines.join("").chars().collect();

    let mut numbers: Vec<Number> = Vec::new();

    let mut sum = 0;

    for (i, line) in lines.iter().enumerate() {
        let line_numbers = find_numbers(line, i);
        numbers.extend(&line_numbers);
    }

    for number in numbers {
        if check_adjacent(&number, line_count, line_length, &complete_array) {
            sum += number.value;
        }
    }

    println!("Sum: {}", sum);

    println!("Time elapsed: {:?}", start.elapsed());
}

pub fn puzzle2() {
    let start = Instant::now();
    println!("Day 3, puzzle 2");

    println!("Time elapsed: {:?}", start.elapsed());
}
