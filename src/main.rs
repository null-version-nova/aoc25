use std::{env::args, fs::File, io::Read};

mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub fn read_from_file(path: &str) -> String {
    let mut file = File::open(path).expect("input load failed :(");
    let mut text = String::new();
    file.read_to_string(&mut text)
        .expect("Reading from string failed :(");
    text
}

fn main() {
    let mut arg = args();
    arg.next(); // We don't need the PWD...
    match arg.next().as_ref().map(|i| i.as_str()) {
        Some("1-1") => {
            println!("Running AOC 2025 Day 1");
            day1::part1();
        }
        Some("1-2") => {
            println!("Running AOC 2025 Day 1 part 2");
            day1::part2();
        }
        Some("2-1") => {
            println!("Running AOC 2025 Day 2 part 1");
            day2::part1();
        }
        Some("2-2") => {
            println!("Running AOC 2025 Day 2 part 2");
            day2::part2();
        }
        Some("3-1") => {
            println!("Running AOC 2025 Day 3 part 1");
            day3::part1();
        }
        Some("3-2") => {
            println!("Running AOC 2025 Day 3 part 2");
            day3::part2();
        }
        Some("4-1") => {
            println!("Running AOC 2025 Day 4 part 1");
            day4::part1();
        }
        Some("4-2") => {
            println!("Running AOC 2025 Day 4 part 2");
            day4::part2();
        }
        Some("5-1") => {
            println!("Running AOC 2025 Day 5 part 1");
            day5::part1();
        }
        Some("5-2") => {
            println!("Running AOC 2025 Day 5 part 2");
            day5::part2();
        }
        Some("6-1") => {
            println!("Running AOC 2025 Day 6 part 1");
            day6::part1();
        }
        Some("6-2") => {
            println!("Running AOC 2025 Day 6 part 2");
            day6::part2();
        }
        Some("7-1") => {
            println!("Running AOC 2025 Day 7 part 1");
            day7::part1();
        }
        Some("7-2") => {
            println!("Running AOC 2025 Day 7 part 2");
            day7::part2();
        }
        Some("8-1") => {
            println!("Running AOC 2025 Day 8 part 1");
            day8::part1();
        }
        Some("8-2") => {
            println!("Running AOC 2025 Day 8 part 2");
            day8::part2();
        }
        Some("9-1") => {
            println!("Running AOC 2025 Day 9 part 1");
            day9::part1();
        }
        Some("9-2") => {
            println!("Running AOC 2025 Day 9 part 2");
            day9::part2();
        }
        Some("10-1") => {
            println!("Running AOC 2025 Day 10 part 1");
            day10::part1();
        }
        Some("10-2") => {
            println!("Running AOC 2025 Day 10 part 2");
            day10::part2();
        }
        Some("11-1") => {
            println!("Running AOC 2025 Day 11 part 1");
            day11::part1();
        }
        Some("11-2") => {
            println!("Running AOC 2025 Day 11 part 2");
            day11::part2();
        }
        Some("12-1") => {
            println!("Running AOC 2025 Day 12 part 1");
            day12::part1();
        }
        Some("12-2") => {
            println!("Running AOC 2025 Day 12 part 2");
            day12::part2();
        }
        _ => {
            println!("Invalid input, exiting!");
        }
    }
}
