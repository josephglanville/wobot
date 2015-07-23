use std::fmt;
use direction::Direction::*;

pub enum Direction {
    North,
    East,
    South,
    West
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            North => write!(f, "NORTH"),
            East => write!(f, "EAST"),
            South => write!(f, "SOUTH"),
            West => write!(f, "WEST")
        }
    }
}

pub fn parse_direction(input: &str) -> Result<Direction, &'static str> {
    match input {
        "NORTH" => Ok(North),
        "EAST" => Ok(East),
        "SOUTH" => Ok(South),
        "WEST" => Ok(West),
        _ => Err("Invalid direction")
    }
}
