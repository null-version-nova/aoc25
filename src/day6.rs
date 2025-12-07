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

pub fn part2() {
    let problems = {
        let input = read_to_string("./inputs/6.txt").expect("File should exist");
        let mut lines = input.lines();
        let mut row1 = lines.next().expect("Row should exist.").chars();
        let mut row2 = lines.next().expect("Row should exist.").chars();
        let mut row3 = lines.next().expect("Row should exist.").chars();
        let mut row4 = lines.next().expect("Row should exist.").chars();
        let mut row5 = lines.next().expect("Row should exist.").chars();
        let mut problems: Vec<(Vec<u32>, bool)> = vec![];
        let mut problem: (Vec<u32>, bool) = (vec![], false);
        for idx in 0..row1.clone().count() {
            let chars = [
                row1.next().expect("We're not exceeding here!"),
                row2.next().expect("We're not exceeding here!"),
                row3.next().expect("We're not exceeding here!"),
                row4.next().expect("We're not exceeding here!"),
                row5.next().expect("We're not exceeding here!"),
            ];
            if chars[4] != ' ' {
                // print_number_rep(&problem, idx);
                if problem.0.len() == 0 && idx != 0 {
                    println!("Something is wrong with the last problem!");
                }
                problems.push(problem);
                problem = (vec![], chars[4] == '*');
            }
            let mut number = 0;
            for char in &chars[0..4] {
                match char {
                    ' ' => {}
                    _ => {
                        number *= 10;
                        number += char.to_digit(10).expect("This should be a digit.");
                    }
                }
            }
            if number != 0 {
                problem.0.push(number);
            }
        }
        problems.push(problem);
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
