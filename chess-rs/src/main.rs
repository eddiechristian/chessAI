extern crate termion;
extern crate uuid;

#[macro_use]
extern crate lazy_static;

mod game;
mod game_state;
mod knight;
mod bishop;
mod rook;
mod king;

use std::process::Command;
use game_state::{PlayerColor};
use std::str;

fn main() {
    let display_available_moves = true;

    let mut the_game = game::Game::new();
    let move_sequence = vec![
        "D7-D5",
        "C2-C4",
        "H7-H6",
        "C4-D5",
        "C7-C5",
        "D5-en-passantC6-C5",
        "C8-F5",
        "A2-A3",
        "D8-D7",
        "D2-D3",
        "B8-A6",
        "B1-C3"
    ];
    for this_move in &move_sequence {
        let id = the_game.state.get_id();
        if display_available_moves {
            print!("{}{}{}\n{}",
                termion::clear::All,
                termion::cursor::Goto(1, 1),
                the_game.state,
                match the_game.state.player_turn {
                    PlayerColor::WHITE => {
                        "WHITE's turn:"
                    },
                    PlayerColor::BLACK => {
                        "BLACK's turn:"
                    },
                });
            the_game.print_allowed_moves();

        } else {
            print!("{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), the_game.state);

        }
        the_game.take_action( this_move);
    }
    if display_available_moves {
        print!("{}{}{}\n{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            the_game.state,
            match the_game.state.player_turn {
                PlayerColor::WHITE => {
                    "WHITE's turn:"
                },
                PlayerColor::BLACK => {
                    "BLACK's turn:"
                },
            });
        the_game.print_allowed_moves();

    } else {
        print!("{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), the_game.state);

    }
}
