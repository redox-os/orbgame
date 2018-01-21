use std::cell::{Cell, RefCell};

use orbtk::Rect;
use orbimage::Image;

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename = "Sprite")]
pub struct SpriteConfig {
    pub sheet: String,
    pub animations: Vec<Vec<i32>>,
}

#[derive(Clone)]
pub struct Sprite {
    sheet: RefCell<Option<Image>>,
    animations: Vec<Rect>,
    animation_step: Cell<usize>,
}

impl Sprite {
    pub fn from_config(config: &SpriteConfig) -> Sprite {
        let mut sheet = None;

        if let Ok(image) = Image::from_path(&config.sheet) {
            sheet = Some(image)
        }

        let mut animations = vec![];

        for animation in &config.animations {
            animations.push(Rect::new(
                animation[0],
                animation[1],
                animation[2] as u32,
                animation[3] as u32,
            ));
        }

        Sprite {
            sheet: RefCell::new(sheet),
            animations,
            animation_step: Cell::new(0),
        }
    }

    pub fn sheet(&self) -> &RefCell<Option<Image>> {
        &self.sheet
    }

    pub fn animtion_rect(&self, index: usize) -> &Rect {
        &self.animations[index]
    }

    pub fn current_animation_rect(&self) -> &Rect {
        self.animtion_rect(self.animation_step.get())
    }

    pub fn animation_step(&self) -> &Cell<usize> {
        &self.animation_step
    }
}
