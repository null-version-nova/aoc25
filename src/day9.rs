use std::fs::read_to_string;

fn get_area(first: (i64, i64), second: (i64, i64)) -> i64 {
    let answer = ((second.0 - first.0 + 1) * (second.1 - first.1 + 1)).abs();
    println!(
        "Between {}-{} and {}-{} the area is {}",
        first.0, first.1, second.0, second.1, answer
    );
    answer
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
    let mut most_area = 0;
    for tile1 in &red_tiles {
        for tile2 in &red_tiles {
            let area = get_area(*tile1, *tile2);
            if area > most_area {
                most_area = area;
            }
        }
    }
    println!("The most area is {}", most_area);
}

pub fn part2() {}
