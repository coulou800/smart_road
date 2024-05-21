use sdl2::rect::Point;

#[derive(Debug, PartialEq)]
pub enum CardinalPoint {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq)]
pub enum Lane {
    Left,
    Right,
    Center,
}

pub struct Vehicle {
    pub position: Point,
    pub from: CardinalPoint,
    pub lane: Lane,
    pub speed: f64,
}

impl Vehicle {
    pub fn new(from: CardinalPoint) -> Self {
        Self {
            position: todo!(),
            from,
            speed: todo!(),
            lane: todo!(),
        }
    }
}
