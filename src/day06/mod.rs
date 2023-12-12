use crate::utils;
use std::time::Instant;

#[derive(Debug)]
struct Record {
    time: u64,
    distance: u64,
}

impl Record {
    fn calc_distance(&self, time: u64) -> u64 {
        let speed = time;
        let remaining_time = self.time - time;

        speed * remaining_time
    }

    fn calc_wins(&self) -> u64 {
        let mut wins = 0;
        for time in 1..self.time - 1 {
            let distance = self.calc_distance(time);
            if distance > self.distance {
                wins += 1;
            }
        }
        wins
    }
}

pub fn puzzle1() {
    let start = Instant::now();
    println!("Day 6, puzzle 1");

    let input = utils::file::read_input("src/day06/input.txt");
    let lines = input.lines().collect::<Vec<&str>>();

    let times = lines[0].split(':').collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let distances = lines[1].split(':').collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let records: Vec<Record> = times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| Record {
            time: *t,
            distance: *d,
        })
        .collect::<Vec<Record>>();

    let mut wins: Vec<u64> = Vec::new();

    for record in records {
        wins.push(record.calc_wins());
    }

    let wins_product = wins.into_iter().product::<u64>();

    println!("wins: {:?}", wins_product);

    println!("Time elapsed: {:?}", start.elapsed());
}

pub fn puzzle2() {
    let start = Instant::now();
    println!("Day 6, puzzle 2");

    let input = utils::file::read_input("src/day06/input.txt");
    let lines = input.lines().collect::<Vec<&str>>();

    let time = lines[0].split(':').collect::<Vec<&str>>()[1]
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();

    let distance = lines[1].split(':').collect::<Vec<&str>>()[1]
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();

    let record = Record { time, distance };

    println!("wins: {:?}", record.calc_wins());

    println!("Time elapsed: {:?}", start.elapsed());
}
