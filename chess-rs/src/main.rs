extern crate termion;
extern crate uuid;

#[macro_use]
extern crate lazy_static;

mod game;
mod game_state;
mod knight;

use std::process::Command;

fn main() {
    let mut the_game = game::Game::new();
    println!("{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), the_game.state);
    the_game.take_action( "D2-D4");
    println!("{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), the_game.state);
    the_game.take_action( "D7-D5");
    println!("{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), the_game.state);
    the_game.take_action( "C2-C4");
    println!("{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), the_game.state);
    the_game.take_action( "E7-E6");
    println!("{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), the_game.state);
    the_game.take_action( "C4-D5");
    println!("{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), the_game.state);
    the_game.take_action( "C7-C5");
    println!("{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), the_game.state);
    the_game.take_action( "D5-en-passantC6-C5");
    println!("{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), the_game.state);
    the_game.take_action( "A7-A5");
    println!("{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), the_game.state);
    the_game.take_action( "E2-E3");
    println!("{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), the_game.state);
    the_game.take_action( "A5-A4");
    println!("{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), the_game.state);
    the_game.take_action( "B2-B4");
    println!("{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), the_game.state);
}
