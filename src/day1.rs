// Contains both day 1 answers!

use std::str::FromStr;

use crate::read_from_file;

pub fn rotate(dial: i32, amount: i32) -> (i32, i32) {
    let mut result = dial + amount;
    let mut zeroes = 0;
    let mut start_at_zero = dial == 0;
    while result < 0 {
        result += 100;
        if start_at_zero {
            start_at_zero = false; // eek!
        } else {
            zeroes += 1;
        }
    }
    while result > 99 {
        result -= 100;
        zeroes += 1;
    }
    if amount < 0 && result == 0 {
        zeroes += 1;
    }
    (result, zeroes)
}

pub fn read_to_iterator(text: &str) -> impl Iterator<Item = i32> {
    text.split('\n').filter_map(|line| {
        let mut result = 1;
        if line.contains('R') {
            result *= 1;
        } else if line.contains('L') {
            result *= -1;
        } else {
            return None;
        }
        let number = line.get(1..)?;
        result *= i32::from_str(number).ok()?;
        // println!("{} read as {}", line, result);
        Some(result)
    })
}

pub fn part1() {
    let file = read_from_file("./inputs/1.txt");
    let lines = read_to_iterator(file.as_str());

    let mut dial = 50;
    let mut zeroes = 0;
    for line in lines {
        dial = rotate(dial, line).0;
        if dial == 0 {
            zeroes += 1;
        }
        println!(
            "Dial rotated by {} to {}, {} zeroes found...",
            line, dial, zeroes
        );
    }
    println!("Dial reads {}!", dial);
    println!("{} zeroes found!", zeroes);
}

pub fn part2() {
    let file = read_from_file("./inputs/1.txt");
    let lines = read_to_iterator(file.as_str());

    let mut dial = 50;
    let mut zeroes = 0;
    let mut rotation: (i32, i32);
    for line in lines {
        rotation = rotate(dial, line);
        dial = rotation.0;
        zeroes += rotation.1;
        println!(
            "Dial rotated by {} to {}, {} zeroes found...",
            line, dial, zeroes
        );
    }
    println!("Dial reads {}!", dial);
    println!("{} zeroes found!", zeroes);
}
