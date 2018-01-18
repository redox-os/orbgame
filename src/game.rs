use std::sync::Arc;

use toml;
use std::io::Read;
use std::fs::File;

use orbtk::{Place, Rect, Window};

use super::Stage;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    title: String,
    stage: String,
    target_fps: u32,
    width: u32,
    height: u32,
    ui_css: String,
}

impl Config {
    pub fn from_toml(path: &str) -> Self {
        let config = {
            // todo: handel result
            let mut file = File::open(path).unwrap();
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).unwrap();
            toml::from_slice(&buf).unwrap()
        };

        config
    }
}

pub struct Game {
    config: Config,
    stage: Arc<Stage>,
}

impl Game {
    pub fn from_toml(path: &str) -> Self {
        Game::from_config(Config::from_toml(path))
    }

    pub fn from_config(config: Config) -> Self {
        let stage = Stage::from_toml(&config.stage[..]);
        stage.size(config.width, config.height);

        Game { config, stage }
    }

    pub fn update(&mut self) {}

    pub fn exec(&mut self) {
        let mut window = Window::new(
            Rect::new(0, 0, self.config.width, self.config.height),
            &self.config.title[..],
        );

        window.add(&self.stage);

        'event: while window.running.get() {
            window.drain_events();
            self.update();
            window.draw();
            window.drain_orbital_events();
        }
    }
}
