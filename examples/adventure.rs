extern crate orbgame;

use std::env;

use orbgame::Game;

fn main() {
    Game::from_toml("res/adventure/game.toml").exec();
}
