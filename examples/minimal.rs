use orbgame::prelude::*;

fn main() {
    let mut game = Game::default();
    game
        .create_window()
        .bounds((100.0, 100.0, 420.0, 730.0))
        .title("OrbGame - minimal example")
        .root(TextBlock::create().text("OrbGame").into())
        .debug_flag(false)
        .build();
    game.run();
}
