extern crate orbgame;

use std::env;

use orbgame::Game;

fn main() {
    let args: Vec<String> = env::args().collect();
    Game::from_ron(&args[1][..]).expect("Could not load game").exec();
}
