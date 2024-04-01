mod game;
mod cell;

use game::Game;
use cell::cell;


pub fn main() -> iced::Result {
    iced::run("Custom Widget - Iced", Game::update, Game::view)
}

