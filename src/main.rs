extern crate orbgame;

use std::env;

use orbgame::Game;

fn main() {
    let args: Vec<String> = env::args().collect();
    Game::from_toml(&args[1][..]).exec();
}
