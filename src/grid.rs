pub struct Grid {
    x: i64,
    y: i64
}

impl Grid {
    pub fn new(x: i64, y: i64) -> Grid {
        Grid {
            x: x,
            y: y
        }
    }

    pub fn in_bounds(&self, x: i64, y: i64) -> bool {
        if self.x > x && self.y > y {
            return true
        }
        return false
    }
}
