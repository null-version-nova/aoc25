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

fn find_most_and_split(bank: &str, size_desired: usize) -> (u64, &str) {
    let mut most: u64 = 0;
    let mut most_index: usize = 0;
    for battery in bank.char_indices() {
        let joltage = char_to_int(battery.1);
        if joltage > most {
            most = joltage;
            most_index = battery.0;
        }
    }
    if (bank.len() - most_index) >= size_desired {
        let split = bank.split_at(most_index).1;
        // println!("Split bank {} into {} and {}", bank, most, split);
        return (most, split);
    }
    todo!();
}

fn iterate_split(bank: &str, size_desired: usize, pending_result: u64) -> u64 {
    let result = find_most_and_split(bank, size_desired);
    println!("Split bank into {} with result {}", bank, pending_result);
    if size_desired > 1 {
        iterate_split(result.1, size_desired - 1, pending_result * 10 + result.0)
    } else {
        pending_result
    }
}

pub fn part2() {
    let file = read_from_file("./inputs/3.txt");
    let banks = file.lines();
    let mut maximum_joltage: u64 = 0;
    for bank in banks {
        let mut joltage: u64;
        joltage = iterate_split(bank, 12, 0);
        maximum_joltage += joltage;
    }
    println!("The total output joltage is {}!", maximum_joltage);
}
