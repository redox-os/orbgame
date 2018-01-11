extern crate orbgame;

use orbgame::Game;
// static CHAR: &'static [u8; 1290] = include_bytes!("../res/adventure_character.png");
// static LEVEL: &'static [u8; 32877] = include_bytes!("../res/adventure_level.toml");
// static LEVEL_SHEET: &'static [u8; 100846] = include_bytes!("../res/adventure_sheet.png");

fn main() {


    Game::from_toml("res/adventure/game.toml").exec();
    // let config = Config::from_toml("res/adventure/game.toml");

    // println!("{:?}", config);
    // let mut window = Window::new(Rect::new(100, 100, 800, 600), "OrbAdventure");

    // let level = Level::from_data(LEVEL);

    // let map = level.map().clone();

    // let main_scene = Scene::new();
    // main_scene.size(window.width(), window.height());

    // if let Ok(sheet) = orbimage::parse_png(LEVEL_SHEET) {
    //     // todo: use correct width and height. Maybe need a extra animation file .toml
    //     let player = Entity::new(Rect::new(32, 32, 14, 21), 256.0);

    //     if let Ok(player_image) = orbimage::parse_png(CHAR) {
    //         main_scene
    //             .camera(Camera::new(
    //                 Rect::new(0, 0, window.width(), window.height()),
    //                 Point::new(
    //                     map.column_count() as i32 * map.tile_size() as i32 - window.width() as i32,
    //                     map.row_count() as i32 * map.tile_size() as i32 - window.height() as i32,
    //                 ),
    //             ))
    //             .level(level)
    //             .level_sheet(sheet)
    //             .player(player)
    //             .player_image(player_image);
    //     }
    // }

    // window.add(&main_scene);

    // let fps_counter = Label::new();
    // fps_counter.position(10, 10).size(16, 16);
    // fps_counter.bg.set(Color::rgba(0, 0, 0, 0));
    // fps_counter.fg.set(Color::rgb(255, 255, 255));
    // window.add(&fps_counter);

    // let mut game = Game::new(window);
    // game.fps_label(&fps_counter);
    // game.add(&main_scene);
    // game.exec();
}
