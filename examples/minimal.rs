use orbgame::prelude::*;

struct GameView;

impl Widget for GameView {
    fn create() -> Template {
        Template::default()
            .as_parent_type(ParentType::Single)
            .with_debug_name("GameView")
            .with_child(
                Container::create()
                    .as_parent_type(ParentType::Single)
                    .with_child(TextBlock::create().with_property(Label::from("OrbGame"))),
            )
    }
}

fn main() {
    let mut game = Game::default();
    game.create_window()
        .with_bounds(Bounds::new(0, 0, 420, 730))
        .with_title("OrbGame - minimal example")
        .with_root(GameView::create())
        .with_debug_flag(true)
        .build();
    game.run();
}
