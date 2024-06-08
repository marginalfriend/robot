#[derive(PartialEq, Debug)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(PartialEq, Debug)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {

    pub fn new(x: i32, y: i32) -> Coordinate {
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
            coordinate: Coordinate::new(0,0),
        }
    }

    pub fn move_cursor(&mut self, direction: Direction) {
        if direction == self.direction {
            match direction {
                Direction::N => self.coordinate.y += 1,
                Direction::E => self.coordinate.x += 1,
                Direction::S => self.coordinate.y -= 1,
                Direction::W => self.coordinate.x -= 1,
            }
        } else {
            self.direction = direction
        }
    }

}