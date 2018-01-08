extern crate orbgame;
extern crate orbimage;
extern crate orbtk;

use orbtk::{Point, Rect, Window};
use orbimage::Image;
use orbgame::*;

fn main() {
    let mut window = Window::new(Rect::new(100, 100, 800, 600), "OrbAdventure");

    let level = Level::from_path("res/adventure_level.toml");
    let sheet = Image::from_path(&level.sheet());
    let player_image = Image::from_path("res/adventure_character.png");


    let map = level.map().clone();

    let main_scene = Scene::new();

    if let Ok(sheet) = sheet {
        // todo: use correct width and height. Maybe need a extra animation file .toml
        let player = Entity::new(Rect::new(32, 32, 14, 21), 256.0);

        if let Ok(player_image) = player_image {
            main_scene
                .camera(Camera::new(
                    Rect::new(0, 0, window.width(), window.height()),
                    Point::new(
                        map.column_count() as i32 * map.tile_size() as i32 - 640,
                        map.row_count() as i32 * map.tile_size() as i32 - 576,
                    ),
                ))
                .level(level)
                .level_sheet(sheet)
                .player(player)
                .player_image(player_image);
        }
    }

    window.add(&main_scene);
    window.exec();
}
