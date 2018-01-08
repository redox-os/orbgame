use std::cell::Cell;

use orbtk::{Point, Rect};
use Entity;

pub struct Camera {
    rect: Cell<Rect>,
    maximum: Cell<Point>,
    speed: Cell<u32>,
}

impl Camera {
    pub fn new(rect: Rect, maximum: Point) -> Self {
        Camera {
            rect: Cell::new(rect),
            maximum: Cell::new(maximum),
            speed: Cell::new(256),
        }
    }

    pub fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    pub fn maximum(&self) -> &Cell<Point> {
        &self.maximum
    }

    pub fn speed(&self) -> &Cell<u32> {
        &self.speed
    }

    pub fn mov(&mut self, delta: f32, dir_x: i32, dir_y: i32) {
        let mut rect = self.rect.get();
        let speed = self.speed.get();
        let maximum = self.maximum.get();

        rect.x += (dir_x as f32 * speed as f32 * delta) as i32;
        rect.y += (dir_y as f32 * speed as f32 * delta) as i32;

        let zero: i32 = 0;

        // adjust to respect the render_bounds
        rect.x = zero.max(rect.x.min(maximum.x));
        rect.y = zero.max(rect.y.min(maximum.y));

        self.rect().set(rect);
    }

    pub fn follow(&mut self, entity: &mut Entity) {
        let mut screen_position = entity.screen_position().get();
        let entity_rect = entity.rect().get();
        let mut rect = self.rect.get();
        let maximum = self.maximum.get();

        screen_position.x = rect.width as i32 / 2;
        screen_position.y = rect.height as i32 / 2;

        // make the camera follow the sprite
        rect.x = entity_rect.x - rect.width as i32 / 2;
        rect.y = entity_rect.y - rect.height as i32 / 2;

        let zero: i32 = 0;

        // clamp values
        rect.x = zero.max(rect.x.min(maximum.x));
        rect.y = zero.max(rect.y.min(maximum.y));

        // in map corners, the sprite cannot be placed in the center of the screen
        // and we have to change its screen coordinates

        // left and right sides
        if entity_rect.x < rect.width as i32 / 2
            || entity_rect.x > maximum.x + rect.width as i32 / 2
        {
            let new_x = entity_rect.x - rect.x;
            screen_position.x = new_x;
        }
        // top and bottom sides
        if entity_rect.y < rect.height as i32 / 2
            || entity_rect.y > maximum.y + rect.height as i32 / 2
        {
            let new_y = entity_rect.y - rect.y;
            screen_position.y = new_y;
        }

        entity.screen_position().set(screen_position);
        self.rect.set(rect);
    }
}
