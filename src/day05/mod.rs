use std::time::Instant;

use crate::utils;

#[derive(Debug)]
struct MapItem {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

#[derive(Debug)]
struct Map {
    items: Vec<MapItem>,
    name: String,
}

impl Map {
    fn map_destination(&self, seed: u64) -> u64 {
        for item in &self.items {
            if seed >= item.source_range_start
                && seed < (item.source_range_start + item.range_length)
            {
                let offset = seed - item.source_range_start;
                return item.destination_range_start + offset;
            }
        }
        // fallback if no ranges mapping found
        seed
    }
}

fn parse_map(lines: &Vec<&str>, idx: &mut usize) -> Map {
    let line = lines[*idx];
    let name = line.replace(" map:", "");
    *idx += 1;

    let mut items: Vec<MapItem> = Vec::new();

    while *idx < lines.len() && !lines[*idx].is_empty() {
        let line = lines[*idx];
        let split = line.split_whitespace().collect::<Vec<&str>>();

        let destination_range_start = split[0].parse::<u64>().unwrap();
        let source_range_start = split[1].parse::<u64>().unwrap();
        let range_length = split[2].parse::<u64>().unwrap();

        items.push(MapItem {
            destination_range_start,
            source_range_start,
            range_length,
        });

        *idx += 1;
    }

    items.sort_by(|a, b| a.source_range_start.cmp(&b.source_range_start));

    Map { items, name }
}

pub fn puzzle1() {
    let start = Instant::now();
    println!("Day 5, puzzle 1");

    let input = utils::file::read_input("src/day05/input.txt");
    let lines = input.lines().collect::<Vec<&str>>();

    let mut seeds: Vec<u64> = Vec::new();
    let mut maps: Vec<Map> = Vec::new();

    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];
        if line.is_empty() {
            i += 1;
            continue;
        }

        if line.starts_with("seeds:") {
            let seed_strings = lines[i].split(':').collect::<Vec<&str>>()[1];
            seeds = seed_strings
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            i += 1;
            continue;
        }

        let first_char = line.chars().next().unwrap();
        if first_char.is_alphabetic() {
            let map = parse_map(&lines, &mut i);
            maps.push(map);
        }

        i += 1;
    }

    let mut lowest_location = u64::MAX;

    for seed in seeds {
        let mut destination = seed;
        for map in &maps {
            destination = map.map_destination(destination);
        }
        lowest_location = lowest_location.min(destination);
    }

    println!("Lowest location: {}", lowest_location);

    println!("Time elapsed: {:?}", start.elapsed());
}

#[derive(Debug)]
struct Seed {
    start: u64,
    length: u64,
}

fn parse_range_seeds(raw_seeds: Vec<u64>) -> Vec<Seed> {
    let mut seeds: Vec<Seed> = Vec::new();

    let mut i = 0;
    while i < raw_seeds.len() {
        let start = raw_seeds[i];
        let length = raw_seeds[i + 1];
        seeds.push(Seed { start, length });

        i += 2;
    }

    seeds.sort_by(|a, b| a.start.cmp(&b.start));

    seeds
}
pub fn puzzle2() {
    let start = Instant::now();
    println!("Day 5, puzzle 2");

    let input = utils::file::read_input("src/day05/input.txt");
    let lines = input.lines().collect::<Vec<&str>>();

    let mut seeds: Vec<u64> = Vec::new();
    let mut maps: Vec<Map> = Vec::new();

    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];
        if line.is_empty() {
            i += 1;
            continue;
        }

        if line.starts_with("seeds:") {
            let seed_strings = lines[i].split(':').collect::<Vec<&str>>()[1];
            seeds = seed_strings
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            i += 1;
            continue;
        }

        let first_char = line.chars().next().unwrap();
        if first_char.is_alphabetic() {
            let map = parse_map(&lines, &mut i);
            maps.push(map);
        }

        i += 1;
    }

    let mut lowest_location = u64::MAX;

    let range_seeds = parse_range_seeds(seeds);

    for range_seed in range_seeds {
        println! {"{:?}", range_seed};
        for seed in range_seed.start..(range_seed.start + range_seed.length) {
            let mut destination = seed;
            for map in &maps {
                destination = map.map_destination(destination);
            }
            lowest_location = lowest_location.min(destination);
        }
    }

    println!("Lowest location: {}", lowest_location);

    println!("Time elapsed: {:?}", start.elapsed());
}
