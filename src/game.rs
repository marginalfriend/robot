use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Write};
use itertools::Itertools;
use crate::board::create_board;

pub fn start_game() {
    let play_board = create_board();
    let mut cursor = play_board.cursor;
    let mut board = play_board.board;

    let guide = "How to play : \
    W : Move forward\
    D : Turn Right\
    A : Turn Left";

    enable_raw_mode().expect("TODO: panic message");

    println!("Press any key to see its code. Press 'q' to quit.");

    loop {
        for x in &board {
            println!("{}", x.iter().format(", "));
            println!(" ");
        }

        if event::poll(std::time::Duration::from_secs(1)).expect("") {

            if let Ok(Event::Key(key_event)) = event::read() {

                match key_event.code {
                    KeyCode::Char('q') => {
                        println!("Thanks for playing :D");
                        break;
                    }
                    KeyCode::Char('w') => {
                        cursor.move_forward()
                    }
                    KeyCode::Char('d') => {
                        cursor.turn_right()
                    }
                    KeyCode::Char('a') => {
                        cursor.turn_left()
                    }
                    _ => {
                        continue
                    }
                }
            }
        }
    }

    // Disable raw mode before exiting
    disable_raw_mode().expect("TODO: panic message");
}