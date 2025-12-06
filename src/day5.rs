use std::{
    cmp::{max, min},
    ops::RangeInclusive,
};

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

fn join_range<T>(first: &RangeInclusive<T>, second: &RangeInclusive<T>) -> Option<RangeInclusive<T>>
where
    T: Ord + Copy,
{
    if second.contains(first.end()) || first.contains(second.end()) {
        return Some(min(*first.start(), *second.start())..=max(*first.end(), *second.end()));
    }
    None
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
            start_subsumed = true;
        }
        if other_range.contains(consol_range.end()) {
            consol_range = join_range(&consol_range, other_range).unwrap();
            end_subsumed = true;
        }
        if start_subsumed && end_subsumed && (range != other_range) {
            return None;
        }
    }
    Some(consol_range)
}

enum ConsolData {
    Changed(usize, RangeInclusive<u64>),
    Unchanged,
    Removed(usize),
}

fn consolidate_ranges(ranges: &Vec<RangeInclusive<u64>>) -> ConsolData {
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
            return ConsolData::Changed(idx, consol_range);
        }
    }
    ConsolData::Unchanged
}

fn check_overlap(ranges: &Vec<RangeInclusive<u64>>) -> bool {
    for range in ranges {
        for other_range in ranges {
            if range != other_range && other_range.contains(range.start()) {
                return true;
            }
        }
    }
    false
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
    ranges.sort_by(|a, b| a.start().cmp(b.start()));
    let mut counter = 10;
    println!("Overlap check yields result: {}!", check_overlap(&ranges));
    loop {
        println!("Starting pass...");
        let result = consolidate_ranges(&ranges);
        match result {
            ConsolData::Removed(index) => {
                ranges.remove(index);
            }
            ConsolData::Unchanged => {
                if counter > 0 {
                    counter -= 1;
                } else {
                    break;
                }
            }
            ConsolData::Changed(idx, new) => {
                ranges[idx] = new;
            }
        }
    }
    println!("Overlap check yields result: {}!", check_overlap(&ranges));
    let mut size: u64 = 0;
    for idx in 0..ranges.len() {
        println!(
            "{}: {}-{}, size of {}, added to {}",
            idx,
            ranges[idx].start(),
            ranges[idx].end(),
            ranges[idx].end() - ranges[idx].start() + 1,
            size
        );
        size += ranges[idx].end() - ranges[idx].start() + 1;
    }
    println!("There are {} fresh items!", size);
}
