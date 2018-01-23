use std::cell::{Cell, RefCell};
use orbtk::{Rect, Point};

use sprite::{Sprite, SpriteConfig};
use ScriptEngine;
use TileMap;

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename = "Entity")]
pub struct EntityConfig {
    pub id: String,
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
    id: String,
    layer: i32,
    rect: Cell<Rect>,
    screen_position: Cell<Point>,
    sprite: RefCell<Option<Sprite>>,
    script: RefCell<String>,
}

impl Entity {
    pub fn from_config(config: &EntityConfig) -> Self {
        Entity {
            id: config.id.clone(),
            layer: config.layer,
            rect: Cell::new(Rect::new(config.x, config.y, config.width, config.height)),
            screen_position: Cell::new(Point::new(config.x, config.y)),
            sprite: RefCell::new(Some(Sprite::from_config(&config.sprite))),
            script: RefCell::new(ScriptEngine::load_script(&config.script)),
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn layer(&self) -> i32 {
        self.layer
    }

    pub fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    pub fn screen_position(&self) -> &Cell<Point> {
        &self.screen_position
    }

    pub fn sprite(&self) -> &RefCell<Option<Sprite>> {
        &self.sprite
    }

    pub fn script(&self) -> &RefCell<String> {
        &self.script
    }

    pub fn animation_step(&mut self, animation_step: f64) {
        println!("{}", animation_step);
        if let Some(ref sprite) = *self.sprite.borrow_mut() {
            sprite.animation_step().set(animation_step);
        }
    }

    pub fn mov(&mut self, dir_x: f64, dir_y: f64, map: TileMap) {
        let mut rect = self.rect.get();

        rect.x += dir_x as i32;
        self.check_tile_collison(&mut rect, dir_x as f32, 0.0, &map);

        rect.y += dir_y as i32;
        self.check_tile_collison(&mut rect, 0.0, dir_y as f32, &map);

        let max_x = map.column_count() * map.tile_size() as usize - rect.width as usize;
        let max_y = map.row_count() * map.tile_size() as usize - rect.height as usize;

        let zero_x: i32 = 0 + rect.width as i32;
        let zero_y: i32 = 0 + rect.height as i32;

        // adjust to respect the render_bounds
        rect.x = zero_x.max(rect.x.min(max_x as i32));
        rect.y = zero_y.max(rect.y.min(max_y as i32));

        self.rect.set(rect);
    }

    fn check_tile_collison(&mut self, rect: &mut Rect, dir_x: f32, dir_y: f32, map: &TileMap) {
        let left = rect.x as f32 + 1.0;
        let right = rect.x as f32 + rect.width as f32 - 1.0;
        let top = rect.y as f32 + 1.0;
        let bottom = rect.y as f32 + rect.height as f32 - 1.0;

        // check for collisions on sprite sides
        let collision = map.is_tile_blocked(left, top) || map.is_tile_blocked(right, top)
            || map.is_tile_blocked(right, bottom)
            || map.is_tile_blocked(left, bottom);

        if !collision {
            return;
        }

        if dir_y > 0.0 {
            rect.y = map.get_y(map.get_row(bottom)) as i32 - rect.height as i32;
        } else if dir_y < 0.0 {
            rect.y = map.get_y(map.get_row(top) + 1.0) as i32;
        } else if dir_x > 0.0 {
            rect.x = map.get_x(map.get_column(right)) as i32 - rect.width as i32;
        } else if dir_x < 0.0 {
            rect.x = map.get_x(map.get_column(left) + 1.0) as i32;
        }
    }
}
