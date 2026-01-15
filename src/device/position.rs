pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn move_x(&mut self, delta: i32) {
        self.x += delta;
    }

    pub fn move_y(&mut self, delta: i32) {
        self.y += delta;
    }
    
    pub fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}
