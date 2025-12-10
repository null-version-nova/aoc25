use std::fs::read_to_string;

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
    distance: Vec<i64>,
    up_first: bool,
    circumference: usize,
}

impl Manifold {
    pub fn new(corners: Vec<(i64, i64)>) -> Self {
        let mut distance = vec![];
        let up_first = corners[0].0 == corners[1].1;
        let mut is_vertical = up_first;
        for i in 0..(corners.len() - 1) {
            let curr_distance = if is_vertical {
                corners[i + 1].0 - corners[i].0
            } else {
                corners[i + 1].1 - corners[i].1
            };
            distance.push(curr_distance);
            is_vertical = !is_vertical;
        }
        let curr_distance = if is_vertical {
            corners.last().unwrap().1 - corners[0].1
        } else {
            corners.last().unwrap().0 - corners[0].0
        };
        distance.push(curr_distance);
        let mut circumference = 0;
        for i in &distance {
            // println!("Adding {} to {}", *i, circumference);
            circumference += i.abs();
        }
        println!(
            "Manifold created with circumference {}, with {} corners, and with {} distances",
            circumference,
            corners.len(),
            distance.len()
        );
        Manifold {
            corners,
            distance,
            up_first,
            circumference: circumference as usize,
        }
    }

    pub fn len(&self) -> usize {
        self.circumference as usize
    }

    pub fn index(&self, index: usize) -> (i64, i64) {
        let idx = index;
        let mut is_vertical = self.up_first;
        let mut starting_point = self.corners[0];
        let mut iterator = self.distance.iter();
        loop {
            let distance = *iterator.next().unwrap();
            if idx > (distance.abs() as usize) {
                if is_vertical {
                    starting_point.1 += distance;
                } else {
                    starting_point.0 += distance;
                }
                is_vertical = !is_vertical;
            } else {
                let amount = if distance.is_negative() {
                    -(idx as i64)
                } else {
                    idx as i64
                };
                if is_vertical {
                    starting_point.1 += amount;
                } else {
                    starting_point.0 += amount;
                }
                return starting_point;
            }
        }
    }

    pub fn get_area(&self, first: usize, second: usize) -> Option<i64> {
        let first_tile = self.index(first);
        let mut left1 = false;
        let mut right1 = false;
        let mut up1 = false;
        let mut down1 = false;
        let second_tile = self.index(second);
        let mut left2 = false;
        let mut right2 = false;
        let mut up2 = false;
        let mut down2 = false;
        for idx in first..=second {
            let tile = self.index(idx);
            if tile.0 > first_tile.0 {
                right1 = true;
            } else if tile.0 < first_tile.0 {
                left1 = true;
            }
            if tile.1 > first_tile.1 {
                up1 = true;
            } else if tile.1 < first_tile.1 {
                down1 = true;
            }
            if tile.0 > second_tile.0 {
                right2 = true;
            } else if tile.0 < second_tile.0 {
                left2 = true;
            }
            if tile.1 > second_tile.1 {
                up2 = true;
            } else if tile.1 < second_tile.1 {
                down2 = true;
            }
        }
        if left1 && right1 && up1 && down1 {
            return None;
        }
        if left2 && right2 && up2 && down2 {
            return None;
        }
        Some(
            ((second_tile.0 - first_tile.0).abs() + 1)
                * ((second_tile.1 - first_tile.1 + 1).abs() + 1),
        )
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
            println!("At index {} and {}", first, second);
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
