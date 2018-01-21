extern crate orbgame;

use std::env;

use orbgame::GameBuilder;

fn main() {
    let args: Vec<String> = env::args().collect();
    let game_builder = GameBuilder::new(&args[1][..]);
    game_builder.build().expect("Could not load game").exec();
}
