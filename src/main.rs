use std::io::stdin;

mod robot;
mod grid;
mod direction;
mod instruction;

use instruction::parse_instruction;
use robot::Robot;
use grid::Grid;

fn main() {
    let grid = Grid::new(5,5);
    let mut robot = Robot::new(grid);
    let mut input = stdin();
    loop {
        let mut instruction = String::new();
        let _ = input.read_line(&mut instruction).unwrap();
        robot.send(parse_instruction(instruction.trim()).unwrap());
    }
}
