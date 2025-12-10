use std::fs::read_to_string;

fn get_area(first: (i64, i64), second: (i64, i64)) -> i64 {
    ((second.0 - first.0) * (second.1 - first.1)).abs()
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
    println!("The results are {} and {}", result_1,result_2);
}

pub fn part2() {}
