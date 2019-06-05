use std::cell::Cell;

use orbtk::prelude::*;

/// The camera is use to describes the viewport on a screen like a part of a tile map.
///
/// The camera can be moved.
#[derive(Clone, Default, Debug)]
pub struct Camera {
    rect: Cell<Rect>,
    maximum: Cell<Point>,
    speed: Cell<f64>,
}

/// Describes the base behavior methods of a camera.
pub trait CameraExt {
    fn rect(&self) -> &Cell<Rect>;

    fn maximum(&self) -> &Cell<Point>;

    fn speed(&self) -> &Cell<f64>;

    fn mov(&mut self, delta: f64, dir_x: f64, dir_y: f64);
}

impl Camera {
    pub fn new(rect: Rect, maximum: Point) -> Self {
        Camera {
            rect: Cell::new(rect),
            maximum: Cell::new(maximum),
            speed: Cell::new(256.0),
        }
    }
}

impl CameraExt for Camera {
    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn maximum(&self) -> &Cell<Point> {
        &self.maximum
    }

    fn speed(&self) -> &Cell<f64> {
        &self.speed
    }

    fn mov(&mut self, delta: f64, dir_x: f64, dir_y: f64) {
        let mut rect = self.rect.get();
        let speed = self.speed.get();
        let maximum = self.maximum.get();

        rect.x += (dir_x as f64 * speed as f64 * delta) as f64;
        rect.y += (dir_y as f64 * speed as f64 * delta) as f64;

        let zero: f64 = 0.0;

        // adjust to respect the render_bounds
        rect.x = zero.max(rect.x.min(maximum.x));
        rect.y = zero.max(rect.y.min(maximum.y));

        self.rect().set(rect);
    }

    // pub fn follow(&mut self, entity: &mut Entity) {
    //     let mut screen_position = entity.screen_position().get();
    //     let entity_rect = entity.rect().get();
    //     let mut rect = self.rect.get();
    //     let maximum = self.maximum.get();

    //     screen_position.x = rect.width as f64 / 2.0;
    //     screen_position.y = rect.height as f64 / 2;

    //     // make the camera follow the sprite
    //     rect.x = entity_rect.x - rect.width as f64 / 2;
    //     rect.y = entity_rect.y - rect.height as f64 / 2;

    //     let zero: f64 = 0;

    //     // clamp values
    //     rect.x = zero.max(rect.x.min(maximum.x));
    //     rect.y = zero.max(rect.y.min(maximum.y));

    //     // in map corners, the sprite cannot be placed in the center of the screen
    //     // and we have to change its screen coordinates

    //     // left and right sides
    //     if entity_rect.x < rect.width as f64 / 2
    //         || entity_rect.x > maximum.x + rect.width as f64 / 2
    //     {
    //         let new_x = entity_rect.x - rect.x;
    //         screen_position.x = new_x;
    //     }
    //     // top and bottom sides
    //     if entity_rect.y < rect.height as f64 / 2
    //         || entity_rect.y > maximum.y + rect.height as f64 / 2
    //     {
    //         let new_y = entity_rect.y - rect.y;
    //         screen_position.y = new_y;
    //     }

    //     entity.screen_position().set(screen_position);
    //     self.rect.set(rect);
    // }
}
