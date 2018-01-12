// use std::sync::Arc;
// use std::cell::RefCell;
// use std::time;

use std::cell::{Cell, RefCell};
use std::sync::Arc;
use std::cmp;

use toml;
use std::io::Read;
use std::fs::File;

use orbclient::Renderer;
use orbtk::{Event, Label, Place, Point, Rect, Text, Widget, Window};
use orbtk::theme::Theme;
use orbimage::{Image, ImageRoi};
use fps_counter::FPSCounter;

use Camera;
use TileMap;

static MAP_KEY: &str = "map";

pub struct Stage {
    rect: Cell<Rect>,
    tile_map: RefCell<Option<TileMap>>,
    camera: RefCell<Camera>,
}

impl Stage {
    pub fn from_toml(path: &str) -> Arc<Self> {
        let value = super::load_toml_value(path);

        // todo handle Result of tilemap and use None for error (not found)
        Arc::new(Stage {
            tile_map: RefCell::new(Some(TileMap::from_toml_value(&value[MAP_KEY]))),
            rect: Cell::new(Rect::new(0, 0, 0, 0)),
            camera: RefCell::new(Camera::new(Rect::new(0, 0, 800, 600), Point::new(1000, 1000))),
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

        let mut tile_size = 0;
        let mut start_column = 0;
        let mut end_column = 0;
        let mut start_row = 0;
        let mut end_row = 0;
        let mut offset_x = 0.;
        let mut offset_y = 0.;

        let tile_map = self.tile_map.borrow();
        if let Some(ref tile_map) = *tile_map {
            tile_size = tile_map.tile_size();

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
            }
        }
    }

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

                    Stage::draw_image_part(
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

    pub fn draw_layer(&self, renderer: &mut Renderer, layer: u32) {}

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

impl Widget for Stage {
    fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    fn name(&self) -> &str {
        "stage"
    }

    fn draw(&self, renderer: &mut Renderer, _focused: bool, theme: &Theme) {
        self.draw_all_layers(renderer);
    }

    fn event(&self, event: Event, focused: bool, redraw: &mut bool) -> bool {
        false
    }
}

impl Place for Stage {}
