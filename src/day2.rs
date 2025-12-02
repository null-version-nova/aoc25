use std::ops::RangeInclusive;

use crate::read_from_file;

fn get_ids_from_file(file: &str) -> impl Iterator<Item = RangeInclusive<i64>> {
    file.split(',').filter_map(|item| {
        let split = item.trim().split_once('-')?;
        let start = split.0.parse::<i64>().expect("Range parse failed!");
        let end = split.1.parse::<i64>().expect("End range parse failed!");
        println!(
            "\nInput {} split into first {} and last {}",
            item, start, end
        );
        Some(start..=end)
    })
}

fn check_repeat_pt1(input: &str) -> bool {
    let length = input.len();
    for i in 0..length {
        let (first, second) = input.split_at(i);
        if first == second {
            // println!(
            //     "With input {} split into {} and {}, they are equal!",
            //     input, first, second
            // );
            return true;
        }
    }
    false
}

fn check_repeat_pt2(input: &str) -> bool {
    let length = input.len();
    for i in 1..length {
        if length % i == 0 {
            // is the candidate section divisible within the host even?
            let candidate = input.split_at(i).0;
            let matches = input.matches(candidate);
            if length / i == matches.count() {
                return true;
            }
        }
    }
    false
}

pub fn part1() {
    let file = read_from_file("./inputs/2.txt");
    let ids = get_ids_from_file(file.as_str());
    let mut sum: i64 = 0;
    for id in ids {
        for i in id {
            if check_repeat_pt1(i.to_string().as_str()) {
                sum += i;
                println!("Adding repeat {} to sum to produce {}", i, sum);
            }
        }
    }
    println!("Result is {}!", sum);
}

pub fn part2() {
    let file = read_from_file("./inputs/2.test.txt");
    let ids = get_ids_from_file(file.as_str());
    let mut sum: i64 = 0;
    for id in ids {
        for i in id {
            if check_repeat_pt2(i.to_string().as_str()) {
                sum += i;
                println!("Adding repeat {} to sum to produce {}", i, sum);
            }
        }
    }
    println!("Result is {}!", sum);
}
