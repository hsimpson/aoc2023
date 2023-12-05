mod utils {
    pub mod file;
}

mod day01;
mod day02;

fn main() {
    println!("Advent of Code 2023");
    day01::puzzle1();
    day01::puzzle2();

    println!();
    day02::puzzle1();
    day02::puzzle2();
}
