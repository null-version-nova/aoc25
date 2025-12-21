use std::{collections::BTreeSet, fs::read_to_string, thread};

fn add_buttons(
    first: &(BTreeSet<usize>, usize),
    second: &(BTreeSet<usize>, usize),
) -> (BTreeSet<usize>, usize) {
    let mut set = BTreeSet::new();
    for i in &first.0 {
        set.insert(*i);
    }
    for i in &second.0 {
        if set.contains(i) {
            set.remove(i);
        } else {
            set.insert(*i);
        }
    }
    (set, first.1 + second.1)
}

struct Machine {
    diagram: BTreeSet<usize>,
    buttons: Vec<(BTreeSet<usize>, usize)>,
    already_combined: Vec<(usize, usize)>,
    joltage: Vec<usize>,
    initial_button_count: usize,
}

impl Machine {
    pub fn new(
        diagram: BTreeSet<usize>,
        buttons: Vec<(BTreeSet<usize>, usize)>,
        joltage: Vec<usize>,
    ) -> Self {
        let initial_button_count = buttons.len();
        Machine {
            diagram,
            buttons,
            already_combined: vec![],
            joltage,
            initial_button_count,
        }
    }

    pub fn diagram(&self) -> &BTreeSet<usize> {
        &self.diagram
    }

    pub fn buttons(&self) -> &Vec<(BTreeSet<usize>, usize)> {
        &self.buttons
    }

    pub fn joltage(&self) -> &Vec<usize> {
        &self.joltage
    }

    pub fn check(&self, button: &(BTreeSet<usize>, usize)) -> Option<usize> {
        if button.0 == self.diagram {
            Some(button.1)
        } else {
            None
        }
    }

    pub fn combine_all(&mut self) -> Option<usize> {
        let mut candidates = usize::MAX;
        for first in 0..self.initial_button_count {
            for second in 0..self.buttons.len() {
                // println!("Combining buttons {} and {}", first, second);
                if self.already_combined.contains(&(first, second)) {
                    continue;
                }
                if self.already_combined.contains(&(second, first)) {
                    continue;
                }
                if second == first {
                    continue;
                }
                println!("Combining buttons {} and {}", first, second);
                let combined = add_buttons(&self.buttons[first], &self.buttons[second]);
                let result = self.check(&combined);
                if result.is_some() {
                    if candidates > result.unwrap() {
                        candidates = result.unwrap();
                    }
                }
                self.buttons.push(combined);
                self.already_combined.push((first, second));
            }
        }
        if candidates < usize::MAX {
            Some(candidates)
        } else {
            None
        }
    }
}

pub fn part1() {
    let mut machines = {
        let input = read_to_string("./inputs/10.test.txt").unwrap();
        let mut machines = vec![];
        for line in input.lines() {
            let items = Vec::from_iter(line.split_whitespace());
            let lights_str = items[0].trim_matches(|c| c == '[' || c == ']');
            let joltage_str = items
                .last()
                .unwrap()
                .trim_matches(|c| c == '{' || c == '}')
                .split(',');
            let mut buttons_str = vec![];
            for i in 1..(items.len() - 1) {
                buttons_str.push(items[i].trim_matches(|c| c == '(' || c == ')').split(','));
            }
            let mut lights = BTreeSet::new();
            for i in lights_str.char_indices() {
                if i.1 == '#' {
                    lights.insert(i.0);
                }
            }
            let mut buttons = vec![];
            for button_str in buttons_str {
                let mut button = BTreeSet::new();
                for element in button_str {
                    button.insert(element.parse().expect("This is a number!"));
                }
                buttons.push((button, 1));
            }
            let mut joltage = vec![];
            for i in joltage_str {
                joltage.push(i.parse().expect("This is a number!"));
            }
            machines.push(Machine::new(lights, buttons, joltage));
        }
        machines
    };
    let mut total = 0;
    thread::scope(|s| {
        let mut results = vec![];
        for mut machine in machines {
            let result = s.spawn(move || {
                loop {
                    let result = machine.combine_all();
                    if result.is_some() {
                        // println!("Result for this machine is {}", result.unwrap());
                        return result.unwrap();
                    }
                }
            });
            results.push(result);
        }
        let mut counter = 0;
        for i in results {
            total += i.join().unwrap();
            println!("Machine {} has finished", counter);
            counter += 1;
        }
    });
    println!("Total result is {}", total);
}

pub fn part2() {}
