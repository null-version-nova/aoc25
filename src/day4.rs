use std::fs::read_to_string;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Space {
    Paper,
    Available,
    Removed,
    Unavailable,
    Empty,
}

struct FullSpace {
    pub full_space: Vec<Vec<Space>>,
}

impl FullSpace {
    pub fn new(space: Vec<Vec<Space>>) -> Self {
        FullSpace { full_space: space }
    }

    pub fn scan(&mut self, index: (usize, usize)) -> Result<bool, Box<dyn std::error::Error>> {
        let mut amount = 0;
        if self.full_space[index.0][index.1] == Space::Empty
            || self.full_space[index.0][index.1] == Space::Removed
        {
            return Ok(false);
        }
        let mut did_something_happen: bool = false;
        for i in -1..=1 {
            for j in -1..=1 {
                let space: Option<&mut Space> = self.get(
                    i + TryInto::<i32>::try_into(index.0)?,
                    j + TryInto::<i32>::try_into(index.1)?,
                );
                if space.is_some() {
                    let sp = space.unwrap();
                    if *sp != Space::Empty || *sp != Space::Removed {
                        amount += 1;
                    }
                } else {
                    continue;
                }
            }
        }
        if amount < 5 {
            self.full_space[index.0][index.1] = Space::Available;
            did_something_happen = true;
        } else {
            self.full_space[index.0][index.1] = Space::Unavailable;
        }
        Ok(did_something_happen)
    }

    fn get(&mut self, x: i32, y: i32) -> Option<&mut Space> {
        self.full_space
            .get_mut(TryInto::<usize>::try_into(x).ok()?)?
            .get_mut(TryInto::<usize>::try_into(y).ok()?)
    }

    pub fn remove(&mut self) -> bool {
        let mut has_anything_happened = false;
        for i in &mut self.full_space {
            for j in i {
                if *j == Space::Available {
                    *j = Space::Removed;
                    has_anything_happened = true;
                }
            }
        }
        has_anything_happened
    }
}

fn turn_to_data(file: &str) -> Vec<Vec<Space>> {
    let mut vec = Vec::new();
    for line in file.lines() {
        let mut vector = Vec::new();
        for char in line.chars() {
            match char {
                '.' => {
                    vector.push(Space::Empty);
                }
                '@' => {
                    vector.push(Space::Paper);
                }
                _ => {
                    panic!("This wasn't a valid character!")
                }
            }
        }
        vec.push(vector);
    }
    vec
}

pub fn part1() {
    let file = read_to_string("./inputs/4.txt").expect("oh no the table its broken");
    let mut output = FullSpace::new(turn_to_data(file.as_str()));
    for i in 0..output.full_space.len() {
        for j in 0..output.full_space[i].len() {
            output.scan((i, j)).expect("Uh oh!");
        }
    }
    let mut total_available = 0;
    for i in output.full_space {
        total_available += i.iter().filter(|j| **j == Space::Available).count();
    }
    println!("Total available is {}!", total_available);
}

fn recursively_remove(space: &mut FullSpace) {
    for i in 0..space.full_space.len() {
        for j in 0..space.full_space[i].len() {
            space.scan((i, j)).expect("Uh oh!");
        }
    }
    if space.remove() {
        recursively_remove(space);
    }
}

pub fn part2() {
    let file = read_to_string("./inputs/4.txt").expect("oh no the table its broken");
    let mut output = FullSpace::new(turn_to_data(file.as_str()));
    recursively_remove(&mut output);
    let mut total_available = 0;
    for i in output.full_space {
        total_available += i.iter().filter(|j| **j == Space::Removed).count();
    }
    println!("Total available is {}!", total_available);
}
