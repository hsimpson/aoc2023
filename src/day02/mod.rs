use crate::utils;
use std::{cmp, time::Instant};

pub fn puzzle1() {
    let start = Instant::now();
    println!("Day 2, puzzle 1");

    let input = utils::file::read_input("src/day02/input.txt");
    let lines = input.lines().collect::<Vec<&str>>();
    let mut sum: u32 = 0;

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    for (i, line) in lines.iter().enumerate() {
        let game_number = i + 1;
        let game_id = format!("Game {}:", game_number);
        let game_id_len = game_id.len();

        let cube_games = &line[game_id_len..];
        let cube_games = cube_games.split(';').collect::<Vec<&str>>();

        let mut valid = true;

        for cube_game in cube_games {
            let cubes = cube_game.split(',').collect::<Vec<&str>>();

            for cube in cubes {
                let cube = cube.trim();
                let segments = cube.split(' ').collect::<Vec<&str>>();
                let amount = segments[0].parse::<u32>().unwrap();
                let color = segments[1];

                if color == "red" && amount > max_red
                    || color == "green" && amount > max_green
                    || color == "blue" && amount > max_blue
                {
                    valid = false;
                    break;
                }
            }

            if !valid {
                break;
            }
        }

        if valid {
            sum += game_number as u32;
        }
    }

    println!("Sum: {}", sum);

    println!("Time elapsed: {:?}", start.elapsed());
}

pub fn puzzle2() {
    let start = Instant::now();
    println!("Day 2, puzzle 2");

    let input = utils::file::read_input("src/day02/input.txt");
    let lines = input.lines().collect::<Vec<&str>>();
    let mut sum: u32 = 0;

    for (i, line) in lines.iter().enumerate() {
        let game_number = i + 1;
        let game_id = format!("Game {}:", game_number);
        let game_id_len = game_id.len();

        let cube_games = &line[game_id_len..];
        let cube_games = cube_games.split(';').collect::<Vec<&str>>();

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for cube_game in cube_games {
            let cubes = cube_game.split(',').collect::<Vec<&str>>();

            for cube in cubes {
                let cube = cube.trim();
                let segments = cube.split(' ').collect::<Vec<&str>>();
                let amount = segments[0].parse::<u32>().unwrap();
                let color = segments[1];

                if color == "red" {
                    max_red = cmp::max(max_red, amount);
                } else if color == "green" {
                    max_green = cmp::max(max_green, amount);
                } else if color == "blue" {
                    max_blue = cmp::max(max_blue, amount);
                }
            }
        }
        let power = max_red * max_green * max_blue;
        sum += power;
    }

    println!("Sum: {}", sum);

    println!("Time elapsed: {:?}", start.elapsed());
}
