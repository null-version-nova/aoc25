use crate::read_from_file;

fn char_to_int(input: char) -> u64 {
    input
        .to_digit(10)
        .unwrap_or_else(|| {
            println!(
                "Tried to convert to number but input is {} which does not work!",
                input
            );
            panic!();
        })
        .into()
}

pub fn part1() {
    let file = read_from_file("./inputs/3.txt");
    let banks = file.lines();
    let mut maximum_joltage: u64 = 0;
    for bank in banks {
        let mut most: u64 = 0;
        let mut most_index: usize = 0;
        for battery in bank.char_indices() {
            let size = char_to_int(battery.1);
            if size > most {
                most = size;
                most_index = battery.0;
            }
        }
        let mut second_most: u64 = 0;
        let mut second_most_index: usize = 0;
        for battery in bank.char_indices() {
            let size = char_to_int(battery.1);
            if most_index == bank.len() - 1 {
                if size >= second_most && battery.0 != most_index {
                    second_most = size;
                    second_most_index = battery.0;
                }
            } else {
                if size >= second_most && battery.0 > most_index {
                    second_most = size;
                    second_most_index = battery.0;
                }
            }
        }
        let size = if second_most_index < most_index {
            second_most * 10 + most
        } else {
            most * 10 + second_most
        };
        maximum_joltage += size;
        println!("The joltage of bank {} is {}!", bank, size);
    }
    println!("The total output joltage is {}!", maximum_joltage);
}

#[derive(Debug)]
struct SplitFailure {
    failure_point: u64,
}

fn find_most_and_split(
    bank: &str,
    size_desired: usize,
    failure_point: u64,
) -> Result<(u64, &str), SplitFailure> {
    let mut most: u64 = 0;
    let mut most_index: usize = 0;
    for battery in bank.char_indices() {
        let joltage = char_to_int(battery.1);
        if joltage > most && joltage < failure_point {
            most = joltage;
            most_index = battery.0;
        }
    }
    if (bank.len() - most_index) >= size_desired {
        let split = bank.split_at_checked(most_index + 1).unwrap_or(("", "")).1;
        // println!("Split bank {} into {} and {}", bank, most, split);
        return Ok((most, split));
    } else {
        return Err(SplitFailure {
            failure_point: most,
        });
    }
}

fn iterate_split(bank: &str, size_desired: usize, pending_result: u64, failure_point: u64) -> u64 {
    let result = find_most_and_split(bank, size_desired, failure_point);
    println!(
        "Split bank into {} with result {} with {} remaining and failure point {}...",
        bank, pending_result, size_desired, failure_point
    );
    if result.is_ok() {
        let res = result.unwrap();
        if size_desired > 0 {
            return iterate_split(res.1, size_desired - 1, pending_result * 10 + res.0, 10);
        } else {
            return pending_result;
        }
    } else {
        let res = result.unwrap_err();
        return iterate_split(bank, size_desired, pending_result, res.failure_point);
    }
}

pub fn part2() {
    let file = read_from_file("./inputs/3.txt");
    let banks = file.lines();
    let mut maximum_joltage: u64 = 0;
    for bank in banks {
        let joltage = iterate_split(bank, 12, 0, 10);
        maximum_joltage += joltage;
        println!(
            "Bank analysis shows output of {} and maximum output of {}!",
            joltage, maximum_joltage
        );
    }
    println!("The total output joltage is {}!", maximum_joltage);
}
