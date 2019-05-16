use orbgame::prelude::*;

fn main() {
    Game::new()
        .window(|ctx| {
            Window::create()
                .title("OrbGame - minimal example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(TextBlock::create().text("OrbGame").build(ctx))
                .build(ctx)
        })
        .run();
}
