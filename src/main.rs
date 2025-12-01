use std::env::args;

mod day1;

fn main() {
    let mut arg = args();
    arg.next();
    let day = arg.next();
    let dayref = day.as_ref();
    match dayref.map(|i| i.as_str()) {
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
        }
        Some("3-1") => {
            println!("Running AOC 2025 Day 3 part 1");
        }
        Some("4-1") => {
            println!("Running AOC 2025 Day 4 part 1");
        }
        Some("5-1") => {
            println!("Running AOC 2025 Day 5 part 1");
        }
        Some("6-1") => {
            println!("Running AOC 2025 Day 6 part 1");
        }
        Some("7-1") => {
            println!("Running AOC 2025 Day 7 part 1");
        }
        Some("8-1") => {
            println!("Running AOC 2025 Day 8 part 1");
        }
        Some("9-1") => {
            println!("Running AOC 2025 Day 9 part 1");
        }
        Some("10-1") => {
            println!("Running AOC 2025 Day 10 part 1");
        }
        Some("11-1") => {
            println!("Running AOC 2025 Day 11 part 1");
        }
        Some("12-1") => {
            println!("Running AOC 2025 Day 12 part 1");
        }
        None | Some(_) => {
            println!("Invalid input, exiting!");
        }
    }
}
