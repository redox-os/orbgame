extern crate orbgame;

use std::env;

use orbgame::Game;

fn main() {
    Game::from_toml("examples/adventure/game.toml").exec();
}
 