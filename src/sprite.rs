use std::cell::RefCell;

use toml::Value;

use orbimage::Image;

pub struct Sprite {
    sheet: RefCell<Option<Image>>,
}

impl Sprite {
    pub fn from_toml_value(value: &Value) -> Self {
        Sprite {
            sheet: RefCell::new(None)
        }
    }
}