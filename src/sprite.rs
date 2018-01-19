use std::cell::{Cell, RefCell};

use toml::Value;

use orbtk::Rect;
use orbimage::Image;

static SHEET_KEY: &str = "sheet";
static ANIMATIONS_KEY: &str = "animations";

#[derive(Clone)]
pub struct Sprite {
    sheet: RefCell<Option<Image>>,
    animations: Vec<Rect>,
    animation_step: Cell<usize>,
}

impl Sprite {
    pub fn from_toml_value(value: &Value) -> Self {
        let mut sheet = None;
        if let Ok(image) =
            Image::from_path(value[SHEET_KEY].as_str().expect("property sheet not found"))
        {
            sheet = Some(image)
        }

        let mut animations = vec![];

        for animation in value[ANIMATIONS_KEY]
            .as_array()
            .expect("proeprty animations not found")
        {
            let animation = animation.as_array().expect("property is not an array");

            animations.push(Rect::new(
                animation[0].as_integer().expect("property x not found") as i32,
                animation[1].as_integer().expect("property y not found") as i32,
                animation[2].as_integer().expect("property width not found") as u32,
                animation[3]
                    .as_integer()
                    .expect("property height not found") as u32,
            ))
        }

        Sprite {
            sheet: RefCell::new(sheet),
            animations: animations,
            animation_step: Cell::new(6),
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
