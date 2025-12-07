use std::fs::read_to_string;

#[derive(Clone, Copy, PartialEq)]
enum ManifoldObject {
    Splitter,
    Empty,
    Beam,
}

use ManifoldObject::*;

fn advance_beam(
    manifold: &mut Vec<Vec<ManifoldObject>>,
    location: (isize, usize),
    ignore_left: bool,
    ignore_right: bool,
    counter: &mut usize,
) -> usize {
    let (x, y) = location;
    if x < 0 || x as usize >= manifold.first().unwrap().len() {
        return 0;
    }
    let x: usize = x.try_into().unwrap();
    if y == manifold.len() - 1 {
        return 0;
    }
    if manifold[y + 1][x] == Splitter {
        *counter += 1;
        if !ignore_right {
            manifold[y][x] = Empty;
            advance_beam(manifold, (x as isize + 1, y), true, false, counter);
        }
        if !ignore_left {
            manifold[y][x] = Empty;
            advance_beam(manifold, (x as isize - 1, y), false, true, counter);
        }
    } else {
        manifold[y][x] = Empty;
        manifold[y + 1][x] = Beam;
    } // Don't need to check for anything more, beams only ever occupy one row!
    0
}

fn print_manifold(manifold: &Vec<Vec<ManifoldObject>>) {
    for row in manifold {
        for item in row {
            print!(
                "{}",
                match item {
                    Splitter => "^",
                    Empty => ".",
                    Beam => "|",
                }
            )
        }
        println!("");
    }
}

fn advance_beams(manifold: &mut Vec<Vec<ManifoldObject>>) -> usize {
    let mut counter = 0;
    for y in 0..manifold.len() {
        print_manifold(&manifold);
        for x in 0..manifold.len() {
            let target = manifold
                .get(y)
                .expect("Should stop before the last row?")
                .get(x);
            if target.is_none() {
                continue;
            }
            let target = target.unwrap();
            if *target == Beam {
                counter += advance_beam(
                    manifold,
                    (x.try_into().unwrap(), y),
                    false,
                    false,
                    &mut counter,
                );
            }
        }
    }
    counter
}

pub fn part1() {
    let mut manifold = {
        let input: String = read_to_string("./inputs/7.txt").expect("You forgot the file!");
        let mut manifold: Vec<Vec<ManifoldObject>> = vec![];
        for line in input.lines() {
            let mut row = vec![];
            for char in line.char_indices() {
                match char.1 {
                    'S' => {
                        row.push(Beam);
                    }
                    '^' => {
                        row.push(Splitter);
                    }
                    '.' => {
                        row.push(Empty);
                    }
                    _ => {
                        panic!(
                            "Help! I only know the meaning of three UTF-8 characters! I'll never be literate at this rate..."
                        );
                    }
                }
            }
            manifold.push(row);
        }
        manifold
    };
    let counter = advance_beams(&mut manifold);
    println!("{} beams escaped!", counter);
}

#[derive(PartialEq)]
enum QuantumManifoldObject {
    Splitter(Option<u64>),
    Empty,
    Beam,
}

fn advance_beam_quantum(
    manifold: &mut Vec<Vec<QuantumManifoldObject>>,
    location: (usize, usize),
    counter: usize,
) -> u64 {
    let (x, y) = location;
    let x: usize = x.try_into().unwrap();
    if y == manifold.len() - 1 {
        return 1;
    }
    match manifold[y + 1][x] {
        QuantumManifoldObject::Splitter(amount) => {
            return amount.unwrap_or_else(|| {
                let left = advance_beam_quantum(manifold, (x - 1, y), counter);
                let right = advance_beam_quantum(manifold, (x + 1, y), counter);
                manifold[y + 1][x] = QuantumManifoldObject::Splitter(Some(left + right));
                return left + right;
            });
        }
        _ => {
            return advance_beam_quantum(manifold, (x, y + 1), counter);
        }
    };
}

pub fn part2() {
    let mut start = 0;
    let mut manifold = {
        let input: String = read_to_string("./inputs/7.txt").expect("You forgot the file!");
        let mut manifold = vec![];
        for line in input.lines() {
            let mut row = vec![];
            for char in line.char_indices() {
                match char.1 {
                    'S' => {
                        row.push(QuantumManifoldObject::Beam);
                        start = char.0;
                    }
                    '^' => {
                        row.push(QuantumManifoldObject::Splitter(None));
                    }
                    '.' => {
                        row.push(QuantumManifoldObject::Empty);
                    }
                    _ => {
                        panic!(
                            "Help! I only know the meaning of three UTF-8 characters! I'll never be literate at this rate..."
                        );
                    }
                }
            }
            manifold.push(row);
        }
        manifold
    };
    let counter = advance_beam_quantum(&mut manifold, (start, 0), 0);
    println!("Amount of timelines present is {}", counter);
}
