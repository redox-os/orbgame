// use std::sync::Arc;
// use std::cell::RefCell;
// use std::time;

use std::cell::{Cell, RefCell};
use std::sync::Arc;

use toml;
use std::io::Read;
use std::fs::File;

use orbclient::Renderer;
use orbtk::{Event, Label, Place, Point, Rect, Text, Widget, Window};
use orbtk::theme::Theme;
use fps_counter::FPSCounter;

use super::{TileMap, Stage};

pub trait UpdatableWidget: Widget {
    fn update(&self);
}

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
    // window: Window,
    // updateable_widgets: RefCell<Vec<Arc<UpdatableWidget>>>,
    // last_tick_time: time::Instant,
    // fps_in_nanos: f32,
    // fps_counter: FPSCounter,
    // fps_label: RefCell<Option<Arc<Label>>>,
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

            // if let Some(ref label) = *self.fps_label.borrow() {
            //     label.text(format!("{}", self.fps_counter.tick()));
            // }
        }
    }



    // pub fn new(window: Window) -> Self {
    //     Game {
    //         window,
    //         updateable_widgets: RefCell::new(Vec::new()),
    //         last_tick_time: time::Instant::now(),
    //         fps_in_nanos: (1. / 60.) * 1_000_000_000.,
    //         fps_counter: FPSCounter::new(),
    //         fps_label: RefCell::new(None),
    //     }
    // }

    // pub fn elapsed(&self) -> f32 {
    //     let time = self.last_tick_time.elapsed();
    //     let total_nanos = time.as_secs() * 1_000_000_000 + time.subsec_nanos() as u64;
    //     self.fps_in_nanos - (total_nanos as f32)
    // }

    // pub fn update(&mut self) {
    //     if self.elapsed() > 0. {
    //         return
    //     }

    //     self.last_tick_time = time::Instant::now();

    //     for i in 0..self.updateable_widgets.borrow().len() {
    //         if let Some(widget) = self.updateable_widgets.borrow().get(i) {
    //             widget.update();
    //         }
    //     }
    // }

    // pub fn add<T: UpdatableWidget>(&self, widget: &Arc<T>) -> usize {
    //     let mut widgets = self.updateable_widgets.borrow_mut();
    //     let id = widgets.len();
    //     widgets.push(widget.clone());
    //     id
    // }


    // pub fn fps_label(&self, fps_label: &Arc<Label>) -> &Self {
    //     (*self.fps_label.borrow_mut()) = Some(fps_label.clone());
    //     self
    // }

    // pub fn exec(&mut self) {
    //     'event: while self.window.running.get() {
    //         self.window.drain_events();
    //         self.update();
    //         self.window.draw();
    //         self.window.drain_orbital_events();

    //         if let Some(ref label) = *self.fps_label.borrow() {
    //             label.text(format!("{}", self.fps_counter.tick()));
    //         }
    //     }
    // }
}
