#[derive(PartialEq, Debug)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    pub fn ordinal(&self) -> usize{
        match self {
            Direction::N => 0,
            Direction::E => 1,
            Direction::S => 2,
            Direction::W => 3,
        }
    }

    pub fn from_ordinal(ordinal: usize) -> Option<Direction> {
        match ordinal {
            0 => Some(Direction::N),
            1 => Some(Direction::E),
            2 => Some(Direction::S),
            3 => Some(Direction::W),
            _ => None
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {

    pub fn new(x: usize, y: usize) -> Coordinate {
        Coordinate {
            x,
            y,
        }
    }

}

#[derive(PartialEq, Debug)]
pub struct Cursor {

    pub direction: Direction,
    pub coordinate: Coordinate,

}

impl Cursor {
    pub fn new() -> Cursor {
        Cursor {
            direction: Direction::N,
            coordinate: Coordinate::new(0, 0),
        }
    }

    pub fn move_forward(&mut self) {
        match self.direction {
            Direction::N => self.coordinate.y += 1,
            Direction::E => self.coordinate.x += 1,
            Direction::S => self.coordinate.y -= 1,
            Direction::W => self.coordinate.x -= 1,
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction
    }

    pub fn turn_right(&mut self) {
        self.set_direction(Direction::from_ordinal((1 + &self.direction.ordinal()) % 3).unwrap())
    }

    pub fn turn_left(&mut self) {
        self.set_direction(Direction::from_ordinal((2 + &self.direction.ordinal()) % 3).unwrap())
    }
}