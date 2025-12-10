use std::{collections::BTreeSet, fs::read_to_string};

fn get_area(first: (i64, i64), second: (i64, i64)) -> i64 {
    ((second.0 - first.0 + 1) * (second.1 - first.1 + 1)).abs()
}

pub fn part1() {
    let mut red_tiles = {
        let input = read_to_string("./inputs/9.txt").unwrap();
        let mut red_tiles = vec![];
        for line in input.lines() {
            let (x, y) = line.split_once(",").unwrap();
            red_tiles.push((x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()));
        }
        red_tiles
    };
    let mut height = 0;
    let mut width = 0;
    for tile in &red_tiles {
        if tile.0 > width {
            width = tile.0;
        }
        if tile.1 > height {
            height = tile.1;
        }
    }
    // In quadratic time you could just check the areas of all of them and get the biggest one. I did a quadratic time solution yesterday though, and it's too obvious and boring.
    red_tiles.sort_by(|first, second| {
        (first.0 * first.0 + first.1 * first.1).cmp(&(second.0 * second.0 + second.1 * second.1))
    });
    let lowest_left = *red_tiles.first().unwrap();
    let highest_right = *red_tiles.last().unwrap();
    red_tiles.sort_by(|first, second| {
        (first.0 * first.0 + (first.1 - height) * (first.1 - height))
            .cmp(&(second.0 * second.0 + (second.1 - height) * (second.1 - height)))
    });
    let highest_left = *red_tiles.first().unwrap();
    let lowest_right = *red_tiles.last().unwrap();
    let result_1 = get_area(lowest_left, highest_right);
    let result_2 = get_area(highest_left, lowest_right);
    println!("The result is {}", result_1.max(result_2));
}

struct Manifold {
    corners: Vec<(i64, i64)>,
    corner_directions: Vec<(bool, bool)>,
    distance: Vec<i64>,
    up_first: bool,
    circumference: usize,
    relevant_indices: Option<Vec<Vec<(i64, i64)>>>,
}

impl Manifold {
    pub fn new(corners: Vec<(i64, i64)>) -> Self {
        let mut distance = vec![];
        let up_first = corners[0].0 == corners[1].0;
        let mut is_vertical = up_first;
        let mut corner_directions = vec![];
        let curr_distance = if is_vertical {
            corners[1].1 - corners[0].1
        } else {
            corners[1].0 - corners[0].0
        };
        distance.push(curr_distance);
        if is_vertical {
            corner_directions.push((false, curr_distance.is_positive()));
        } else {
            corner_directions.push((curr_distance.is_positive(), false));
        }
        is_vertical = !is_vertical;
        for i in 1..(corners.len() - 1) {
            let curr_distance = if is_vertical {
                corners[i + 1].1 - corners[i].1
            } else {
                corners[i + 1].0 - corners[i].0
            };
            distance.push(curr_distance);
            if is_vertical {
                corner_directions
                    .push((distance[i - 1].is_positive(), curr_distance.is_positive()));
            } else {
                corner_directions
                    .push((curr_distance.is_positive(), distance[i - 1].is_positive()));
            }
            is_vertical = !is_vertical;
        }
        let curr_distance = if is_vertical {
            corners.last().unwrap().1 - corners[0].1
        } else {
            corners.last().unwrap().0 - corners[0].0
        };
        distance.push(curr_distance);
        if is_vertical {
            corner_directions.push((
                distance[corners.len() - 1].is_positive(),
                curr_distance.is_positive(),
            ));
        } else {
            corner_directions.push((
                curr_distance.is_positive(),
                distance[corners.len() - 1].is_positive(),
            ));
        }
        if is_vertical {
            corner_directions[0].1 = curr_distance.is_positive();
        } else {
            corner_directions[0].0 = curr_distance.is_positive();
        }
        let mut circumference = 0;
        for i in &distance {
            // println!("Adding {} to {}", *i, circumference);
            circumference += i.abs();
        }
        println!(
            "Manifold created with circumference {}, with {} corners, with {} corner directions, and with {} distances",
            circumference,
            corners.len(),
            corner_directions.len(),
            distance.len()
        );
        let mut output = Manifold {
            corners,
            corner_directions,
            distance,
            up_first,
            circumference: circumference as usize,
            relevant_indices: None,
        };
        output.compute_relevant_indices();
        output
    }

    pub fn len(&self) -> usize {
        self.corners.len()
    }


    pub fn index(&self, index: usize) -> (i64, i64) {
        self.corners[index]
    }

    pub fn get_area(&self, first: usize, second: usize) -> Option<i64> {
        let first_tile = self.index(first);
        let second_tile = self.index(second);

        let x_maximum = first_tile.0.max(second_tile.0);
        let x_minimum = first_tile.0.min(second_tile.0);
        let y_maximum = first_tile.1.max(second_tile.1);
        let y_minimum = first_tile.1.min(second_tile.1);

        if self.corner_directions[first] == self.corner_directions[second] {
            println!(
                "Square ({},{})-({},{}) is bounded by like corners, which invalidates it",
                first_tile.0, first_tile.1, second_tile.0, second_tile.1
            );
            return None;
        }

        for idx in 0..self.corners.len() {
            let span = &self.relevant_indices.as_ref().unwrap()[idx];
            for tile in span {
                if tile.0 < x_maximum
                    && tile.0 > x_minimum
                    && tile.1 < y_maximum
                    && tile.1 > y_minimum
                {
                    println!(
                        "Tile {}-{} intersects the square of ({},{})-({},{}) and thus disqualifies it",
                        tile.0, tile.1, first_tile.0, first_tile.1, second_tile.0, second_tile.1
                    );
                    return None;
                }
            }
        }
        let result = Some(
            (((second_tile.0.abs_diff(first_tile.0)) + 1)
                * (second_tile.1.abs_diff(first_tile.1) + 1)) as i64,
        );
        println!(
            "At index {} ({}-{}) and {} ({}-{}) with result {}",
            first,
            first_tile.0,
            first_tile.1,
            second,
            second_tile.0,
            second_tile.1,
            result.unwrap()
        );
        result
    }

    fn compute_relevant_indices(&mut self) {
        println!("Computing relevant indices...");
        let mut x_coordinates = BTreeSet::new();
        let mut y_coordinates = BTreeSet::new();
        for corner in &self.corners {
            x_coordinates.insert(corner.0);
            y_coordinates.insert(corner.1);
        }
        let mut relevant_indices: Vec<Vec<(i64, i64)>> = vec![];
        let mut tracker = self.corners[0];
        let mut is_vertical = self.up_first;
        for span in &self.distance {
            let mut current_span = vec![];
            let step = if span.is_negative() { -1 } else { 1 };
            if is_vertical {
                for _ in 0..span.abs() {
                    tracker.1 += step;
                    if y_coordinates.contains(&tracker.1) {
                        current_span.push(tracker);
                    } else if y_coordinates.contains(&(tracker.1 + 1)) {
                        current_span.push(tracker);
                    } else if y_coordinates.contains(&(tracker.1 - 1)) {
                        current_span.push(tracker);
                    }
                }
            } else {
                for _ in 0..span.abs() {
                    tracker.0 += step;
                    if x_coordinates.contains(&tracker.0) {
                        current_span.push(tracker);
                    } else if x_coordinates.contains(&(tracker.0 + 1)) {
                        current_span.push(tracker);
                    } else if x_coordinates.contains(&(tracker.0 - 1)) {
                        current_span.push(tracker);
                    }
                }
            }
            is_vertical = !is_vertical;
            relevant_indices.push(current_span);
        }
        let length = {
            let mut length = 0;
            for i in &relevant_indices {
                length += i.len();
            }
            length
        };
        println!(
            "Relevant indices computed with a length of {} compared to the original length of {}!",
            length, self.circumference
        );
        let _ = self.relevant_indices.insert(relevant_indices);
    }
}

pub fn part2() {
    let red_tiles = {
        let input = read_to_string("./inputs/9.txt").unwrap();
        let mut red_tiles = vec![];
        for line in input.lines() {
            let (x, y) = line.split_once(",").unwrap();
            red_tiles.push((x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()));
        }
        Manifold::new(red_tiles)
    };
    let mut most_area = 0;
    for first in 0..red_tiles.len() {
        // This on the other hand is probably worse than quadratic time...
        for second in 0..red_tiles.len() {
            // println!("At index {} and {}", first, second,);
            let area = red_tiles.get_area(first, second);
            if area.is_some() {
                if area.unwrap() > most_area {
                    most_area = area.unwrap();
                }
            }
        }
    }
    println!("The result is {}", most_area);
}
