#![allow(dead_code)]

use crate::utils;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq)]
#[repr(u8)]
enum Pipe {
    Ground = b'.',
    Start = b'S',
    ConnectNorthSouth = b'|',
    ConnectEastWest = b'-',
    ConnectNorthEast = b'L',
    ConnectNorthWest = b'J',
    ConnectSouthWest = b'7',
    ConnectSouthEast = b'F',
}

impl Pipe {
    pub fn from(c: char) -> Pipe {
        match c {
            '.' => Pipe::Ground,
            'S' => Pipe::Start,
            '|' => Pipe::ConnectNorthSouth,
            '-' => Pipe::ConnectEastWest,
            'L' => Pipe::ConnectNorthEast,
            'J' => Pipe::ConnectNorthWest,
            '7' => Pipe::ConnectSouthWest,
            'F' => Pipe::ConnectSouthEast,
            _ => panic!("Invalid pipe character: {}", c),
        }
    }
}

pub fn puzzle1() {
    let start = Instant::now();
    println!("Day 10, puzzle 1");

    let input = utils::file::read_input("src/day10/test.txt");
    let lines = input.lines().collect::<Vec<&str>>();

    let mut map: Vec<Vec<Pipe>> = Vec::new();
    let mut start_pipe: (usize, usize) = (0, 0);

    for (i, line) in lines.iter().enumerate() {
        let mut row: Vec<Pipe> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            let pipe = Pipe::from(c);
            if pipe == Pipe::Start {
                start_pipe = (i, j);
            }
            row.push(pipe);
        }
        map.push(row);
    }

    println!("Time elapsed: {:?}", start.elapsed());
}

pub fn puzzle2() {
    let start = Instant::now();
    println!("Day 10, puzzle 2");

    println!("Time elapsed: {:?}", start.elapsed());
}
