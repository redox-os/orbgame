use std::cell::{Cell, RefCell};

use orbimage::Image;

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename = "TileSet")]
pub struct TileSetConfig {
    sheet: String,
    blocked_tiles: Vec<i32>,
    tile_size: u32,
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename = "Layer")]
pub struct LayerConfig {
    tiles: Vec<i32>,
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename = "TileMap")]
pub struct TileMapConfig {
    pub layer_count: usize,
    pub row_count: usize,
    pub column_count: usize,
    pub layers: Vec<LayerConfig>,
    pub tile_set: TileSetConfig,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Layer {
    tiles: Vec<Cell<i32>>,
}

impl Layer {
    pub fn from_config(config: &LayerConfig) -> Self {
        let mut tiles = vec![];

        for tile in config.tiles.clone() {
            tiles.push(Cell::new(tile));
        }

        Layer { tiles }
    }

    pub fn push(&mut self, tile: i32) {
        self.tiles.push(Cell::new(tile));
    }
}

#[derive(Clone)]
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
    pub fn from_config(config: &TileMapConfig) -> TileMap {
        let mut layers = vec![];

        for layer in config.layers.clone() {
            layers.push(Layer::from_config(&layer));
        }

        let mut sheet = None;

        if let Ok(image) = Image::from_path(&config.tile_set.sheet) {
            sheet = Some(image)
        }

        TileMap {
            layer_count: config.layer_count,
            row_count: config.row_count,
            column_count: config.column_count,
            tile_size: config.tile_set.tile_size,
            blocked_tiles: config.tile_set.blocked_tiles.clone(),
            layers,
            sheet: RefCell::new(sheet),
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
                return t.get();
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
