use std::fs::File;

use ron;
use ron::de::from_reader;

use game::{Game, GameConfig};

pub struct GameBuilder {
    path: String,
}

impl GameBuilder {
    pub fn new(path: &str) -> Self {
        GameBuilder {
            path: String::from(path),
        }
    }

    pub fn build(&self) -> Result<Game, String> {
        if let Ok(file) = File::open(&self.path) {
            let config: ron::de::Result<GameConfig> = from_reader(file);
            if let Ok(config) = config {
                return Ok(Game::from_config(&config));
            } else {
                if let Err(err) = config {
                    return Err(String::from(format!("Could not parse game file ({}).", err)));
                }        
            }
        }

        Err(String::from("Could not load game file."))
    }
}
