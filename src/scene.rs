use std::cell::{Cell, RefCell};
use std::sync::Arc;
use std::cmp;

use orbclient::{Color, Renderer};
use orbtk::{Event, Place, Point, Rect, Widget, Label};
use orbimage::Image;

use Camera;
use Level;
use Entity;
use Direction;
use Map;
use UpdatableWidget;

pub struct Scene {
    rect: Cell<Rect>,
    camera: RefCell<Option<Camera>>,
    level: RefCell<Option<Arc<Level>>>,
    level_sheet: RefCell<Option<Image>>,
    direction: Cell<Point>,
    player: RefCell<Option<Entity>>,
    player_image: RefCell<Option<Image>>,
    debug: Cell<bool>,
}

impl Scene {
    pub fn new() -> Arc<Self> {
        Arc::new(Scene {
            rect: Cell::new(Rect::default()),
            camera: RefCell::new(None),
            level: RefCell::new(None),
            level_sheet: RefCell::new(None),
            direction: Cell::new(Point::new(0, 0)),
            player: RefCell::new(None),
            player_image: RefCell::new(None),
            debug: Cell::new(true),
        })
    }

    pub fn camera(&self, camera: Camera) -> &Self {
        *self.camera.borrow_mut() = Some(camera);
        self
    }

    pub fn player(&self, player: Entity) -> &Self {
        *self.player.borrow_mut() = Some(player);
        self
    }

    pub fn player_image(&self, player_image: Image) -> &Self {
        *self.player_image.borrow_mut() = Some(player_image);
        self
    }

    pub fn level(&self, level: Arc<Level>) -> &Self {
        *self.level.borrow_mut() = Some(level);
        self
    }

    pub fn level_sheet(&self, level_sheet: Image) -> &Self {
        *self.level_sheet.borrow_mut() = Some(level_sheet);
        self
    }

    pub fn debug(&self) -> &Cell<bool> {
        &self.debug
    }

    fn draw_layer_by_camera(
        &self,
        layer: usize,
        camera: &Camera,
        map: &Map,
        renderer: &mut Renderer,
    ) {
        let camera_rect = camera.rect().get();
        let rect = self.rect().get();

        let start_column = (camera_rect.x as f32 / map.tile_size() as f32).floor() as usize;
        let end_column =
            start_column + (camera_rect.width as f32 / map.tile_size() as f32).ceil() as usize;
        let start_row = (camera_rect.y as f32 / map.tile_size() as f32).floor() as usize;
        let end_row =
            start_row + (camera_rect.height as f32 / map.tile_size() as f32).ceil() as usize;
        let offset_x =
            rect.x as f32 + -camera_rect.x as f32 + start_column as f32 * map.tile_size() as f32;
        let offset_y =
            rect.y as f32 + -camera_rect.y as f32 + start_row as f32 * map.tile_size() as f32;

        self.draw_layer(
            layer,
            map,
            start_column,
            end_column,
            start_row,
            end_row,
            offset_x,
            offset_y,
            renderer,
        );
    }

    fn draw_layer(
        &self,
        layer: usize,
        map: &Map,
        start_column: usize,
        end_column: usize,
        start_row: usize,
        end_row: usize,
        offset_x: f32,
        offset_y: f32,
        renderer: &mut Renderer,
    ) {
        if let Some(ref image) = *self.level_sheet.borrow() {
           
            // add 1 to prevent missing tiles at the borders
            let mut end_column = end_column + 1;
            let mut end_row = end_row + 1;

            if end_column > map.column_count() {
                end_column = map.column_count();
            }

            if end_row > map.row_count() {
                end_row = map.row_count();
            }

            for r in start_row..end_row {
                for c in start_column..end_column {
                    let tile = map.get_tile(layer, r, c);

                    if tile == -1 {
                        continue;
                    }

                    let tile_column_count = image.width() / map.tile_size();
                    let tile_c = tile as f32 % tile_column_count as f32;
                    let tile_r = (tile as f32 / tile_column_count as f32).floor();

                    Scene::draw_image_part(
                        renderer,
                        image,
                        (((c - start_column) as f32) * map.tile_size() as f32 + offset_x as f32)
                            as i32,
                        (((r - start_row) as f32) * map.tile_size() as f32 + offset_y as f32)
                            as i32,
                        tile_c as u32 * map.tile_size(),
                        tile_r as u32 * map.tile_size(),
                        map.tile_size(),
                        map.tile_size(),
                    );
                }
            }
        }
    }

    // tmp solution
    fn draw_image_part(
        renderer: &mut Renderer,
        image: &Image,
        x: i32,
        y: i32,
        part_x: u32,
        part_y: u32,
        w: u32,
        h: u32,
    ) {
        let stride = image.width();
        let mut offset = (part_y * stride + part_x) as usize;
        let last_offset = cmp::min(
            ((part_y + h) * stride + part_x) as usize,
            image.data().len(),
        );

        let mut y = y;

        while offset < last_offset {
            let next_offset = offset + stride as usize;
            renderer.image(x, y, w, 1, &image.data()[offset..]);
            offset = next_offset;
            y += 1;
        }
    }

    fn draw_entity(&self, entity: &Entity, image: &Image, renderer: &mut Renderer) {
        let position = entity.screen_position().get();
        let rect = entity.rect().get();

        let tile_r: u32;

        let tile_c = entity.animation_step().floor() as u32;

        match *entity.direction() {
            Direction::Up => tile_r = 2,
            Direction::Left => tile_r = 3,
            Direction::Right => tile_r = 1,
            _ => tile_r = 0,
        }

        Scene::draw_image_part(
            renderer,
            image,
            position.x,
            position.y,
            tile_c * rect.width,
            tile_r * rect.height,
            rect.width,
            rect.height,
        );
    }

    fn draw_debugging_grid(&self, level: &Level, camera: &Camera, renderer: &mut Renderer) {
        let rect = self.rect.get();
        let camera_rect = camera.rect().get();
        let tile_size = level.map().tile_size();

        let start_column = (camera.rect().get().x as f32 / tile_size as f32).floor() as i32;
        let end_column =
            1 + start_column + (camera.rect().get().width as f32 / tile_size as f32).ceil() as i32;
        let start_row = (camera_rect.y as f32 / tile_size as f32).floor() as i32;
        let end_row = 1 + start_row + (camera_rect.height as f32 / tile_size as f32).ceil() as i32;

        let offset_x = -camera_rect.x + start_column * tile_size as i32;
        let offset_y = -camera_rect.y + start_row * tile_size as i32;

        for i in start_column..end_column {
            renderer.rect(
                ((i - start_column)) * tile_size as i32 + offset_x,
                0,
                1,
                rect.height,
                Color::rgb(0, 0, 0),
            );
        }

        for i in start_row..end_row {
            renderer.rect(
                0,
                ((i - start_row)) * tile_size as i32 + offset_y,
                rect.width,
                1,
                Color::rgb(0, 0, 0),
            );
        }
    }

    fn draw_debugging_collision(
        &self,
        level: &Level,
        camera: &Camera,
        entity: &Entity,
        renderer: &mut Renderer,
    ) {
        let camera_rect = camera.rect().get();
        let map = level.map();
        let entity_rect = entity.rect().get();

        let left = entity_rect.x as f32 + 1.0;
        let right = entity_rect.x as f32 + entity_rect.width as f32 - 1.0;
        let top = entity_rect.y as f32 + 1.0;
        let bottom = entity_rect.y as f32 + entity_rect.height as f32 - 1.0;

        let start_column = (left / map.tile_size() as f32).floor() as i32 - 2;
        let end_column = (right / map.tile_size() as f32).floor() as i32 + 2;
        let start_row = (top / map.tile_size() as f32).floor() as i32 - 2;
        let end_row = (bottom / map.tile_size() as f32).floor() as i32 + 2;

        // check map render_bounds
        let start_column = {
            if start_column < 0 {
                0
            } else {
                start_column as usize
            }
        };

        let end_column = {
            if end_column as usize > level.map().column_count() {
                level.map().column_count()
            } else {
                end_column as usize
            }
        };


        let start_row = {
            if start_row < 0 {
                0
            } else {
                start_row as usize
            }
        };

        let end_row = {
            if end_row as usize > level.map().row_count() {
                level.map().row_count()
            } else {
                end_row as usize
            }
        };

        for r in start_row..end_row {
            for c in start_column..end_column {
                let x = map.get_x(c as f32) - camera_rect.x as f32;
                let y = map.get_y(r as f32) - camera_rect.y as f32;

                let is_blocked = map.is_blocked(c, r);

                if is_blocked {
                    renderer.rect(
                        x as i32,
                        y as i32,
                        map.tile_size(),
                        map.tile_size(),
                        Color::rgba(255, 0, 0, 130),
                    );
                } else {
                    renderer.rect(
                        x as i32,
                        y as i32,
                        map.tile_size(),
                        map.tile_size(),
                        Color::rgba(13, 255, 0, 130),
                    );
                }
            }
        }
    }
}

impl Place for Scene {}

impl Widget for Scene {
    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn draw(&self, renderer: &mut Renderer, _focused: bool) {
        if let Some(ref camera) = *self.camera.borrow() {
            if let Some(ref level) = *self.level.borrow() {
                self.draw_layer_by_camera(0, &camera, &level.map(), renderer);
                self.draw_layer_by_camera(1, &camera, &level.map(), renderer);
                if let Some(ref player) = *self.player.borrow() {
                    if let Some(ref player_image) = *self.player_image.borrow() {
                        self.draw_entity(player, player_image, renderer);
                    }
                }
                self.draw_layer_by_camera(2, &camera, &level.map(), renderer);

                if !self.debug.get() {
                    return;
                }

                if let Some(ref player) = *self.player.borrow() {
                    self.draw_debugging_collision(level, &camera, player, renderer);
                }

                self.draw_debugging_grid(level, &camera, renderer);
            }
        }
    }

    fn event(&self, event: Event, focused: bool, redraw: &mut bool) -> bool {
        let mut direction = self.direction.get();

        match event {
            Event::LeftArrow => {
                 direction.x = -1;

                *redraw = true;
            }
            Event::UpArrow => {
                direction.y = -1;

                *redraw = true;
            }
            Event::RightArrow => {
                direction.x = 1;

                *redraw = true;
            }
            Event::DownArrow => {
                direction.y = 1;

                *redraw = true;
            }
            _ => (),
        }

        self.direction.set(direction);

        focused
    }
}

impl UpdatableWidget for Scene {
    fn update(&self) {
        if let Some(ref mut player) = *self.player.borrow_mut() {
            if let Some(ref level) = *self.level.borrow() {
                player.mov(
                    0.017,
                    self.direction.get().x as f32,
                    self.direction.get().y as f32,
                    level.map(),
                );

                if let Some(ref mut camera) = *self.camera.borrow_mut() {
                    camera.follow(player);
                }
            }
        }

        self.direction.set(Point::new(0, 0));
    }
}