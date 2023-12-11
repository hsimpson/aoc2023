mod utils {
    pub mod file;
}

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() {
    println!("Advent of Code 2023");
    day01::puzzle1();
    day01::puzzle2();

    println!();
    day02::puzzle1();
    day02::puzzle2();

    println!();
    day03::puzzle1();
    day03::puzzle2();

    println!();
    day04::puzzle1();
    day04::puzzle2();

    println!();
    day05::puzzle1();
    day05::puzzle2();
}
