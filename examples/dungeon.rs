use std::{cell::Cell, rc::Rc};

use orbgame::prelude::*;

fn create_button_with_container(label: &str, handler: MouseEventHandler) -> Template {
    Container::create().with_child(
        Button::create()
            .with_property(Label::from(label))
            .with_event_handler(handler),
    )
}

#[derive(Default)]
struct MenuViewState {
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

struct MenuView;

impl Widget for MenuView {
    fn create() -> Template {
        let state = Rc::new(MenuViewState::default());
        let ng_state = state.clone();
        let q_state = state.clone();

        Template::default()
            .as_parent_type(ParentType::Single)
            .with_debug_name("MenuView")
            .with_child(
                Center::create()
                    .as_parent_type(ParentType::Single)
                    .with_child(
                        Container::create()
                            .with_property(Selector::from("container").with_class("menu"))
                            .with_child(
                                Column::create()
                                    .with_child(create_button_with_container(
                                        "New Game",
                                        MouseEventHandler::default().on_click(Rc::new(
                                            move |_pos: Point| -> bool {
                                                ng_state.start_game();
                                                true
                                            },
                                        )),
                                    ))
                                    .with_child(create_button_with_container(
                                        "Quit",
                                        MouseEventHandler::default().on_click(Rc::new(
                                            move |_pos: Point| -> bool {
                                                q_state.quit_game();
                                                true
                                            },
                                        )),
                                    )),
                            ),
                    ),
            )
            .with_property(Selector::default().with_id("menu-view"))
            .with_state(state)
    }
}

#[derive(Default)]
struct GameViewState {}

impl State for GameViewState {
    fn receive_messages(&self, context: &mut Context<'_>, messages: &Vec<MessageBox>) {
        for message in messages {
            if let Ok(message) = message.downcast_ref::<StringMessage>() {
                match message.0.as_str() {
                    "start" => {
                        if let Some(menu_view) = &mut context.widget_from_id("menu-view") {
                            if let Ok(visibility) = menu_view.borrow_mut_property::<Visibility>() {
                                *visibility = Visibility::Collapsed
                            }
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

struct GameView;

impl Widget for GameView {
    fn create() -> Template {
        let state = Rc::new(GameViewState::default());

        Template::default()
            .as_parent_type(ParentType::Single)
            .with_debug_name("GameView")
            .with_child(
                Stack::create()
                    .with_child(
                        Container::create()
                            .with_child(TextBlock::create().with_property(Label::from("Dungeon"))),
                    )
                    .with_child(MenuView::create()),
            )
            .with_property(Selector::default().with_id("game_view"))
            .with_state(state)
    }
}

fn main() {
    let mut game = Game::default();
    game.create_window()
        .with_bounds(Bounds::new(100, 100, 800, 600))
        .with_title("OrbGame - dungeon example")
        .with_theme(
            Theme::create()
                .with_extenstion_path("examples/res/dungeon/theme.css")
                .build(),
        )
        .with_root(GameView::create())
        .with_debug_flag(false)
        .build();
    game.run();
}
