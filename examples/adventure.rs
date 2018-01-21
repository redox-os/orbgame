extern crate orbgame;

use std::env;

use orbgame::GameBuilder;

fn main() {
    let game_builder = GameBuilder::new("examples/adventure/game.ron");
    game_builder.build().expect("Could not load game").exec();
}
