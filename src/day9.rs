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
    // In quadratic time you could just check the areas of all of them and get the biggest one. I did a quadratic time solution yesterday though, and it's too obvious and boring.
    let red_tiles = {
        // They have been sorted by hypotenuse! No clue if this will work, mind you!
        red_tiles.sort_by(|first, second| {
            (first.0 * first.0 + first.1 * first.1)
                .cmp(&(second.0 * second.0 + second.1 * second.1))
        });
        red_tiles
    };
    let result = get_area(*red_tiles.first().unwrap(), *red_tiles.last().unwrap());
    println!("The result is {}", result);
}

pub fn part2() {}
