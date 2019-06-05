use orbtk::prelude::*;

widget!(
    /// The `TileMap` widget is use to draw a tile map to the screen an to navigate on the map with a camera.
    TileMap {
        /// Sets the image of the tile map.
        image: Image
    }
);

impl Template for TileMap {
    fn template(self, id: Entity, context: &mut BuildContext) -> Self {
        self.name("TileMap")
    }
}