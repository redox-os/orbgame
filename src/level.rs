use std::sync::Arc;
use std::io::Read;
use std::fs::File;

use toml;

use Map;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Level {
    sheet: String,
    map: Map,
}

impl Level {
    /// Loads a new level form the given file path.
    pub fn from_path(path: &str) -> Arc<Self> {
        let level: Level = {
            // todo: handel result
            let mut file = File::open(path).unwrap();

            let mut buf = Vec::new();
            file.read_to_end(&mut buf).unwrap();
            toml::from_slice(&buf).unwrap()
        };

        Arc::new(level)
    }

    pub fn from_data(data: &[u8]) -> Arc<Self> {
        let level: Level = {
            // todo: handel result

            toml::from_slice(data).unwrap()
        };

        Arc::new(level)
    }

    pub fn sheet(&self) -> &String {
        &self.sheet
    }

    pub fn map(&self) -> &Map {
        &self.map
    }
}

impl Default for Level {
    fn default() -> Self {
        Level {
            sheet: String::from(""),
            map: Map::new(),
        }
    }
}
