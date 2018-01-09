use std::cell::Cell;

use orbtk::{Point, Rect};

use Map;

pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

pub struct Entity {
    rect: Cell<Rect>,
    speed: f32,
    direction: Direction,
    animation_step: f32,
    total_animation_steps: f32,
    animation_speed: f32,
    screen_position: Cell<Point>,
}

impl Entity {
    pub fn new(rect: Rect, speed: f32) -> Self {
        Entity {
            rect: Cell::new(rect),
            speed,
            animation_step: 0.0,
            total_animation_steps: 4.0,
            animation_speed: 10.0,
            direction: Direction::Down,
            screen_position: Cell::new(Point::new(rect.x, rect.y)),
        }
    }

    pub fn mov(&mut self, delta: f32, dir_x: f32, dir_y: f32, map: &Map) {
        let mut rect = self.rect.get();

        if dir_y > 0.0 {
            self.direction = Direction::Down;
            self.animation_step += self.animation_speed * delta;
        } else if dir_y < 0.0 {
            self.direction = Direction::Up;
            self.animation_step += self.animation_speed * delta;
        } else if dir_x > 0.0 {
            self.direction = Direction::Right;
            self.animation_step += self.animation_speed * delta;
        } else if dir_x < 0.0 {
            self.direction = Direction::Left;
            self.animation_step += self.animation_speed * delta;
        }

        if self.animation_step > self.total_animation_steps {
            self.animation_step = 0.0;
        }

        rect.x += (dir_x * self.speed * delta) as i32;
        self.check_tile_collison(&mut rect, dir_x, 0.0, map);

        rect.y += (dir_y * self.speed * delta) as i32;
        self.check_tile_collison(&mut rect, 0.0, dir_y, map);

        let max_x = map.column_count() * map.tile_size() as usize - rect.width as usize;
        let max_y = map.row_count() * map.tile_size() as usize - rect.height as usize;

        let zero_x: i32 = 0 + rect.width as i32;
        let zero_y: i32 = 0 + rect.height as i32;

        // adjust to respect the render_bounds
        rect.x = zero_x.max(rect.x.min(max_x as i32));
        rect.y = zero_y.max(rect.y.min(max_y as i32));

        self.rect.set(rect);
    }

    fn check_tile_collison(&mut self, rect: &mut Rect, dir_x: f32, dir_y: f32, map: &Map) {
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

    pub fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    pub fn animation_step(&self) -> f32 {
        self.animation_step
    }

    pub fn screen_position(&self) -> &Cell<Point> {
        &self.screen_position
    }
}
