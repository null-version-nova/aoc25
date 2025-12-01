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
        None | Some(_) => {
            println!("Invalid input, exiting!");
        }
    }
}
