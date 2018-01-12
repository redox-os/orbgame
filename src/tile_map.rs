use std::cell::{Cell, RefCell};

use toml;
use toml::Value;

use orbimage::Image;

static LAYER_COUNT_KEY: &str = "layer_count";
static ROW_COUNT_KEY: &str = "row_count";
static COLUMN_COUNT_KEY: &str = "column_count";
static TILE_SIZE_KEY: &str = "tile_size";
static BLOCKED_TILES_KEY: &str = "blocked_tiles";
static LAYERS_KEY: &str = "layers";
static TILE_SET_KEY: &str = "tile_set";
static SHEET_KEY: &str = "sheet";

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Layer {
    tiles: Vec<Cell<i32>>,
}

impl Layer {
    pub fn new() -> Self {
        Layer { tiles: Vec::new() }
    }

    pub fn push(&mut self, tile: i32) {
        self.tiles.push(Cell::new(tile));
    }
}

#[derive(Clone, Default)]
pub struct TileMap {
    layer_count: usize,
    row_count: usize,
    column_count: usize,
    tile_size: u32,
    blocked_tiles: Vec<i32>,
    layers: Vec<Layer>,
    sheet: RefCell<Option<Image>>,
}

impl TileMap {
    pub fn from_toml_value(value: &Value) -> Self {
        let tile_set = super::load_toml_value(value[TILE_SET_KEY].as_str().expect("property tile_set not found"));

        let blocked_tiles = {
            let toml = tile_set[BLOCKED_TILES_KEY].as_array().expect("property blocked_tiles not found").to_vec();
            let mut blocked_tiles: Vec<i32> = vec![];

            for t in &toml {
                blocked_tiles.push(t.as_integer().expect("cannot parse blocked tiles value") as i32);
            }

            blocked_tiles
        };

        let layers = {
            let toml = value[LAYERS_KEY].as_array().expect("property layers not found").to_vec();
            let mut layers: Vec<Layer> = vec![];

            for l in &toml {            
                let mut layer = Layer::new();

                for (_key, v) in l.as_table().expect("layers not found") {                   
                   for t in v.as_array().expect("layer not found") {
                       layer.push(t.as_integer().expect("cannot parse tile") as i32);
                   }
                }

                layers.push(layer);
            }

            layers
        };

        let mut sheet = None;
        if let Ok(image) = Image::from_path(tile_set[SHEET_KEY].as_str().expect("property sheet not found")) {
            sheet = Some(image)
        }

        TileMap {
            layer_count: value[LAYER_COUNT_KEY].as_integer().expect("property layer_count not found") as usize,
            row_count: value[ROW_COUNT_KEY].as_integer().expect("property row_count not found") as usize,
            column_count: value[COLUMN_COUNT_KEY].as_integer().expect("property column_count not found") as usize,
            tile_size: tile_set[TILE_SIZE_KEY].as_integer().expect("property tile_size not found") as u32,
            blocked_tiles,
            layers,
            sheet: RefCell::new(sheet)
        }
    }

    pub fn new() -> TileMap {
        TileMap {
            ..Default::default()
        }
    }

    pub fn sheet(&self) -> &RefCell<Option<Image>> {
        &self.sheet
    }

    pub fn layer_count(&self) -> usize {
        self.layer_count
    }

    pub fn row_count(&self) -> usize {
        self.row_count
    }

    pub fn column_count(&self) -> usize {
        self.column_count
    }

    pub fn tile_size(&self) -> u32 {
        self.tile_size
    }

    pub fn get_tile(&self, layer: usize, row: usize, column: usize) -> i32 {
        if let Some(l) = self.layers.get(layer) {
            if let Some(t) = l.tiles.get(row * self.column_count + column) {
                return t.get()
            }
        }
        -1
    }

    pub fn get_column(&self, x: f32) -> f32 {
        (x / self.tile_size as f32).floor()
    }

    pub fn get_row(&self, y: f32) -> f32 {
        (y / self.tile_size as f32).floor()
    }

    pub fn get_x(&self, column: f32) -> f32 {
        column * self.tile_size as f32
    }

    pub fn get_y(&self, row: f32) -> f32 {
        row * self.tile_size as f32
    }

    pub fn is_blocked(&self, column: usize, row: usize) -> bool {
        for l in &self.layers {
            if let Some(t) = l.tiles.get(row * self.column_count + column) {
                if self.blocked_tiles.contains(&t.get()) {
                    return true;
                }
            }
        }

        false
    }

    pub fn set_tile(&self, layer: usize, column: usize, row: usize, tile: i32) {
        if let Some(ref l) = self.layers.get(layer) {
            if let Some(ref t) = l.tiles.get(row * self.column_count + column) {
                t.set(tile);
            }
        }
    }

    pub fn is_tile_blocked(&self, x: f32, y: f32) -> bool {
        let column = (x / self.tile_size as f32).floor() as usize;
        let row = (y / self.tile_size as f32).floor() as usize;

        self.is_blocked(column, row)
    }
}
