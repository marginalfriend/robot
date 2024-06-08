use crate::cursor::{Cursor, Direction};

pub struct PlayBoard {
    pub cursor: Cursor,
    pub board: Vec<Vec<char>>
}

pub fn create_board() -> PlayBoard {
    let cursor = Cursor::new();

    let capacity = 15;
    let mut board: Vec<Vec<char>> = Vec::with_capacity(capacity);

    for x in capacity {
        let row: Vec<char> = Vec::with_capacity(capacity);
        board.push(row);
    }

    for x in &mut board {
        for mut y in x {
            y = &mut ' ';
        }
    }

    let direction = match cursor.direction {
        Direction::N() => '^',
        Direction::E() => '>',
        Direction::S() => 'v',
        Direction::W() => '<',
    };

    let x = cursor.coordinate.x;
    let y = cursor.coordinate.y;

    board[y][x] = direction;

    PlayBoard {
        cursor,
        board
    }
}
