use direction::{Direction, parse_direction};

pub struct Place {
    pub x: i64,
    pub y: i64,
    pub dir: Direction
}

pub enum Instruction {
    Place(Place),
    Move,
    Left,
    Right,
    Report
}

pub fn parse_instruction(input: &str) -> Result<Instruction, &'static str> {
    let v: Vec<&str> = input.split(" ").collect();
    match v[0] {
        "PLACE" => {
            let args: Vec<&str> = v[1].split(',').collect();
            if args.len() != 3 {
                return Err("Invalid PLACE instruction, wrong number of arguments")
            };
            let dir = try!(parse_direction(args[2]));
            let x = match args[0].parse() {
                Ok(i) => i,
                Err(_) => {
                    return Err("Invalid x coordinate in PLACE instruction")
                }
            };
            let y = match args[1].parse() {
                Ok(i) => i,
                Err(_) => {
                    return Err("Invalid y coordinate in PLACE instruction")
                }
            };
            Ok(Instruction::Place(Place { x: x, y: y, dir: dir }))
        },
        "MOVE" => Ok(Instruction::Move),
        "REPORT" => Ok(Instruction::Report),
        "LEFT" => Ok(Instruction::Left),
        "RIGHT" => Ok(Instruction::Right),
        _ => Err("Invalid instruction")
    }
}
