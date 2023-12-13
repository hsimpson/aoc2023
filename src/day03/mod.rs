#![allow(dead_code)]

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

fn build_number(pos: usize, complete_array: &[char], line_length: usize) -> Number {
    let mut number_string = String::new();
    number_string.push(complete_array[pos]);

    // left
    let mut l = pos - 1;
    while complete_array[l].is_ascii_digit() {
        number_string.insert(0, complete_array[l]);
        if l % line_length == 0 {
            break;
        }
        l -= 1;
    }

    // right
    let mut r = pos + 1;
    while r < complete_array.len() - 1 && complete_array[r].is_ascii_digit() {
        number_string.push(complete_array[r]);
        r += 1;
        if r == line_length {
            break;
        }
    }

    Number {
        value: number_string.parse::<i32>().unwrap(),
        start: l,
        length: number_string.len(),
        line: pos / line_length,
    }
}

pub fn puzzle2() {
    let start = Instant::now();
    println!("Day 3, puzzle 2");

    let input = utils::file::read_input("src/day03/input.txt");
    let lines = input.lines().collect::<Vec<&str>>();
    let line_length = lines[0].len();
    let complete_array: Vec<char> = lines.join("").chars().collect();

    let mut sum = 0;

    for (i, c) in complete_array.iter().enumerate() {
        if *c != '*' {
            continue;
        }

        // gear found
        let mut numbers = Vec::new();

        // check above
        if i > line_length {
            let top_left_pos = i - line_length - 1;
            let top_right_pos = i - line_length + 1;
            let mut middle_number_found = false;
            let mut found_end = top_left_pos;

            // check left
            if top_left_pos % line_length > 0 && complete_array[top_left_pos].is_ascii_digit() {
                let number = build_number(top_left_pos, &complete_array, line_length);
                found_end = number.start + number.length;
                numbers.push(number);
            } else if complete_array[top_left_pos + 1].is_ascii_digit() {
                // check middle
                middle_number_found = true;
                let number = build_number(top_left_pos + 1, &complete_array, line_length);
                numbers.push(number);
            }

            if !middle_number_found
                && found_end + 1 < top_right_pos
                && top_right_pos % line_length < line_length - 1
                && complete_array[top_right_pos].is_ascii_digit()
            {
                let number = build_number(top_right_pos, &complete_array, line_length);
                numbers.push(number);
            }
        }

        // check left
        if i % line_length > 0 && complete_array[i - 1].is_ascii_digit() {
            let number = build_number(i - 1, &complete_array, line_length);
            numbers.push(number);
        }

        // check right
        if i % line_length < line_length - 1 && complete_array[i + 1].is_ascii_digit() {
            let number = build_number(i + 1, &complete_array, line_length);
            numbers.push(number);
        }

        // check below
        if i < complete_array.len() - line_length {
            let bottom_left_pos = i + line_length - 1;
            let bottom_right_pos = i + line_length + 1;
            let mut middle_number_found = false;
            let mut found_end = bottom_left_pos;

            // check left
            if bottom_left_pos % line_length > 0 && complete_array[bottom_left_pos].is_ascii_digit()
            {
                let number = build_number(bottom_left_pos, &complete_array, line_length);
                found_end = number.start + number.length;
                numbers.push(number);
            } else if complete_array[bottom_left_pos + 1].is_ascii_digit() {
                // check middle
                middle_number_found = true;
                let number = build_number(bottom_left_pos + 1, &complete_array, line_length);
                numbers.push(number);
            }

            if !middle_number_found
                && found_end + 1 < bottom_right_pos
                && bottom_right_pos % line_length < line_length - 1
                && complete_array[bottom_right_pos].is_ascii_digit()
            {
                let number = build_number(bottom_right_pos, &complete_array, line_length);
                numbers.push(number);
            }
        }

        // if 2 numbers found
        match numbers.len() {
            1 => {
                // println!("ERROR: only 1 number found");
            }
            2 => {
                // println!("numbers: {:?} {:?}", numbers[0].value, numbers[1].value);
                sum += numbers[0].value * numbers[1].value;
            }
            _ => {
                // println!("ERROR: more than 2 numbers found")
            }
        }
    }

    println!("Sum: {}", sum);

    println!("Time elapsed: {:?}", start.elapsed());
}
