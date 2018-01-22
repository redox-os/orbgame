use std::sync::Arc;
use std::time;

use fps_counter::FPSCounter;

use orbtk::{Rect, Window, WindowBuilder};

use super::{Scene, SceneConfig, ScriptEngine};

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename = "Game")]
pub struct GameConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub target_fps: u32,
    pub theme: String,
    pub scene: SceneConfig,
}

pub struct Game {
    window: Window,
    target_fps: f64,
    target_fps_nanos: f32,
    script_engine: ScriptEngine,
    scene: Arc<Scene>,
    last_tick_time: time::Instant,
    fps_counter: FPSCounter,
}

impl Game {
    pub fn from_config(config: &GameConfig) -> Game {
        // todo: load theme css
        let window_builder =
            WindowBuilder::new(Rect::new(0, 0, config.width, config.height), &config.title);
        let window = window_builder.build();
        let scene = Scene::from_config(&config.scene);
        window.add(&scene);

        Game {
            window,
            target_fps: config.target_fps as f64,
            target_fps_nanos: (1. / config.target_fps as f32) * 1_000_000_000.,
            script_engine: ScriptEngine::new(),
            scene: scene,
            last_tick_time: time::Instant::now(),
            fps_counter: FPSCounter::new(),
        }
    }

    pub fn elapsed(&self) -> f32 {
        let time = self.last_tick_time.elapsed();
        let total_nanos = time.as_secs() * 1_000_000_000 + time.subsec_nanos() as u64;
        self.target_fps_nanos - (total_nanos as f32)
    }

    fn update(&mut self) {
        if self.elapsed() > 0. {
            return;
        }

        let delta = 1.0 / self.target_fps;

        self.scene.update(&mut self.script_engine, delta);
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
