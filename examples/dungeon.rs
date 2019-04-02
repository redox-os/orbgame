use orbgame::prelude::*;
use std::cell::Cell;

#[derive(Default)]
pub struct MenuViewState {
    start_game: Cell<bool>,
    close_game: Cell<bool>,
}

impl MenuViewState {
    fn start_game(&self) {
        println!("Start game");
        self.start_game.set(true);
    }

    fn quit_game(&self) {
        println!("Quit game");
        self.close_game.set(true);
    }
}

impl State for MenuViewState {
    fn update(&self, context: &mut Context<'_>) {
        if self.close_game.get() {
            context.send_message("game_view", StringMessage::from("quit"));
        }

        if self.start_game.get() {
            context.send_message("game_view", StringMessage::from("start"));
        }
    }
}

widget!(
        MenuView<MenuViewState> {
             selector: Selector
        }
    );

impl Template for MenuView {
    fn template(self, id: Entity, context: &mut BuildContext) -> Self {
        let state = self.clone_state();
        let ng_state = state.clone();
        let q_state = state.clone();

        self.name("MenuView")
            .selector(Selector::default().id("menu_view"))
            .child(
                Grid::create()
                    .selector(Selector::from("grid").class("start"))
                    .child(
                        Container::create()
                            .padding(16.0)
                            .selector(Selector::from("container").class("menu"))
                            .vertical_alignment("Center")
                            .horizontal_alignment("Center")
                            .child(
                                Stack::create()
                                    .child(
                                        TextBlock::create()
                                            .selector(Selector::from("textblock").class("h1"))
                                            .text("Dungeon")
                                            .horizontal_alignment("Center")
                                            .build(context),
                                    )
                                    .child(
                                        Button::create()
                                            .margin((0.0, 16.0, 0.0, 0.0))
                                            .text("Start Game")
                                            .on_click(move |_| {
                                                ng_state.start_game();
                                                true
                                            })
                                            .build(context),
                                    )
                                    .child(
                                        Button::create()
                                            .margin((0.0, 8.0, 0.0, 0.0))
                                            .text("Quit")
                                            .on_click(move |_| {
                                                q_state.quit_game();
                                                true
                                            })
                                            .build(context),
                                    )
                                    .build(context),
                            )
                            .build(context),
                    )
                    .build(context),
            )
    }
}

#[derive(Default)]
pub struct GameViewState {}

impl State for GameViewState {
    fn receive_messages(&self, context: &mut Context<'_>, messages: &Vec<MessageBox>) {
        for message in messages {
            if let Ok(message) = message.downcast_ref::<StringMessage>() {
                match message.0.as_str() {
                    "start" => {
                        if let Some(menu_view) = &mut context.child_by_id("menu_view") {
                            menu_view.set::<Visibility>(Visibility::from("Collapsed"));
                        }
                    }
                    "quit" => {
                        context.push_event(SystemEvent::Quit);
                    }
                    _ => {}
                }
            }
        }
    }
}

widget!(
        GameView<GameViewState> {
            selector: Selector
        }
    );

impl Template for GameView {
    fn template(self, id: Entity, context: &mut BuildContext) -> Self {
        let state = self.clone_state();

        self.name("GameView")
            .selector(Selector::default().id("game_view"))
            .child(
                Grid::create()
                    .child(
                        Container::create()
                            .child(TextBlock::create().text("Dungeon").build(context))
                            .build(context),
                    )
                    .child(MenuView::create().build(context))
                    .build(context),
            )
    }
}

fn main() {
    let mut game = Game::default();
    game.create_window()
        .bounds((100.0, 100.0, 800.0, 600.0))
        .title("OrbGame - dungeon example")
        .debug_flag(false)
        .theme(
            Theme::create()
                .extension_path("examples/res/dungeon/theme.css")
                .build(),
        )
        .build(GameView::create());
    game.run();
}
