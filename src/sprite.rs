use std::cell::{Cell, RefCell};

use orbtk::Rect;
use orbimage::Image;
use orbclient::Renderer;

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename = "Sprite")]
pub struct SpriteConfig {
    pub sheet: String,
    pub rows: u32,
    pub columns: u32,
}

#[derive(Clone)]
pub struct Sprite {
    sheet: RefCell<Option<Image>>,
    animations: Vec<Rect>,
    rows: u32,
    columns: u32,
    row: Cell<u32>,
    column: Cell<f32>,
}

impl Sprite {
    pub fn from_config(config: &SpriteConfig) -> Sprite {
        let mut sheet = None;
        let mut animations = vec![];

        if let Ok(image) = Image::from_path(&config.sheet) {
            let animation_width = image.width() / config.columns;
            let animation_height = image.height() / config.rows;

            for r in 0..config.rows {
                for c in 0..config.columns {
                    animations.push(Rect::new(
                        (c * animation_width) as i32,
                        (r * animation_height) as i32,
                        animation_width,
                        animation_height,
                    ))
                }
            }

            sheet = Some(image)
        }

        Sprite {
            sheet: RefCell::new(sheet),
            animations,
            rows: config.rows,
            columns: config.columns,
            row: Cell::new(0),
            column: Cell::new(0.0),
        }
    }

    pub fn sheet(&self) -> &RefCell<Option<Image>> {
        &self.sheet
    }

    pub fn row(&self) -> &Cell<u32> {
        &self.row
    }

    pub fn column(&self) -> &Cell<f32> {
        &self.column
    }

    pub fn current_animation_rect(&self) -> &Rect {
        if self.row.get() >= self.rows {
            self.row.set(0);
        }

        if self.column.get() >= self.columns as f32 {
            self.column.set(0.0);
        }

        &self.animations[(self.row.get() * self.columns + self.column.get() as u32) as usize]
    }
}
