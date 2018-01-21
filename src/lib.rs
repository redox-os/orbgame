#[macro_use]
extern crate serde_derive;
// extern crate toml;
extern crate ron;

extern crate rhai;
extern crate orbclient;
extern crate orbimage;
extern crate orbtk;

extern crate chrono;
extern crate fps_counter;

use std::io::Read;
use std::fs::File;

// use toml::Value;
use orbtk::Rect;
use ron::value::Value;

// pub trait TomlLoader {
//     fn from_toml_value(value: &Value) -> Self;
// }

static X_KEY: &str = "x";
static Y_KEY: &str = "y";
static WIDTH_KEY: &str = "width";
static HEIGHT_KEY: &str = "height";

// impl TomlLoader for Rect {
//     fn from_toml_value(value: &Value) -> Self {
//         Rect::new(
//                 value[X_KEY]
//                     .as_integer()
//                     .expect("property x not found") as i32,
//                 value[Y_KEY]
//                     .as_integer()
//                     .expect("property y not found") as i32,
//                 value[WIDTH_KEY]
//                     .as_integer()
//                     .expect("property width not found") as u32,
//                 value[HEIGHT_KEY]
//                     .as_integer()
//                     .expect("property height not found") as u32,
//             )
//     }
// }


pub use self::camera::*;
pub use self::entity::*;
pub use self::game_builder::*;
pub use self::game::*;
pub use self::tile_map::*;
pub use self::scene::*;
pub use self::script_engine::ScriptEngine;
// pub use self::sprite::Sprite;
// pub use self::stage::*;

mod camera;
mod entity;
mod game_builder;
mod game;
pub mod tile_map;
pub mod scene;
mod script_engine;
// mod sprite;
// mod stage;