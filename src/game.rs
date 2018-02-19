use std::sync::Arc;
use std::cell::RefCell;
use std::time;

//use fps_counter::FPSCounter;

use orbclient::WindowFlag;

use orbtk::{Rect, Window, WindowBuilder, Widget };
use orbtk::theme::Theme;

use super::{Scene, SceneConfig, ScriptEngine};

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename = "Game")]
pub struct GameConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub target_fps: u32,
    pub theme: String,
    pub scenes: Vec<SceneConfig>,
}

pub struct Game {
    window: Window,
    target_fps: f64,
    target_fps_nanos: f32,
    script_engine: ScriptEngine,
    scenes: Vec<Arc<Scene>>,
    active_scene: RefCell<Arc<Scene>>,
    last_tick_time: time::Instant,
    //fps_counter: FPSCounter,
}

impl Game {
    pub fn from_config(config: &GameConfig) -> Game {
        let theme = Theme::from_path(&config.theme).unwrap();

        let mut window_builder =
            WindowBuilder::new(Rect::new(0, 0, config.width, config.height), &config.title);
        window_builder = window_builder.flags(&[WindowFlag::Async]);
        window_builder = window_builder.theme(theme);
        let window = window_builder.build();

        let mut scenes = vec![];

        for scene in &config.scenes {
            scenes.push(Scene::from_config(scene));
        }

        let active_scene = scenes[0].clone();

        // add first scene to window
        window.add(&active_scene);

        Game {
            window,
            target_fps: config.target_fps as f64,
            target_fps_nanos: (1. / config.target_fps as f32) * 1_000_000_000.,
            script_engine: ScriptEngine::new(),
            scenes,
            active_scene: RefCell::new(active_scene),
            last_tick_time: time::Instant::now(),
            //fps_counter: FPSCounter::new(),
        }
    }

    pub fn elapsed(&self) -> f32 {
        let time = self.last_tick_time.elapsed();
        let total_nanos = time.as_secs() * 1_000_000_000 + time.subsec_nanos() as u64;
        self.target_fps_nanos - (total_nanos as f32)
    }

    pub fn show_scene(&self, id: &str) {
        if let Some(scene) = self.scenes.iter().find(|s| s.id() == id) {
            self.remove_scene_from_window(scene.clone());

            *self.active_scene.borrow_mut() = scene.clone();
            self.window.add(&*self.active_scene.borrow());
        }
    }

    fn remove_scene_from_window(&self, scene: Arc<Widget>) {
        if let Some(position) = (self.window.widgets.borrow()).iter().position(|a| Arc::ptr_eq(a, &scene)) {
                (*self.window.widgets.borrow_mut()).remove(position);
            }     
    }

    fn update(&mut self) {
        if self.elapsed() > 0. {
            return;
        }

        let delta = 1.0 / self.target_fps;

        self.active_scene.borrow().update(&mut self.script_engine, delta);
        self.last_tick_time = time::Instant::now();
    }

    pub fn exec(&mut self) {
        'event: while self.window.running.get() {
            self.window.drain_events();
            self.update();
            self.window.draw();
            self.window.drain_orbital_events();
        }
    }
}
