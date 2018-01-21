use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::sync::Arc;
use std::cmp;

use orbimage::Image;
use orbtk::{Point, Rect, Renderer, Widget, Place, Event};
use orbtk::theme::Theme;

use Camera;
use tile_map::{TileMap, TileMapConfig};

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename = "Scene")]
pub struct SceneConfig {
    pub x: i32,
    pub y: i32,
    pub map: TileMapConfig,
}

#[derive(Clone)]
pub struct Scene {
    rect: Cell<Rect>,
    // entities: HashMap<i32, RefCell<Vec<Entity>>>,
    tile_map: RefCell<Option<TileMap>>,
    camera: RefCell<Camera>,
}

impl Scene {
    pub fn from_config(config: &SceneConfig) -> Arc<Self> {
        Arc::new(Scene {
            rect: Cell::new(Rect::new(config.x, config.y, 800, 600)),
            tile_map: RefCell::new(Some(TileMap::from_config(&config.map))),
            // todo: real camera values
            camera: RefCell::new(Camera::new(
                Rect::new(0, 0, 800, 600),
                Point::new(1000, 1000),
            )),
        })
    }

    pub fn camera(&self, camera: Camera) -> &Self {
        (*self.camera.borrow_mut()) = camera;
        self
    }

    pub fn draw_all_layers(&self, renderer: &mut Renderer) {
        let rect = self.rect.get();
        let camera_rect = self.camera.borrow().rect().get();

        // draw the tile map
        let mut start_column = 0;
        let mut end_column = 0;
        let mut start_row = 0;
        let mut end_row = 0;
        let mut offset_x = 0.;
        let mut offset_y = 0.;

        let tile_map = self.tile_map.borrow();
        if let Some(ref tile_map) = *tile_map {
            let tile_size = tile_map.tile_size();

            start_column = (camera_rect.x as f32 / tile_size as f32).floor() as usize;
            end_column =
                start_column + (camera_rect.width as f32 / tile_size as f32).ceil() as usize;
            start_row = (camera_rect.y as f32 / tile_size as f32).floor() as usize;
            end_row = start_row + (camera_rect.height as f32 / tile_size as f32).ceil() as usize;
            offset_x =
                rect.x as f32 + -camera_rect.x as f32 + start_column as f32 * tile_size as f32;
            offset_y = rect.y as f32 + -camera_rect.y as f32 + start_row as f32 * tile_size as f32;
        }

        for i in 0..3 {
            if let Some(ref tile_map) = *tile_map {
                self.draw_tile_map_layer(
                    i,
                    tile_map,
                    start_column,
                    end_column,
                    start_row,
                    end_row,
                    offset_x,
                    offset_y,
                    renderer,
                );

                let layer = i as i32;

                // if let Some(entities) = self.entities.get(&layer) {
                //     for entity in &*entities.borrow() {
                //         self.draw_entity(renderer, entity);
                //     }
                // }
            }
        }
    }

    // fn draw_entity(&self, renderer: &mut Renderer, entity: &Entity) {
    //     let rect = entity.rect().get();

    //     if let Some(ref sprite) = *entity.sprite().borrow() {
    //         let sheet = sprite.sheet();
    //         let animation_rect = sprite.current_animation_rect();

    //         if let Some(ref sheet) = *sheet.borrow() {
    //             Scene::draw_image_part(
    //                 renderer,
    //                 sheet,
    //                 rect.x,
    //                 rect.y,
    //                 animation_rect.x as u32,
    //                 animation_rect.y as u32,
    //                 animation_rect.width,
    //                 animation_rect.height,
    //             );
    //         }
    //     }
    // }

    fn draw_tile_map_layer(
        &self,
        layer: usize,
        tile_map: &TileMap,
        start_column: usize,
        end_column: usize,
        start_row: usize,
        end_row: usize,
        offset_x: f32,
        offset_y: f32,
        renderer: &mut Renderer,
    ) {
        if let Some(ref image) = *tile_map.sheet().borrow() {
            // add 1 to prevent missing tiles at the borders
            let mut end_column = end_column + 1;
            let mut end_row = end_row + 1;

            if end_column > tile_map.column_count() {
                end_column = tile_map.column_count();
            }

            if end_row > tile_map.row_count() {
                end_row = tile_map.row_count();
            }

            for r in start_row..end_row {
                for c in start_column..end_column {
                    let tile = tile_map.get_tile(layer, r, c);

                    if tile == -1 {
                        continue;
                    }

                    let tile_column_count = image.width() / tile_map.tile_size();
                    let tile_c = tile as f32 % tile_column_count as f32;
                    let tile_r = (tile as f32 / tile_column_count as f32).floor();

                    Scene::draw_image_part(
                        renderer,
                        image,
                        (((c - start_column) as f32) * tile_map.tile_size() as f32
                            + offset_x as f32) as i32,
                        (((r - start_row) as f32) * tile_map.tile_size() as f32 + offset_y as f32)
                            as i32,
                        tile_c as u32 * tile_map.tile_size(),
                        tile_r as u32 * tile_map.tile_size(),
                        tile_map.tile_size(),
                        tile_map.tile_size(),
                    );
                }
            }
        }
    }

    pub fn draw_layer(&self, _renderer: &mut Renderer, _layer: u32) {}

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
}

impl Widget for Scene {
    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn name(&self) -> &str {
        "stage"
    }

    fn draw(&self, renderer: &mut Renderer, _focused: bool, _theme: &Theme) {
        self.draw_all_layers(renderer);
    }

    fn event(&self, _event: Event, _focused: bool, _redraw: &mut bool) -> bool {
        _focused
    }
}

impl Place for Scene {}
