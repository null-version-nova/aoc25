use std::env::args;

mod day1;

fn main() {
    let mut arg = args();
    arg.next();
    let day = arg.next();
    let dayref = day.as_ref();
    match dayref.map(|i| i.as_str()) {
        Some("1") => {
            println!("Running AOC 2025 Day 1");
            day1::run();
        }
        Some("2") => {
            println!("Running AOC 2025 Day 2");
        }
        Some("3") => {
            println!("Running AOC 2025 Day 3");
        }
        Some("4") => {
            println!("Running AOC 2025 Day 4");
        }
        Some("5") => {
            println!("Running AOC 2025 Day 5");
        }
        Some("6") => {
            println!("Running AOC 2025 Day 6");
        }
        Some("7") => {
            println!("Running AOC 2025 Day 7");
        }
        Some("8") => {
            println!("Running AOC 2025 Day 8");
        }
        Some("9") => {
            println!("Running AOC 2025 Day 9");
        }
        Some("10") => {
            println!("Running AOC 2025 Day 10");
        }
        Some("11") => {
            println!("Running AOC 2025 Day 11");
        }
        Some("12") => {
            println!("Running AOC 2025 Day 12");
        }
        None | Some(_) => {
            println!("Invalid input, exiting!");
        }
    }
}
