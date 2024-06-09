use crate::cursor::{Cursor, Direction};

pub struct PlayBoard {
    pub cursor: Cursor,
    pub board: Vec<Vec<char>>
}

pub fn create_board() -> PlayBoard {
    let cursor = Cursor::new();

    let mut board: Vec<Vec<char>> = Vec::with_capacity(15);

    for x in 0..15 {
        let mut row = Vec::with_capacity(15);

        for i in 0..15 {
            row.push(' ')
        }

        board.push(row)
    }

    let direction = match cursor.direction {
        Direction::N => '^',
        Direction::E => '>',
        Direction::S => 'v',
        Direction::W => '<',
    };

    let x = cursor.coordinate.x;
    let y = cursor.coordinate.y;

    board[y + 7][x + 7] = direction;

    PlayBoard {
        cursor,
        board
    }
}
