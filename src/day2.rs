use std::fmt::Error;

use crate::read_from_file;

struct Id {
    lead: String,
    trail: String,
}

impl TryFrom<String> for Id {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let split = value.split_once('-').ok_or(Error)?;
        Ok(Id {
            lead: String::from(split.0),
            trail: String::from(split.1),
        })
    }
}

pub fn part1() {
    let file = read_from_file("./inputs/2.txt");
}

pub fn part2() {}
