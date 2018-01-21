use std::cell::{Cell, RefCell};
use orbtk::{CloneCell, Rect};

use sprite::{Sprite, SpriteConfig};

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename = "Entity")]
pub struct EntityConfig {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub layer: i32,
    pub script: String,
    pub sprite: SpriteConfig,
}

#[derive(Clone)]
pub struct Entity {
    layer: i32,
    rect: Cell<Rect>,
    sprite: RefCell<Option<Sprite>>,
    script: RefCell<String>,
}

impl Entity {
    pub fn from_config(config: &EntityConfig) -> Self {
        Entity {
            layer: config.layer,
            rect: Cell::new(Rect::new(config.x, config.y, config.width, config.height)),
            sprite: RefCell::new(Some(Sprite::from_config(&config.sprite))),
            // todo: support file loading and inplace code
            script: RefCell::new(config.script.clone()),
        }
    }

    pub fn layer(&self) -> i32 {
        self.layer
    }

    pub fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    pub fn sprite(&self) -> &RefCell<Option<Sprite>> {
        &self.sprite
    }

    pub fn script(&self) -> &RefCell<String> {
        &self.script
    }
}
