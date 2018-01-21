#[derive(Clone)]
pub struct Entity {

}
// use std::cell::{Cell, RefCell};
// use orbtk::{CloneCell, Rect};

// use sprite::Sprite;
// use TomlLoader;

// static LAYER_KEY: &str = "layer";
// static SPRITE_KEY: &str = "sprite";
// static SCRIPT_KEY: &str = "script";

// #[derive(Clone)]
// pub struct Entity {
//     layer: i32,
//     rect: Cell<Rect>,
//     sprite: RefCell<Option<Sprite>>,
//     script: RefCell<String>,
// }

// impl Entity {
//     pub fn from_toml(path: &str) -> Self {
//         let value = super::load_toml_value(path).unwrap();

//         Entity {
//             layer: value[LAYER_KEY]
//                 .as_integer()
//                 .expect("property layer not found") as i32,
//             rect: Cell::new(Rect::from_toml_value(&value)),
//             sprite: RefCell::new(Some(Sprite::from_toml_value(&value[SPRITE_KEY]))),
//             script: RefCell::new(String::from(super::read_file_as_string(
//                 value[SCRIPT_KEY]
//                     .as_str()
//                     .expect("property script not found"),
//             ))),
//         }
//     }

//     pub fn layer(&self) -> i32 {
//         self.layer
//     }

//     pub fn rect(&self) -> &Cell<Rect> {
//         &self.rect
//     }

//     pub fn sprite(&self) -> &RefCell<Option<Sprite>> {
//         &self.sprite
//     }

//     pub fn script(&self) -> &RefCell<String> {
//         &self.script
//     }
// }
