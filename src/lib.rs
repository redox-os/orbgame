#[macro_use]
extern crate serde_derive;
extern crate toml;

extern crate orbclient;
extern crate orbimage;
extern crate orbtk;

extern crate chrono;
extern crate fps_counter;

use std::io::Read;
use std::fs::File;

use toml::Value;

pub use self::camera::*;
pub use self::entity::*;
pub use self::game::*;
pub use self::tile_map::*;
pub use self::sprite::Sprite;
pub use self::stage::*;

mod camera;
mod entity;
mod game;
mod tile_map;
mod sprite;
mod stage;

pub fn load_toml_value(path: &str) -> Result<Value, String> {
    let mut file = File::open(path).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("cannot read file");
    if let Ok(value) = contents.parse::<toml::Value>() {
        return Result::Ok(value)
    }

    Result::Err(String::from(format!("Could not parse file; {}", path)))
}
