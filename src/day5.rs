use std::{
    cmp::{max, min},
    ops::RangeInclusive, vec,
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
    println!("Overlap check yields result: {}!", check_overlap(&ranges));
    let mut unique: Vec<RangeInclusive<u64>> = vec![];
    for range in &ranges {
        if let Some(last) = unique.last_mut() {
            if let Some(joined) = join_range(last, &range) {
                *last = joined;
                continue;
            }
        }
        unique.push(range.clone());
    }
    println!("Overlap check yields result: {}!", check_overlap(&unique));
    let mut size: u64 = 0;
    for idx in 0..unique.len() {
        println!(
            "{}: {}-{}, size of {}, added to {}",
            idx,
            unique[idx].start(),
            unique[idx].end(),
            unique[idx].end() - unique[idx].start() + 1,
            size
        );
        size += unique[idx].end() - unique[idx].start() + 1;
    }
    println!("There are {} fresh items!", size);
}
