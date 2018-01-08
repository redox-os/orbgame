use std::cell::Cell;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Layer {
    tiles: Vec<Cell<i32>>,
}

impl Layer {
    pub fn new() -> Self {
        Layer { tiles: Vec::new() }
    }
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct Map {
    layer_count: usize,
    row_count: usize,
    column_count: usize,
    tile_size: u32,
    blocked_tiles: Vec<i32>,
    layers: Vec<Layer>,
}

impl Map {
    pub fn new() -> Map {
        Map {
            ..Default::default()
        }
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
