use std::{collections::BTreeSet, fs::read_to_string};

#[derive(Clone)]
struct JunctionBox {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub connected: Vec<usize>,
    furthest_connected_distance: i64,
}

impl JunctionBox {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        JunctionBox {
            x,
            y,
            z,
            connected: vec![],
            furthest_connected_distance: 0,
        }
    }
    pub fn get_difference(&self, other: &JunctionBox) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
    pub fn get_closest(&self, other: &Vec<JunctionBox>) -> Option<(usize, i64)> {
        let mut closest = None;
        let mut is_this_close = i64::MAX;
        for i in 0..other.len() {
            let distance = self.get_difference(&other[i]);
            if distance < is_this_close && distance > self.furthest_connected_distance {
                is_this_close = distance;
                closest.replace((i, distance));
            }
        }
        closest
    }
    pub fn connect(first: usize, second: usize, boxes: &mut Vec<JunctionBox>, distance: i64) {
        println!("Connecting boxes {} and {}", first, second);
        boxes[first].connected.push(second);
        boxes[second].connected.push(first);
        if boxes[first].furthest_connected_distance < distance {
            boxes[first].furthest_connected_distance = distance;
        }
        if boxes[second].furthest_connected_distance < distance {
            boxes[second].furthest_connected_distance = distance;
        }
    }
}

fn scan_purge_add(
    index: usize,
    list: &Vec<JunctionBox>,
    site: &mut BTreeSet<usize>,
    circuit: &mut Vec<usize>,
) -> bool {
    if !site.contains(&index) {
        println!("Skipping index {}", index);
        return false;
    }
    println!("Consolidating index {}", index);
    site.remove(&index);
    circuit.push(index);
    for idx in &list[index].connected {
        scan_purge_add(*idx, list, site, circuit);
    }
    println!("Finished consolidating index {}", index);
    true
}

pub fn part1() {
    let mut boxes = {
        let input = read_to_string("./inputs/8.txt")
            .expect("Runtime environment reports that you forgot the file you moron!");
        let mut boxes = vec![];
        for line in input.lines() {
            let mut line = line.split(',');
            boxes.push(JunctionBox::new(
                line.next().unwrap().parse().unwrap(),
                line.next().unwrap().parse().unwrap(),
                line.next().unwrap().parse().unwrap(),
            ));
        }
        boxes
    };
    for _ in 0..1000 {
        let mut mostest_closest: Option<(usize, usize)> = None;
        let mut closest_distance = i64::MAX;
        for idx in 0..boxes.len() {
            println!("Finding closest to {}", idx);
            let closest = boxes[idx]
                .get_closest(&boxes)
                .expect("There isn't a closest box somehow...");
            if closest_distance > closest.1 {
                closest_distance = closest.1;
                mostest_closest.replace((idx, closest.0));
            }
        }
        if mostest_closest.is_some() {
            let (first, second) = mostest_closest.unwrap();
            JunctionBox::connect(first, second, &mut boxes, closest_distance);
        } else {
            println!("Something's going wrong...");
        }
    }
    let box_list = boxes;
    let mut boxes = BTreeSet::from_iter(0..box_list.len());
    let mut circuits: Vec<Vec<usize>> = vec![];
    while !boxes.is_empty() {
        // Due to the condition we can verify that boxes.first() is Some(_)
        let mut circuit = vec![];
        let idx = *boxes.first().unwrap();
        scan_purge_add(idx, &box_list, &mut boxes, &mut circuit);
        circuits.push(circuit);
    }
    circuits.sort_by(|first, second| second.len().cmp(&first.len()));
    println!(
        "The answer is {}",
        circuits[0].len() * circuits[1].len() * circuits[2].len()
    );
}

pub fn part2() {}
