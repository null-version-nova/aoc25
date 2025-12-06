use std::ops::RangeInclusive;

use crate::read_from_file;

pub fn part1() {
    let input = read_from_file("./inputs/5.txt");
    let mut ranges: Vec<RangeInclusive<u64>> = vec![];
    let mut ids: Vec<u64> = vec![];
    let mut is_range = true;
    for i in input.lines() {
        if i.len() == 0 {
            is_range = false;
            continue;
        }
        if is_range {
            let result = i.split_once('-').expect("Not a range?");
            ranges.push(result.0.parse().unwrap()..=result.1.parse().unwrap());
        } else {
            ids.push(i.parse().expect("ID did not parse as integer correctly!"));
        }
    }
    let mut amount_fresh: u64 = 0;
    'i: for id in ids {
        for range in &ranges {
            if range.contains(&id) {
                amount_fresh += 1;
                continue 'i;
            }
        }
    }
    println!("Total amount fresh is {}!", amount_fresh);
}

fn consolidate_range(
    ranges: &Vec<RangeInclusive<u64>>,
    range: &RangeInclusive<u64>,
) -> Option<RangeInclusive<u64>> {
    let mut consol_range = range.clone();
    for other_range in ranges {
        let mut start_subsumed = false;
        let mut end_subsumed = false;
        if other_range.contains(consol_range.start()) {
            consol_range = *other_range.start()..=*consol_range.end();
            start_subsumed = true;
        }
        if other_range.contains(consol_range.end()) {
            consol_range = *consol_range.start()..=*other_range.end();
            end_subsumed = true;
        }
        if other_range.start() - 1 == *consol_range.end() {
            consol_range = *consol_range.start()..=*other_range.end();
        }
        if start_subsumed && end_subsumed && (range != other_range) {
            return None;
        }
    }
    Some(consol_range)
}

enum ConsolData {
    Changed(bool),
    Removed(usize),
}

fn consolidate_ranges(ranges: &mut Vec<RangeInclusive<u64>>) -> ConsolData {
    for idx in 0..ranges.len() {
        let consol_range = consolidate_range(ranges, &ranges[idx]);
        if consol_range.is_none() {
            println!(
                "At index {} redundant range {}-{} is removed!",
                idx,
                ranges[idx].start(),
                ranges[idx].end()
            );
            return ConsolData::Removed(idx);
        }
        let consol_range = consol_range.expect("we just checked it");
        if consol_range != ranges[idx] {
            println!(
                "At index {} range {}-{} is changed to {}-{}",
                idx,
                ranges[idx].start(),
                ranges[idx].end(),
                consol_range.start(),
                consol_range.end()
            );
            ranges[idx] = consol_range;
            return ConsolData::Changed(true);
        }
    }
    ConsolData::Changed(false)
}

pub fn part2() {
    let input = read_from_file("./inputs/5.txt");
    let mut ranges: Vec<RangeInclusive<u64>> = vec![];
    for i in input.lines() {
        if i.len() == 0 {
            break;
        }
        let result = i.split_once('-').expect("Not a range?");
        ranges.push(result.0.parse().unwrap()..=result.1.parse().unwrap());
    }
    let mut counter = 10;
    loop {
        println!("Starting pass...");
        let result = consolidate_ranges(&mut ranges);
        match result {
            ConsolData::Removed(index) => {
                ranges.remove(index);
            }
            ConsolData::Changed(false) => {
                if counter > 0 {
                    counter -= 1;
                } else {
                    break;
                }
            }
            _ => {}
        }
    }
    let mut size: u64 = 0;
    for idx in 0..ranges.len() {
        println!("{}: {}-{}", idx, ranges[idx].start(), ranges[idx].end());
        size += (ranges[idx].end() - ranges[idx].start()) + 1;
    }
    println!("There are {} fresh items!", size);
}
