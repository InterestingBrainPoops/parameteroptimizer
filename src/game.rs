pub struct Game {
    pub snakes: Vec<Snake>,
    pub food: Vec<Coordinate>,
    pub width: usize,
    pub height: usize,
    pub turn: u64,
}

pub struct Coordinate {
    x: u32,
    y: u32,
}

pub struct Snake {
    id: u8,
    body: Vec<Coordinate>,
    health: u8,
}

pub struct Move {
    id: u8,
    snake_move: Direction,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
