use std::cell::Cell;
use orbtk::Rect;

use sprite::Sprite;

static LAYER_KEY: &str = "layer";
static X_KEY: &str = "x";
static Y_KEY: &str = "y";
static WIDTH_KEY: &str = "width";
static HEIGHT_KEY: &str = "height";
static SPRITE_KEY: &str = "sprite";

pub struct Entity {
    layer: i32,
    rect: Cell<Rect>,
    sprite: Cell<Option<Sprite>>,
}

impl Entity {
    pub fn from_toml(path: &str) -> Self {
        let value = super::load_toml_value(path).unwrap();

        println!("{:?}", value);

        Entity {
            layer: value[LAYER_KEY]
                .as_integer()
                .expect("property layer not found") as i32,
            rect: Cell::new(Rect::new(
                value[X_KEY]
                    .as_integer()
                    .expect("property x not found") as i32,
                value[Y_KEY]
                    .as_integer()
                    .expect("property y not found") as i32,
                value[WIDTH_KEY]
                    .as_integer()
                    .expect("property width not found") as u32,
                value[HEIGHT_KEY]
                    .as_integer()
                    .expect("property height not found") as u32,
            )),
            sprite: Cell::new(Some(Sprite::from_toml_value(&value[SPRITE_KEY]))),
        }
    }

    pub fn layer(&self) -> i32 {
        self.layer
    }

    pub fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    pub fn sprite(&self) -> &Cell<Option<Sprite>> {
        &self.sprite
    }
}
