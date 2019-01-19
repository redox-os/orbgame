use std::{cell::Cell, fs::File, io::prelude::*, rc::Rc};

use orbgame::prelude::*;

fn create_button_with_container(label: &str, handler: MouseEventHandler) -> Template {
    Container::create().with_child(
        Button::create()
            .with_property(Label::from(label))
            .with_event_handler(handler),
    )
}

// todo: better way in OrbTk to act with parent state.
fn create_menu(new_game: MouseEventHandler, quit: MouseEventHandler) -> Template {
    let visibility = SharedProperty::new(Visibility::Visible);

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
                                .with_child(
                                    create_button_with_container("New Game", new_game)
                                        .with_shared_property(visibility.clone()),
                                )
                                .with_child(
                                    create_button_with_container("Quit", quit)
                                        .with_shared_property(visibility.clone()),
                                ),
                        ),
                ),
        )
        .with_property(Selector::default().with_id("menu-view"))
        .with_shared_property(visibility)
}

#[derive(Default)]
struct GameViewState {
    game_started: Cell<bool>,
    close_game: Cell<bool>,
}

impl GameViewState {
    fn start_game(&self) {
        println!("Start game");
        self.game_started.set(true);
    }

    fn quit_game(&self) {
        println!("Quit game");
        self.close_game.set(true);
    }
}

impl State for GameViewState {
    fn update(&self, context: &mut Context<'_>) {
        if self.close_game.get() {
            context.push_event(SystemEvent::Quit);
        }
        if let Some(menu_view) = &mut context.widget_from_id("menu-view") {
            if self.game_started.get() {
                if let Ok(visibility) = menu_view.borrow_mut_property::<Visibility>() {
                    *visibility = Visibility::Collapsed
                }
            }
        }
    }
}

struct GameView;

impl Widget for GameView {
    fn create() -> Template {
        let state = Rc::new(GameViewState::default());
        let ng_state = state.clone();
        let q_state = state.clone();

        Template::default()
            .as_parent_type(ParentType::Single)
            .with_debug_name("GameView")
            .with_child(
                Stack::create()
                    .with_child(
                        Container::create()
                            .with_child(TextBlock::create().with_property(Label::from("Dungeon"))),
                    )
                    .with_child(Container::create().with_child(create_menu(
                        MouseEventHandler::default().on_click(Rc::new(
                            move |_pos: Point| -> bool {
                                ng_state.start_game();
                                true
                            },
                        )),
                        MouseEventHandler::default().on_click(Rc::new(
                            move |_pos: Point| -> bool {
                                q_state.quit_game();
                                true
                            },
                        )),
                    ))),
            )
            .with_state(state)
    }
}

fn main() {
    let mut theme = File::open("examples/res/dungeon/theme.css").unwrap();
    let mut contents = String::new();
    theme.read_to_string(&mut contents).unwrap();

    let theme = format!("{}{}", contents, DEFAULT_THEME_CSS);

    let mut game = Game::default();
    game.create_window()
        .with_bounds(Bounds::new(100, 100, 800, 600))
        .with_title("OrbGame - dungeon example")
        .with_theme(Theme::parse(&theme))
        .with_root(GameView::create())
        .with_debug_flag(false)
        .build();
    game.run();
}
