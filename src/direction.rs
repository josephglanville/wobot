use std::fmt;

pub enum Direction {
    North,
    East,
    South,
    West
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Direction::North => write!(f, "NORTH"),
            Direction::East => write!(f, "EAST"),
            Direction::South => write!(f, "SOUTH"),
            Direction::West => write!(f, "WEST")
        }
    }
}

pub fn parse_direction(input: &str) -> Result<Direction, &'static str> {
    match input {
        "NORTH" => Ok(Direction::North),
        "EAST" => Ok(Direction::East),
        "SOUTH" => Ok(Direction::South),
        "WEST" => Ok(Direction::West),
        _ => Err("Invalid direction")
    }
}
