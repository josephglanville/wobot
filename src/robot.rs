use grid::Grid;
use direction::Direction;
use direction::Direction::*;
use instruction::Instruction;
use instruction::Instruction::*;

pub struct Robot {
    x: i64,
    y: i64,
    grid: Grid,
    dir: Direction
}

impl Robot {
    pub fn new(grid: Grid) -> Robot {
        Robot {
            grid: grid,
            x: 0,
            y: 0,
            dir: North
        }
    }
    
    pub fn place(&mut self, x: i64, y: i64, dir: Direction) {
        if self.grid.in_bounds(x, y) {
            self.y = y;
            self.x = x;
            self.dir = dir;
        }
        // What to do if invalid placement?
    }

    pub fn right(&mut self) {
        self.dir = match self.dir {
            North => East,
            East  => South,
            South => West,
            West  => North
        }
    }

    pub fn left(&mut self) {
        self.dir = match self.dir {
            North => West,
            West  => South,
            South => East,
            East  => North
        }
    }

    pub fn step(&mut self) {
        match self.dir {
            North => { self.update_pos(0, 1) }
            West  => { self.update_pos(-1, 0) }
            South => { self.update_pos(0, -1) }
            East  => { self.update_pos(1, 0) }
        }
    }

    fn update_pos(&mut self, x: i64, y: i64) {
        if self.grid.in_bounds(self.x + x, self.y + x) {
            self.x = self.x + x;
            self.y = self.y + y;
        }
    }

    pub fn report(&self) {
        println!("{},{},{}", self.x, self.y, self.dir)
    }

    pub fn send(&mut self, ins: Instruction) {
        match ins {
            Place(p) => self.place(p.x, p.y, p.dir),
            Right => self.right(),
            Left => self.left(),
            Move => self.step(),
            Report => self.report()
        }
    }
}
