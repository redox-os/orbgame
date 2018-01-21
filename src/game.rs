// use std::sync::Arc;
// use std::cell::RefCell;

// use toml;
// use std::io::Read;
use std::fs::File;
// use ron::value::Value;
use ron;
use ron::de::from_reader;

use orbtk::{Rect, Window};
// use orbtk::{Place, Rect, Window};

use super::{ScriptEngine};

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename = "Game")]
pub struct Config {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub target_fps: u32,
    pub theme: String,
}

pub struct Game {
    window: Window,
    target_fps: u32,
    script_engine: ScriptEngine,
}

impl Game {
    pub fn from_config(config: &Config) -> Game {
        Game {
            window: Window::new(Rect::new(0, 0, config.width, config.height), &config.title),
            target_fps: config.target_fps,
            script_engine: ScriptEngine::new(),
        }
    }

    fn update(&mut self) {}

    pub fn exec(&mut self) {
        'event: while self.window.running.get() {
            self.window.drain_events();
            self.update();
            self.window.draw();
            self.window.drain_orbital_events();
        }
    }
}

// #[derive(Clone, Serialize, Deserialize, Debug)]
// pub struct Config {
//     title: String,
//     stage: String,
//     target_fps: u32,
//     width: u32,
//     height: u32,
//     ui_css: String,
// }

// impl Config {
//     pub fn from_toml(path: &str) -> Self {
//         let config = {
//             // todo: handel result
//             let mut file = File::open(path).unwrap();
//             let mut buf = Vec::new();
//             file.read_to_end(&mut buf).unwrap();
//             toml::from_slice(&buf).unwrap()
//         };

//         config
//     }
// }

// pub struct Game {
//     title: String,
//     width: u32,
//     height :u32,
//     target_fps: u32,
//     config: Config,
//     stage: Arc<Stage>,
//     script_engine: ScriptEngine,
// }

// impl Game {
//     pub fn from_ron(path: &str) -> Self {
//         let value = super::load_ron_value(path);

//         Game {
//             titile: String::from("test"),
//             width: 0,
//             height: 0,

//         }
//     }

//     pub fn from_toml(path: &str) -> Self {
//         Game::from_config(Config::from_toml(path))
//     }

//     pub fn from_config(config: Config) -> Self {
//         let stage = Stage::from_toml(&config.stage[..]);
//         stage.size(config.width, config.height);

//         let script_engine = ScriptEngine::new();

//         Game {
//             config,
//             stage,
//             script_engine,
//         }
//     }

//     pub fn update(&mut self) {
//         self.script_engine.update();
//     }

//     pub fn exec(&mut self) {
//         let mut window = Window::new(
//             Rect::new(0, 0, self.config.width, self.config.height),
//             &self.config.title[..],
//         );

//         window.add(&self.stage);

//         'event: while window.running.get() {
//             window.drain_events();
//             self.update();
//             window.draw();
//             window.drain_orbital_events();
//         }
//     }
// }
