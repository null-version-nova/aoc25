use std::fs::read_to_string;

pub fn part1() {
    let problems = {
        let input = read_to_string("./inputs/6.txt").expect("File should exist");
        let mut lines = input.lines();
        let row1 = lines
            .next()
            .expect("First row should exist.")
            .split_whitespace();
        let row2 = lines
            .next()
            .expect("Second row should exist.")
            .split_whitespace();
        let row3 = lines
            .next()
            .expect("Third row should exist.")
            .split_whitespace();
        let row4 = lines
            .next()
            .expect("Fourth row should exist.")
            .split_whitespace();
        let row5 = lines
            .next()
            .expect("Fifth row should exist.")
            .split_whitespace();
        let mut numbers1: Vec<i32> = vec![];
        for num in row1 {
            numbers1.push(num.parse().expect("String not a number"));
        }
        let mut numbers2: Vec<i32> = vec![];
        for num in row2 {
            numbers2.push(num.parse().expect("String not a number"));
        }
        let mut numbers3: Vec<i32> = vec![];
        for num in row3 {
            numbers3.push(num.parse().expect("String not a number"));
        }
        let mut numbers4: Vec<i32> = vec![];
        for num in row4 {
            numbers4.push(num.parse().expect("String not a number"));
        }
        let mut operators: Vec<bool> = vec![];
        for num in row5 {
            operators.push(num == "*");
        }
        if numbers1.len() != numbers2.len() {
            println!(
                "Something is very wrong! {} and {} should be the same number!",
                numbers1.len(),
                numbers2.len()
            );
        }
        let mut problems: Vec<(Vec<i32>, bool)> = vec![];
        for idx in 0..numbers1.len() {
            let mut numbers: Vec<i32> = vec![];
            numbers.push(numbers1[idx]);
            numbers.push(numbers2[idx]);
            numbers.push(numbers3[idx]);
            numbers.push(numbers4[idx]);
            problems.push((numbers, operators[idx]));
        }
        problems
    };
    let mut sum = 0;
    for problem in problems {
        let mut solution: i64;
        if problem.1 {
            solution = 1;
            for num in problem.0 {
                solution *= Into::<i64>::into(num);
            }
        } else {
            solution = 0;
            for num in problem.0 {
                solution += Into::<i64>::into(num);
            }
        }
        sum += solution;
    }
    println!("Total sum is {}!", sum);
}

fn line_to_numbers(line: &str) -> Vec<String> {
    let mut output = vec![];
    let mut prev_whitespace = true;
    let mut temp_string: String = String::new();
    for char in line.chars() {
        if char == ' ' && prev_whitespace == false {
            prev_whitespace = true;
            output.push(temp_string.clone());
            // println!("Pushing string \"{}\"!", temp_string);
            temp_string.clear();
            continue;
        }
        if char != ' ' {
            prev_whitespace = false;
        }
        temp_string.push(char);
    }
    output.push(temp_string);
    output
}

fn print_string_rep(
    r1: &Vec<String>,
    r2: &Vec<String>,
    r3: &Vec<String>,
    r4: &Vec<String>,
    idx: usize,
) {
    println!("String rep idx: {}", idx);
    println!("\"{}\"", r1[idx]);
    println!("\"{}\"", r2[idx]);
    println!("\"{}\"", r3[idx]);
    println!("\"{}\"", r4[idx]);
}

fn print_number_rep(numbers: &Vec<(Vec<u32>, bool)>, idx: usize) {
    println!(
        "At index {} a problem with mult-value {} contains these numbers:",
        idx, numbers[idx].1
    );
    for i in &numbers[idx].0 {
        println!("{}", i)
    }
}

pub fn part2() {
    let problems = {
        let input = read_to_string("./inputs/6.txt").expect("File should exist");
        let mut lines = input.lines();
        let row1 = line_to_numbers(lines.next().expect("Row should exist."));
        let row2 = line_to_numbers(lines.next().expect("Row should exist."));
        let row3 = line_to_numbers(lines.next().expect("Row should exist."));
        let row4 = line_to_numbers(lines.next().expect("Row should exist."));
        let row5 = lines
            .next()
            .expect("Fifth row should exist.")
            .split_whitespace();
        let mut operators: Vec<bool> = vec![];
        for num in row5 {
            operators.push(num == "*");
        }
        if row1.len() != row2.len() {
            println!(
                "Something is very wrong! {} and {} should be the same number!",
                row1.len(),
                row2.len()
            );
        }
        if row1.len() != operators.len() {
            println!(
                "Something is very wrong! {} and {} should be the same number!",
                row1.len(),
                operators.len()
            );
        }
        let mut problems: Vec<(Vec<u32>, bool)> = vec![];
        for idx in 0..row1.len() {
            let mut numbers: Vec<u32> = vec![];
            let mut counter = 0;
            loop {
                let mut all_none = true;
                let number_chars = [
                    row1[idx].chars().nth(counter),
                    row2[idx].chars().nth(counter),
                    row3[idx].chars().nth(counter),
                    row4[idx].chars().nth(counter),
                ];
                let mut number: u32 = 0;
                for char in number_chars {
                    match char {
                        Some(' ') => {}
                        Some(c) => {
                            all_none = false;
                            number *= 10;
                            number += c.to_digit(10).expect("Nothing except spaces and digits!");
                        }
                        None => {}
                    }
                }
                if all_none {
                    break;
                }
                numbers.push(number);
                counter += 1;
            }
            // println!("Problem {} is...", idx);
            // for i in &numbers {
            //     println!("{}", i);
            // }
            if numbers.len() == 0 {
                print_string_rep(&row1, &row2, &row3, &row4, idx);
                println!("Something is strange at index {}!", idx);
            }
            problems.push((numbers, operators[idx]));
        }
        // print_number_rep(&problems, 990);
        problems
    };
    let mut sum = 0;
    for problem in problems {
        let mut solution: i64;
        if problem.1 {
            solution = 1;
            for num in problem.0 {
                solution *= Into::<i64>::into(num);
            }
        } else {
            solution = 0;
            for num in problem.0 {
                solution += Into::<i64>::into(num);
            }
        }
        sum += solution;
    }
    println!("Total sum is {}!", sum);
}
