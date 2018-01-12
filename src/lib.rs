#[macro_use]
extern crate serde_derive;
extern crate toml;

extern crate orbclient;
extern crate orbimage;
extern crate orbtk;

extern crate chrono;
extern crate fps_counter;

use toml::Value;
use std::io::Read;
use std::fs::File;

pub use self::camera::*;
// pub use self::entity::*;
pub use self::game::*;
// pub use self::level::*;
pub use self::tile_map::*;
// pub use self::scene::*;
pub use self::stage::*;

mod camera;
// mod entity;
mod game;
// mod level;
mod tile_map;
// mod scene;
mod stage;

pub fn load_toml_value(path: &str) -> Value {
    let mut file = File::open(path).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("cannot read file");
    contents.parse::<Value>().expect("toml value not found")
}
